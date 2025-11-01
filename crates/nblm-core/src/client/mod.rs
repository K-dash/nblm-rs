use std::{sync::Arc, time::Duration};

use reqwest::{Client, Url};

use crate::auth::{ensure_drive_scope, TokenProvider};
use crate::env::EnvironmentConfig;
use crate::error::Result;

mod api;
mod http;
mod retry;
mod url;

pub use self::retry::{RetryConfig, Retryer};

use self::api::backends::{BackendContext, ClientBackends};
use self::http::HttpClient;
use self::url::{new_url_builder, UrlBuilder};

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

pub struct NblmClient {
    pub(self) http: Arc<HttpClient>,
    pub(self) url_builder: Arc<dyn UrlBuilder>,
    backends: ClientBackends,
    environment: EnvironmentConfig,
    timeout: Duration,
}

impl NblmClient {
    pub fn new(
        token_provider: Arc<dyn TokenProvider>,
        environment: EnvironmentConfig,
    ) -> Result<Self> {
        let client = Client::builder()
            .user_agent(concat!("nblm-cli/", env!("CARGO_PKG_VERSION")))
            .timeout(DEFAULT_TIMEOUT)
            .build()
            .map_err(crate::error::Error::from)?;

        let retryer = Retryer::new(RetryConfig::default());
        let http = Arc::new(HttpClient::new(client, token_provider, retryer, None));
        let url_builder = new_url_builder(
            environment.profile(),
            environment.base_url().to_string(),
            environment.parent_path().to_string(),
        );
        let ctx = BackendContext::new(Arc::clone(&http), Arc::clone(&url_builder));
        let backends = ClientBackends::new(environment.profile(), ctx);

        Ok(Self {
            http,
            url_builder,
            backends,
            environment,
            timeout: DEFAULT_TIMEOUT,
        })
    }

    #[deprecated(note = "Use EnvironmentConfig::enterprise(...) with NblmClient::new")]
    pub fn new_enterprise(
        token_provider: Arc<dyn TokenProvider>,
        project_number: impl Into<String>,
        location: impl Into<String>,
        endpoint_location: impl Into<String>,
    ) -> Result<Self> {
        let env = EnvironmentConfig::enterprise(project_number, location, endpoint_location)?;
        Self::new(token_provider, env)
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        // Update the underlying HTTP client's timeout
        let client = Client::builder()
            .user_agent(concat!("nblm-cli/", env!("CARGO_PKG_VERSION")))
            .timeout(timeout)
            .build()
            .expect("Failed to rebuild client with new timeout");

        let token_provider = Arc::clone(&self.http.token_provider);
        let retryer = self.http.retryer.clone();
        let user_project = self.http.user_project.clone();
        self.http = Arc::new(HttpClient::new(
            client,
            token_provider,
            retryer,
            user_project,
        ));
        self.rebuild_backends();
        self
    }

    pub fn with_retry_config(mut self, config: RetryConfig) -> Self {
        let client = Client::builder()
            .user_agent(concat!("nblm-cli/", env!("CARGO_PKG_VERSION")))
            .timeout(self.timeout)
            .build()
            .expect("Failed to rebuild client");

        let token_provider = Arc::clone(&self.http.token_provider);
        let retryer = Retryer::new(config);
        let user_project = self.http.user_project.clone();
        self.http = Arc::new(HttpClient::new(
            client,
            token_provider,
            retryer,
            user_project,
        ));
        self.rebuild_backends();
        self
    }

    pub fn with_user_project(mut self, project: impl Into<String>) -> Self {
        let client = Client::builder()
            .user_agent(concat!("nblm-cli/", env!("CARGO_PKG_VERSION")))
            .timeout(self.timeout)
            .build()
            .expect("Failed to rebuild client");

        let token_provider = Arc::clone(&self.http.token_provider);
        let retryer = self.http.retryer.clone();
        let user_project = Some(project.into());
        self.http = Arc::new(HttpClient::new(
            client,
            token_provider,
            retryer,
            user_project,
        ));
        self.rebuild_backends();
        self
    }

    /// Override API base URL (for tests). Accepts absolute URL. Trims trailing slash.
    pub fn with_base_url(mut self, base: impl Into<String>) -> Result<Self> {
        let base = base.into().trim().trim_end_matches('/').to_string();
        // Basic sanity check: absolute URL
        let _ = Url::parse(&base).map_err(crate::error::Error::from)?;
        self.environment = self.environment.clone().with_base_url(base.clone());
        let parent = self.environment.parent_path().to_string();
        self.url_builder = new_url_builder(self.environment.profile(), base, parent);
        self.rebuild_backends();
        Ok(self)
    }
}

impl NblmClient {
    fn rebuild_backends(&mut self) {
        let ctx = BackendContext::new(Arc::clone(&self.http), Arc::clone(&self.url_builder));
        self.backends = ClientBackends::new(self.environment.profile(), ctx);
    }

    pub(crate) async fn ensure_drive_scope_if_needed(&self, includes_drive: bool) -> Result<()> {
        if includes_drive {
            ensure_drive_scope(self.http.token_provider.as_ref()).await?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_base_url_accepts_absolute_url() {
        let provider = Arc::new(crate::auth::StaticTokenProvider::new("test"));
        let env = EnvironmentConfig::enterprise("123", "global", "us").unwrap();
        let client = NblmClient::new(provider, env).unwrap();
        let result = client.with_base_url("http://localhost:8080/v1alpha");
        assert!(result.is_ok());
    }

    #[test]
    fn with_base_url_trims_trailing_slash() {
        let provider = Arc::new(crate::auth::StaticTokenProvider::new("test"));
        let env = EnvironmentConfig::enterprise("123", "global", "us").unwrap();
        let client = NblmClient::new(provider, env)
            .unwrap()
            .with_base_url("http://example.com/v1alpha/")
            .unwrap();

        // Test that URL building works correctly
        let url = client.url_builder.build_url("/test").unwrap();
        assert_eq!(url.as_str(), "http://example.com/v1alpha/test");
    }

    #[test]
    fn with_base_url_rejects_relative_path() {
        let provider = Arc::new(crate::auth::StaticTokenProvider::new("test"));
        let env = EnvironmentConfig::enterprise("123", "global", "us").unwrap();
        let client = NblmClient::new(provider, env).unwrap();
        let result = client.with_base_url("/relative/path");
        assert!(result.is_err());
    }

    #[test]
    #[allow(deprecated)]
    fn new_enterprise_constructs_client_correctly() {
        let provider = Arc::new(crate::auth::StaticTokenProvider::new("test"));
        let client = NblmClient::new_enterprise(provider, "123", "global", "us").unwrap();

        // Verify base URL is constructed correctly
        let url = client.url_builder.build_url("/test").unwrap();
        assert!(url
            .as_str()
            .starts_with("https://us-discoveryengine.googleapis.com/v1alpha"));

        // Verify parent path is set correctly
        let notebooks_url = client.url_builder.notebooks_collection();
        assert_eq!(notebooks_url, "projects/123/locations/global/notebooks");
    }

    #[test]
    #[allow(deprecated)]
    fn new_enterprise_handles_invalid_endpoint() {
        let provider = Arc::new(crate::auth::StaticTokenProvider::new("test"));
        let result = NblmClient::new_enterprise(provider, "123", "global", "invalid");
        assert!(result.is_err());
    }
}
