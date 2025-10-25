use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::error::PyResult;

use super::extra_to_pydict;

#[pyclass(module = "nblm")]
pub struct NotebookSourceYoutubeMetadata {
    #[pyo3(get)]
    pub channel_name: Option<String>,
    #[pyo3(get)]
    pub video_id: Option<String>,
    #[pyo3(get)]
    pub extra: Py<PyDict>,
}

#[pymethods]
impl NotebookSourceYoutubeMetadata {
    pub fn __repr__(&self) -> String {
        format!(
            "NotebookSourceYoutubeMetadata(channel_name={:?}, video_id={:?})",
            self.channel_name, self.video_id
        )
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }
}

impl NotebookSourceYoutubeMetadata {
    pub(crate) fn from_core(
        py: Python,
        metadata: nblm_core::models::NotebookSourceYoutubeMetadata,
    ) -> PyResult<Self> {
        Ok(Self {
            channel_name: metadata.channel_name,
            video_id: metadata.video_id,
            extra: extra_to_pydict(py, &metadata.extra)?,
        })
    }
}

#[pyclass(module = "nblm")]
pub struct NotebookSourceSettings {
    #[pyo3(get)]
    pub status: Option<String>,
    #[pyo3(get)]
    pub extra: Py<PyDict>,
}

#[pymethods]
impl NotebookSourceSettings {
    pub fn __repr__(&self) -> String {
        format!("NotebookSourceSettings(status={:?})", self.status)
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }
}

impl NotebookSourceSettings {
    pub(crate) fn from_core(
        py: Python,
        settings: nblm_core::models::NotebookSourceSettings,
    ) -> PyResult<Self> {
        Ok(Self {
            status: settings.status,
            extra: extra_to_pydict(py, &settings.extra)?,
        })
    }
}

#[pyclass(module = "nblm")]
pub struct NotebookSourceId {
    #[pyo3(get)]
    pub id: Option<String>,
    #[pyo3(get)]
    pub extra: Py<PyDict>,
}

#[pymethods]
impl NotebookSourceId {
    pub fn __repr__(&self) -> String {
        format!("NotebookSourceId(id={:?})", self.id)
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }
}

impl NotebookSourceId {
    pub(crate) fn from_core(
        py: Python,
        source_id: nblm_core::models::NotebookSourceId,
    ) -> PyResult<Self> {
        Ok(Self {
            id: source_id.id,
            extra: extra_to_pydict(py, &source_id.extra)?,
        })
    }
}

#[pyclass(module = "nblm")]
pub struct NotebookSourceMetadata {
    #[pyo3(get)]
    pub source_added_timestamp: Option<String>,
    #[pyo3(get)]
    pub word_count: Option<u64>,
    #[pyo3(get)]
    pub youtube_metadata: Option<Py<NotebookSourceYoutubeMetadata>>,
    #[pyo3(get)]
    pub extra: Py<PyDict>,
}

#[pymethods]
impl NotebookSourceMetadata {
    pub fn __repr__(&self) -> String {
        format!(
            "NotebookSourceMetadata(source_added_timestamp={:?}, word_count={:?})",
            self.source_added_timestamp, self.word_count
        )
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }
}

impl NotebookSourceMetadata {
    pub(crate) fn from_core(
        py: Python,
        metadata: nblm_core::models::NotebookSourceMetadata,
    ) -> PyResult<Self> {
        let youtube_metadata = match metadata.youtube_metadata {
            Some(youtube) => Some(Py::new(
                py,
                NotebookSourceYoutubeMetadata::from_core(py, youtube)?,
            )?),
            None => None,
        };
        Ok(Self {
            source_added_timestamp: metadata.source_added_timestamp,
            word_count: metadata.word_count,
            youtube_metadata,
            extra: extra_to_pydict(py, &metadata.extra)?,
        })
    }
}

#[pyclass(module = "nblm")]
pub struct NotebookSource {
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub title: Option<String>,
    #[pyo3(get)]
    pub metadata: Option<Py<NotebookSourceMetadata>>,
    #[pyo3(get)]
    pub settings: Option<Py<NotebookSourceSettings>>,
    #[pyo3(get)]
    pub source_id: Option<Py<NotebookSourceId>>,
    #[pyo3(get)]
    pub extra: Py<PyDict>,
}

#[pymethods]
impl NotebookSource {
    pub fn __repr__(&self, _py: Python) -> String {
        let metadata_present = self.metadata.is_some();
        let settings_present = self.settings.is_some();
        let source_id_present = self.source_id.is_some();
        format!(
            "NotebookSource(name='{}', title={:?}, metadata={}, settings={}, source_id={})",
            self.name, self.title, metadata_present, settings_present, source_id_present
        )
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }
}

impl NotebookSource {
    pub(crate) fn from_core(
        py: Python,
        source: nblm_core::models::NotebookSource,
    ) -> PyResult<Self> {
        let metadata = match source.metadata {
            Some(meta) => Some(Py::new(py, NotebookSourceMetadata::from_core(py, meta)?)?),
            None => None,
        };
        let settings = match source.settings {
            Some(settings) => Some(Py::new(
                py,
                NotebookSourceSettings::from_core(py, settings)?,
            )?),
            None => None,
        };
        let source_id = match source.source_id {
            Some(source_id) => Some(Py::new(py, NotebookSourceId::from_core(py, source_id)?)?),
            None => None,
        };
        Ok(Self {
            name: source.name,
            title: source.title,
            metadata,
            settings,
            source_id,
            extra: extra_to_pydict(py, &source.extra)?,
        })
    }
}
