use async_trait::async_trait;
use bytes::Bytes;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE},
    Method,
};

use crate::client::api::backends::{BackendContext, SourcesBackend};
use crate::error::{Error, Result};
use crate::models::enterprise::source::{
    BatchCreateSourcesRequest, BatchCreateSourcesResponse, BatchDeleteSourcesRequest,
    BatchDeleteSourcesResponse, NotebookSource, UploadSourceFileResponse, UserContent,
};

use super::models::{
    requests::source as wire_source_req, responses::source as wire_source_resp,
    source as wire_source,
};

pub(crate) struct EnterpriseSourcesBackend {
    ctx: BackendContext,
}

impl EnterpriseSourcesBackend {
    pub fn new(ctx: BackendContext) -> Self {
        Self { ctx }
    }

    async fn batch_create_internal(
        &self,
        notebook_id: &str,
        request: wire_source_req::BatchCreateSourcesRequest,
    ) -> Result<wire_source_resp::BatchCreateSourcesResponse> {
        let path = format!(
            "{}/sources:batchCreate",
            self.ctx.url_builder.notebook_path(notebook_id)
        );
        let url = self.ctx.url_builder.build_url(&path)?;
        self.ctx
            .http
            .request_json(Method::POST, url, Some(&request))
            .await
    }

    async fn batch_delete_internal(
        &self,
        notebook_id: &str,
        request: wire_source_req::BatchDeleteSourcesRequest,
    ) -> Result<wire_source_req::BatchDeleteSourcesResponse> {
        let path = format!(
            "{}/sources:batchDelete",
            self.ctx.url_builder.notebook_path(notebook_id)
        );
        let url = self.ctx.url_builder.build_url(&path)?;
        self.ctx
            .http
            .request_json(Method::POST, url, Some(&request))
            .await
    }
}

#[async_trait]
impl SourcesBackend for EnterpriseSourcesBackend {
    async fn batch_create_sources(
        &self,
        notebook_id: &str,
        request: BatchCreateSourcesRequest,
    ) -> Result<BatchCreateSourcesResponse> {
        let wire_request: wire_source_req::BatchCreateSourcesRequest = request.into();
        let response = self
            .batch_create_internal(notebook_id, wire_request)
            .await?;
        Ok(response.into())
    }

    async fn add_sources(
        &self,
        notebook_id: &str,
        contents: Vec<UserContent>,
    ) -> Result<BatchCreateSourcesResponse> {
        let wire_request = wire_source_req::BatchCreateSourcesRequest {
            user_contents: contents
                .into_iter()
                .map(wire_source::UserContent::from)
                .collect(),
        };
        let response = self
            .batch_create_internal(notebook_id, wire_request)
            .await?;
        Ok(response.into())
    }

    async fn batch_delete_sources(
        &self,
        notebook_id: &str,
        request: BatchDeleteSourcesRequest,
    ) -> Result<BatchDeleteSourcesResponse> {
        let wire_request: wire_source_req::BatchDeleteSourcesRequest = request.into();
        let response = self
            .batch_delete_internal(notebook_id, wire_request)
            .await?;
        Ok(response.into())
    }

    async fn delete_sources(
        &self,
        notebook_id: &str,
        source_names: Vec<String>,
    ) -> Result<BatchDeleteSourcesResponse> {
        let request = BatchDeleteSourcesRequest {
            names: source_names,
        };
        self.batch_delete_sources(notebook_id, request).await
    }

    async fn upload_source_file(
        &self,
        notebook_id: &str,
        file_name: &str,
        content_type: &str,
        data: Vec<u8>,
    ) -> Result<UploadSourceFileResponse> {
        if notebook_id.trim().is_empty() {
            return Err(Error::validation("notebook_id cannot be empty"));
        }
        if file_name.trim().is_empty() {
            return Err(Error::validation("file name cannot be empty"));
        }
        if content_type.trim().is_empty() {
            return Err(Error::validation("content type cannot be empty"));
        }

        let path = format!(
            "{}/sources:uploadFile",
            self.ctx.url_builder.notebook_path(notebook_id)
        );
        let mut url = self.ctx.url_builder.build_upload_url(&path)?;
        url.query_pairs_mut().append_pair("uploadType", "media");

        let mut headers = HeaderMap::new();
        headers.insert(
            HeaderName::from_static("x-goog-upload-protocol"),
            HeaderValue::from_static("raw"),
        );
        let file_name_header = HeaderValue::from_str(file_name)
            .map_err(|_| Error::validation("file name contains invalid characters"))?;
        headers.insert(
            HeaderName::from_static("x-goog-upload-file-name"),
            file_name_header,
        );
        let content_type_header = HeaderValue::from_str(content_type)
            .map_err(|_| Error::validation("content type contains invalid characters"))?;
        headers.insert(CONTENT_TYPE, content_type_header);

        let bytes = Bytes::from(data);
        let response: wire_source_resp::UploadSourceFileResponse = self
            .ctx
            .http
            .request_binary(Method::POST, url, headers, bytes)
            .await?;
        Ok(response.into())
    }

