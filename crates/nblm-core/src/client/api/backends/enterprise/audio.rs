use async_trait::async_trait;
use reqwest::Method;

use crate::client::api::backends::{AudioBackend, BackendContext};
use crate::error::Result;
use crate::models::{AudioOverviewRequest, AudioOverviewResponse};

pub(crate) struct EnterpriseAudioBackend {
    ctx: BackendContext,
}

impl EnterpriseAudioBackend {
    pub fn new(ctx: BackendContext) -> Self {
        Self { ctx }
    }
}

#[async_trait]
impl AudioBackend for EnterpriseAudioBackend {
    async fn create_audio_overview(
        &self,
        notebook_id: &str,
        request: AudioOverviewRequest,
    ) -> Result<AudioOverviewResponse> {
        let path = format!(
            "{}/audioOverviews",
            self.ctx.url_builder.notebook_path(notebook_id)
        );
        let url = self.ctx.url_builder.build_url(&path)?;

        let api_response: crate::models::responses::audio::AudioOverviewApiResponse = self
            .ctx
            .http
            .request_json(Method::POST, url, Some(&request))
            .await?;

        Ok(api_response.audio_overview)
    }

    async fn delete_audio_overview(&self, notebook_id: &str) -> Result<()> {
        let path = format!(
            "{}/audioOverviews/default",
            self.ctx.url_builder.notebook_path(notebook_id)
        );
        let url = self.ctx.url_builder.build_url(&path)?;
        let _response: serde_json::Value = self
            .ctx
            .http
            .request_json(Method::DELETE, url, None::<&()>)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::StaticTokenProvider;
    use crate::client::http::HttpClient;
    use crate::client::url_builder::UrlBuilder;
    use crate::client::{RetryConfig, Retryer};
    use crate::env::EnvironmentConfig;
    use std::sync::Arc;
    use std::time::Duration;

    fn create_test_backend() -> EnterpriseAudioBackend {
        let env = EnvironmentConfig::enterprise("123", "global", "us").unwrap();
        let client = reqwest::Client::builder()
            .timeout(Duration::from_millis(10))
            .build()
            .unwrap();
        let token = Arc::new(StaticTokenProvider::new("token"));
        let retryer = Retryer::new(RetryConfig::default());
        let http = Arc::new(HttpClient::new(client, token, retryer, None));
        let url_builder = Arc::new(UrlBuilder::new(
            env.base_url().to_string(),
            env.parent_path().to_string(),
        ));
        let ctx = BackendContext::new(http, url_builder);
        EnterpriseAudioBackend::new(ctx)
    }

    #[test]
    fn create_audio_overview_url_construction() {
        let backend = create_test_backend();
        let path = format!(
            "{}/audioOverviews",
            backend.ctx.url_builder.notebook_path("test-notebook")
        );
        let url = backend.ctx.url_builder.build_url(&path).unwrap();
        assert!(url.as_str().contains("test-notebook"));
        assert!(url.as_str().contains("audioOverviews"));
        assert!(!url.as_str().contains("default"));
    }

    #[test]
    fn delete_audio_overview_url_construction() {
        let backend = create_test_backend();
        let path = format!(
            "{}/audioOverviews/default",
            backend.ctx.url_builder.notebook_path("test-notebook")
        );
        let url = backend.ctx.url_builder.build_url(&path).unwrap();
        assert!(url.as_str().contains("test-notebook"));
        assert!(url.as_str().contains("audioOverviews/default"));
    }

    #[test]
    fn backend_construction() {
        let backend = create_test_backend();
        assert!(Arc::strong_count(&backend.ctx.http) >= 1);
        assert!(Arc::strong_count(&backend.ctx.url_builder) >= 1);
    }
}
