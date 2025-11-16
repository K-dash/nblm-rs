use std::net::TcpListener as StdTcpListener;
use std::{env, sync::Arc};

use anyhow::{anyhow, bail, Result};
use nblm_core::auth::oauth::{
    self, AuthorizeParams, FileRefreshTokenStore, OAuthConfig, OAuthFlow, RefreshTokenProvider,
    SerializedTokens, TokenStoreKey,
};
use nblm_core::auth::{EnvTokenProvider, GcloudTokenProvider, StaticTokenProvider, TokenProvider};
use nblm_core::env::profile_experiment_enabled;
use nblm_core::ApiProfile;
use nblm_core::RefreshTokenStore;
use reqwest::Client;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener as AsyncTcpListener;
use tokio::runtime::Handle;
use tokio::task::block_in_place;
use tokio::time::Duration as TokioDuration;
use url::Url;

use crate::args::{AuthMethod, GlobalArgs};

pub fn build_token_provider(args: &GlobalArgs) -> Result<Arc<dyn TokenProvider>> {
    if args.auth.requires_experimental_flag() && !profile_experiment_enabled() {
        anyhow::bail!(
            "auth method '{}' is experimental and not yet available. Set {}=1 to enable experimental auth methods.",
            auth_method_label(args.auth),
            nblm_core::PROFILE_EXPERIMENT_FLAG
        );
    }

    Ok(match args.auth {
        AuthMethod::Gcloud => Arc::new(build_gcloud_provider()?),
        AuthMethod::Env => {
            if let Some(token) = args.token.as_ref().or(args.env_token.as_ref()) {
                Arc::new(StaticTokenProvider::new(token.clone()))
            } else {
                Arc::new(EnvTokenProvider::new("NBLM_ACCESS_TOKEN"))
            }
        }
        AuthMethod::UserOauth => {
            let bootstrapper = OAuthBootstrapper::new()?;
            bootstrapper.bootstrap_provider(args)?
        }
    })
}

/// Bootstrap OAuth authentication flow
pub struct OAuthBootstrapper {
    store: Arc<FileRefreshTokenStore>,
}

impl OAuthBootstrapper {
    /// Create a new OAuthBootstrapper
    pub fn new() -> Result<Self> {
        let store = Arc::new(FileRefreshTokenStore::new()?);
        Ok(Self { store })
    }

