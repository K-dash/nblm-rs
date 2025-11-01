use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

use crate::error::PyResult;

use super::{extra_to_pydict, Notebook, NotebookSource, NotebookSourceId};

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
        response: nblm_core::models::enterprise::notebook::ListRecentlyViewedResponse,
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
        _response: nblm_core::models::enterprise::notebook::BatchDeleteNotebooksResponse,
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

#[pyclass(module = "nblm")]
pub struct BatchCreateSourcesResponse {
    #[pyo3(get)]
    pub sources: Py<PyList>,
    #[pyo3(get)]
    pub error_count: Option<i32>,
}

#[pymethods]
impl BatchCreateSourcesResponse {
    pub fn __repr__(&self, py: Python) -> String {
        let count = self.sources.bind(py).len();
        format!(
            "BatchCreateSourcesResponse(sources={} items, error_count={:?})",
            count, self.error_count
        )
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }
}

impl BatchCreateSourcesResponse {
    pub fn from_core(
        py: Python,
        response: nblm_core::models::enterprise::source::BatchCreateSourcesResponse,
    ) -> PyResult<Self> {
        let sources_list = PyList::empty(py);
        for source in response.sources {
            let py_source = NotebookSource::from_core(py, source)?;
            sources_list.append(py_source)?;
        }
        Ok(Self {
            sources: sources_list.unbind(),
            error_count: response.error_count,
        })
    }
}

#[pyclass(module = "nblm")]
pub struct BatchDeleteSourcesResponse {
    #[pyo3(get)]
    pub extra: Py<PyDict>,
}

#[pymethods]
impl BatchDeleteSourcesResponse {
    pub fn __repr__(&self, py: Python) -> String {
        let keys = self.extra.bind(py).len();
        format!("BatchDeleteSourcesResponse(extra_keys={})", keys)
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }
}

impl BatchDeleteSourcesResponse {
    pub fn from_core(
        py: Python,
        response: nblm_core::models::enterprise::source::BatchDeleteSourcesResponse,
    ) -> PyResult<Self> {
        Ok(Self {
            extra: extra_to_pydict(py, &response.extra)?,
        })
    }
}

#[pyclass(module = "nblm")]
pub struct UploadSourceFileResponse {
    #[pyo3(get)]
    pub source_id: Option<Py<NotebookSourceId>>,
    #[pyo3(get)]
    pub extra: Py<PyDict>,
}

#[pymethods]
impl UploadSourceFileResponse {
    pub fn __repr__(&self, py: Python) -> String {
        let has_id = self.source_id.is_some();
        let extra_keys = self.extra.bind(py).len();
        format!(
            "UploadSourceFileResponse(source_id={}, extra_keys={})",
            has_id, extra_keys
        )
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }
}

impl UploadSourceFileResponse {
    pub fn from_core(
        py: Python,
        response: nblm_core::models::enterprise::source::UploadSourceFileResponse,
    ) -> PyResult<Self> {
        let source_id = match response.source_id {
            Some(id) => Some(Py::new(py, NotebookSourceId::from_core(py, id)?)?),
            None => None,
        };
        Ok(Self {
            source_id,
            extra: extra_to_pydict(py, &response.extra)?,
        })
    }
}
