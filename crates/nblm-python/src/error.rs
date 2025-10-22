use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;

create_exception!(nblm, NblmError, PyException);

pub type PyResult<T> = Result<T, PyErr>;

pub(crate) fn map_nblm_error(err: nblm_core::Error) -> PyErr {
    NblmError::new_err(err.to_string())
}

pub(crate) trait IntoPyResult<T> {
    fn into_py_result(self) -> PyResult<T>;
}

impl<T> IntoPyResult<T> for Result<T, nblm_core::Error> {
    fn into_py_result(self) -> PyResult<T> {
        self.map_err(map_nblm_error)
    }
}
