pub(crate) mod backends;

use crate::client::NblmClient;
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

impl NblmClient {
    pub async fn create_notebook(&self, title: impl Into<String>) -> Result<Notebook> {
        self.backends
            .notebooks()
            .create_notebook(title.into())
            .await
    }

    pub async fn batch_delete_notebooks(
        &self,
        request: BatchDeleteNotebooksRequest,
    ) -> Result<BatchDeleteNotebooksResponse> {
        self.backends
            .notebooks()
            .batch_delete_notebooks(request)
            .await
    }

    pub async fn delete_notebooks(
        &self,
        notebook_names: Vec<String>,
    ) -> Result<BatchDeleteNotebooksResponse> {
        self.backends
            .notebooks()
            .delete_notebooks(notebook_names)
            .await
    }

    pub async fn share_notebook(
        &self,
        notebook_id: &str,
        accounts: Vec<AccountRole>,
    ) -> Result<ShareResponse> {
        self.backends
            .notebooks()
            .share_notebook(notebook_id, accounts)
            .await
    }

    pub async fn list_recently_viewed(
        &self,
        page_size: Option<u32>,
    ) -> Result<ListRecentlyViewedResponse> {
        self.backends
            .notebooks()
            .list_recently_viewed(page_size)
            .await
    }

    pub async fn batch_create_sources(
        &self,
        notebook_id: &str,
        request: BatchCreateSourcesRequest,
    ) -> Result<BatchCreateSourcesResponse> {
        let includes_drive = has_drive_content(request.user_contents.iter());
        self.ensure_drive_scope_if_needed(includes_drive).await?;
        self.backends
            .sources()
            .batch_create_sources(notebook_id, request)
            .await
    }

    pub async fn add_sources(
        &self,
        notebook_id: &str,
        contents: Vec<UserContent>,
    ) -> Result<BatchCreateSourcesResponse> {
        let includes_drive = has_drive_content(contents.iter());
        self.ensure_drive_scope_if_needed(includes_drive).await?;
        self.backends
            .sources()
            .add_sources(notebook_id, contents)
            .await
    }

    pub async fn batch_delete_sources(
        &self,
        notebook_id: &str,
        request: BatchDeleteSourcesRequest,
    ) -> Result<BatchDeleteSourcesResponse> {
        self.backends
            .sources()
            .batch_delete_sources(notebook_id, request)
            .await
    }

    pub async fn delete_sources(
        &self,
        notebook_id: &str,
        source_names: Vec<String>,
    ) -> Result<BatchDeleteSourcesResponse> {
        self.backends
            .sources()
            .delete_sources(notebook_id, source_names)
            .await
    }

    pub async fn upload_source_file(
        &self,
        notebook_id: &str,
        file_name: &str,
        content_type: &str,
        data: Vec<u8>,
    ) -> Result<UploadSourceFileResponse> {
        self.backends
            .sources()
            .upload_source_file(notebook_id, file_name, content_type, data)
            .await
    }

    pub async fn get_source(&self, notebook_id: &str, source_id: &str) -> Result<NotebookSource> {
        self.backends
            .sources()
            .get_source(notebook_id, source_id)
            .await
    }

    pub async fn create_audio_overview(
        &self,
        notebook_id: &str,
        request: AudioOverviewRequest,
    ) -> Result<AudioOverviewResponse> {
        self.backends
            .audio()
            .create_audio_overview(notebook_id, request)
            .await
    }

    pub async fn delete_audio_overview(&self, notebook_id: &str) -> Result<()> {
        self.backends
            .audio()
            .delete_audio_overview(notebook_id)
            .await
    }
}

