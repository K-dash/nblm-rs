use pyo3::prelude::*;
use std::sync::Arc;

use crate::auth::PyTokenProvider;
use crate::error::{map_runtime_error, IntoPyResult, PyResult};
use crate::models::{BatchDeleteNotebooksResponse, ListRecentlyViewedResponse, Notebook};
use std::future::Future;

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
