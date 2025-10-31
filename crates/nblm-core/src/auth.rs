use std::env;

use async_trait::async_trait;
use tokio::process::Command;

use crate::error::{Error, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderKind {
    GcloudOauth,
    EnvAccessToken,
    StaticToken,
    UserOauth,
}

impl ProviderKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            ProviderKind::GcloudOauth => "gcloud-oauth",
            ProviderKind::EnvAccessToken => "env-access-token",
            ProviderKind::StaticToken => "static-token",
            ProviderKind::UserOauth => "user-oauth",
        }
    }

    pub fn is_experimental(&self) -> bool {
        matches!(self, ProviderKind::UserOauth)
    }
}

#[async_trait]
pub trait TokenProvider: Send + Sync {
    async fn access_token(&self) -> Result<String>;
    async fn refresh_token(&self) -> Result<String> {
        self.access_token().await
    }

    fn kind(&self) -> ProviderKind {
        ProviderKind::StaticToken
    }
}

#[derive(Debug, Default, Clone)]
pub struct GcloudTokenProvider {
    binary: String,
}

impl GcloudTokenProvider {
    pub fn new(binary: impl Into<String>) -> Self {
        Self {
            binary: binary.into(),
        }
    }
}

#[async_trait]
impl TokenProvider for GcloudTokenProvider {
    async fn access_token(&self) -> Result<String> {
        let output = Command::new(&self.binary)
            .arg("auth")
            .arg("print-access-token")
            .output()
            .await
            .map_err(|err| {
                Error::TokenProvider(format!(
                    "Failed to execute gcloud command. Make sure gcloud CLI is installed and in PATH.\nError: {}",
                    err
                ))
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Error::TokenProvider(format!(
                "Failed to get access token from gcloud. Please run 'gcloud auth login' to authenticate.\nError: {}",
                stderr.trim()
            )));
        }

        let token = String::from_utf8(output.stdout)
            .map_err(|err| Error::TokenProvider(format!("invalid UTF-8 token: {err}")))?;

        Ok(token.trim().to_owned())
    }

    fn kind(&self) -> ProviderKind {
        ProviderKind::GcloudOauth
    }
}

#[derive(Debug, Clone)]
pub struct EnvTokenProvider {
    key: String,
}

impl EnvTokenProvider {
    pub fn new(key: impl Into<String>) -> Self {
        Self { key: key.into() }
    }
}

#[async_trait]
impl TokenProvider for EnvTokenProvider {
    async fn access_token(&self) -> Result<String> {
        env::var(&self.key)
            .map_err(|_| Error::TokenProvider(format!("environment variable {} missing", self.key)))
    }

    fn kind(&self) -> ProviderKind {
        ProviderKind::EnvAccessToken
    }
}

#[derive(Debug, Clone)]
pub struct StaticTokenProvider {
    token: String,
}

impl StaticTokenProvider {
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
        }
    }
}

#[async_trait]
impl TokenProvider for StaticTokenProvider {
    async fn access_token(&self) -> Result<String> {
        Ok(self.token.clone())
    }

    fn kind(&self) -> ProviderKind {
        ProviderKind::StaticToken
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn static_token_provider_returns_token() {
        let provider = StaticTokenProvider::new("test-token-123");
        let token = provider.access_token().await.unwrap();
        assert_eq!(token, "test-token-123");
    }

    #[tokio::test]
    async fn env_token_provider_reads_from_env() {
        std::env::set_var("TEST_NBLM_TOKEN", "env-token-456");
        let provider = EnvTokenProvider::new("TEST_NBLM_TOKEN");
        let token = provider.access_token().await.unwrap();
        assert_eq!(token, "env-token-456");
        std::env::remove_var("TEST_NBLM_TOKEN");
    }

    #[tokio::test]
    async fn env_token_provider_errors_when_missing() {
        std::env::remove_var("NONEXISTENT_TOKEN");
        let provider = EnvTokenProvider::new("NONEXISTENT_TOKEN");
        let result = provider.access_token().await;
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("environment variable NONEXISTENT_TOKEN missing"));
    }

    #[test]
    fn provider_kind_as_str_returns_correct_labels() {
        assert_eq!(ProviderKind::GcloudOauth.as_str(), "gcloud-oauth");
        assert_eq!(ProviderKind::EnvAccessToken.as_str(), "env-access-token");
        assert_eq!(ProviderKind::StaticToken.as_str(), "static-token");
        assert_eq!(ProviderKind::UserOauth.as_str(), "user-oauth");
    }

    #[test]
    fn provider_kind_is_experimental_only_for_user_oauth() {
        assert!(!ProviderKind::GcloudOauth.is_experimental());
        assert!(!ProviderKind::EnvAccessToken.is_experimental());
        assert!(!ProviderKind::StaticToken.is_experimental());
        assert!(ProviderKind::UserOauth.is_experimental());
    }

    #[test]
    fn gcloud_token_provider_returns_correct_kind() {
        let provider = GcloudTokenProvider::new("gcloud");
        assert_eq!(provider.kind(), ProviderKind::GcloudOauth);
    }

    #[test]
    fn env_token_provider_returns_correct_kind() {
        let provider = EnvTokenProvider::new("TEST_TOKEN");
        assert_eq!(provider.kind(), ProviderKind::EnvAccessToken);
    }

    #[test]
    fn static_token_provider_returns_correct_kind() {
        let provider = StaticTokenProvider::new("token");
        assert_eq!(provider.kind(), ProviderKind::StaticToken);
    }
}
