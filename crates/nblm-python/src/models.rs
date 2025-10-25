use pyo3::prelude::*;
use pyo3::types::{PyBool, PyDict, PyFloat, PyList, PyNone, PyString};
use pyo3::IntoPyObject;
use serde_json::Value;
use std::collections::HashMap;

use crate::error::PyResult;

/// Convert `serde_json::Value` to a Python object.
fn json_value_to_py(py: Python, value: &Value) -> PyResult<Py<PyAny>> {
    Ok(match value {
        Value::Null => PyNone::get(py).to_owned().into_any().unbind(),
        Value::Bool(b) => PyBool::new(py, *b).to_owned().into_any().unbind(),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                i.into_pyobject(py)?.into_any().unbind()
            } else if let Some(u) = n.as_u64() {
                u.into_pyobject(py)?.into_any().unbind()
            } else if let Some(f) = n.as_f64() {
                PyFloat::new(py, f).into_any().unbind()
            } else {
                PyString::new(py, &n.to_string()).into_any().unbind()
            }
        }
        Value::String(s) => PyString::new(py, s).into_any().unbind(),
        Value::Array(arr) => {
            let list = PyList::empty(py);
            for item in arr {
                list.append(json_value_to_py(py, item)?)?;
            }
            list.into_any().unbind()
        }
        Value::Object(map) => {
            let dict = PyDict::new(py);
            for (k, v) in map {
                dict.set_item(k, json_value_to_py(py, v)?)?;
            }
            dict.into_any().unbind()
        }
    })
}

/// Convert `HashMap<String, Value>` to `PyDict`.
fn extra_to_pydict(py: Python, extra: &HashMap<String, Value>) -> PyResult<Py<PyDict>> {
    let dict = PyDict::new(py);
    for (k, v) in extra {
        dict.set_item(k, json_value_to_py(py, v)?)?;
    }
    Ok(dict.unbind())
}

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
    fn from_core(
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
    fn from_core(
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
    fn from_core(py: Python, source_id: nblm_core::models::NotebookSourceId) -> PyResult<Self> {
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
    fn from_core(
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
    fn from_core(py: Python, source: nblm_core::models::NotebookSource) -> PyResult<Self> {
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

#[pyclass(module = "nblm")]
pub struct NotebookMetadata {
    #[pyo3(get)]
    pub create_time: Option<String>,
    #[pyo3(get)]
    pub is_shareable: Option<bool>,
    #[pyo3(get)]
    pub is_shared: Option<bool>,
    #[pyo3(get)]
    pub last_viewed: Option<String>,
    #[pyo3(get)]
    pub extra: Py<PyDict>,
}

#[pymethods]
impl NotebookMetadata {
    pub fn __repr__(&self) -> String {
        format!(
            "NotebookMetadata(create_time={:?}, last_viewed={:?})",
            self.create_time, self.last_viewed
        )
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }
}

impl NotebookMetadata {
    fn from_core(py: Python, metadata: nblm_core::models::NotebookMetadata) -> PyResult<Self> {
        Ok(Self {
            create_time: metadata.create_time,
            is_shareable: metadata.is_shareable,
            is_shared: metadata.is_shared,
            last_viewed: metadata.last_viewed,
            extra: extra_to_pydict(py, &metadata.extra)?,
        })
    }
}

#[pyclass(module = "nblm")]
pub struct Notebook {
    #[pyo3(get)]
    pub name: Option<String>,
    #[pyo3(get)]
    pub title: String,
    #[pyo3(get)]
    pub notebook_id: Option<String>,
    #[pyo3(get)]
    pub emoji: Option<String>,
    #[pyo3(get)]
    pub metadata: Option<Py<NotebookMetadata>>,
    #[pyo3(get)]
    pub sources: Py<PyList>,
    #[pyo3(get)]
    pub extra: Py<PyDict>,
}

#[pymethods]
impl Notebook {
    pub fn __repr__(&self, py: Python) -> String {
        let source_count = self.sources.bind(py).len();
        format!(
            "Notebook(title='{}', notebook_id={:?}, sources={} items)",
            self.title, self.notebook_id, source_count
        )
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }
}

impl Notebook {
    pub fn from_core(py: Python, notebook: nblm_core::models::Notebook) -> PyResult<Self> {
        let extra = extra_to_pydict(py, &notebook.extra)?;
        let metadata = match notebook.metadata {
            Some(meta) => Some(Py::new(py, NotebookMetadata::from_core(py, meta)?)?),
            None => None,
        };
        let sources_list = PyList::empty(py);
        for source in notebook.sources {
            let py_source = NotebookSource::from_core(py, source)?;
            sources_list.append(py_source)?;
        }
        Ok(Self {
            name: notebook.name,
            title: notebook.title,
            notebook_id: notebook.notebook_id,
            emoji: notebook.emoji,
            metadata,
            sources: sources_list.unbind(),
            extra,
        })
    }
}

#[pyclass(module = "nblm")]
pub struct ListRecentlyViewedResponse {
    #[pyo3(get)]
    pub notebooks: Py<PyList>,
}

#[pymethods]
impl ListRecentlyViewedResponse {
    pub fn __repr__(&self, py: Python) -> String {
        let count = self.notebooks.bind(py).len();
        format!("ListRecentlyViewedResponse(notebooks={} items)", count)
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }
}

impl ListRecentlyViewedResponse {
    pub fn from_core(
        py: Python,
        response: nblm_core::models::ListRecentlyViewedResponse,
    ) -> PyResult<Self> {
        let notebooks_list = PyList::empty(py);
        for notebook in response.notebooks {
            let py_notebook = Notebook::from_core(py, notebook)?;
            notebooks_list.append(py_notebook)?;
        }
        Ok(Self {
            notebooks: notebooks_list.unbind(),
        })
    }
}

#[pyclass(module = "nblm")]
pub struct BatchDeleteNotebooksResponse {
    #[pyo3(get)]
    pub deleted_notebooks: Py<PyList>,
    #[pyo3(get)]
    pub failed_notebooks: Py<PyList>,
}

#[pymethods]
impl BatchDeleteNotebooksResponse {
    pub fn __repr__(&self, py: Python) -> String {
        let deleted_count = self.deleted_notebooks.bind(py).len();
        let failed_count = self.failed_notebooks.bind(py).len();
        format!(
            "BatchDeleteNotebooksResponse(deleted={}, failed={})",
            deleted_count, failed_count
        )
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }
}

impl BatchDeleteNotebooksResponse {
    pub fn from_core(
        py: Python,
        _response: nblm_core::models::BatchDeleteNotebooksResponse,
        deleted: Vec<String>,
        failed: Vec<String>,
    ) -> PyResult<Self> {
        let deleted_list = PyList::empty(py);
        for name in deleted {
            deleted_list.append(name)?;
        }
        let failed_list = PyList::empty(py);
        for name in failed {
            failed_list.append(name)?;
        }
        Ok(Self {
            deleted_notebooks: deleted_list.unbind(),
            failed_notebooks: failed_list.unbind(),
        })
    }
}