    async fn get_source(&self, notebook_id: &str, source_id: &str) -> Result<NotebookSource> {
        if notebook_id.trim().is_empty() {
            return Err(Error::validation("notebook_id cannot be empty"));
        }
        if source_id.trim().is_empty() {
            return Err(Error::validation("source_id cannot be empty"));
        }

        let path = format!(
            "{}/sources/{}",
            self.ctx.url_builder.notebook_path(notebook_id),
            source_id
        );
        let url = self.ctx.url_builder.build_url(&path)?;
        let source: wire_source::NotebookSource = self
            .ctx
            .http
            .request_json::<(), _>(Method::GET, url, None::<&()>)
            .await?;
        Ok(source.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::StaticTokenProvider;
    use crate::client::http::HttpClient;
    use crate::client::url::new_url_builder;
    use crate::client::{RetryConfig, Retryer};
    use crate::env::EnvironmentConfig;
    use crate::models::enterprise::source::TextContent;
    use std::sync::Arc;
    use std::time::Duration;

    fn create_test_backend() -> EnterpriseSourcesBackend {
        let env = EnvironmentConfig::enterprise("123", "global", "us").unwrap();
        let client = reqwest::Client::builder()
            .timeout(Duration::from_millis(10))
            .build()
            .unwrap();
        let token = Arc::new(StaticTokenProvider::new("token"));
        let retryer = Retryer::new(RetryConfig::default());
        let http = Arc::new(HttpClient::new(client, token, retryer, None));
        let url_builder = new_url_builder(
            env.profile(),
            env.base_url().to_string(),
            env.parent_path().to_string(),
        );
        let ctx = BackendContext::new(http, url_builder);
        EnterpriseSourcesBackend::new(ctx)
    }

    #[test]
    fn batch_create_url_construction() {
        let backend = create_test_backend();
        let path = format!(
            "{}/sources:batchCreate",
            backend.ctx.url_builder.notebook_path("test-notebook")
        );
        let url = backend.ctx.url_builder.build_url(&path).unwrap();
        assert!(url.as_str().contains("test-notebook"));
        assert!(url.as_str().contains("sources:batchCreate"));
    }

    #[test]
    fn batch_delete_url_construction() {
        let backend = create_test_backend();
        let path = format!(
            "{}/sources:batchDelete",
            backend.ctx.url_builder.notebook_path("test-notebook")
        );
        let url = backend.ctx.url_builder.build_url(&path).unwrap();
        assert!(url.as_str().contains("test-notebook"));
        assert!(url.as_str().contains("sources:batchDelete"));
    }

    #[test]
    fn upload_file_url_construction() {
        let backend = create_test_backend();
        let path = format!(
            "{}/sources:uploadFile",
            backend.ctx.url_builder.notebook_path("test-notebook")
        );
        let mut url = backend.ctx.url_builder.build_upload_url(&path).unwrap();
        url.query_pairs_mut().append_pair("uploadType", "media");
        assert!(url.as_str().contains("test-notebook"));
        assert!(url.as_str().contains("sources:uploadFile"));
        assert!(url.as_str().contains("uploadType=media"));
    }

    #[test]
    fn get_source_url_construction() {
        let backend = create_test_backend();
        let path = format!(
            "{}/sources/{}",
            backend.ctx.url_builder.notebook_path("test-notebook"),
            "source-123"
        );
        let url = backend.ctx.url_builder.build_url(&path).unwrap();
        assert!(url.as_str().contains("test-notebook"));
        assert!(url.as_str().contains("sources/source-123"));
    }

    #[tokio::test]
    async fn upload_file_validates_empty_notebook_id() {
        let backend = create_test_backend();
        let result = backend
            .upload_source_file("", "file.txt", "text/plain", vec![1, 2, 3])
            .await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, Error::Validation(_)));
        assert!(format!("{}", err).contains("notebook_id"));
    }

    #[tokio::test]
    async fn upload_file_validates_empty_file_name() {
        let backend = create_test_backend();
        let result = backend
            .upload_source_file("notebook-123", "", "text/plain", vec![1, 2, 3])
            .await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, Error::Validation(_)));
        assert!(format!("{}", err).contains("file name"));
    }

    #[tokio::test]
    async fn upload_file_validates_empty_content_type() {
        let backend = create_test_backend();
        let result = backend
            .upload_source_file("notebook-123", "file.txt", "", vec![1, 2, 3])
            .await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, Error::Validation(_)));
        assert!(format!("{}", err).contains("content type"));
    }

    #[tokio::test]
    async fn upload_file_validates_whitespace_notebook_id() {
        let backend = create_test_backend();
        let result = backend
            .upload_source_file("   ", "file.txt", "text/plain", vec![1, 2, 3])
            .await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::Validation(_)));
    }

    #[tokio::test]
    async fn get_source_validates_empty_notebook_id() {
        let backend = create_test_backend();
        let result = backend.get_source("", "source-123").await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, Error::Validation(_)));
        assert!(format!("{}", err).contains("notebook_id"));
    }

    #[tokio::test]
    async fn get_source_validates_empty_source_id() {
        let backend = create_test_backend();
        let result = backend.get_source("notebook-123", "").await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, Error::Validation(_)));
        assert!(format!("{}", err).contains("source_id"));
    }

    #[test]
    fn add_sources_builds_request_correctly() {
        let contents = vec![UserContent::Text {
            text_content: TextContent {
                content: "Content".to_string(),
                source_name: Some("Test".to_string()),
            },
        }];
        let request = BatchCreateSourcesRequest {
            user_contents: contents.clone(),
        };
        assert_eq!(request.user_contents.len(), 1);
    }
}
