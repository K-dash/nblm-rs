use pyo3::prelude::*;
use std::sync::Arc;

use crate::auth::PyTokenProvider;
use crate::error::{IntoPyResult, PyResult};

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
}
