use crate::error::{map_runtime_error, IntoPyResult, PyResult};
use std::future::Future;
use tokio::runtime::Handle;

/// Block on an async future, creating a Tokio runtime if needed.
pub fn block_on_with_runtime<F, T>(future: F) -> PyResult<T>
where
    F: Future<Output = nblm_core::Result<T>> + Send + 'static,
    T: Send + 'static,
{
    if let Ok(handle) = Handle::try_current() {
        return handle.block_on(future).into_py_result();
    }

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .map_err(map_runtime_error)?;
    runtime.block_on(future).into_py_result()
}
