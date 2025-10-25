use pyo3::prelude::*;
use std::fs;
use std::future::Future;
use std::path::PathBuf;
use std::sync::Arc;

use crate::auth::PyTokenProvider;
use crate::error::{map_nblm_error, map_runtime_error, IntoPyResult, PyResult};
use crate::models::{
    BatchCreateSourcesResponse, BatchDeleteNotebooksResponse, BatchDeleteSourcesResponse,
    ListRecentlyViewedResponse, Notebook, TextSource, UploadSourceFileResponse, VideoSource,
    WebSource,
};
use nblm_core::models::{TextContent, UserContent, VideoContent, WebContent};

#[pyclass(module = "nblm")]
pub struct NblmClient {
    #[allow(dead_code)]
    inner: Arc<nblm_core::NblmClient>,
}

#[pymethods]
impl NblmClient {
    #[new]
    #[pyo3(signature = (token_provider, project_number, location = "global".to_string(), endpoint_location = "global".to_string()))]
    fn new(
        token_provider: PyTokenProvider,
        project_number: String,
        location: String,
        endpoint_location: String,
    ) -> PyResult<Self> {
        let provider = token_provider.get_inner();
        let client =
            nblm_core::NblmClient::new(provider, project_number, location, endpoint_location)
                .into_py_result()?;

        Ok(Self {
            inner: Arc::new(client),
        })
    }

    pub fn __repr__(&self) -> String {
        "NblmClient()".to_string()
    }

    /// Create a new notebook with the given title.
    ///
    /// Args:
    ///     title: The title of the notebook
    ///
    /// Returns:
    ///     Notebook: The created notebook
    ///
    /// Raises:
    ///     NblmError: If the notebook creation fails
    fn create_notebook(&self, py: Python, title: String) -> PyResult<Notebook> {
        let inner = self.inner.clone();
        py.allow_threads(move || {
            let future = async move { inner.create_notebook(title).await };
            let result = block_on_with_runtime(future)?;
            Python::with_gil(|py| Notebook::from_core(py, result))
        })
    }

    /// List recently viewed notebooks.
    ///
    /// Args:
    ///     page_size: Maximum number of notebooks to return (1-500, default: 500)
    ///
    /// Returns:
    ///     ListRecentlyViewedResponse: Response containing notebooks list
    ///
    /// Raises:
    ///     NblmError: If the request fails
    #[pyo3(signature = (page_size = None))]
    fn list_recently_viewed(
        &self,
        py: Python,
        page_size: Option<u32>,
    ) -> PyResult<ListRecentlyViewedResponse> {
        let inner = self.inner.clone();
        py.allow_threads(move || {
            let future = async move { inner.list_recently_viewed(page_size).await };
            let result = block_on_with_runtime(future)?;
            Python::with_gil(|py| ListRecentlyViewedResponse::from_core(py, result))
        })
    }

    /// Delete one or more notebooks.
    ///
    /// Args:
    ///     notebook_names: List of full notebook resource names to delete
    ///
    /// Returns:
    ///     BatchDeleteNotebooksResponse: Response (typically empty)
    ///
    /// Raises:
    ///     NblmError: If deletion fails
    ///
    /// Note:
    ///     Despite the underlying API being named "batchDelete", it only accepts
    ///     one notebook at a time (as of 2025-10-19). This method works around
    ///     this limitation by calling the API sequentially for each notebook.
    fn delete_notebooks(
        &self,
        py: Python,
        notebook_names: Vec<String>,
    ) -> PyResult<BatchDeleteNotebooksResponse> {
        let inner = self.inner.clone();
        let names_clone = notebook_names.clone();
        py.allow_threads(move || {
            let future = async move { inner.delete_notebooks(notebook_names).await };
            let result = block_on_with_runtime(future)?;
            Python::with_gil(|py| {
                // All notebooks were deleted successfully if we reach here
                BatchDeleteNotebooksResponse::from_core(py, result, names_clone, vec![])
            })
        })
    }

