use pyo3::prelude::*;

mod auth;
mod client;
mod error;
mod models;

pub use auth::{
    EnvTokenProvider, GcloudTokenProvider, TokenProvider, DEFAULT_ENV_TOKEN_KEY,
    DEFAULT_GCLOUD_BINARY,
};
pub use client::NblmClient;
pub use error::NblmError;
pub use models::{
    AudioOverviewRequest, AudioOverviewResponse, BatchCreateSourcesResponse,
    BatchDeleteNotebooksResponse, BatchDeleteSourcesResponse, GoogleDriveSource,
    ListRecentlyViewedResponse, Notebook, NotebookMetadata, NotebookSource, NotebookSourceId,
    NotebookSourceMetadata, NotebookSourceSettings, NotebookSourceYoutubeMetadata, TextSource,
    UploadSourceFileResponse, VideoSource, WebSource,
};

/// NotebookLM Enterprise API client for Python
#[pymodule]
fn nblm(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<NblmClient>()?;
    m.add_class::<GcloudTokenProvider>()?;
    m.add_class::<EnvTokenProvider>()?;
    m.add_class::<Notebook>()?;
    m.add_class::<NotebookMetadata>()?;
    m.add_class::<NotebookSource>()?;
    m.add_class::<NotebookSourceMetadata>()?;
    m.add_class::<NotebookSourceSettings>()?;
    m.add_class::<NotebookSourceYoutubeMetadata>()?;
    m.add_class::<NotebookSourceId>()?;
    m.add_class::<WebSource>()?;
    m.add_class::<TextSource>()?;
    m.add_class::<GoogleDriveSource>()?;
    m.add_class::<VideoSource>()?;
    m.add_class::<UploadSourceFileResponse>()?;
    m.add_class::<BatchCreateSourcesResponse>()?;
    m.add_class::<BatchDeleteSourcesResponse>()?;
    m.add_class::<ListRecentlyViewedResponse>()?;
    m.add_class::<BatchDeleteNotebooksResponse>()?;
    m.add_class::<AudioOverviewRequest>()?;
    m.add_class::<AudioOverviewResponse>()?;
    m.add("NblmError", m.py().get_type::<NblmError>())?;
    m.add("DEFAULT_GCLOUD_BINARY", DEFAULT_GCLOUD_BINARY)?;
    m.add("DEFAULT_ENV_TOKEN_KEY", DEFAULT_ENV_TOKEN_KEY)?;

    Ok(())
}
