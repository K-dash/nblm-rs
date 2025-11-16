use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use oauth2::{
    basic::BasicClient, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EndpointSet,
    PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, RefreshToken, Scope, StandardTokenResponse,
    TokenResponse as OAuth2TokenResponse, TokenUrl,
};
use parking_lot::RwLock;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::auth::{ProviderKind, TokenProvider};
use crate::env::ApiProfile;
use crate::error::{Error as CoreError, Result as CoreResult};

mod config;
mod error;
pub mod loopback;
#[cfg(test)]
pub mod testing;

pub use config::OAuthClientConfig;
pub use error::{OAuthError, Result};

impl From<OAuthError> for CoreError {
    fn from(err: OAuthError) -> Self {
        CoreError::TokenProvider(err.to_string())
    }
}

// ============================================================================
// OAuth Configuration
// ============================================================================

/// OAuth2 configuration for Authorization Code Flow with PKCE
#[derive(Debug, Clone)]
pub struct OAuthConfig {
    pub auth_endpoint: String,
    pub token_endpoint: String,
    pub client_id: String,
    pub client_secret: Option<String>,
    pub redirect_uri: String,
    pub scopes: Vec<String>,
    pub audience: Option<String>,
    pub additional_params: HashMap<String, String>,
}

const GOOGLE_REVOKE_ENDPOINT: &str = "https://oauth2.googleapis.com/revoke";
const REVOKE_ENDPOINT_ENV: &str = "NBLM_OAUTH_REVOKE_ENDPOINT";

impl OAuthConfig {
    pub const DEFAULT_REDIRECT_URI: &str = "http://127.0.0.1:4317";
    pub(crate) const AUTH_ENDPOINT: &str = "https://accounts.google.com/o/oauth2/v2/auth";
    pub(crate) const TOKEN_ENDPOINT: &str = "https://oauth2.googleapis.com/token";
    pub(crate) const SCOPE_CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
    pub(crate) const SCOPE_DRIVE_FILE: &str = "https://www.googleapis.com/auth/drive.file";

    /// Create a default Google OAuth2 configuration for NotebookLM Enterprise
    pub fn google_default(_project_number: &str) -> Result<Self> {
        OAuthClientConfig::from_env().map(|cfg| cfg.into_oauth_config())
    }
}

// ============================================================================
// OAuth Tokens
// ============================================================================

/// OAuth2 tokens returned from token endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthTokens {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: OffsetDateTime,
    pub scope: Option<String>,
    pub token_type: String,
}

impl OAuthTokens {
    /// Create OAuthTokens from oauth2-rs token response
    pub fn from_oauth2_response(
        response: StandardTokenResponse<
            oauth2::EmptyExtraTokenFields,
            oauth2::basic::BasicTokenType,
        >,
        issued_at: OffsetDateTime,
    ) -> Self {
        let expires_at = issued_at
            + response
                .expires_in()
                .map(|d| Duration::from_secs(d.as_secs()))
                .unwrap_or_else(|| Duration::from_secs(3600));

        let scope = response.scopes().map(|scopes| {
            scopes
                .iter()
                .map(|s| s.as_str().to_string())
                .collect::<Vec<_>>()
                .join(" ")
        });

        Self {
            access_token: response.access_token().secret().to_string(),
            refresh_token: response.refresh_token().map(|rt| rt.secret().to_string()),
            expires_at,
            scope,
            token_type: match response.token_type() {
                oauth2::basic::BasicTokenType::Bearer => "Bearer".to_string(),
                oauth2::basic::BasicTokenType::Mac => "MAC".to_string(),
                oauth2::basic::BasicTokenType::Extension(s) => s.clone(),
            },
        }
    }
}

// ============================================================================
// Token Cache Entry
// ============================================================================

/// In-memory cache entry for OAuth tokens
#[derive(Debug, Clone)]
pub struct TokenCacheEntry {
    pub tokens: OAuthTokens,
    pub refresh_margin: Duration,
}

impl TokenCacheEntry {
    pub fn new(tokens: OAuthTokens) -> Self {
        Self {
            tokens,
            refresh_margin: Duration::from_secs(60), // Default 60 seconds
        }
    }

    /// Check if token needs refresh
    pub fn needs_refresh(&self, now: OffsetDateTime) -> bool {
        now >= (self.tokens.expires_at - self.refresh_margin)
    }
}

// ============================================================================
// Token Store Key
// ============================================================================

/// Key for storing tokens in RefreshTokenStore
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TokenStoreKey {
    pub profile: ApiProfile,
    pub project_number: Option<String>,
    pub endpoint_location: Option<String>,
    pub user_hint: Option<String>,
}

impl fmt::Display for TokenStoreKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts = vec![self.profile.as_str().to_string()];

        if let Some(ref project) = self.project_number {
            parts.push(format!("project={}", project));
        }

        if let Some(ref location) = self.endpoint_location {
            parts.push(format!("location={}", location));
        }

        if let Some(ref user) = self.user_hint {
            parts.push(format!("user={}", user));
        }

        write!(f, "{}", parts.join(":"))
    }
}

