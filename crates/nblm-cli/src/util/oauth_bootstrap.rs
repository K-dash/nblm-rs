use std::sync::Arc;

use anyhow::{anyhow, Result};
use reqwest::Client;
use time::OffsetDateTime;
use tokio::runtime::Handle;
use tokio::task::block_in_place;

use crate::args::GlobalArgs;
use crate::util::oauth_browser::OAuthBrowserFlow;

use nblm_core::auth::oauth::{
    FileRefreshTokenStore, OAuthClientConfig, OAuthConfig, OAuthFlow, RefreshTokenProvider,
    SerializedTokens, TokenStoreKey,
};
use nblm_core::auth::TokenProvider;
use nblm_core::RefreshTokenStore;

pub struct OAuthBootstrapper {
    store: Arc<FileRefreshTokenStore>,
}

impl OAuthBootstrapper {
    pub fn new() -> Result<Self> {
        let store = Arc::new(FileRefreshTokenStore::new()?);
        Ok(Self { store })
    }

    fn get_project_number(args: &GlobalArgs) -> Result<String> {
        args.project_number
            .as_ref()
            .ok_or_else(|| {
                anyhow!(
                    "project-number is required for user-oauth authentication. Set --project-number or NBLM_PROJECT_NUMBER environment variable"
                )
            })
            .cloned()
    }

    fn build_store_key(args: &GlobalArgs, project_number: String) -> TokenStoreKey {
        TokenStoreKey {
            profile: args.profile.into(),
            project_number: Some(project_number),
            endpoint_location: Some(args.endpoint_location.clone()),
            user_hint: None,
        }
    }

    fn create_http_client() -> Result<Arc<Client>> {
        Client::builder()
            .user_agent(concat!("nblm-cli/", env!("CARGO_PKG_VERSION")))
            .build()
            .map(Arc::new)
            .map_err(|e| anyhow!("failed to create HTTP client: {}", e))
    }

    pub fn bootstrap_provider(&self, args: &GlobalArgs) -> Result<Arc<dyn TokenProvider>> {
        let project_number = Self::get_project_number(args)?;
        let store_key = Self::build_store_key(args, project_number.clone());
        let client_config = OAuthClientConfig::from_env().map_err(|e| {
            anyhow!(
                "OAuth configuration error: {}\nSee guide: https://github.com/K-dash/nblm-rs/blob/main/docs/guides/oauth2-authentication.md",
                e
            )
        })?;
        let oauth_config = client_config.into_oauth_config();
        let http_client = Self::create_http_client()?;

        let skip_bootstrap = is_bootstrap_disabled();
        if !skip_bootstrap {
            self.ensure_tokens_blocking(
                &oauth_config,
                Arc::clone(&http_client),
                &project_number,
                &store_key,
            )?;
        }

        let flow = OAuthFlow::new(oauth_config, Arc::clone(&http_client))
            .map_err(|e| anyhow!("failed to create OAuth flow: {}", e))?;
        let provider: Arc<RefreshTokenProvider<FileRefreshTokenStore>> = Arc::new(
            RefreshTokenProvider::new(flow, Arc::clone(&self.store), store_key),
        );

        if !skip_bootstrap {
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

        Ok(provider as Arc<dyn TokenProvider>)
    }

    fn ensure_tokens_blocking(
        &self,
        config: &OAuthConfig,
        http_client: Arc<Client>,
        project_number: &str,
        store_key: &TokenStoreKey,
    ) -> Result<()> {
        block_in_place(|| {
            let handle = Handle::try_current()
                .map_err(|_| anyhow!("user-oauth authentication requires a Tokio runtime"))?;
            let config = config.clone();
            handle.block_on(async {
                if self.store.load(store_key).await?.is_some() {
                    return Ok(());
                }
                self.run_browser_flow(config, http_client, project_number.to_string(), store_key)
                    .await
            })
        })
    }

    async fn run_browser_flow(
        &self,
        config: OAuthConfig,
        http_client: Arc<Client>,
        project_number: String,
        store_key: &TokenStoreKey,
    ) -> Result<()> {
        let browser_flow = OAuthBrowserFlow::new(config, http_client);
        let tokens = browser_flow.run().await?;
        let refresh_token = tokens
            .refresh_token
            .as_ref()
            .ok_or_else(|| anyhow!("no refresh token received"))?;

        let serialized = SerializedTokens {
            refresh_token: refresh_token.clone(),
            scopes: tokens
                .scope
                .as_ref()
                .map(|s| s.split_whitespace().map(String::from).collect())
                .unwrap_or_default(),
            expires_at: Some(tokens.expires_at),
            token_type: tokens.token_type,
            updated_at: OffsetDateTime::now_utc(),
        };

        self.store.save(store_key, &serialized).await?;

        eprintln!(
            "Authentication successful! Tokens have been saved for project {}.",
            project_number
        );
        Ok(())
    }
}

pub(crate) fn is_bootstrap_disabled() -> bool {
    std::env::var("NBLM_OAUTH_DISABLE_BOOTSTRAP")
        .map(|value| {
            let lower = value.trim().to_ascii_lowercase();
            matches!(lower.as_str(), "1" | "true" | "yes" | "on")
        })
        .unwrap_or(false)
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
            Self {
                key,
                original: std::env::var(key).ok(),
            }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            if let Some(value) = &self.original {
                std::env::set_var(self.key, value);
            } else {
                std::env::remove_var(self.key);
            }
        }
    }

    fn make_args() -> GlobalArgs {
        GlobalArgs {
            project_number: Some("123456".to_string()),
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
        }
    }

    #[test]
    fn disable_flag_recognizes_truthy_values() {
        let _guard = EnvGuard::new("NBLM_OAUTH_DISABLE_BOOTSTRAP");
        for value in ["1", "true", "TRUE", "yes", "YES", "on", "ON"] {
            std::env::set_var("NBLM_OAUTH_DISABLE_BOOTSTRAP", value);
            assert!(
                is_bootstrap_disabled(),
                "{} should disable bootstrap",
                value
            );
        }
        for value in ["0", "false", "", " off ", "noop"] {
            std::env::set_var("NBLM_OAUTH_DISABLE_BOOTSTRAP", value);
            assert!(
                !is_bootstrap_disabled(),
                "{} should keep bootstrap enabled",
                value
            );
        }
    }

    #[test]
    #[serial]
    fn build_provider_requires_client_id() {
        let _guard_id = EnvGuard::new("NBLM_OAUTH_CLIENT_ID");
        std::env::remove_var("NBLM_OAUTH_CLIENT_ID");
        let bootstrapper = OAuthBootstrapper::new().unwrap();
        let err = bootstrapper
            .bootstrap_provider(&make_args())
            .err()
            .expect("expected error");
        assert!(format!("{}", err).contains("OAuth configuration error"));
    }

    #[test]
    #[serial]
    fn build_provider_succeeds_with_valid_config() {
        let _guard_id = EnvGuard::new("NBLM_OAUTH_CLIENT_ID");
        let _guard_disable = EnvGuard::new("NBLM_OAUTH_DISABLE_BOOTSTRAP");
        std::env::set_var("NBLM_OAUTH_CLIENT_ID", "test-client-id");
        std::env::set_var("NBLM_OAUTH_DISABLE_BOOTSTRAP", "1");

        let bootstrapper = OAuthBootstrapper::new().unwrap();
        let provider = bootstrapper.bootstrap_provider(&make_args()).unwrap();
        assert_eq!(provider.kind(), ProviderKind::UserOauth);
    }
}