fn has_drive_content<'a, I>(contents: I) -> bool
where
    I: IntoIterator<Item = &'a UserContent>,
{
    contents
        .into_iter()
        .any(|content| matches!(content, UserContent::GoogleDrive { .. }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::StaticTokenProvider;
    use crate::env::EnvironmentConfig;
    use crate::error::Error;
    use serde_json::json;
    use serial_test::serial;
    use std::sync::Arc;
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    struct EnvGuard {
        key: &'static str,
        original: Option<String>,
    }

    impl EnvGuard {
        fn new(key: &'static str) -> Self {
            let original = std::env::var(key).ok();
            Self { key, original }
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

    async fn build_client(base_url: &str) -> NblmClient {
        let provider = Arc::new(StaticTokenProvider::new("test-token"));
        let env = EnvironmentConfig::enterprise("123", "global", "us").unwrap();
        NblmClient::new(provider, env)
            .unwrap()
            .with_base_url(base_url)
            .unwrap()
    }

    #[rstest::rstest]
    #[case::with_drive_scope("https://www.googleapis.com/auth/drive.file", true, 1)]
    #[case::without_drive_scope("https://www.googleapis.com/auth/cloud-platform", false, 0)]
    #[tokio::test]
    #[serial]
    async fn add_sources_validates_drive_scope(
        #[case] scope: &str,
        #[case] should_succeed: bool,
        #[case] api_call_count: u64,
    ) {
        let server = MockServer::start().await;
        let tokeninfo_url = format!("{}/tokeninfo", server.uri());
        let _guard = EnvGuard::new("NBLM_TOKENINFO_ENDPOINT");
        std::env::set_var("NBLM_TOKENINFO_ENDPOINT", &tokeninfo_url);

        Mock::given(method("GET"))
            .and(path("/tokeninfo"))
            .and(query_param("access_token", "test-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "scope": scope
            })))
            .expect(1)
            .mount(&server)
            .await;

        Mock::given(method("POST"))
            .and(path(
                "/v1alpha/projects/123/locations/global/notebooks/notebook-id/sources:batchCreate",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "sources": [],
                "errorCount": 0
            })))
            .expect(api_call_count)
            .mount(&server)
            .await;

        let client = build_client(&format!("{}/v1alpha", server.uri())).await;

        let result = client
            .add_sources(
                "notebook-id",
                vec![UserContent::google_drive(
                    "doc".to_string(),
                    "application/pdf".to_string(),
                    None,
                )],
            )
            .await;

        if should_succeed {
            assert!(
                result.is_ok(),
                "expected add_sources to succeed: {:?}",
                result
            );
        } else {
            let err = result.expect_err("expected add_sources to fail when drive scope is missing");
            match err {
                Error::TokenProvider(message) => {
                    assert!(
                        message.contains("drive.file"),
                        "unexpected message: {message}"
                    );
                }
                other => panic!("expected TokenProvider error, got {other:?}"),
            }
        }
    }

    #[tokio::test]
    #[serial]
    async fn batch_create_sources_validates_drive_scope() {
        let server = MockServer::start().await;
        let tokeninfo_url = format!("{}/tokeninfo", server.uri());
        let _guard = EnvGuard::new("NBLM_TOKENINFO_ENDPOINT");
        std::env::set_var("NBLM_TOKENINFO_ENDPOINT", &tokeninfo_url);

        Mock::given(method("GET"))
            .and(path("/tokeninfo"))
            .and(query_param("access_token", "test-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "scope": "https://www.googleapis.com/auth/drive"
            })))
            .expect(1)
            .mount(&server)
            .await;

        Mock::given(method("POST"))
            .and(path(
                "/v1alpha/projects/123/locations/global/notebooks/notebook-id/sources:batchCreate",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "sources": [],
                "errorCount": 0
            })))
            .expect(1)
            .mount(&server)
            .await;

        let client = build_client(&format!("{}/v1alpha", server.uri())).await;

        let request = BatchCreateSourcesRequest {
            user_contents: vec![UserContent::google_drive(
                "doc".to_string(),
                "application/pdf".to_string(),
                None,
            )],
        };

        let result = client
            .batch_create_sources("notebook-id", request)
            .await
            .expect("expected batch_create_sources to succeed");

        assert!(result.sources.is_empty());
    }
}
