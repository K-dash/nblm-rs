use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

mod auth;
mod client;
mod error;
mod models;

pub use auth::{
    default_service_account_scopes, EnvTokenProvider, GcloudTokenProvider,
    ServiceAccountTokenProvider, TokenProvider, DEFAULT_ENV_TOKEN_KEY, DEFAULT_GCLOUD_BINARY,
    DEFAULT_SERVICE_ACCOUNT_SCOPES,
};
pub use client::NblmClient;
pub use error::NblmError;
pub use models::{BatchDeleteNotebooksResponse, ListRecentlyViewedResponse, Notebook};

/// NotebookLM Enterprise API client for Python
#[pymodule]
fn nblm(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<NblmClient>()?;
    m.add_class::<GcloudTokenProvider>()?;
    m.add_class::<ServiceAccountTokenProvider>()?;
    m.add_class::<EnvTokenProvider>()?;
    m.add_class::<Notebook>()?;
    m.add_class::<ListRecentlyViewedResponse>()?;
    m.add_class::<BatchDeleteNotebooksResponse>()?;
    m.add("NblmError", m.py().get_type::<NblmError>())?;
    m.add("DEFAULT_GCLOUD_BINARY", DEFAULT_GCLOUD_BINARY)?;
    m.add("DEFAULT_ENV_TOKEN_KEY", DEFAULT_ENV_TOKEN_KEY)?;
    m.add(
        "DEFAULT_SERVICE_ACCOUNT_SCOPES",
        default_service_account_scopes(),
    )?;
    m.add_function(wrap_pyfunction!(default_service_account_scopes_py, m)?)?;

    Ok(())
}

#[pyfunction(name = "default_service_account_scopes")]
fn default_service_account_scopes_py() -> Vec<String> {
    default_service_account_scopes()
}
