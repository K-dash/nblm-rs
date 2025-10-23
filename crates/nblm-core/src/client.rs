use std::{sync::Arc, time::Duration};

use reqwest::{Client, Method, StatusCode, Url};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::auth::TokenProvider;
use crate::error::{Error, Result};
use crate::models::{
    AccountRole, AudioOverviewRequest, AudioOverviewResponse, BatchCreateSourcesRequest,
    BatchCreateSourcesResponse, BatchDeleteNotebooksRequest, BatchDeleteNotebooksResponse,
    BatchDeleteSourcesRequest, BatchDeleteSourcesResponse, CreateNotebookRequest,
    ListRecentlyViewedResponse, Notebook, ShareRequest, ShareResponse, UserContent,
};
use crate::retry::{RetryConfig, Retryer};

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);
const PAGE_SIZE_MIN: u32 = 1;
const PAGE_SIZE_MAX: u32 = 500;

pub struct NblmClient {
    http: Client,
    token_provider: Arc<dyn TokenProvider>,
    base: String,
    parent: String,
    timeout: Duration,
    retryer: Retryer,
    user_project: Option<String>,
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

        let http = Client::builder()
            .user_agent(concat!("nblm-cli/", env!("CARGO_PKG_VERSION")))
            .timeout(DEFAULT_TIMEOUT)
            .build()
            .map_err(Error::from)?;

        Ok(Self {
            http,
            token_provider,
            base: base.trim_end_matches('/').to_string(),
            parent,
            timeout: DEFAULT_TIMEOUT,
            retryer: Retryer::new(RetryConfig::default()),
            user_project: None,
        })
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn with_retry_config(mut self, config: RetryConfig) -> Self {
        self.retryer = Retryer::new(config);
        self
    }

    pub fn with_user_project(mut self, project: impl Into<String>) -> Self {
        self.user_project = Some(project.into());
        self
    }

    /// Override API base URL (for tests). Accepts absolute URL. Trims trailing slash.
    pub fn with_base_url(mut self, base: impl Into<String>) -> Result<Self> {
        let base = base.into().trim().trim_end_matches('/').to_string();
        // Basic sanity check: absolute URL
        let _ = Url::parse(&base).map_err(Error::from)?;
        self.base = base;
        Ok(self)
    }

    fn notebooks_collection(&self) -> String {
        format!("{}/notebooks", self.parent)
    }

    fn notebook_path(&self, notebook_id: &str) -> String {
        format!("{}/notebooks/{}", self.parent, notebook_id)
    }

    fn build_url(&self, path: &str) -> Result<Url> {
        let path = path.trim_start_matches('/');
        Url::parse(&format!("{}/{}", self.base, path)).map_err(Error::from)
    }

    async fn request_json<B, R>(&self, method: Method, url: Url, body: Option<&B>) -> Result<R>
    where
        B: Serialize + ?Sized,
        R: DeserializeOwned,
    {
        let client = self.http.clone();
        let method_clone = method.clone();
        let url_clone = url.clone();
        let timeout = self.timeout;
        let body_ref = body;
        let provider = Arc::clone(&self.token_provider);
        let user_project = self.user_project.clone();

        let run = || {
            let client = client.clone();
            let method = method_clone.clone();
            let url = url_clone.clone();
            let provider = Arc::clone(&provider);
            let user_project = user_project.clone();
            async move {
                let token = provider.access_token().await?;
                let mut builder = client
                    .request(method, url)
                    .bearer_auth(token)
                    .timeout(timeout);
                if let Some(project) = &user_project {
                    builder = builder.header("x-goog-user-project", project);
                }
                if let Some(body) = body_ref {
                    builder = builder.json(body);
                }
                let request = builder.build().map_err(Error::Request)?;
                let response = client.execute(request).await.map_err(Error::Request)?;
                Ok(response)
            }
        };

        let mut response = self.retryer.run_with_retry(run).await?;

        if response.status() == StatusCode::UNAUTHORIZED {
            let _ = response.bytes().await;
            let run_refresh = || {
                let client = client.clone();
                let method = method_clone.clone();
                let url = url_clone.clone();
                let provider = Arc::clone(&provider);
                let user_project = user_project.clone();
                async move {
                    let token = provider.refresh_token().await?;
                    let mut builder = client
                        .request(method, url)
                        .bearer_auth(token)
                        .timeout(timeout);
                    if let Some(project) = &user_project {
                        builder = builder.header("x-goog-user-project", project);
                    }
                    if let Some(body) = body_ref {
                        builder = builder.json(body);
                    }
                    let request = builder.build().map_err(Error::Request)?;
                    let response = client.execute(request).await.map_err(Error::Request)?;
                    Ok(response)
                }
            };
            response = self.retryer.run_with_retry(run_refresh).await?;
            if !response.status().is_success() {
                let status = response.status();
                let body = response.text().await.unwrap_or_default();
                return Err(Error::http(status, body));
            }
            return Ok(response.json::<R>().await?);
        }

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(Error::http(status, body));
        }

