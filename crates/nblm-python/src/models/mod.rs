use pyo3::prelude::*;
use pyo3::types::{PyBool, PyDict, PyFloat, PyList, PyNone, PyString};
use pyo3::IntoPyObject;
use serde_json::Value;
use std::collections::HashMap;

use crate::error::PyResult;

mod audio;
mod notebook;
mod notebook_source;
mod responses;
mod source;

pub use audio::*;
pub use notebook::*;
pub use notebook_source::*;
pub use responses::*;
pub use source::*;

/// Convert `serde_json::Value` to a Python object.
pub(crate) fn json_value_to_py(py: Python, value: &Value) -> PyResult<Py<PyAny>> {
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
pub(crate) fn extra_to_pydict(py: Python, extra: &HashMap<String, Value>) -> PyResult<Py<PyDict>> {
    let dict = PyDict::new(py);
    for (k, v) in extra {
        dict.set_item(k, json_value_to_py(py, v)?)?;
    }
    Ok(dict.unbind())
}
