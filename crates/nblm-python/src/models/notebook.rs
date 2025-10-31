use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

use crate::error::PyResult;

use super::{extra_to_pydict, NotebookSource};

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
    pub(crate) fn from_core(
        py: Python,
        metadata: nblm_core::models::enterprise::notebook::NotebookMetadata,
    ) -> PyResult<Self> {
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
    pub fn from_core(
        py: Python,
        notebook: nblm_core::models::enterprise::notebook::Notebook,
    ) -> PyResult<Self> {
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