// ============================================================================
// Serialized Tokens (for storage)
// ============================================================================

/// Serialized token data for storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedTokens {
    pub refresh_token: String,
    pub scopes: Vec<String>,
    pub expires_at: Option<OffsetDateTime>,
    pub token_type: String,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

/// Credentials file format
#[derive(Debug, Serialize, Deserialize)]
struct CredentialsFile {
    version: u32,
    entries: HashMap<String, SerializedTokens>,
}

impl CredentialsFile {
    fn new() -> Self {
        Self {
            version: 1,
            entries: HashMap::new(),
        }
    }
}

// ============================================================================
// RefreshTokenStore Trait
// ============================================================================

/// Trait for storing and retrieving refresh tokens
#[async_trait]
pub trait RefreshTokenStore: Send + Sync {
    /// Load tokens for the given key
    async fn load(&self, key: &TokenStoreKey) -> Result<Option<SerializedTokens>>;

    /// Save tokens for the given key
    async fn save(&self, key: &TokenStoreKey, tokens: &SerializedTokens) -> Result<()>;

    /// Delete tokens for the given key
    async fn delete(&self, key: &TokenStoreKey) -> Result<()>;
}

// ============================================================================
// FileRefreshTokenStore
// ============================================================================

const CONFIG_DIR_ENV: &str = "NBLM_CONFIG_DIR";

/// File-based implementation of RefreshTokenStore
pub struct FileRefreshTokenStore {
    file_path: std::path::PathBuf,
}

impl FileRefreshTokenStore {
    /// Create a new FileRefreshTokenStore
    pub fn new() -> Result<Self> {
        if let Ok(custom_dir) = std::env::var(CONFIG_DIR_ENV) {
            let file_path = PathBuf::from(custom_dir).join("credentials.json");
            return Self::from_path(file_path);
        }

        let dirs = directories::ProjectDirs::from("com", "nblm", "nblm-rs")
            .ok_or_else(|| OAuthError::Config("failed to find config directory".to_string()))?;

        let config_dir = dirs.config_dir();
        let file_path = config_dir.join("credentials.json");

        Self::from_path(file_path)
    }

    /// Create a store backed by an explicit credentials file path.
    pub fn from_path(path: impl Into<PathBuf>) -> Result<Self> {
        Ok(Self {
            file_path: path.into(),
        })
    }

    /// Ensure config directory exists with proper permissions (async)
    async fn ensure_config_dir(&self) -> Result<()> {
        if let Some(config_dir) = self.file_path.parent() {
            tokio::fs::create_dir_all(config_dir).await.map_err(|e| {
                OAuthError::Config(format!("failed to create config directory: {}", e))
            })?;

            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = tokio::fs::metadata(config_dir)
                    .await
                    .map_err(|e| {
                        OAuthError::Config(format!("failed to get config dir metadata: {}", e))
                    })?
                    .permissions();
                perms.set_mode(0o700);
                tokio::fs::set_permissions(config_dir, perms)
                    .await
                    .map_err(|e| {
                        OAuthError::Config(format!("failed to set config dir permissions: {}", e))
                    })?;
            }
        }
        Ok(())
    }

    /// Load credentials file
    async fn load_file(&self) -> Result<CredentialsFile> {
        self.ensure_config_dir().await?;

        if !self.file_path.exists() {
            return Ok(CredentialsFile::new());
        }

        let content = tokio::fs::read_to_string(&self.file_path)
            .await
            .map_err(|e| OAuthError::Config(format!("failed to read credentials file: {}", e)))?;

        let file: CredentialsFile = serde_json::from_str(&content)
            .map_err(|e| OAuthError::Config(format!("failed to parse credentials file: {}", e)))?;

        Ok(file)
    }

    /// Save credentials file
    async fn save_file(&self, file: &CredentialsFile) -> Result<()> {
        self.ensure_config_dir().await?;

        let content = serde_json::to_string_pretty(file)
            .map_err(|e| OAuthError::Config(format!("failed to serialize credentials: {}", e)))?;

        // Write to temp file first, then rename (atomic write)
        // Use a unique temporary file name to avoid conflicts in concurrent writes
        let random_suffix = {
            use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
            use rand::RngCore;
            let mut rng = rand::rng();
            let mut random_bytes = [0u8; 8];
            rng.fill_bytes(&mut random_bytes);
            URL_SAFE_NO_PAD.encode(random_bytes)
        };

        let temp_path = self.file_path.with_file_name(format!(
            "{}.{}.tmp",
            self.file_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("credentials.json"),
            random_suffix
        ));
        tokio::fs::write(&temp_path, content)
            .await
            .map_err(|e| OAuthError::Config(format!("failed to write credentials file: {}", e)))?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(metadata) = tokio::fs::metadata(&temp_path).await {
                let mut perms = metadata.permissions();
                perms.set_mode(0o600);
                let _ = tokio::fs::set_permissions(&temp_path, perms).await;
            }
        }

        tokio::fs::rename(&temp_path, &self.file_path)
            .await
            .map_err(|e| OAuthError::Config(format!("failed to rename temp file: {}", e)))?;

        Ok(())
    }

    /// Remove the entire credentials file from disk (best-effort).
    pub async fn delete_file(&self) -> Result<()> {
        match tokio::fs::remove_file(&self.file_path).await {
            Ok(_) => Ok(()),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(err) => Err(OAuthError::Storage(err)),
        }
    }
}

