use async_trait::async_trait;
use reqwest::Method;

use crate::client::api::backends::{BackendContext, NotebooksBackend};
use crate::error::Result;
use crate::models::enterprise::notebook::{
    BatchDeleteNotebooksRequest, BatchDeleteNotebooksResponse, ListRecentlyViewedResponse, Notebook,
};

use super::models::{
    notebook as wire_notebook, requests::notebook as wire_notebook_req,
    responses::list as wire_list_resp,
};

pub(crate) struct EnterpriseNotebooksBackend {
    ctx: BackendContext,
}

impl EnterpriseNotebooksBackend {
    pub fn new(ctx: BackendContext) -> Self {
        Self { ctx }
    }

    async fn batch_delete_internal(
        &self,
        request: wire_notebook_req::BatchDeleteNotebooksRequest,
    ) -> Result<wire_notebook_req::BatchDeleteNotebooksResponse> {
        let path = format!(
            "{}:batchDelete",
            self.ctx.url_builder.notebooks_collection()
        );
        let url = self.ctx.url_builder.build_url(&path)?;
        self.ctx
            .http
            .request_json(Method::POST, url, Some(&request))
            .await
    }
}

const PAGE_SIZE_MIN: u32 = 1;
const PAGE_SIZE_MAX: u32 = 500;

#[async_trait]
impl NotebooksBackend for EnterpriseNotebooksBackend {
    async fn create_notebook(&self, title: String) -> Result<Notebook> {
        let url = self
            .ctx
            .url_builder
            .build_url(&self.ctx.url_builder.notebooks_collection())?;
        let request = wire_notebook_req::CreateNotebookRequest { title };
        let notebook: wire_notebook::Notebook = self
            .ctx
            .http
            .request_json(Method::POST, url, Some(&request))
            .await?;
        Ok(notebook.into())
    }

    async fn batch_delete_notebooks(
        &self,
        request: BatchDeleteNotebooksRequest,
    ) -> Result<BatchDeleteNotebooksResponse> {
        let wire_request: wire_notebook_req::BatchDeleteNotebooksRequest = request.into();
        let response = self.batch_delete_internal(wire_request).await?;
        Ok(response.into())
    }

    async fn delete_notebooks(
        &self,
        notebook_names: Vec<String>,
    ) -> Result<BatchDeleteNotebooksResponse> {
        for name in &notebook_names {
            let request = wire_notebook_req::BatchDeleteNotebooksRequest {
                names: vec![name.clone()],
            };
            self.batch_delete_internal(request).await?;
        }
        Ok(BatchDeleteNotebooksResponse::default())
    }

    async fn list_recently_viewed(
        &self,
        page_size: Option<u32>,
    ) -> Result<ListRecentlyViewedResponse> {
        let path = format!(
            "{}:listRecentlyViewed",
            self.ctx.url_builder.notebooks_collection()
        );
        let mut url = self.ctx.url_builder.build_url(&path)?;
        if let Some(size) = page_size {
            let clamped = size.clamp(PAGE_SIZE_MIN, PAGE_SIZE_MAX);
            url.query_pairs_mut()
                .append_pair("pageSize", &clamped.to_string());
        }
        let response: wire_list_resp::ListRecentlyViewedResponse = self
            .ctx
            .http
            .request_json::<(), _>(Method::GET, url, None::<&()>)
            .await?;
        Ok(response.into())
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
    use std::sync::Arc;
    use std::time::Duration;

    fn create_test_backend() -> EnterpriseNotebooksBackend {
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
        EnterpriseNotebooksBackend::new(ctx)
    }

    #[test]
    fn notebooks_collection_url_construction() {
        let backend = create_test_backend();
        let collection = backend.ctx.url_builder.notebooks_collection();
        assert_eq!(collection, "projects/123/locations/global/notebooks");
    }

    #[test]
    fn batch_delete_url_construction() {
        let backend = create_test_backend();
        let collection = backend.ctx.url_builder.notebooks_collection();
        let path = format!("{}:batchDelete", collection);
        let url = backend.ctx.url_builder.build_url(&path).unwrap();
        assert!(url.as_str().contains(":batchDelete"));
        assert!(url.as_str().contains("notebooks"));
    }

    #[test]
    fn list_recently_viewed_url_without_page_size() {
        let backend = create_test_backend();
        let path = format!(
            "{}:listRecentlyViewed",
            backend.ctx.url_builder.notebooks_collection()
        );
        let url = backend.ctx.url_builder.build_url(&path).unwrap();
        assert!(url.as_str().contains(":listRecentlyViewed"));
        assert!(!url.as_str().contains("pageSize"));
    }

    #[test]
    fn list_recently_viewed_clamps_page_size_min() {
        let backend = create_test_backend();
        let path = format!(
            "{}:listRecentlyViewed",
            backend.ctx.url_builder.notebooks_collection()
        );
        let mut url = backend.ctx.url_builder.build_url(&path).unwrap();
        let clamped = 0_u32.clamp(PAGE_SIZE_MIN, PAGE_SIZE_MAX);
        url.query_pairs_mut()
            .append_pair("pageSize", &clamped.to_string());
        assert!(url.as_str().contains("pageSize=1"));
    }

    #[test]
    fn list_recently_viewed_clamps_page_size_max() {
        let backend = create_test_backend();
        let path = format!(
            "{}:listRecentlyViewed",
            backend.ctx.url_builder.notebooks_collection()
        );
        let mut url = backend.ctx.url_builder.build_url(&path).unwrap();
        let clamped = 1000_u32.clamp(PAGE_SIZE_MIN, PAGE_SIZE_MAX);
        url.query_pairs_mut()
            .append_pair("pageSize", &clamped.to_string());
        assert!(url.as_str().contains("pageSize=500"));
    }

    #[test]
    fn list_recently_viewed_accepts_valid_page_size() {
        let backend = create_test_backend();
        let path = format!(
            "{}:listRecentlyViewed",
            backend.ctx.url_builder.notebooks_collection()
        );
        let mut url = backend.ctx.url_builder.build_url(&path).unwrap();
        let clamped = 50_u32.clamp(PAGE_SIZE_MIN, PAGE_SIZE_MAX);
        url.query_pairs_mut()
            .append_pair("pageSize", &clamped.to_string());
        assert!(url.as_str().contains("pageSize=50"));
    }
}
