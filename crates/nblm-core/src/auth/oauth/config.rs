use super::{OAuthConfig, OAuthError, Result};

/// OAuth client configuration loaded from the environment.
#[derive(Debug, Clone)]
pub struct OAuthClientConfig {
    pub client_id: String,
    pub client_secret: Option<String>,
    pub redirect_uri: String,
    pub audience: Option<String>,
}

impl OAuthClientConfig {
    /// Load client configuration from environment variables.
    pub fn from_env() -> Result<Self> {
        let client_id = std::env::var("NBLM_OAUTH_CLIENT_ID")
            .map_err(|_| OAuthError::MissingEnvVar("NBLM_OAUTH_CLIENT_ID"))?;

        let client_secret = std::env::var("NBLM_OAUTH_CLIENT_SECRET").ok();
        let redirect_uri = std::env::var("NBLM_OAUTH_REDIRECT_URI")
            .unwrap_or_else(|_| OAuthConfig::DEFAULT_REDIRECT_URI.to_string());
        let audience = std::env::var("NBLM_OAUTH_AUDIENCE").ok();

        Ok(Self {
            client_id,
            client_secret,
            redirect_uri,
            audience,
        })
    }

    /// Convert this configuration into a complete `OAuthConfig` value.
    pub fn into_oauth_config(self) -> OAuthConfig {
        OAuthConfig {
            auth_endpoint: OAuthConfig::AUTH_ENDPOINT.to_string(),
            token_endpoint: OAuthConfig::TOKEN_ENDPOINT.to_string(),
            client_id: self.client_id,
            client_secret: self.client_secret,
            redirect_uri: self.redirect_uri,
            scopes: vec![
                OAuthConfig::SCOPE_CLOUD_PLATFORM.to_string(),
                OAuthConfig::SCOPE_DRIVE_FILE.to_string(),
            ],
            audience: self.audience,
            additional_params: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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

    #[test]
    #[serial]
    fn from_env_requires_client_id() {
        let _guard = EnvGuard::new("NBLM_OAUTH_CLIENT_ID");
        std::env::remove_var("NBLM_OAUTH_CLIENT_ID");
        let err = OAuthClientConfig::from_env().unwrap_err();
        assert!(matches!(
            err,
            OAuthError::MissingEnvVar("NBLM_OAUTH_CLIENT_ID")
        ));
    }

    #[test]
    #[serial]
    fn from_env_uses_defaults() {
        let _guard_id = EnvGuard::new("NBLM_OAUTH_CLIENT_ID");
        let _guard_secret = EnvGuard::new("NBLM_OAUTH_CLIENT_SECRET");
        let _guard_redirect = EnvGuard::new("NBLM_OAUTH_REDIRECT_URI");
        let _guard_audience = EnvGuard::new("NBLM_OAUTH_AUDIENCE");

        std::env::set_var("NBLM_OAUTH_CLIENT_ID", "client-id");
        std::env::remove_var("NBLM_OAUTH_CLIENT_SECRET");
        std::env::remove_var("NBLM_OAUTH_REDIRECT_URI");
        std::env::remove_var("NBLM_OAUTH_AUDIENCE");

        let config = OAuthClientConfig::from_env().unwrap();
        assert_eq!(config.client_id, "client-id");
        assert_eq!(config.redirect_uri, OAuthConfig::DEFAULT_REDIRECT_URI);
        assert!(config.client_secret.is_none());
        assert!(config.audience.is_none());
    }
}