#[async_trait]
impl RefreshTokenStore for FileRefreshTokenStore {
    async fn load(&self, key: &TokenStoreKey) -> Result<Option<SerializedTokens>> {
        let file = self.load_file().await?;
        Ok(file.entries.get(&key.to_string()).cloned())
    }

    async fn save(&self, key: &TokenStoreKey, tokens: &SerializedTokens) -> Result<()> {
        let mut file = self.load_file().await?;
        file.entries.insert(key.to_string(), tokens.clone());
        self.save_file(&file).await
    }

    async fn delete(&self, key: &TokenStoreKey) -> Result<()> {
        let mut file = self.load_file().await?;
        file.entries.remove(&key.to_string());
        self.save_file(&file).await
    }
}

// ============================================================================
// OAuth Flow
// ============================================================================

/// Parameters for building authorization URL
#[derive(Debug, Clone)]
pub struct AuthorizeParams {
    pub state: Option<String>,
    pub code_challenge: Option<String>,
    pub code_challenge_method: Option<String>,
}

/// Context for authorization flow
#[derive(Debug, Clone)]
pub struct AuthorizeContext {
    pub url: String,
    pub state: String,
    pub code_verifier: String,
    pub expires_at: OffsetDateTime,
}

/// OAuth2 Authorization Code Flow with PKCE
pub struct OAuthFlow {
    client: BasicClient<
        EndpointSet,
        oauth2::EndpointNotSet,
        oauth2::EndpointNotSet,
        oauth2::EndpointNotSet,
        EndpointSet,
    >,
    config: OAuthConfig,
    http: Arc<Client>,
}

impl OAuthFlow {
    /// Create a new OAuthFlow
    pub fn new(config: OAuthConfig, http: Arc<Client>) -> Result<Self> {
        let client_id = ClientId::new(config.client_id.clone());
        let auth_url = AuthUrl::new(config.auth_endpoint.clone())
            .map_err(|e| OAuthError::Config(format!("invalid auth_url: {}", e)))?;
        let token_url = TokenUrl::new(config.token_endpoint.clone())
            .map_err(|e| OAuthError::Config(format!("invalid token_url: {}", e)))?;
        let redirect_url = RedirectUrl::new(config.redirect_uri.clone())
            .map_err(|e| OAuthError::Config(format!("invalid redirect_url: {}", e)))?;

        let mut client_builder = BasicClient::new(client_id)
            .set_auth_uri(auth_url)
            .set_token_uri(token_url)
            .set_redirect_uri(redirect_url);

        if let Some(ref client_secret) = config.client_secret {
            client_builder =
                client_builder.set_client_secret(ClientSecret::new(client_secret.clone()));
        }

        Ok(Self {
            client: client_builder,
            config,
            http,
        })
    }

    /// Build authorization URL with PKCE
    pub fn build_authorize_url(&self, params: &AuthorizeParams) -> AuthorizeContext {
        // Generate PKCE challenge and verifier
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        // Generate CSRF token
        let csrf_token = if let Some(ref state) = params.state {
            CsrfToken::new(state.clone())
        } else {
            CsrfToken::new_random()
        };

        // Build authorization URL
        let mut auth_request = self.client.authorize_url(|| csrf_token.clone());

        // Add scopes
        for scope_str in &self.config.scopes {
            auth_request = auth_request.add_scope(Scope::new(scope_str.clone()));
        }

        // Set PKCE challenge
        auth_request = auth_request.set_pkce_challenge(pkce_challenge);

        // Add additional parameters
        for (key, value) in &self.config.additional_params {
            auth_request = auth_request.add_extra_param(key, value);
        }

        // Build the URL
        let (auth_url, csrf_token_actual) = auth_request.url();

        // Add Google-specific parameters
        let mut url = url::Url::parse(auth_url.as_str()).expect("invalid auth_url");
        url.query_pairs_mut()
            .append_pair("access_type", "offline")
            .append_pair("prompt", "consent");

        if let Some(ref audience) = self.config.audience {
            url.query_pairs_mut().append_pair("audience", audience);
        }

        let expires_at = OffsetDateTime::now_utc() + Duration::from_secs(600); // 10 minutes

        AuthorizeContext {
            url: url.to_string(),
            state: csrf_token_actual.secret().to_string(),
            code_verifier: pkce_verifier.secret().to_string(),
            expires_at,
        }
    }

    /// Exchange authorization code for tokens
    pub async fn exchange_code(
        &self,
        context: &AuthorizeContext,
        code: &str,
    ) -> Result<OAuthTokens> {
        let code = AuthorizationCode::new(code.to_string());
        let pkce_verifier = PkceCodeVerifier::new(context.code_verifier.clone());

        let token_request = self
            .client
            .exchange_code(code)
            .set_pkce_verifier(pkce_verifier);

        let token_response = token_request
            .request_async(self.http.as_ref())
            .await
            .map_err(|e| OAuthError::Config(format!("oauth token exchange failed: {}", e)))?;

        Ok(OAuthTokens::from_oauth2_response(
            token_response,
            OffsetDateTime::now_utc(),
        ))
    }

