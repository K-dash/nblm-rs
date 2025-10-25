use pyo3::prelude::*;
use std::sync::Arc;

use crate::error::PyResult;

pub const DEFAULT_GCLOUD_BINARY: &str = "gcloud";
pub const DEFAULT_ENV_TOKEN_KEY: &str = "NBLM_ACCESS_TOKEN";

pub trait TokenProvider: Send + Sync {
    fn get_inner(&self) -> Arc<dyn nblm_core::TokenProvider>;
}

#[pyclass(module = "nblm")]
#[derive(Clone)]
pub struct GcloudTokenProvider {
    inner: Arc<nblm_core::GcloudTokenProvider>,
}

#[pymethods]
impl GcloudTokenProvider {
    #[new]
    #[pyo3(signature = (binary = DEFAULT_GCLOUD_BINARY.to_string()))]
    pub fn new(binary: String) -> Self {
        Self {
            inner: Arc::new(nblm_core::GcloudTokenProvider::new(binary)),
        }
    }
}

impl TokenProvider for GcloudTokenProvider {
    fn get_inner(&self) -> Arc<dyn nblm_core::TokenProvider> {
        self.inner.clone()
    }
}

#[pyclass(module = "nblm")]
#[derive(Clone)]
pub struct EnvTokenProvider {
    inner: Arc<nblm_core::EnvTokenProvider>,
}

#[pymethods]
impl EnvTokenProvider {
    #[new]
    #[pyo3(signature = (key = DEFAULT_ENV_TOKEN_KEY.to_string()))]
    pub fn new(key: String) -> Self {
        Self {
            inner: Arc::new(nblm_core::EnvTokenProvider::new(key)),
        }
    }
}

impl TokenProvider for EnvTokenProvider {
    fn get_inner(&self) -> Arc<dyn nblm_core::TokenProvider> {
        self.inner.clone()
    }
}

pub(crate) enum PyTokenProvider {
    Gcloud(GcloudTokenProvider),
    Env(EnvTokenProvider),
}

impl PyTokenProvider {
    pub fn get_inner(&self) -> Arc<dyn nblm_core::TokenProvider> {
        match self {
            PyTokenProvider::Gcloud(p) => p.get_inner(),
            PyTokenProvider::Env(p) => p.get_inner(),
        }
    }
}

impl<'py> FromPyObject<'py> for PyTokenProvider {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        if let Ok(p) = ob.extract::<GcloudTokenProvider>() {
            return Ok(PyTokenProvider::Gcloud(p));
        }
        if let Ok(p) = ob.extract::<EnvTokenProvider>() {
            return Ok(PyTokenProvider::Env(p));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Expected a TokenProvider instance",
        ))
    }
}
