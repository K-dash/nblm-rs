use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::error::PyResult;

use super::extra_to_pydict;

/// Request for creating an audio overview.
///
/// Note: As of the current API version, this request must be empty.
/// All fields are reserved for future use.
#[pyclass(module = "nblm")]
#[derive(Clone, Default)]
pub struct AudioOverviewRequest {
    // Currently, the API only accepts an empty request body
    // Fields are commented out for future compatibility
    // #[pyo3(get, set)]
    // pub source_ids: Option<Vec<String>>,
    // #[pyo3(get, set)]
    // pub episode_focus: Option<String>,
    // #[pyo3(get, set)]
    // pub language_code: Option<String>,
}

#[pymethods]
impl AudioOverviewRequest {
    #[new]
    pub fn new() -> Self {
        Self {}
    }

    pub fn __repr__(&self) -> String {
        "AudioOverviewRequest()".to_string()
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }
}

impl AudioOverviewRequest {
    pub(crate) fn to_core(&self) -> nblm_core::models::enterprise::audio::AudioOverviewRequest {
        nblm_core::models::enterprise::audio::AudioOverviewRequest::default()
    }
}

/// Response from creating or getting an audio overview.
#[pyclass(module = "nblm")]
pub struct AudioOverviewResponse {
    #[pyo3(get)]
    pub audio_overview_id: Option<String>,
    #[pyo3(get)]
    pub name: Option<String>,
    #[pyo3(get)]
    pub status: Option<String>,
    #[pyo3(get)]
    pub generation_options: Py<PyAny>,
    #[pyo3(get)]
    pub extra: Py<PyDict>,
}

#[pymethods]
impl AudioOverviewResponse {
    pub fn __repr__(&self) -> String {
        format!(
            "AudioOverviewResponse(audio_overview_id={:?}, name={:?}, status={:?})",
            self.audio_overview_id, self.name, self.status
        )
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }
}

impl AudioOverviewResponse {
    pub(crate) fn from_core(
        py: Python,
        response: nblm_core::models::enterprise::audio::AudioOverviewResponse,
    ) -> PyResult<Self> {
        let generation_options = match response.generation_options {
            Some(value) => crate::models::json_value_to_py(py, &value)?,
            None => py.None(),
        };

        Ok(Self {
            audio_overview_id: response.audio_overview_id,
            name: response.name,
            status: response.status,
            generation_options,
            extra: extra_to_pydict(py, &response.extra)?,
        })
    }
}
