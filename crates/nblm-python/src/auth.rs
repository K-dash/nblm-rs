use pyo3::prelude::*;
use std::sync::Arc;

use crate::error::{IntoPyResult, PyResult};

pub const DEFAULT_GCLOUD_BINARY: &str = "gcloud";
pub const DEFAULT_ENV_TOKEN_KEY: &str = "NBLM_ACCESS_TOKEN";
pub const DEFAULT_SERVICE_ACCOUNT_SCOPE: &str = "https://www.googleapis.com/auth/cloud-platform";
pub const DEFAULT_SERVICE_ACCOUNT_SCOPES: &[&str] = &[DEFAULT_SERVICE_ACCOUNT_SCOPE];

pub fn default_service_account_scopes() -> Vec<String> {
    DEFAULT_SERVICE_ACCOUNT_SCOPES
        .iter()
        .map(|scope| scope.to_string())
        .collect()
}

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

#[pyclass(module = "nblm")]
#[derive(Clone)]
pub struct ServiceAccountTokenProvider {
    inner: Arc<nblm_core::ServiceAccountTokenProvider>,
}

#[pymethods]
impl ServiceAccountTokenProvider {
    #[staticmethod]
    #[pyo3(signature = (path, scopes = default_service_account_scopes()))]
    pub fn from_file(path: String, scopes: Vec<String>) -> PyResult<Self> {
        let provider =
            nblm_core::ServiceAccountTokenProvider::from_file(path, scopes).into_py_result()?;
        Ok(Self {
            inner: Arc::new(provider),
        })
    }

    #[staticmethod]
    #[pyo3(signature = (json_data, scopes = default_service_account_scopes()))]
    pub fn from_json(json_data: String, scopes: Vec<String>) -> PyResult<Self> {
        let provider = nblm_core::ServiceAccountTokenProvider::from_json(&json_data, scopes)
            .into_py_result()?;
        Ok(Self {
            inner: Arc::new(provider),
        })
    }
}

impl TokenProvider for ServiceAccountTokenProvider {
    fn get_inner(&self) -> Arc<dyn nblm_core::TokenProvider> {
        self.inner.clone()
    }
}

pub(crate) enum PyTokenProvider {
    Gcloud(GcloudTokenProvider),
    Env(EnvTokenProvider),
    ServiceAccount(ServiceAccountTokenProvider),
}

impl PyTokenProvider {
    pub fn get_inner(&self) -> Arc<dyn nblm_core::TokenProvider> {
        match self {
            PyTokenProvider::Gcloud(p) => p.get_inner(),
            PyTokenProvider::Env(p) => p.get_inner(),
            PyTokenProvider::ServiceAccount(p) => p.get_inner(),
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
        if let Ok(p) = ob.extract::<ServiceAccountTokenProvider>() {
            return Ok(PyTokenProvider::ServiceAccount(p));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Expected a TokenProvider instance",
        ))
    }
}
