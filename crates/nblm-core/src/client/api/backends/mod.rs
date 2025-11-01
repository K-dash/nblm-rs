pub(crate) mod enterprise;

use std::sync::Arc;

use async_trait::async_trait;

use crate::client::{http::HttpClient, url::UrlBuilder};
use crate::env::ApiProfile;
use crate::error::Result;
use crate::models::enterprise::{
    audio::{AudioOverviewRequest, AudioOverviewResponse},
    notebook::{
        BatchDeleteNotebooksRequest, BatchDeleteNotebooksResponse, ListRecentlyViewedResponse,
        Notebook,
    },
    share::{AccountRole, ShareResponse},
    source::{
        BatchCreateSourcesRequest, BatchCreateSourcesResponse, BatchDeleteSourcesRequest,
        BatchDeleteSourcesResponse, NotebookSource, UploadSourceFileResponse, UserContent,
    },
};

pub(crate) struct BackendContext {
    pub http: Arc<HttpClient>,
    pub url_builder: Arc<dyn UrlBuilder>,
}

impl Clone for BackendContext {
    fn clone(&self) -> Self {
        Self {
            http: Arc::clone(&self.http),
            url_builder: Arc::clone(&self.url_builder),
        }
    }
}

impl BackendContext {
    pub fn new(http: Arc<HttpClient>, url_builder: Arc<dyn UrlBuilder>) -> Self {
        Self { http, url_builder }
    }
}

#[async_trait]
pub(crate) trait NotebooksBackend: Send + Sync + 'static {
    async fn create_notebook(&self, title: String) -> Result<Notebook>;
    async fn batch_delete_notebooks(
        &self,
        request: BatchDeleteNotebooksRequest,
    ) -> Result<BatchDeleteNotebooksResponse>;
    async fn delete_notebooks(
        &self,
        notebook_names: Vec<String>,
    ) -> Result<BatchDeleteNotebooksResponse>;
    async fn share_notebook(
        &self,
        notebook_id: &str,
        accounts: Vec<AccountRole>,
    ) -> Result<ShareResponse>;
    async fn list_recently_viewed(
        &self,
        page_size: Option<u32>,
    ) -> Result<ListRecentlyViewedResponse>;
}

#[async_trait]
pub(crate) trait SourcesBackend: Send + Sync + 'static {
    async fn batch_create_sources(
        &self,
        notebook_id: &str,
        request: BatchCreateSourcesRequest,
    ) -> Result<BatchCreateSourcesResponse>;
    async fn add_sources(
        &self,
        notebook_id: &str,
        contents: Vec<UserContent>,
    ) -> Result<BatchCreateSourcesResponse>;
    async fn batch_delete_sources(
        &self,
        notebook_id: &str,
        request: BatchDeleteSourcesRequest,
    ) -> Result<BatchDeleteSourcesResponse>;
    async fn delete_sources(
        &self,
        notebook_id: &str,
        source_names: Vec<String>,
    ) -> Result<BatchDeleteSourcesResponse>;
    async fn upload_source_file(
        &self,
        notebook_id: &str,
        file_name: &str,
        content_type: &str,
        data: Vec<u8>,
    ) -> Result<UploadSourceFileResponse>;
    async fn get_source(&self, notebook_id: &str, source_id: &str) -> Result<NotebookSource>;
}

#[async_trait]
pub(crate) trait AudioBackend: Send + Sync + 'static {
    async fn create_audio_overview(
        &self,
        notebook_id: &str,
        request: AudioOverviewRequest,
    ) -> Result<AudioOverviewResponse>;
    async fn delete_audio_overview(&self, notebook_id: &str) -> Result<()>;
}

pub(crate) struct ClientBackends {
    notebooks: Arc<dyn NotebooksBackend>,
    sources: Arc<dyn SourcesBackend>,
    audio: Arc<dyn AudioBackend>,
}