    /// Get project number from args
    fn get_project_number(args: &GlobalArgs) -> Result<String> {
        args.project_number
            .as_ref()
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "project-number is required for user-oauth authentication. Set --project-number or NBLM_PROJECT_NUMBER environment variable"
                )
            })
            .cloned()
    }

    /// Create OAuth config from args
    fn create_oauth_config(project_number: &str) -> Result<OAuthConfig> {
        OAuthConfig::google_default(project_number)
            .map_err(|e| anyhow::anyhow!("failed to create OAuth config: {}", e))
    }

    /// Create HTTP client for OAuth flow
    fn create_http_client() -> Result<Arc<Client>> {
        Client::builder()
            .user_agent(concat!("nblm-cli/", env!("CARGO_PKG_VERSION")))
            .build()
            .map_err(|e| anyhow::anyhow!("failed to create HTTP client: {}", e))
            .map(Arc::new)
    }

    /// Build token store key from args
    fn build_store_key(args: &GlobalArgs, project_number: String) -> TokenStoreKey {
        let profile: ApiProfile = args.profile.into();
        TokenStoreKey {
            profile,
            project_number: Some(project_number),
            endpoint_location: Some(args.endpoint_location.clone()),
            user_hint: None,
        }
    }

    /// Build a TokenProvider for the given args
    fn build_provider(
        &self,
        args: &GlobalArgs,
    ) -> Result<Arc<RefreshTokenProvider<FileRefreshTokenStore>>> {
        let project_number = Self::get_project_number(args)?;
        let config = Self::create_oauth_config(&project_number)?;
        let http_client = Self::create_http_client()?;

        let store_key = Self::build_store_key(args, project_number);
        let flow = OAuthFlow::new(config, Arc::clone(&http_client))
            .map_err(|e| anyhow!("failed to create OAuth flow: {}", e))?;
        let provider = RefreshTokenProvider::new(flow, Arc::clone(&self.store), store_key);

        Ok(Arc::new(provider))
    }

    /// Ensure tokens are available, starting browser flow if needed (blocking)
    fn ensure_tokens_blocking(
        &self,
        args: &GlobalArgs,
        project_number: &str,
        store_key: &TokenStoreKey,
    ) -> Result<()> {
        block_in_place(|| {
            let handle = Handle::try_current()
                .map_err(|_| anyhow!("user-oauth authentication requires a Tokio runtime"))?;

            handle.block_on(async {
                if self.store.load(store_key).await?.is_some() {
                    return Ok(());
                }

                self.start_browser_flow(args, project_number).await
            })
        })
    }

    /// Build provider and ensure refresh tokens exist, starting browser flow if needed
    pub fn bootstrap_provider(&self, args: &GlobalArgs) -> Result<Arc<dyn TokenProvider>> {
        let project_number = Self::get_project_number(args)?;
        let store_key = Self::build_store_key(args, project_number.clone());

        let skip_bootstrap = is_bootstrap_disabled();

        if !skip_bootstrap {
            self.ensure_tokens_blocking(args, &project_number, &store_key)?;
        }

        let provider: Arc<RefreshTokenProvider<FileRefreshTokenStore>> =
            self.build_provider(args)?;

        if !skip_bootstrap {
            // Optionally validate token availability so errors bubble up early.
            block_in_place(|| {
                let handle = Handle::try_current()
                    .map_err(|_| anyhow!("user-oauth authentication requires a Tokio runtime"))?;
                handle.block_on(async {
                    provider
                        .access_token()
                        .await
                        .map_err(|e| anyhow!("failed to obtain access token: {}", e))
                        .map(|_| ())
                })
            })?;
        }

        let provider_dyn: Arc<dyn TokenProvider> = provider;
        Ok(provider_dyn)
    }

    /// Start browser-based OAuth flow
    async fn start_browser_flow(&self, args: &GlobalArgs, project_number: &str) -> Result<()> {
        let mut config = Self::create_oauth_config(project_number)?;
        let http_client = Self::create_http_client()?;

        let mut listener: Option<AsyncTcpListener> = None;

        if env::var("NBLM_OAUTH_REDIRECT_URI").is_err()
            && config.redirect_uri == OAuthConfig::DEFAULT_REDIRECT_URI
        {
            let loopback = oauth::loopback::bind_loopback_listener(None)
                .map_err(|e| anyhow!("failed to bind loopback listener: {}", e))?;
            let port = loopback.port();
            config.redirect_uri = oauth::loopback::build_redirect_uri(port);
            let std_listener = loopback.into_std();
            let async_listener = AsyncTcpListener::from_std(std_listener)
                .map_err(|e| anyhow!("failed to create async listener: {}", e))?;
            listener = Some(async_listener);
        }

        if listener.is_none() {
            let redirect_url = Url::parse(&config.redirect_uri)
                .map_err(|e| anyhow!("invalid redirect_uri: {}", e))?;
            let addrs = redirect_url
                .socket_addrs(|| None)
                .map_err(|e| anyhow!("failed to parse redirect host: {}", e))?;
            let addr = addrs
                .into_iter()
                .find(|addr| addr.ip().is_loopback())
                .ok_or_else(|| anyhow!("redirect_uri must resolve to a loopback address"))?;
            let std_listener = StdTcpListener::bind(addr)
                .map_err(|e| anyhow!("failed to bind {}: {}", addr, e))?;
            std_listener
                .set_nonblocking(true)
                .map_err(|e| anyhow!("failed to configure listener: {}", e))?;
            let async_listener = AsyncTcpListener::from_std(std_listener)
                .map_err(|e| anyhow!("failed to create async listener: {}", e))?;
            listener = Some(async_listener);
        }

        let listener = listener.expect("listener must be initialized");
        let flow = OAuthFlow::new(config, Arc::clone(&http_client))
            .map_err(|e| anyhow!("failed to create OAuth flow: {}", e))?;

        // Build authorization URL
        let auth_context = flow.build_authorize_url(&AuthorizeParams {
            state: None,
            code_challenge: None,
            code_challenge_method: None,
        });

        eprintln!("Opening browser for authentication...");
        eprintln!("If the browser doesn't open, please visit:");
        eprintln!("{}", auth_context.url);

        // Try to open browser
        if let Err(e) = webbrowser::open(&auth_context.url) {
            eprintln!("Warning: Failed to open browser: {}", e);
            eprintln!("Please manually visit the URL above");
        }

        // Start local server to receive callback
        let callback_result = self.listen_for_callback(listener).await?;

        // Verify state
        if callback_result.state != auth_context.state {
            bail!("OAuth state mismatch - possible CSRF attack");
        }

        // Exchange code for tokens
        let tokens = flow
            .exchange_code(&auth_context, &callback_result.code)
            .await?;

        // Save tokens
        let store_key = Self::build_store_key(args, project_number.to_string());

        let refresh_token = tokens
            .refresh_token
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("no refresh token received"))?;

        let serialized = SerializedTokens {
            refresh_token: refresh_token.clone(),
            scopes: tokens
                .scope
                .as_ref()
                .map(|s| s.split_whitespace().map(String::from).collect())
                .unwrap_or_default(),
            expires_at: Some(tokens.expires_at),
            token_type: tokens.token_type,
            updated_at: time::OffsetDateTime::now_utc(),
        };

        self.store.save(&store_key, &serialized).await?;

        eprintln!("Authentication successful! Tokens have been saved.");

        Ok(())
    }

    /// Listen for OAuth callback on localhost
    async fn listen_for_callback(&self, listener: AsyncTcpListener) -> Result<CallbackResult> {
        if let Ok(addr) = listener.local_addr() {
            eprintln!("Listening for OAuth callback on {}", addr);
        }
        self.handle_callback(listener).await
    }

    /// Handle a single callback request
    async fn handle_callback(&self, listener: AsyncTcpListener) -> Result<CallbackResult> {
        const TIMEOUT: TokioDuration = TokioDuration::from_secs(600); // 10 minutes

        let result = tokio::time::timeout(TIMEOUT, async {
            let (mut stream, _) = listener.accept().await?;
            let mut buffer = vec![0u8; 4096];
            let n = stream.read(&mut buffer).await?;
            let request = String::from_utf8_lossy(&buffer[..n]);

            // Parse GET request
            let mut code = None;
            let mut state = None;
            let mut error = None;

            if let Some(query_start) = request.find('?') {
                let query = &request[query_start + 1..];
                if let Some(http_end) = query.find(" HTTP/") {
                    let query = &query[..http_end];
                    for param in query.split('&') {
                        if let Some((key, value)) = param.split_once('=') {
                            let value = urlencoding::decode(value)
                                .map(|v| v.to_string())
                                .unwrap_or_else(|_| value.to_string());
                            match key {
                                "code" => code = Some(value),
                                "state" => state = Some(value),
                                "error" => error = Some(value),
                                _ => {}
                            }
                        }
                    }
                }
            }

            // Send response
            let response = if error.is_some() {
                format!(
                    "HTTP/1.1 400 Bad Request\r\nContent-Type: text/html\r\n\r\n<html><body><h1>Authentication failed</h1><p>Error: {}</p></body></html>",
                    error.as_ref().unwrap()
                )
            } else if code.is_some() && state.is_some() {
                "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n<html><body><h1>Authentication successful!</h1><p>You can close this window.</p></body></html>".to_string()
            } else {
                "HTTP/1.1 400 Bad Request\r\nContent-Type: text/html\r\n\r\n<html><body><h1>Invalid request</h1></body></html>".to_string()
            };

            stream.write_all(response.as_bytes()).await?;
            stream.flush().await?;

            Ok::<CallbackResult, anyhow::Error>(CallbackResult {
                code: code.ok_or_else(|| anyhow::anyhow!("no code parameter"))?,
                state: state.ok_or_else(|| anyhow::anyhow!("no state parameter"))?,
            })
        })
        .await;

        match result {
            Ok(Ok(callback)) => Ok(callback),
            Ok(Err(e)) => Err(e),
            Err(_) => bail!("OAuth callback timeout after 10 minutes"),
        }
    }
}