    /// Add sources to a notebook.
    ///
    /// Args:
    ///     notebook_id: Notebook identifier (notebook resource ID, not full name)
    ///     web_sources: Optional list of WebSource objects
    ///     text_sources: Optional list of TextSource objects
    ///     video_sources: Optional list of VideoSource objects
    ///
    /// Returns:
    ///     BatchCreateSourcesResponse: API response containing source ingestion results
    ///
    /// Raises:
    ///     NblmError: If the request fails or validation fails
    #[pyo3(signature = (notebook_id, web_sources=None, text_sources=None, video_sources=None))]
    fn add_sources(
        &self,
        py: Python,
        notebook_id: String,
        web_sources: Option<Vec<WebSource>>,
        text_sources: Option<Vec<TextSource>>,
        video_sources: Option<Vec<VideoSource>>,
    ) -> PyResult<BatchCreateSourcesResponse> {
        let inner = self.inner.clone();
        py.allow_threads(move || {
            let future = async move {
                let mut contents = Vec::<UserContent>::new();

                if let Some(sources) = web_sources {
                    for source in sources {
                        contents.push(UserContent::Web {
                            web_content: WebContent {
                                url: source.url,
                                source_name: source.name,
                            },
                        });
                    }
                }

                if let Some(sources) = text_sources {
                    for source in sources {
                        if source.content.trim().is_empty() {
                            return Err(nblm_core::Error::validation(
                                "text content cannot be empty",
                            ));
                        }
                        contents.push(UserContent::Text {
                            text_content: TextContent {
                                content: source.content,
                                source_name: source.name,
                            },
                        });
                    }
                }

                if let Some(sources) = video_sources {
                    for source in sources {
                        contents.push(UserContent::Video {
                            video_content: VideoContent { url: source.url },
                        });
                    }
                }

                if contents.is_empty() {
                    return Err(nblm_core::Error::validation(
                        "at least one source must be provided",
                    ));
                }

                inner.add_sources(&notebook_id, contents).await
            };

            let result = block_on_with_runtime(future)?;
            Python::with_gil(|py| BatchCreateSourcesResponse::from_core(py, result))
        })
    }

    /// Upload a local file as a notebook source.
    ///
    /// Args:
    ///     notebook_id: Notebook identifier (resource ID, not full name)
    ///     path: Path to the file to upload
    ///     content_type: Optional HTTP Content-Type to send with the upload
    ///     display_name: Optional display name to use instead of the file name
    ///
    /// Returns:
    ///     UploadSourceFileResponse: Response containing the created source ID
    ///
    /// Raises:
    ///     NblmError: If validation or the API call fails
    #[pyo3(signature = (notebook_id, path, *, content_type=None, display_name=None))]
    fn upload_source_file(
        &self,
        py: Python,
        notebook_id: String,
        path: PathBuf,
        content_type: Option<String>,
        display_name: Option<String>,
    ) -> PyResult<UploadSourceFileResponse> {
        if !path.exists() {
            return Err(map_nblm_error(nblm_core::Error::validation(format!(
                "file not found: {}",
                path.display()
            ))));
        }
        if !path.is_file() {
            return Err(map_nblm_error(nblm_core::Error::validation(format!(
                "path is not a file: {}",
                path.display()
            ))));
        }

        let data = fs::read(&path).map_err(PyErr::from)?;
        if data.is_empty() {
            return Err(map_nblm_error(nblm_core::Error::validation(
                "cannot upload empty files",
            )));
        }

        let file_name = if let Some(name) = display_name {
            let trimmed = name.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.to_string())
            }
        } else {
            None
        }
        .or_else(|| {
            path.file_name()
                .and_then(|name| name.to_str())
                .map(|s| s.to_string())
        });

        let file_name = match file_name {
            Some(name) => name,
            None => {
                return Err(map_nblm_error(nblm_core::Error::validation(
                    "could not determine file name; provide display_name",
                )));
            }
        };

        let content_type = content_type
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty())
            .unwrap_or_else(|| {
                mime_guess::from_path(&path)
                    .first_or_octet_stream()
                    .essence_str()
                    .to_string()
            });

        let inner = self.inner.clone();
        py.allow_threads(move || {
            let future = async move {
                inner
                    .upload_source_file(&notebook_id, &file_name, &content_type, data)
                    .await
            };
            let result = block_on_with_runtime(future)?;
            Python::with_gil(|py| UploadSourceFileResponse::from_core(py, result))
        })
    }

    /// Delete sources from a notebook.
    ///
    /// Args:
    ///     notebook_id: Notebook identifier (notebook resource ID, not full name)
    ///     source_names: List of full source resource names to delete
    ///
    /// Returns:
    ///     BatchDeleteSourcesResponse: API response (typically empty)
    ///
    /// Raises:
    ///     NblmError: If the request fails
    fn delete_sources(
        &self,
        py: Python,
        notebook_id: String,
        source_names: Vec<String>,
    ) -> PyResult<BatchDeleteSourcesResponse> {
        let inner = self.inner.clone();
        py.allow_threads(move || {
            let future = async move { inner.delete_sources(&notebook_id, source_names).await };
            let result = block_on_with_runtime(future)?;
            Python::with_gil(|py| BatchDeleteSourcesResponse::from_core(py, result))
        })
    }
}

fn block_on_with_runtime<F, T>(future: F) -> PyResult<T>
where
    F: Future<Output = Result<T, nblm_core::Error>> + Send + 'static,
    T: Send + 'static,
{
    if let Ok(handle) = tokio::runtime::Handle::try_current() {
        return handle.block_on(future).into_py_result();
    }

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .map_err(map_runtime_error)?;
    runtime.block_on(future).into_py_result()
}
