use reqwest::Method;

use crate::error::Result;
use crate::models::{AudioOverviewRequest, AudioOverviewResponse};

use crate::client::NblmClient;

/// Audio-related API implementations
impl NblmClient {
    pub async fn create_audio_overview(
        &self,
        notebook_id: &str,
        request: AudioOverviewRequest,
    ) -> Result<AudioOverviewResponse> {
        let path = format!(
            "{}/audioOverviews",
            self.url_builder.notebook_path(notebook_id)
        );
        let url = self.url_builder.build_url(&path)?;
        self.http
            .request_json(Method::POST, url, Some(&request))
            .await
    }

    pub async fn delete_audio_overview(&self, notebook_id: &str) -> Result<()> {
        let path = format!(
            "{}/audioOverviews/default",
            self.url_builder.notebook_path(notebook_id)
        );
        let url = self.url_builder.build_url(&path)?;
        let _response: serde_json::Value = self
            .http
            .request_json(Method::DELETE, url, None::<&()>)
            .await?;
        Ok(())
    }
}