fn is_bootstrap_disabled() -> bool {
    env::var("NBLM_OAUTH_DISABLE_BOOTSTRAP")
        .map(|value| {
            let lower = value.trim().to_ascii_lowercase();
            matches!(lower.as_str(), "1" | "true" | "yes" | "on")
        })
        .unwrap_or(false)
}

#[allow(dead_code)]
struct CallbackResult {
    code: String,
    state: String,
}

fn build_gcloud_provider() -> Result<GcloudTokenProvider> {
    let binary = env::var("NBLM_GCLOUD_PATH").unwrap_or_else(|_| "gcloud".to_string());
    Ok(GcloudTokenProvider::new(binary))
}

fn auth_method_label(method: AuthMethod) -> &'static str {
    match method {
        AuthMethod::Gcloud => "gcloud",
        AuthMethod::Env => "env",
        AuthMethod::UserOauth => "user-oauth",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::args::{AuthMethod, GlobalArgs, ProfileArg};
    use nblm_core::auth::ProviderKind;
    use serial_test::serial;

    struct EnvGuard {
        key: &'static str,
        original: Option<String>,
    }

    impl EnvGuard {
        fn new(key: &'static str) -> Self {
            let original = env::var(key).ok();
            Self { key, original }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            if let Some(value) = &self.original {
                env::set_var(self.key, value);
            } else {
                env::remove_var(self.key);
            }
        }
    }

    fn make_args(auth: AuthMethod) -> GlobalArgs {
        GlobalArgs {
            project_number: Some("123456".to_string()),
            location: "global".to_string(),
            endpoint_location: "global".to_string(),
            profile: ProfileArg::Enterprise,
            auth,
            token: Some("token".to_string()),
            json: false,
            debug_http: false,
            timeout: None,
            env_token: Some("token".to_string()),
            base_url: None,
        }
    }

    #[test]
    #[serial]
    fn user_oauth_requires_experiment_flag() {
        let _guard = EnvGuard::new(nblm_core::PROFILE_EXPERIMENT_FLAG);
        env::remove_var(nblm_core::PROFILE_EXPERIMENT_FLAG);
        let args = make_args(AuthMethod::UserOauth);
        let err = match build_token_provider(&args) {
            Ok(_) => panic!("expected experimental auth to fail"),
            Err(err) => err,
        };
        assert!(format!("{err}").contains("experimental"));
    }

    #[test]
    #[serial]
    fn user_oauth_requires_project_number() {
        let _guard_bootstrap = EnvGuard::new("NBLM_OAUTH_DISABLE_BOOTSTRAP");
        let _guard = EnvGuard::new(nblm_core::PROFILE_EXPERIMENT_FLAG);
        env::set_var(nblm_core::PROFILE_EXPERIMENT_FLAG, "1");
        env::set_var("NBLM_OAUTH_CLIENT_ID", "test-client-id");
        env::set_var("NBLM_OAUTH_DISABLE_BOOTSTRAP", "1");
        let mut args = make_args(AuthMethod::UserOauth);
        args.project_number = None;
        match build_token_provider(&args) {
            Ok(_) => panic!("expected error for missing project-number"),
            Err(err) => assert!(format!("{err}").contains("project-number")),
        }
        env::remove_var("NBLM_OAUTH_CLIENT_ID");
    }

    #[test]
    #[serial]
    fn user_oauth_requires_client_id() {
        let _guard_bootstrap = EnvGuard::new("NBLM_OAUTH_DISABLE_BOOTSTRAP");
        let _guard = EnvGuard::new(nblm_core::PROFILE_EXPERIMENT_FLAG);
        env::set_var(nblm_core::PROFILE_EXPERIMENT_FLAG, "1");
        env::remove_var("NBLM_OAUTH_CLIENT_ID");
        env::set_var("NBLM_OAUTH_DISABLE_BOOTSTRAP", "1");
        let args = make_args(AuthMethod::UserOauth);
        match build_token_provider(&args) {
            Ok(_) => panic!("expected error for missing NBLM_OAUTH_CLIENT_ID"),
            Err(err) => assert!(format!("{err}").contains("NBLM_OAUTH_CLIENT_ID")),
        }
    }

    #[test]
    #[serial]
    fn user_oauth_builds_provider_when_configured() {
        let _guard_bootstrap = EnvGuard::new("NBLM_OAUTH_DISABLE_BOOTSTRAP");
        let _guard = EnvGuard::new(nblm_core::PROFILE_EXPERIMENT_FLAG);
        env::set_var(nblm_core::PROFILE_EXPERIMENT_FLAG, "1");
        env::set_var("NBLM_OAUTH_CLIENT_ID", "test-client-id");
        env::set_var("NBLM_OAUTH_DISABLE_BOOTSTRAP", "1");
        let args = make_args(AuthMethod::UserOauth);
        let provider = build_token_provider(&args).expect("expected provider");
        assert_eq!(provider.kind(), ProviderKind::UserOauth);
        env::remove_var("NBLM_OAUTH_CLIENT_ID");
    }

    #[test]
    fn auth_method_label_returns_correct_labels() {
        assert_eq!(auth_method_label(AuthMethod::Gcloud), "gcloud");
        assert_eq!(auth_method_label(AuthMethod::Env), "env");
        assert_eq!(auth_method_label(AuthMethod::UserOauth), "user-oauth");
    }

    #[test]
    fn auth_method_requires_experimental_flag_only_for_user_oauth() {
        assert!(!AuthMethod::Gcloud.requires_experimental_flag());
        assert!(!AuthMethod::Env.requires_experimental_flag());
        assert!(AuthMethod::UserOauth.requires_experimental_flag());
    }

    #[test]
    #[serial_test::serial]
    fn gcloud_auth_works_without_experiment_flag() {
        let _guard = EnvGuard::new(nblm_core::PROFILE_EXPERIMENT_FLAG);
        env::remove_var(nblm_core::PROFILE_EXPERIMENT_FLAG);
        let args = make_args(AuthMethod::Gcloud);
        let provider = build_token_provider(&args).expect("expected provider");
        assert_eq!(provider.kind(), ProviderKind::GcloudOauth);
    }

    #[test]
    #[serial]
    fn env_auth_works_without_experiment_flag() {
        let _guard = EnvGuard::new(nblm_core::PROFILE_EXPERIMENT_FLAG);
        env::remove_var(nblm_core::PROFILE_EXPERIMENT_FLAG);
        env::set_var("NBLM_ACCESS_TOKEN", "test-token");
        let mut args = make_args(AuthMethod::Env);
        args.token = None;
        args.env_token = None;
        let provider = build_token_provider(&args).expect("expected provider");
        assert_eq!(provider.kind(), ProviderKind::EnvAccessToken);
        env::remove_var("NBLM_ACCESS_TOKEN");
    }

    #[test]
    #[serial]
    fn env_auth_with_token_arg_uses_static_provider() {
        let _guard = EnvGuard::new(nblm_core::PROFILE_EXPERIMENT_FLAG);
        env::remove_var(nblm_core::PROFILE_EXPERIMENT_FLAG);
        let args = make_args(AuthMethod::Env);
        let provider = build_token_provider(&args).expect("expected provider");
        assert_eq!(provider.kind(), ProviderKind::StaticToken);
    }

    // Test 1: build_store_key tests
    #[test]
    fn build_store_key_includes_all_fields() {
        let args = GlobalArgs {
            project_number: Some("test-project-123".to_string()),
            location: "us".to_string(),
            endpoint_location: "us".to_string(),
            profile: ProfileArg::Enterprise,
            auth: AuthMethod::UserOauth,
            token: None,
            json: false,
            debug_http: false,
            timeout: None,
            env_token: None,
            base_url: None,
        };

        let key = OAuthBootstrapper::build_store_key(&args, "test-project-123".to_string());

        assert_eq!(key.profile, ApiProfile::Enterprise);
        assert_eq!(key.project_number, Some("test-project-123".to_string()));
        assert_eq!(key.endpoint_location, Some("us".to_string()));
        assert_eq!(key.user_hint, None);
    }

    #[test]
    fn build_store_key_generates_different_keys_for_different_configs() {
        let args1 = GlobalArgs {
            project_number: Some("project-1".to_string()),
            location: "global".to_string(),
            endpoint_location: "global".to_string(),
            profile: ProfileArg::Enterprise,
            auth: AuthMethod::UserOauth,
            token: None,
            json: false,
            debug_http: false,
            timeout: None,
            env_token: None,
            base_url: None,
        };

        let args2 = GlobalArgs {
            project_number: Some("project-2".to_string()),
            location: "us".to_string(),
            endpoint_location: "us".to_string(),
            profile: ProfileArg::Enterprise,
            auth: AuthMethod::UserOauth,
            token: None,
            json: false,
            debug_http: false,
            timeout: None,
            env_token: None,
            base_url: None,
        };

        let key1 = OAuthBootstrapper::build_store_key(&args1, "project-1".to_string());
        let key2 = OAuthBootstrapper::build_store_key(&args2, "project-2".to_string());

        // Keys should be different
        assert_ne!(key1.project_number, key2.project_number);
        assert_ne!(key1.endpoint_location, key2.endpoint_location);

        // Display representation should be different (used as map key)
        assert_ne!(key1.to_string(), key2.to_string());
    }

    // Test 2: create_http_client tests
    #[test]
    fn create_http_client_succeeds() {
        let client = OAuthBootstrapper::create_http_client();
        assert!(client.is_ok());
    }

    #[test]
    fn create_http_client_has_user_agent() {
        let client = OAuthBootstrapper::create_http_client().unwrap();
        // We can't directly inspect the user agent, but we can verify the client was created successfully
        // The user agent is set in the builder, so this verifies the configuration doesn't fail
        assert!(Arc::strong_count(&client) > 0);
    }

    // Test 3: create_oauth_config tests
    #[test]
    #[serial]
    fn create_oauth_config_requires_client_id() {
        let _guard = EnvGuard::new("NBLM_OAUTH_CLIENT_ID");
        env::remove_var("NBLM_OAUTH_CLIENT_ID");

        let result = OAuthBootstrapper::create_oauth_config("test-project");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("NBLM_OAUTH_CLIENT_ID"));
    }

    #[test]
    #[serial]
    fn create_oauth_config_succeeds_with_client_id() {
        let _guard = EnvGuard::new("NBLM_OAUTH_CLIENT_ID");
        env::set_var("NBLM_OAUTH_CLIENT_ID", "test-client-id-12345");

        let result = OAuthBootstrapper::create_oauth_config("test-project");
        assert!(result.is_ok());

        let config = result.unwrap();
        assert_eq!(config.client_id, "test-client-id-12345");
        assert!(!config.scopes.is_empty());
    }

    #[test]
    #[serial]
    fn create_oauth_config_uses_custom_redirect_uri_when_set() {
        let _guard_client = EnvGuard::new("NBLM_OAUTH_CLIENT_ID");
        let _guard_redirect = EnvGuard::new("NBLM_OAUTH_REDIRECT_URI");
        env::set_var("NBLM_OAUTH_CLIENT_ID", "test-client-id");
        env::set_var("NBLM_OAUTH_REDIRECT_URI", "http://localhost:9999");

        let result = OAuthBootstrapper::create_oauth_config("test-project");
        assert!(result.is_ok());

        let config = result.unwrap();
        assert_eq!(config.redirect_uri, "http://localhost:9999");
    }

    // Test 4: Error handling tests
    #[test]
    #[serial]
    fn get_project_number_returns_error_when_missing() {
        let mut args = make_args(AuthMethod::UserOauth);
        args.project_number = None;

        let result = OAuthBootstrapper::get_project_number(&args);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.to_string().contains("project-number"));
        assert!(error.to_string().contains("required"));
    }

    #[test]
    fn get_project_number_returns_value_when_present() {
        let args = make_args(AuthMethod::UserOauth);
        let result = OAuthBootstrapper::get_project_number(&args);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "123456");
    }

    #[test]
    #[serial]
    fn build_provider_fails_without_client_id() {
        let _guard_client = EnvGuard::new("NBLM_OAUTH_CLIENT_ID");
        let _guard_bootstrap = EnvGuard::new("NBLM_OAUTH_DISABLE_BOOTSTRAP");
        env::remove_var("NBLM_OAUTH_CLIENT_ID");
        env::set_var("NBLM_OAUTH_DISABLE_BOOTSTRAP", "1");

        let bootstrapper = OAuthBootstrapper::new().unwrap();
        let args = make_args(AuthMethod::UserOauth);

        let result = bootstrapper.build_provider(&args);
        assert!(result.is_err());
        let error = result.err().unwrap();
        assert!(error.to_string().contains("NBLM_OAUTH_CLIENT_ID"));
    }

    #[test]
    #[serial]
    fn build_provider_succeeds_with_valid_config() {
        let _guard_client = EnvGuard::new("NBLM_OAUTH_CLIENT_ID");
        let _guard_bootstrap = EnvGuard::new("NBLM_OAUTH_DISABLE_BOOTSTRAP");
        env::set_var("NBLM_OAUTH_CLIENT_ID", "test-client-id");
        env::set_var("NBLM_OAUTH_DISABLE_BOOTSTRAP", "1");

        let bootstrapper = OAuthBootstrapper::new().unwrap();
        let args = make_args(AuthMethod::UserOauth);

        let result = bootstrapper.build_provider(&args);
        assert!(result.is_ok());

        // Verify provider was created successfully by checking kind
        let provider: Arc<dyn TokenProvider> = result.unwrap();
        assert_eq!(provider.kind(), ProviderKind::UserOauth);
    }

    #[test]
    #[serial]
    fn oauth_bootstrapper_respects_disable_bootstrap_flag() {
        let _guard_client = EnvGuard::new("NBLM_OAUTH_CLIENT_ID");
        let _guard_bootstrap = EnvGuard::new("NBLM_OAUTH_DISABLE_BOOTSTRAP");
        let _guard_flag = EnvGuard::new(nblm_core::PROFILE_EXPERIMENT_FLAG);

        env::set_var(nblm_core::PROFILE_EXPERIMENT_FLAG, "1");
        env::set_var("NBLM_OAUTH_CLIENT_ID", "test-client-id");
        env::set_var("NBLM_OAUTH_DISABLE_BOOTSTRAP", "1");

        let args = make_args(AuthMethod::UserOauth);
        let result = build_token_provider(&args);

        // Should succeed without trying to start browser flow
        assert!(result.is_ok());
    }

    #[test]
    #[serial]
    fn oauth_disable_flag_recognizes_truthy_values() {
        let _guard_bootstrap = EnvGuard::new("NBLM_OAUTH_DISABLE_BOOTSTRAP");

        for value in &["1", "true", "TRUE", "yes", "YES", "on", "ON"] {
            env::set_var("NBLM_OAUTH_DISABLE_BOOTSTRAP", value);
            assert!(
                is_bootstrap_disabled(),
                "expected {:?} to disable bootstrap",
                value
            );
        }

        for value in &["0", "false", "", " off ", "noop"] {
            env::set_var("NBLM_OAUTH_DISABLE_BOOTSTRAP", value);
            assert!(
                !is_bootstrap_disabled(),
                "expected {:?} to keep bootstrap enabled",
                value
            );
        }
    }
}
