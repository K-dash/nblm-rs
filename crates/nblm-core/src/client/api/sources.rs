use bytes::Bytes;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE},
    Method,
};

use crate::error::{Error, Result};
use crate::models::{
    BatchCreateSourcesRequest, BatchCreateSourcesResponse, BatchDeleteSourcesRequest,
    BatchDeleteSourcesResponse, UploadSourceFileResponse, UserContent,
};

use crate::client::NblmClient;

/// Source-related API implementations
impl NblmClient {
    pub async fn batch_create_sources(
        &self,
        notebook_id: &str,
        request: BatchCreateSourcesRequest,
    ) -> Result<BatchCreateSourcesResponse> {
        let path = format!(
            "{}/sources:batchCreate",
            self.url_builder.notebook_path(notebook_id)
        );
        let url = self.url_builder.build_url(&path)?;
        self.http
            .request_json(Method::POST, url, Some(&request))
            .await
    }

    pub async fn add_sources(
        &self,
        notebook_id: &str,
        contents: Vec<UserContent>,
    ) -> Result<BatchCreateSourcesResponse> {
        let request = BatchCreateSourcesRequest {
            user_contents: contents,
        };
        self.batch_create_sources(notebook_id, request).await
    }

    pub async fn batch_delete_sources(
        &self,
        notebook_id: &str,
        request: BatchDeleteSourcesRequest,
    ) -> Result<BatchDeleteSourcesResponse> {
        let path = format!(
            "{}/sources:batchDelete",
            self.url_builder.notebook_path(notebook_id)
        );
        let url = self.url_builder.build_url(&path)?;
        self.http
            .request_json(Method::POST, url, Some(&request))
            .await
    }

    pub async fn delete_sources(
        &self,
        notebook_id: &str,
        source_names: Vec<String>,
    ) -> Result<BatchDeleteSourcesResponse> {
        let request = BatchDeleteSourcesRequest {
            names: source_names,
        };
        self.batch_delete_sources(notebook_id, request).await
    }

    pub async fn upload_source_file(
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
            self.url_builder.notebook_path(notebook_id)
        );
        let mut url = self.url_builder.build_upload_url(&path)?;
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
        self.http
            .request_binary(Method::POST, url, headers, bytes)
            .await
    }

    /// Get a single source by its ID.
    ///
    /// # Arguments
    ///
    /// * `notebook_id` - The ID of the notebook containing the source
    /// * `source_id` - The ID of the source to retrieve
    ///
    /// # Returns
    ///
    /// The requested source information
    pub async fn get_source(
        &self,
        notebook_id: &str,
        source_id: &str,
    ) -> Result<crate::models::NotebookSource> {
        if notebook_id.trim().is_empty() {
            return Err(Error::validation("notebook_id cannot be empty"));
        }
        if source_id.trim().is_empty() {
            return Err(Error::validation("source_id cannot be empty"));
        }

        let path = format!(
            "{}/sources/{}",
            self.url_builder.notebook_path(notebook_id),
            source_id
        );
        let url = self.url_builder.build_url(&path)?;
        self.http
            .request_json::<(), _>(Method::GET, url, None::<&()>)
            .await
    }
}
