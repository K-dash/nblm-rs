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