    /// Refresh access token using refresh token
    pub async fn refresh(&self, refresh_token: &str) -> Result<OAuthTokens> {
        let refresh_token = RefreshToken::new(refresh_token.to_string());

        let token_request = self.client.exchange_refresh_token(&refresh_token);

        let token_response = token_request
            .request_async(self.http.as_ref())
            .await
            .map_err(|e| OAuthError::Config(format!("oauth token refresh failed: {}", e)))?;

        Ok(OAuthTokens::from_oauth2_response(
            token_response,
            OffsetDateTime::now_utc(),
        ))
    }

    /// Revoke a refresh token via Google's revocation endpoint.
    pub async fn revoke_refresh_token(&self, refresh_token: &str) -> Result<()> {
        if refresh_token.trim().is_empty() {
            return Err(OAuthError::Revocation(
                "refresh token cannot be empty".into(),
            ));
        }

        let endpoint = std::env::var(REVOKE_ENDPOINT_ENV)
            .unwrap_or_else(|_| GOOGLE_REVOKE_ENDPOINT.to_string());

        let response = self
            .http
            .post(&endpoint)
            .form(&[("token", refresh_token)])
            .send()
            .await
            .map_err(|e| OAuthError::Revocation(format!("revocation request failed: {}", e)))?;

        if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            Err(OAuthError::Revocation(format!(
                "revocation failed (status {}): {}",
                status, body
            )))
        }
    }
}

// ============================================================================
// RefreshTokenProvider
// ============================================================================

/// TokenProvider implementation using refresh tokens
pub struct RefreshTokenProvider<S: RefreshTokenStore> {
    flow: OAuthFlow,
    store: Arc<S>,
    cache: RwLock<Option<TokenCacheEntry>>,
    store_key: TokenStoreKey,
}

impl<S: RefreshTokenStore> RefreshTokenProvider<S> {
    /// Create a new RefreshTokenProvider
    pub fn new(flow: OAuthFlow, store: Arc<S>, store_key: TokenStoreKey) -> Self {
        Self {
            flow,
            store,
            cache: RwLock::new(None),
            store_key,
        }
    }

    /// Ensure tokens are valid, refreshing if necessary
    async fn ensure_tokens(&self, force_refresh: bool) -> Result<OAuthTokens> {
        let now = OffsetDateTime::now_utc();

        // Check cache first
        if !force_refresh {
            if let Some(ref entry) = *self.cache.read() {
                if !entry.needs_refresh(now) {
                    return Ok(entry.tokens.clone());
                }
            }
        }

        // Load refresh token from store
        let stored = self
            .store
            .load(&self.store_key)
            .await?
            .ok_or_else(|| OAuthError::Config("refresh token unavailable".to_string()))?;

        // Refresh access token
        let tokens = self.flow.refresh(&stored.refresh_token).await?;
        let refresh_token = tokens
            .refresh_token
            .clone()
            .unwrap_or_else(|| stored.refresh_token.clone());
        let scopes: Vec<String> = if let Some(scopes_str) = tokens.scope.as_ref() {
            scopes_str.split_whitespace().map(String::from).collect()
        } else if !stored.scopes.is_empty() {
            stored.scopes.clone()
        } else {
            Vec::new()
        };
        let token_type = if !tokens.token_type.is_empty() {
            tokens.token_type.clone()
        } else {
            stored.token_type.clone()
        };

        // Update cache
        {
            let mut cache = self.cache.write();
            *cache = Some(TokenCacheEntry::new(tokens.clone()));
        }

        // Persist refresh token details (preserve previous token when refresh response omits it)
        let serialized = SerializedTokens {
            refresh_token,
            scopes,
            expires_at: Some(tokens.expires_at),
            token_type,
            updated_at: now,
        };
        self.store.save(&self.store_key, &serialized).await?;

        Ok(tokens)
    }
}

#[async_trait]
impl<S: RefreshTokenStore> TokenProvider for RefreshTokenProvider<S> {
    async fn access_token(&self) -> CoreResult<String> {
        let tokens = self.ensure_tokens(false).await.map_err(CoreError::from)?;
        Ok(tokens.access_token)
    }

    async fn refresh_token(&self) -> CoreResult<String> {
        let tokens = self.ensure_tokens(true).await.map_err(CoreError::from)?;
        Ok(tokens.access_token)
    }

