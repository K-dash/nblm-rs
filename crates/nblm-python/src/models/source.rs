use pyo3::prelude::*;

/// Source type for adding web URLs to a notebook.
#[pyclass(module = "nblm")]
#[derive(Clone)]
pub struct WebSource {
    #[pyo3(get)]
    pub url: String,
    #[pyo3(get)]
    pub name: Option<String>,
}

#[pymethods]
impl WebSource {
    #[new]
    #[pyo3(signature = (url, name=None))]
    fn new(url: String, name: Option<String>) -> Self {
        Self { url, name }
    }

    pub fn __repr__(&self) -> String {
        format!("WebSource(url={:?}, name={:?})", self.url, self.name)
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }
}

/// Source type for adding text content to a notebook.
#[pyclass(module = "nblm")]
#[derive(Clone)]
pub struct TextSource {
    #[pyo3(get)]
    pub content: String,
    #[pyo3(get)]
    pub name: Option<String>,
}

#[pymethods]
impl TextSource {
    #[new]
    #[pyo3(signature = (content, name=None))]
    fn new(content: String, name: Option<String>) -> Self {
        Self { content, name }
    }

    pub fn __repr__(&self) -> String {
        format!(
            "TextSource(content={:?}, name={:?})",
            self.content, self.name
        )
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }
}

/// Source type for adding Google Drive documents to a notebook.
#[pyclass(module = "nblm")]
#[derive(Clone)]
pub struct GoogleDriveSource {
    #[pyo3(get)]
    pub document_id: String,
    #[pyo3(get)]
    pub mime_type: String,
    #[pyo3(get)]
    pub name: Option<String>,
}

#[pymethods]
impl GoogleDriveSource {
    #[new]
    #[pyo3(signature = (document_id, mime_type, name=None))]
    fn new(document_id: String, mime_type: String, name: Option<String>) -> Self {
        Self {
            document_id,
            mime_type,
            name,
        }
    }

    pub fn __repr__(&self) -> String {
        format!(
            "GoogleDriveSource(document_id={:?}, mime_type={:?}, name={:?})",
            self.document_id, self.mime_type, self.name
        )
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }
}

/// Source type for adding YouTube videos to a notebook.
#[pyclass(module = "nblm")]
#[derive(Clone)]
pub struct VideoSource {
    #[pyo3(get)]
    pub url: String,
}

#[pymethods]
impl VideoSource {
    #[new]
    fn new(url: String) -> Self {
        Self { url }
    }

    pub fn __repr__(&self) -> String {
        format!("VideoSource(url={:?})", self.url)
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }
}
