use reqwest::Method;

use crate::error::Result;
use crate::models::{
    AccountRole, BatchDeleteNotebooksRequest, BatchDeleteNotebooksResponse, CreateNotebookRequest,
    ListRecentlyViewedResponse, Notebook, ShareRequest, ShareResponse,
};

use crate::client::NblmClient;

const PAGE_SIZE_MIN: u32 = 1;
const PAGE_SIZE_MAX: u32 = 500;

/// Notebook-related API implementations
impl NblmClient {
    pub async fn create_notebook(&self, title: impl Into<String>) -> Result<Notebook> {
        let url = self
            .url_builder
            .build_url(&self.url_builder.notebooks_collection())?;
        let request = CreateNotebookRequest {
            title: title.into(),
        };
        self.http
            .request_json(Method::POST, url, Some(&request))
            .await
    }

    /// Delete notebooks using the batchDelete API.
    ///
    /// # Known Issues (as of 2025-10-19)
    ///
    /// The API only accepts a single notebook name despite being named "batchDelete".
    /// Multiple names result in HTTP 400 error. Use `delete_notebooks` which handles
    /// this limitation by calling the API once per notebook.
    pub async fn batch_delete_notebooks(
        &self,
        request: BatchDeleteNotebooksRequest,
    ) -> Result<BatchDeleteNotebooksResponse> {
        let path = format!("{}:batchDelete", self.url_builder.notebooks_collection());
        let url = self.url_builder.build_url(&path)?;
        self.http
            .request_json(Method::POST, url, Some(&request))
            .await
    }

    /// Delete one or more notebooks.
    ///
    /// # Implementation Note
    ///
    /// Despite the underlying API being named "batchDelete", it only accepts one notebook
    /// at a time (as of 2025-10-19). This method works around this limitation by calling
    /// the API sequentially for each notebook.
    pub async fn delete_notebooks(
        &self,
        notebook_names: Vec<String>,
    ) -> Result<BatchDeleteNotebooksResponse> {
        // TODO: Remove sequential processing when API supports true batch deletion
        for name in &notebook_names {
            let request = BatchDeleteNotebooksRequest {
                names: vec![name.clone()],
            };
            self.batch_delete_notebooks(request).await?;
        }
        // Return empty response after all deletions succeed
        Ok(BatchDeleteNotebooksResponse::default())
    }

    // TODO: This method has not been tested due to the requirement of setting up additional user accounts.
    pub async fn share_notebook(
        &self,
        notebook_id: &str,
        accounts: Vec<AccountRole>,
    ) -> Result<ShareResponse> {
        let path = format!("{}:share", self.url_builder.notebook_path(notebook_id));
        let url = self.url_builder.build_url(&path)?;
        let request = ShareRequest {
            account_and_roles: accounts,
        };
        self.http
            .request_json(Method::POST, url, Some(&request))
            .await
    }

    /// List recently viewed notebooks.
    ///
    /// # Pagination
    ///
    /// The `page_size` parameter can be used to limit the number of results.
    /// Valid range is 1-500. Default is 500 notebooks.
    pub async fn list_recently_viewed(
        &self,
        page_size: Option<u32>,
    ) -> Result<ListRecentlyViewedResponse> {
        let path = format!(
            "{}:listRecentlyViewed",
            self.url_builder.notebooks_collection()
        );
        let mut url = self.url_builder.build_url(&path)?;
        if let Some(size) = page_size {
            let clamped = size.clamp(PAGE_SIZE_MIN, PAGE_SIZE_MAX);
            url.query_pairs_mut()
                .append_pair("pageSize", &clamped.to_string());
        }
        self.http
            .request_json::<(), _>(Method::GET, url, None::<&()>)
            .await
    }
}
