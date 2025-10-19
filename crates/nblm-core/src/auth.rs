use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::{env, fs};

use async_trait::async_trait;
use chrono::{Duration as ChronoDuration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::process::Command;
use tokio::sync::Mutex;

use crate::error::{Error, Result};

#[async_trait]
pub trait TokenProvider: Send + Sync {
    async fn access_token(&self) -> Result<String>;
    async fn refresh_token(&self) -> Result<String> {
        self.access_token().await
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
            .map_err(|err| Error::TokenProvider(err.to_string()))?;

        if !output.status.success() {
            return Err(Error::TokenProvider(format!(
                "gcloud exited with status {}",
                output.status
            )));
        }

        let token = String::from_utf8(output.stdout)
            .map_err(|err| Error::TokenProvider(format!("invalid UTF-8 token: {err}")))?;

        Ok(token.trim().to_owned())
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
}

#[derive(Debug, Clone, Deserialize)]
struct ServiceAccountKey {
    #[serde(rename = "client_email")]
    client_email: String,
    #[serde(rename = "private_key")]
    private_key: String,
    #[serde(rename = "token_uri")]
    token_uri: String,
}

#[derive(Debug, Clone)]
struct CachedToken {
    token: String,
    expires_at: Instant,
}

#[derive(Debug, Clone)]
pub struct ServiceAccountTokenProvider {
    key: ServiceAccountKey,
    scopes: Vec<String>,
    cache: Arc<Mutex<Option<CachedToken>>>,
    client: Client,
    leeway: Duration,
    http_timeout: Duration,
}

impl ServiceAccountTokenProvider {
    pub fn from_file(path: impl AsRef<Path>, scopes: Vec<String>) -> Result<Self> {
        let data = fs::read_to_string(path).map_err(|err| Error::TokenProvider(err.to_string()))?;
        Self::from_json(&data, scopes)
    }

    pub fn from_json(data: &str, scopes: Vec<String>) -> Result<Self> {
        let key: ServiceAccountKey = serde_json::from_str(data).map_err(|err| {
            Error::TokenProvider(format!("failed to parse service account key: {err}"))
        })?;
        let client = Client::builder()
            .build()
            .map_err(|err| Error::TokenProvider(format!("failed to build HTTP client: {err}")))?;
        Ok(Self {
            key,
            scopes,
            cache: Arc::new(Mutex::new(None)),
            client,
            leeway: Duration::from_secs(60),
            http_timeout: Duration::from_secs(10),
        })
    }

    pub fn with_leeway(mut self, leeway: Duration) -> Self {
        self.leeway = leeway;
        self
    }

    pub fn with_http_timeout(mut self, timeout: Duration) -> Self {
        self.http_timeout = timeout;
        self
    }

    async fn cached_token(&self) -> Option<String> {
        let cache = self.cache.lock().await;
        cache
            .as_ref()
            .filter(|cached| Instant::now() < cached.expires_at)
            .map(|cached| cached.token.clone())
    }

    async fn store_token(&self, token: String, expires_in: i64) {
        let valid_for = Duration::from_secs(expires_in.max(0) as u64);
        let now = Instant::now();
        let expires_at = now + valid_for;
        let expires_at = expires_at.checked_sub(self.leeway).unwrap_or(now);
        let mut cache = self.cache.lock().await;
        *cache = Some(CachedToken { token, expires_at });
    }

    fn create_jwt(&self) -> Result<String> {
        #[derive(Serialize)]
        struct Claims<'a> {
            iss: &'a str,
            scope: String,
            aud: &'a str,
            exp: i64,
            iat: i64,
        }

        let now = Utc::now();
        let exp = now + ChronoDuration::seconds(3600);
        let claims = Claims {
            iss: &self.key.client_email,
            scope: self.scopes.join(" "),
            aud: &self.key.token_uri,
            exp: exp.timestamp(),
            iat: now.timestamp(),
        };

        let header = Header::new(Algorithm::RS256);
        encode(
            &header,
            &claims,
            &EncodingKey::from_rsa_pem(self.key.private_key.as_bytes())
                .map_err(|err| Error::TokenProvider(err.to_string()))?,
        )
        .map_err(|err| Error::TokenProvider(err.to_string()))
    }
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: i64,
}

#[derive(Serialize)]
struct TokenRequest<'a> {
    grant_type: &'a str,
    assertion: &'a str,
}

#[async_trait]
impl TokenProvider for ServiceAccountTokenProvider {
    async fn access_token(&self) -> Result<String> {
        if let Some(token) = self.cached_token().await {
            return Ok(token);
        }

        let assertion = self.create_jwt()?;
        let body = TokenRequest {
            grant_type: "urn:ietf:params:oauth:grant-type:jwt-bearer",
            assertion: &assertion,
        };

        let response = self
            .client
            .post(&self.key.token_uri)
            .timeout(self.http_timeout)
            .form(&body)
            .send()
            .await
            .map_err(|err| Error::TokenProvider(err.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response
                .text()
                .await
                .unwrap_or_else(|_| "<failed to read body>".to_string());
            return Err(Error::TokenProvider(format!(
                "token endpoint error {}: {}",
                status, text
            )));
        }

        let token_response: TokenResponse = response
            .json()
            .await
            .map_err(|err| Error::TokenProvider(format!("invalid token response: {err}")))?;

        self.store_token(
            token_response.access_token.clone(),
            token_response.expires_in,
        )
        .await;
        Ok(token_response.access_token)
    }

    async fn refresh_token(&self) -> Result<String> {
        {
            let mut cache = self.cache.lock().await;
            *cache = None;
        }
        self.access_token().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_KEY: &str = r#"{
        "client_email": "test@example.com",
        "private_key": "-----BEGIN PRIVATE KEY-----\nTEST\n-----END PRIVATE KEY-----\n",
        "token_uri": "https://example.com"
    }"#;

    fn provider() -> ServiceAccountTokenProvider {
        ServiceAccountTokenProvider::from_json(TEST_KEY, vec!["scope".to_string()]).unwrap()
    }

    #[tokio::test]
    async fn service_account_cache_respects_leeway() {
        let provider = provider().with_leeway(Duration::from_secs(90));
        provider.store_token("token".to_string(), 60).await;
        assert!(provider.cached_token().await.is_none());
    }

    #[tokio::test]
    async fn service_account_cache_keeps_valid_token() {
        let provider = provider();
        provider.store_token("token".to_string(), 120).await;
        assert_eq!(provider.cached_token().await, Some("token".to_string()));
    }
}