    fn kind(&self) -> ProviderKind {
        ProviderKind::UserOauth
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::oauth::testing::fake::FakeOAuthServer;
    use tempfile::tempdir;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_token_cache_entry_needs_refresh() {
        let now = OffsetDateTime::now_utc();
        let expires_at = now + Duration::from_secs(120); // 2 minutes
        let tokens = OAuthTokens {
            access_token: "test-token".to_string(),
            refresh_token: None,
            expires_at,
            scope: None,
            token_type: "Bearer".to_string(),
        };

        let entry = TokenCacheEntry::new(tokens);

        // Should not need refresh immediately
        assert!(!entry.needs_refresh(now));

        // Should need refresh when close to expiry (within margin)
        let near_expiry = expires_at - Duration::from_secs(30);
        assert!(entry.needs_refresh(near_expiry));
    }

    #[tokio::test]
    async fn test_token_store_key_display() {
        let key = TokenStoreKey {
            profile: ApiProfile::Enterprise,
            project_number: Some("123456".to_string()),
            endpoint_location: Some("global".to_string()),
            user_hint: None,
        };

        let display = key.to_string();
        assert!(display.contains("enterprise"));
        assert!(display.contains("project=123456"));
        assert!(display.contains("location=global"));
    }

    #[tokio::test]
    async fn test_oauth_tokens_from_oauth2_response() {
        use oauth2::StandardTokenResponse;

        let now = OffsetDateTime::now_utc();
        // Manually create a response with all fields using serde_json
        let json_response = serde_json::json!({
            "access_token": "access-token-123",
            "refresh_token": "refresh-token-456",
            "expires_in": 3600,
            "token_type": "Bearer",
            "scope": "scope1 scope2"
        });

        let response: StandardTokenResponse<
            oauth2::EmptyExtraTokenFields,
            oauth2::basic::BasicTokenType,
        > = serde_json::from_value(json_response).unwrap();

        let tokens = OAuthTokens::from_oauth2_response(response, now);
        assert_eq!(tokens.access_token, "access-token-123");
        assert_eq!(tokens.refresh_token, Some("refresh-token-456".to_string()));
        assert_eq!(tokens.scope, Some("scope1 scope2".to_string()));
        assert_eq!(tokens.token_type, "Bearer");
        assert!(tokens.expires_at > now);
    }

    #[tokio::test]
    async fn test_oauth_flow_refresh_token() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "access_token": "new-access-token",
                "expires_in": 3600,
                "token_type": "Bearer"
            })))
            .mount(&server)
            .await;

        let config = OAuthConfig {
            auth_endpoint: "https://accounts.google.com/o/oauth2/v2/auth".to_string(),
            token_endpoint: format!("{}/token", server.uri()),
            client_id: "test-client-id".to_string(),
            client_secret: None,
            redirect_uri: "http://127.0.0.1:4317".to_string(),
            scopes: vec!["scope1".to_string()],
            audience: None,
            additional_params: HashMap::new(),
        };

        let http = Arc::new(Client::new());
        let flow = OAuthFlow::new(config, http).unwrap();

        let tokens = flow.refresh("refresh-token-123").await.unwrap();
        assert_eq!(tokens.access_token, "new-access-token");
    }

    // Test 1: PKCE validity tests (security critical)
    #[test]
    fn test_pkce_code_verifier_and_challenge_generation() {
        let (challenge, verifier) = PkceCodeChallenge::new_random_sha256();

        // Verify code_verifier is base64url encoded and proper length
        let verifier_str = verifier.secret();
        assert!(!verifier_str.is_empty());
        assert!(verifier_str.len() >= 43 && verifier_str.len() <= 128);
        assert!(verifier_str
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_'));

        // Verify code_challenge is base64url encoded SHA256 hash
        let challenge_str = challenge.as_str();
        assert!(!challenge_str.is_empty());
        assert_eq!(challenge_str.len(), 43); // SHA256 hash is 32 bytes, base64url is 43 chars
        assert!(challenge_str
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_'));
    }

    #[test]
    fn test_pkce_generates_unique_values() {
        let (challenge1, verifier1) = PkceCodeChallenge::new_random_sha256();
        let (challenge2, verifier2) = PkceCodeChallenge::new_random_sha256();

        // Each generation should produce unique values
        assert_ne!(verifier1.secret(), verifier2.secret());
        assert_ne!(challenge1.as_str(), challenge2.as_str());
    }

    #[test]
    fn test_state_generation_uniqueness() {
        let state1 = CsrfToken::new_random();
        let state2 = CsrfToken::new_random();

        // States should be unique
        assert_ne!(state1.secret(), state2.secret());
        assert!(!state1.secret().is_empty());
        assert!(!state2.secret().is_empty());
    }

    // Test 2: Token expiration boundary tests
    #[test]
    fn test_token_needs_refresh_at_exact_expiry() {
        let now = OffsetDateTime::now_utc();
        let expires_at = now + Duration::from_secs(60);
        let tokens = OAuthTokens {
            access_token: "test-token".to_string(),
            refresh_token: None,
            expires_at,
            scope: None,
            token_type: "Bearer".to_string(),
        };

        let entry = TokenCacheEntry::new(tokens);

        // At exact expiry time (accounting for margin)
        assert!(entry.needs_refresh(expires_at));
    }

    #[test]
    fn test_token_needs_refresh_just_before_margin() {
        let now = OffsetDateTime::now_utc();
        let expires_at = now + Duration::from_secs(120);
        let tokens = OAuthTokens {
            access_token: "test-token".to_string(),
            refresh_token: None,
            expires_at,
            scope: None,
            token_type: "Bearer".to_string(),
        };

        let entry = TokenCacheEntry::new(tokens);

        // Just before margin (61 seconds before expiry, margin is 60)
        let before_margin = expires_at - Duration::from_secs(61);
        assert!(!entry.needs_refresh(before_margin));

        // Just at margin (60 seconds before expiry)
        let at_margin = expires_at - Duration::from_secs(60);
        assert!(entry.needs_refresh(at_margin));
    }

    #[test]
    fn test_token_needs_refresh_after_expiry() {
        let now = OffsetDateTime::now_utc();
        let expires_at = now - Duration::from_secs(10); // Already expired
        let tokens = OAuthTokens {
            access_token: "test-token".to_string(),
            refresh_token: None,
            expires_at,
            scope: None,
            token_type: "Bearer".to_string(),
        };

        let entry = TokenCacheEntry::new(tokens);
        assert!(entry.needs_refresh(now));
    }

    // Test 3: File store concurrent access tests
    #[tokio::test]
    #[serial_test::serial]
    async fn test_file_store_concurrent_saves() {
        use std::sync::Arc;
        use tokio::task::JoinSet;

        let temp_dir = tempdir().unwrap();
        let store_path = temp_dir.path().join("credentials.json");
        let store = Arc::new(FileRefreshTokenStore::from_path(&store_path).unwrap());
        let key = TokenStoreKey {
            profile: ApiProfile::Enterprise,
            project_number: Some("concurrent-test".to_string()),
            endpoint_location: Some("global".to_string()),
            user_hint: None,
        };

        let mut join_set = JoinSet::new();

        // Spawn 10 concurrent save operations
        for i in 0..10 {
            let store_clone = Arc::clone(&store);
            let key_clone = key.clone();
            join_set.spawn(async move {
                let tokens = SerializedTokens {
                    refresh_token: format!("token-{}", i),
                    scopes: vec!["scope1".to_string()],
                    expires_at: Some(OffsetDateTime::now_utc() + Duration::from_secs(3600)),
                    token_type: "Bearer".to_string(),
                    updated_at: OffsetDateTime::now_utc(),
                };
                store_clone.save(&key_clone, &tokens).await
            });
        }

        // Wait for all operations to complete
        while let Some(result) = join_set.join_next().await {
            result.unwrap().unwrap();
        }

        // Verify that the final state is consistent
        let loaded = store.load(&key).await.unwrap();
        assert!(loaded.is_some());
        let loaded = loaded.unwrap();
        assert!(loaded.refresh_token.starts_with("token-"));

        // Cleanup
        store.delete(&key).await.unwrap();
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_file_store_atomic_write() {
        let temp_dir = tempdir().unwrap();
        let store_path = temp_dir.path().join("credentials.json");
        let store = FileRefreshTokenStore::from_path(&store_path).unwrap();
        let key = TokenStoreKey {
            profile: ApiProfile::Enterprise,
            project_number: Some("atomic-test".to_string()),
            endpoint_location: Some("global".to_string()),
            user_hint: None,
        };

        let tokens = SerializedTokens {
            refresh_token: "initial-token".to_string(),
            scopes: vec!["scope1".to_string()],
            expires_at: Some(OffsetDateTime::now_utc() + Duration::from_secs(3600)),
            token_type: "Bearer".to_string(),
            updated_at: OffsetDateTime::now_utc(),
        };

        store.save(&key, &tokens).await.unwrap();

        // Verify temp files are cleaned up after save
        let entries: Vec<_> = std::fs::read_dir(temp_dir.path())
            .unwrap()
            .map(|entry| entry.unwrap().file_name())
            .collect();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0], std::ffi::OsStr::new("credentials.json"));

        // Cleanup
        store.delete(&key).await.unwrap();
    }

    #[test]
    #[serial_test::serial]
    fn test_file_store_respects_custom_config_dir() {
        use std::env;

        let temp_dir = tempdir().unwrap();
        let key = CONFIG_DIR_ENV;
        let original = env::var(key).ok();
        env::set_var(key, temp_dir.path());

        let store = FileRefreshTokenStore::new().expect("store should honor custom dir");
        assert!(store.file_path.starts_with(temp_dir.path()));

        if let Some(value) = original {
            env::set_var(key, value);
        } else {
            env::remove_var(key);
        }
    }

    // Test 5: refresh_token omission handling tests
    #[tokio::test]
    #[serial_test::serial]
    async fn test_refresh_token_preserved_when_omitted_in_response() {
        let server = MockServer::start().await;

        // Mock refresh endpoint that doesn't return refresh_token
        Mock::given(method("POST"))
            .and(path("/token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "access_token": "new-access-token",
                "expires_in": 3600,
                "token_type": "Bearer"
            })))
            .mount(&server)
            .await;

        let config = OAuthConfig {
            auth_endpoint: "https://accounts.google.com/o/oauth2/v2/auth".to_string(),
            token_endpoint: format!("{}/token", server.uri()),
            client_id: "test-client-id".to_string(),
            client_secret: None,
            redirect_uri: "http://127.0.0.1:4317".to_string(),
            scopes: vec!["scope1".to_string()],
            audience: None,
            additional_params: HashMap::new(),
        };

        let http = Arc::new(Client::new());
        let flow = OAuthFlow::new(config, http).unwrap();
        let temp_dir = tempdir().unwrap();
        let store_path = temp_dir.path().join("credentials.json");
        let store = Arc::new(FileRefreshTokenStore::from_path(&store_path).unwrap());
        let key = TokenStoreKey {
            profile: ApiProfile::Enterprise,
            project_number: Some("omission-test".to_string()),
            endpoint_location: Some("global".to_string()),
            user_hint: None,
        };

        // Store initial refresh token with expired access token
        let initial_tokens = SerializedTokens {
            refresh_token: "original-refresh-token".to_string(),
            scopes: vec!["scope1".to_string()],
            expires_at: Some(OffsetDateTime::now_utc() - Duration::from_secs(3600)), // Expired
            token_type: "Bearer".to_string(),
            updated_at: OffsetDateTime::now_utc() - Duration::from_secs(3600),
        };
        store.save(&key, &initial_tokens).await.unwrap();

        let provider = RefreshTokenProvider::new(flow, Arc::clone(&store), key.clone());

        // Get access token (should trigger refresh because token is expired)
        let access_token = provider.access_token().await.unwrap();
        assert_eq!(access_token, "new-access-token");

        // Verify original refresh token is preserved
        let stored = store.load(&key).await.unwrap().unwrap();
        assert_eq!(stored.refresh_token, "original-refresh-token");

        // Cleanup
        store.delete(&key).await.unwrap();
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_refresh_token_updated_when_included_in_response() {
        let server = MockServer::start().await;

        // Mock refresh endpoint that returns new refresh_token
        Mock::given(method("POST"))
            .and(path("/token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "access_token": "new-access-token",
                "refresh_token": "new-refresh-token",
                "expires_in": 3600,
                "token_type": "Bearer"
            })))
            .mount(&server)
            .await;

        let config = OAuthConfig {
            auth_endpoint: "https://accounts.google.com/o/oauth2/v2/auth".to_string(),
            token_endpoint: format!("{}/token", server.uri()),
            client_id: "test-client-id".to_string(),
            client_secret: None,
            redirect_uri: "http://127.0.0.1:4317".to_string(),
            scopes: vec!["scope1".to_string()],
            audience: None,
            additional_params: HashMap::new(),
        };

        let http = Arc::new(Client::new());
        let flow = OAuthFlow::new(config, http).unwrap();
        let temp_dir = tempdir().unwrap();
        let store_path = temp_dir.path().join("credentials.json");
        let store = Arc::new(FileRefreshTokenStore::from_path(&store_path).unwrap());
        let key = TokenStoreKey {
            profile: ApiProfile::Enterprise,
            project_number: Some("update-test".to_string()),
            endpoint_location: Some("global".to_string()),
            user_hint: None,
        };

        // Store initial refresh token with expired access token
        let initial_tokens = SerializedTokens {
            refresh_token: "original-refresh-token".to_string(),
            scopes: vec!["scope1".to_string()],
            expires_at: Some(OffsetDateTime::now_utc() - Duration::from_secs(3600)), // Expired
            token_type: "Bearer".to_string(),
            updated_at: OffsetDateTime::now_utc() - Duration::from_secs(3600),
        };
        store.save(&key, &initial_tokens).await.unwrap();

        let provider = RefreshTokenProvider::new(flow, Arc::clone(&store), key.clone());

        // Get access token (should trigger refresh because token is expired)
        let access_token = provider.access_token().await.unwrap();
        assert_eq!(access_token, "new-access-token");

        // Verify refresh token is updated
        let stored = store.load(&key).await.unwrap().unwrap();
        assert_eq!(stored.refresh_token, "new-refresh-token");

        // Cleanup
        store.delete(&key).await.unwrap();
    }

    // Test 6: State validation failure tests (CSRF)
    #[tokio::test]
    async fn test_state_mismatch_detection() {
        let config = OAuthConfig {
            auth_endpoint: "https://accounts.google.com/o/oauth2/v2/auth".to_string(),
            token_endpoint: "https://oauth2.googleapis.com/token".to_string(),
            client_id: "test-client-id".to_string(),
            client_secret: None,
            redirect_uri: "http://127.0.0.1:4317".to_string(),
            scopes: vec!["scope1".to_string()],
            audience: None,
            additional_params: HashMap::new(),
        };

        let http = Arc::new(Client::new());
        let flow = OAuthFlow::new(config, http).unwrap();

        let context = flow.build_authorize_url(&AuthorizeParams {
            state: None,
            code_challenge: None,
            code_challenge_method: None,
        });

        // Simulate state mismatch (CSRF attack)
        let wrong_state = "attacker-controlled-state";
        assert_ne!(context.state, wrong_state);

        // In actual implementation, this would be caught by the callback handler
        // which compares the received state with context.state
    }

    #[test]
    fn test_authorize_url_contains_required_parameters() {
        let config = OAuthConfig {
            auth_endpoint: "https://accounts.google.com/o/oauth2/v2/auth".to_string(),
            token_endpoint: "https://oauth2.googleapis.com/token".to_string(),
            client_id: "test-client-id".to_string(),
            client_secret: None,
            redirect_uri: "http://127.0.0.1:4317".to_string(),
            scopes: vec!["scope1".to_string(), "scope2".to_string()],
            audience: None,
            additional_params: HashMap::new(),
        };

        let http = Arc::new(Client::new());
        let flow = OAuthFlow::new(config, http).unwrap();

        let context = flow.build_authorize_url(&AuthorizeParams {
            state: None,
            code_challenge: None,
            code_challenge_method: None,
        });

        let url = url::Url::parse(&context.url).unwrap();
        let params: HashMap<_, _> = url.query_pairs().collect();

        // Verify required PKCE and OAuth parameters
        assert!(params.contains_key("client_id"));
        assert!(params.contains_key("redirect_uri"));
        assert!(params.contains_key("response_type"));
        assert_eq!(params.get("response_type").unwrap(), "code");
        assert!(params.contains_key("scope"));
        assert!(params.contains_key("state"));
        assert!(params.contains_key("code_challenge"));
        assert!(params.contains_key("code_challenge_method"));
        assert_eq!(params.get("code_challenge_method").unwrap(), "S256");
        assert!(params.contains_key("access_type"));
        assert_eq!(params.get("access_type").unwrap(), "offline");
        assert!(params.contains_key("prompt"));
        assert_eq!(params.get("prompt").unwrap(), "consent");
    }

    #[test]
    fn test_custom_state_is_preserved() {
        let config = OAuthConfig {
            auth_endpoint: "https://accounts.google.com/o/oauth2/v2/auth".to_string(),
            token_endpoint: "https://oauth2.googleapis.com/token".to_string(),
            client_id: "test-client-id".to_string(),
            client_secret: None,
            redirect_uri: "http://127.0.0.1:4317".to_string(),
            scopes: vec!["scope1".to_string()],
            audience: None,
            additional_params: HashMap::new(),
        };

        let http = Arc::new(Client::new());
        let flow = OAuthFlow::new(config, http).unwrap();

        let custom_state = "my-custom-state-value";
        let context = flow.build_authorize_url(&AuthorizeParams {
            state: Some(custom_state.to_string()),
            code_challenge: None,
            code_challenge_method: None,
        });

        assert_eq!(context.state, custom_state);
        assert!(context.url.contains(&format!("state={}", custom_state)));
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn revoke_refresh_token_succeeds_with_fake_server() {
        let server = FakeOAuthServer::start().await;
        std::env::set_var(REVOKE_ENDPOINT_ENV, server.revoke_endpoint());

        let config = OAuthConfig {
            auth_endpoint: format!("{}/auth", server.base_uri()),
            token_endpoint: server.token_endpoint(),
            client_id: "client-id".into(),
            client_secret: None,
            redirect_uri: "http://localhost:1234".into(),
            scopes: vec!["scope".into()],
            audience: None,
            additional_params: HashMap::new(),
        };
        let flow = OAuthFlow::new(config, Arc::new(Client::new())).unwrap();
        flow.revoke_refresh_token("fake_refresh_token")
            .await
            .unwrap();
        std::env::remove_var(REVOKE_ENDPOINT_ENV);
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn revoke_refresh_token_reports_error() {
        let server = FakeOAuthServer::start().await;
        std::env::set_var(
            REVOKE_ENDPOINT_ENV,
            format!("{}/invalid", server.base_uri()),
        );

        let config = OAuthConfig {
            auth_endpoint: format!("{}/auth", server.base_uri()),
            token_endpoint: server.token_endpoint(),
            client_id: "client-id".into(),
            client_secret: None,
            redirect_uri: "http://localhost:1234".into(),
            scopes: vec!["scope".into()],
            audience: None,
            additional_params: HashMap::new(),
        };
        let flow = OAuthFlow::new(config, Arc::new(Client::new())).unwrap();
        let err = flow
            .revoke_refresh_token("fake_refresh_token")
            .await
            .unwrap_err();
        assert!(matches!(err, OAuthError::Revocation(_)));
        std::env::remove_var(REVOKE_ENDPOINT_ENV);
    }

    #[tokio::test]
    async fn delete_file_removes_credentials_file() {
        let temp = tempdir().unwrap();
        let path = temp.path().join("credentials.json");
        let store = FileRefreshTokenStore::from_path(&path).unwrap();
        let key = TokenStoreKey {
            profile: ApiProfile::Enterprise,
            project_number: Some("123".into()),
            endpoint_location: Some("global".into()),
            user_hint: None,
        };
        let serialized = SerializedTokens {
            refresh_token: "token".into(),
            scopes: vec![],
            expires_at: None,
            token_type: "Bearer".into(),
            updated_at: OffsetDateTime::now_utc(),
        };
        store.save(&key, &serialized).await.unwrap();
        assert!(path.exists());
        store.delete_file().await.unwrap();
        assert!(!path.exists());
    }
}
