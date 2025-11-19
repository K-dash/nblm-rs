use pyo3::exceptions::{PyIOError, PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use reqwest::Client;
use std::sync::Arc;

use crate::error::PyResult;
use crate::runtime::block_on_with_runtime;
use nblm_core::auth::oauth::{
    FileRefreshTokenStore, OAuthClientConfig, OAuthError, OAuthFlow, RefreshTokenProvider,
    TokenStoreKey,
};
use nblm_core::ApiProfile;
use nblm_core::Error as CoreError;
use nblm_core::RefreshTokenStore;

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

#[pyclass(module = "nblm")]
#[derive(Clone)]
pub struct UserOAuthProvider {
    inner: Arc<RefreshTokenProvider<FileRefreshTokenStore>>,
    endpoint_location: String,
}

#[pymethods]
impl UserOAuthProvider {
    /// Load OAuth2 refresh tokens from the shared credentials file created by the CLI.
    ///
    /// This helper does not run the browser flow. Use the CLI (`nblm --auth user-oauth ...`)
    /// first to create the credentials file, then call this method from Python to reuse the
    /// stored refresh token.
    #[staticmethod]
    #[pyo3(signature = (project_number=None, location="global", user=None, endpoint_location=None))]
    pub fn from_file(
        project_number: Option<i64>,
        location: &str,
        user: Option<&str>,
        endpoint_location: Option<&str>,
    ) -> PyResult<Self> {
        let project_number = project_number.ok_or_else(|| {
            PyValueError::new_err(
                "project_number is required for OAuth2. Run the CLI with --project-number first.",
            )
        })?;
        let project_number = project_number.to_string();
        let user_hint = user
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());

        let store = Arc::new(FileRefreshTokenStore::new().map_err(oauth_error_to_py)?);
        let endpoint_options = endpoint_candidates(location, endpoint_location);
        let (store_key, resolved_endpoint) =
            resolve_store_key(&store, &project_number, user_hint, &endpoint_options)?;

        let client_config = OAuthClientConfig::from_env().map_err(|err| {
            PyValueError::new_err(format!(
                "OAuth configuration error: {}. Set NBLM_OAUTH_CLIENT_ID before calling UserOAuthProvider.from_file().",
                err
            ))
        })?;
        let oauth_config = client_config.into_oauth_config();
        let http_client = build_http_client()?;
        let flow =
            OAuthFlow::new(oauth_config, Arc::clone(&http_client)).map_err(oauth_error_to_py)?;

        let provider = RefreshTokenProvider::new(flow, store, store_key);
        Ok(Self {
            inner: Arc::new(provider),
            endpoint_location: resolved_endpoint,
        })
    }

    #[getter]
    pub fn endpoint_location(&self) -> &str {
        &self.endpoint_location
    }
}

impl TokenProvider for UserOAuthProvider {
    fn get_inner(&self) -> Arc<dyn nblm_core::TokenProvider> {
        self.inner.clone()
    }
}

pub(crate) enum PyTokenProvider {
    Gcloud(GcloudTokenProvider),
    Env(EnvTokenProvider),
    User(UserOAuthProvider),
}

impl PyTokenProvider {
    pub fn get_inner(&self) -> Arc<dyn nblm_core::TokenProvider> {
        match self {
            PyTokenProvider::Gcloud(p) => p.get_inner(),
            PyTokenProvider::Env(p) => p.get_inner(),
            PyTokenProvider::User(p) => p.get_inner(),
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
        if let Ok(p) = ob.extract::<UserOAuthProvider>() {
            return Ok(PyTokenProvider::User(p));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Expected a TokenProvider instance",
        ))
    }
}

fn oauth_error_to_py(err: OAuthError) -> PyErr {
    match err {
        OAuthError::MissingEnvVar(_) | OAuthError::Config(_) => {
            PyValueError::new_err(err.to_string())
        }
        OAuthError::Storage(_) => PyIOError::new_err(err.to_string()),
        _ => PyRuntimeError::new_err(err.to_string()),
    }
}

fn build_http_client() -> PyResult<Arc<Client>> {
    Client::builder()
        .user_agent(concat!("nblm-python/", env!("CARGO_PKG_VERSION")))
        .build()
        .map(Arc::new)
        .map_err(|err| PyRuntimeError::new_err(format!("failed to create HTTP client: {err}")))
}

fn endpoint_candidates(location: &str, endpoint_location: Option<&str>) -> Vec<String> {
    fn push_candidate(values: &mut Vec<String>, candidate: &str) {
        let trimmed = candidate.trim();
        if trimmed.is_empty() {
            return;
        }
        if values.iter().any(|value| value == trimmed) {
            return;
        }
        values.push(trimmed.to_string());
    }

    let mut candidates = Vec::new();
    if let Some(explicit) = endpoint_location {
        push_candidate(&mut candidates, explicit);
    }
    push_candidate(&mut candidates, location);
    push_candidate(&mut candidates, "global");
    candidates
}

fn resolve_store_key(
    store: &Arc<FileRefreshTokenStore>,
    project_number: &str,
    user_hint: Option<String>,
    endpoint_candidates: &[String],
) -> PyResult<(TokenStoreKey, String)> {
    for endpoint in endpoint_candidates {
        let key = TokenStoreKey {
            profile: ApiProfile::Enterprise,
            project_number: Some(project_number.to_string()),
            endpoint_location: Some(endpoint.clone()),
            user_hint: user_hint.clone(),
        };
        if store_contains_entry(store, &key)? {
            return Ok((key, endpoint.clone()));
        }
    }

    Err(PyValueError::new_err(format!(
        "No refresh token found for project {}. Run `nblm --auth user-oauth --project-number {}` in the CLI to bootstrap credentials.",
        project_number, project_number
    )))
}

fn store_contains_entry(store: &Arc<FileRefreshTokenStore>, key: &TokenStoreKey) -> PyResult<bool> {
    let store = Arc::clone(store);
    let key_clone = key.clone();
    let future = async move { store.load(&key_clone).await.map_err(CoreError::from) };
    block_on_with_runtime(future).map(|entry| entry.is_some())
}

#[pyfunction]
#[pyo3(signature = (drive_access=false, force=false))]
/// Log in via Google Cloud SDK (gcloud auth login).
///
/// This function executes `gcloud auth login` to authenticate the user.
/// It opens a browser window for the authentication flow.
///
/// Args:
///     drive_access (bool): If True, requests Google Drive access (adds --enable-gdrive-access).
///     force (bool): If True, forces re-authentication even if valid credentials exist.
///
/// Raises:
///     RuntimeError: If gcloud is not found or authentication fails.
pub fn login(drive_access: bool, force: bool) -> PyResult<()> {
    use std::process::{Command, Stdio};

    let mut command = Command::new(DEFAULT_GCLOUD_BINARY);
    command.arg("auth").arg("login");

    if drive_access {
        command.arg("--enable-gdrive-access");
    }

    if force {
        command.arg("--force");
    }

    // Inherit stdio to allow browser interaction
    let status = command
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .map_err(|err| {
            PyRuntimeError::new_err(format!(
                "Failed to execute gcloud command. Make sure gcloud CLI is installed and in PATH.\nError: {}",
                err
            ))
        })?;

    if !status.success() {
        return Err(PyRuntimeError::new_err("gcloud auth login failed"));
    }

    Ok(())
}
