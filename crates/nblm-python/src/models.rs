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
pub struct Notebook {
    #[pyo3(get)]
    pub name: Option<String>,
    #[pyo3(get)]
    pub title: String,
    #[pyo3(get)]
    pub notebook_id: Option<String>,
    #[pyo3(get)]
    pub extra: Py<PyDict>,
}

#[pymethods]
impl Notebook {
    pub fn __repr__(&self) -> String {
        format!(
            "Notebook(title='{}', notebook_id={:?})",
            self.title, self.notebook_id
        )
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }
}

impl Notebook {
    pub fn from_core(py: Python, notebook: nblm_core::models::Notebook) -> PyResult<Self> {
        let extra = extra_to_pydict(py, &notebook.extra)?;
        Ok(Self {
            name: notebook.name,
            title: notebook.title,
            notebook_id: notebook.notebook_id,
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