        Ok(response.json::<R>().await?)
    }

    pub async fn create_notebook(&self, title: impl Into<String>) -> Result<Notebook> {
        let url = self.build_url(&self.notebooks_collection())?;
        let request = CreateNotebookRequest {
            title: title.into(),
        };
        self.request_json(Method::POST, url, Some(&request)).await
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
        let path = format!("{}:batchDelete", self.notebooks_collection());
        let url = self.build_url(&path)?;
        self.request_json(Method::POST, url, Some(&request)).await
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

    pub async fn batch_create_sources(
        &self,
        notebook_id: &str,
        request: BatchCreateSourcesRequest,
    ) -> Result<BatchCreateSourcesResponse> {
        let path = format!("{}/sources:batchCreate", self.notebook_path(notebook_id));
        let url = self.build_url(&path)?;
        self.request_json(Method::POST, url, Some(&request)).await
    }

    // TODO: This method has not been tested due to the requirement of setting up additional user accounts.
    pub async fn share_notebook(
        &self,
        notebook_id: &str,
        accounts: Vec<AccountRole>,
    ) -> Result<ShareResponse> {
        let path = format!("{}:share", self.notebook_path(notebook_id));
        let url = self.build_url(&path)?;
        let request = ShareRequest {
            account_and_roles: accounts,
        };
        self.request_json(Method::POST, url, Some(&request)).await
    }

    pub async fn create_audio_overview(
        &self,
        notebook_id: &str,
        request: AudioOverviewRequest,
    ) -> Result<AudioOverviewResponse> {
        let path = format!("{}/audioOverviews", self.notebook_path(notebook_id));
        let url = self.build_url(&path)?;
        self.request_json(Method::POST, url, Some(&request)).await
    }

    pub async fn delete_audio_overview(&self, notebook_id: &str) -> Result<()> {
        let path = format!("{}/audioOverviews/default", self.notebook_path(notebook_id));
        let url = self.build_url(&path)?;
        let _response: serde_json::Value =
            self.request_json(Method::DELETE, url, None::<&()>).await?;
        Ok(())
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
        let path = format!("{}:listRecentlyViewed", self.notebooks_collection());
        let mut url = self.build_url(&path)?;
        if let Some(size) = page_size {
            let clamped = size.clamp(PAGE_SIZE_MIN, PAGE_SIZE_MAX);
            url.query_pairs_mut()
                .append_pair("pageSize", &clamped.to_string());
        }
        self.request_json::<(), _>(Method::GET, url, None::<&()>)
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
        let path = format!("{}/sources:batchDelete", self.notebook_path(notebook_id));
        let url = self.build_url(&path)?;
        self.request_json(Method::POST, url, Some(&request)).await
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
}

fn normalize_endpoint_location(input: String) -> Result<String> {
    let trimmed = input.trim().trim_end_matches('-').to_lowercase();
    let normalized = match trimmed.as_str() {
        "us" => "us-",
        "eu" => "eu-",
        "global" => "global-",
        other => {
            return Err(Error::Endpoint(format!(
                "unsupported endpoint location: {other}"
            )))
        }
    };
    Ok(normalized.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_endpoint_location_variants() {
        assert_eq!(
            normalize_endpoint_location("us".into()).unwrap(),
            "us-".to_string()
        );
        assert_eq!(
            normalize_endpoint_location("eu-".into()).unwrap(),
            "eu-".to_string()
        );
        assert_eq!(
            normalize_endpoint_location(" global ".into()).unwrap(),
            "global-".to_string()
        );
    }

    #[test]
    fn normalize_endpoint_location_invalid() {
        let err = normalize_endpoint_location("asia".into()).unwrap_err();
        assert!(format!("{err}").contains("unsupported endpoint location"));
    }

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
        assert_eq!(client.base, "http://example.com/v1alpha");
    }

    #[test]
    fn with_base_url_rejects_relative_path() {
        let provider = Arc::new(crate::auth::StaticTokenProvider::new("test"));
        let client = NblmClient::new(provider, "123", "global", "us").unwrap();
        let result = client.with_base_url("/relative/path");
        assert!(result.is_err());
    }

    #[test]
    fn build_url_combines_base_and_path_correctly() {
        let provider = Arc::new(crate::auth::StaticTokenProvider::new("test"));
        let client = NblmClient::new(provider, "123", "global", "us")
            .unwrap()
            .with_base_url("http://example.com/v1alpha")
            .unwrap();

        // Test with leading slash
        let url = client.build_url("/projects/123/notebooks").unwrap();
        assert_eq!(
            url.as_str(),
            "http://example.com/v1alpha/projects/123/notebooks"
        );

        // Test without leading slash
        let url = client.build_url("projects/123/notebooks").unwrap();
        assert_eq!(
            url.as_str(),
            "http://example.com/v1alpha/projects/123/notebooks"
        );
    }
}