impl ClientBackends {
    pub fn new(profile: ApiProfile, ctx: BackendContext) -> Self {
        match profile {
            ApiProfile::Enterprise => {
                let notebooks = Arc::new(enterprise::EnterpriseNotebooksBackend::new(ctx.clone()))
                    as Arc<dyn NotebooksBackend>;
                let sources = Arc::new(enterprise::EnterpriseSourcesBackend::new(ctx.clone()))
                    as Arc<dyn SourcesBackend>;
                let audio =
                    Arc::new(enterprise::EnterpriseAudioBackend::new(ctx)) as Arc<dyn AudioBackend>;
                Self {
                    notebooks,
                    sources,
                    audio,
                }
            }
            ApiProfile::Personal | ApiProfile::Workspace => {
                unimplemented!(
                    "Client backends for profile '{}' are not implemented",
                    profile.as_str()
                )
            }
        }
    }

    pub fn notebooks(&self) -> &Arc<dyn NotebooksBackend> {
        &self.notebooks
    }

    pub fn sources(&self) -> &Arc<dyn SourcesBackend> {
        &self.sources
    }

    pub fn audio(&self) -> &Arc<dyn AudioBackend> {
        &self.audio
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::StaticTokenProvider;
    use crate::client::{RetryConfig, Retryer};
    use crate::error::Error;
    use crate::EnvironmentConfig;
    use std::time::Duration;

    fn create_test_context() -> BackendContext {
        let env = EnvironmentConfig::enterprise("123", "global", "us").unwrap();
        let client = reqwest::Client::builder()
            .timeout(Duration::from_millis(10))
            .build()
            .unwrap();
        let token = Arc::new(StaticTokenProvider::new("token"));
        let retryer = Retryer::new(RetryConfig::default());
        let http = Arc::new(HttpClient::new(client, token, retryer, None));
        let url_builder = crate::client::url::new_url_builder(
            env.profile(),
            env.base_url().to_string(),
            env.parent_path().to_string(),
        );
        BackendContext::new(http, url_builder)
    }

    #[tokio::test]
    async fn enterprise_profile_builds_enterprise_backends() {
        let ctx = create_test_context();
        let backends = ClientBackends::new(ApiProfile::Enterprise, ctx);

        let delete_response = backends
            .notebooks()
            .delete_notebooks(Vec::new())
            .await
            .expect("enterprise notebooks backend should handle empty deletions");
        assert!(delete_response.extra.is_empty());

        let upload_error = backends
            .sources()
            .upload_source_file("", "", "", vec![])
            .await
            .expect_err("enterprise sources backend should validate inputs");
        assert!(matches!(upload_error, Error::Validation(_)));

        // Audio backend exists and can be retrieved.
        assert!(Arc::strong_count(backends.audio()) >= 1);
    }

    #[test]
    fn backend_context_construction() {
        let ctx = create_test_context();
        assert!(Arc::strong_count(&ctx.http) >= 1);
        assert!(Arc::strong_count(&ctx.url_builder) >= 1);
    }

    #[test]
    fn backend_context_can_be_cloned() {
        let ctx1 = create_test_context();
        let ctx2 = ctx1.clone();
        assert!(Arc::ptr_eq(&ctx1.http, &ctx2.http));
        assert!(Arc::ptr_eq(&ctx1.url_builder, &ctx2.url_builder));
    }

    #[test]
    fn client_backends_provides_all_three_backends() {
        let ctx = create_test_context();
        let backends = ClientBackends::new(ApiProfile::Enterprise, ctx);
        assert!(Arc::strong_count(backends.notebooks()) >= 1);
        assert!(Arc::strong_count(backends.sources()) >= 1);
        assert!(Arc::strong_count(backends.audio()) >= 1);
    }

    #[test]
    fn enterprise_profile_returns_correct_backend_types() {
        let ctx = create_test_context();
        let backends = ClientBackends::new(ApiProfile::Enterprise, ctx);
        let notebooks = backends.notebooks();
        let sources = backends.sources();
        let audio = backends.audio();
        assert!(Arc::strong_count(notebooks) >= 1);
        assert!(Arc::strong_count(sources) >= 1);
        assert!(Arc::strong_count(audio) >= 1);
    }
}
