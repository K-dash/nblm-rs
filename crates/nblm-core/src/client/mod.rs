use std::{sync::Arc, time::Duration};

use reqwest::{Client, Url};

use crate::auth::TokenProvider;
use crate::error::Result;

mod api;
mod http;
mod retry;
mod url_builder;

pub use self::retry::{RetryConfig, Retryer};

use self::http::HttpClient;
use self::url_builder::{normalize_endpoint_location, UrlBuilder};

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

pub struct NblmClient {
    pub(self) http: HttpClient,
    pub(self) url_builder: UrlBuilder,
    timeout: Duration,
}

impl NblmClient {
    pub fn new(
        token_provider: Arc<dyn TokenProvider>,
        project_number: impl Into<String>,
        location: impl Into<String>,
        endpoint_location: impl Into<String>,
    ) -> Result<Self> {
        let project_number = project_number.into();
        let location = location.into();
        let endpoint_location = endpoint_location.into();
        let base = format!(
            "https://{}discoveryengine.googleapis.com/v1alpha",
            normalize_endpoint_location(endpoint_location)?
        );
        let parent = format!("projects/{}/locations/{}", project_number, location);

        let client = Client::builder()
            .user_agent(concat!("nblm-cli/", env!("CARGO_PKG_VERSION")))
            .timeout(DEFAULT_TIMEOUT)
            .build()
            .map_err(crate::error::Error::from)?;

        let retryer = Retryer::new(RetryConfig::default());
        let http = HttpClient::new(client, token_provider, retryer, None);
        let url_builder = UrlBuilder::new(base, parent);

        Ok(Self {
            http,
            url_builder,
            timeout: DEFAULT_TIMEOUT,
        })
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
        self.http = HttpClient::new(client, token_provider, retryer, user_project);
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
        self.http = HttpClient::new(client, token_provider, retryer, user_project);
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
        self.http = HttpClient::new(client, token_provider, retryer, user_project);
        self
    }

    /// Override API base URL (for tests). Accepts absolute URL. Trims trailing slash.
    pub fn with_base_url(mut self, base: impl Into<String>) -> Result<Self> {
        let base = base.into().trim().trim_end_matches('/').to_string();
        // Basic sanity check: absolute URL
        let _ = Url::parse(&base).map_err(crate::error::Error::from)?;
        let parent = self.url_builder.parent.clone();
        self.url_builder = UrlBuilder::new(base, parent);
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_base_url_accepts_absolute_url() {
        let provider = Arc::new(crate::auth::StaticTokenProvider::new("test"));
        let client = NblmClient::new(provider, "123", "global", "us").unwrap();
        let result = client.with_base_url("http://localhost:8080/v1alpha");
        assert!(result.is_ok());
    }

    #[test]
    fn with_base_url_trims_trailing_slash() {
        let provider = Arc::new(crate::auth::StaticTokenProvider::new("test"));
        let client = NblmClient::new(provider, "123", "global", "us")
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
        let client = NblmClient::new(provider, "123", "global", "us").unwrap();
        let result = client.with_base_url("/relative/path");
        assert!(result.is_err());
    }
}
