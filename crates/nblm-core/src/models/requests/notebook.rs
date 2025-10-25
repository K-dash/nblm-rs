use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateNotebookRequest {
    pub title: String,
}

/// Batch delete notebooks request.
///
/// # Known Issues (as of 2025-10-19)
///
/// Despite the API being named "batchDelete" and accepting an array of names,
/// the API returns HTTP 400 error when multiple notebook names are provided.
/// Only single notebook deletion works (array with 1 element).
///
/// To delete multiple notebooks, call this API multiple times with one notebook at a time.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchDeleteNotebooksRequest {
    pub names: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchDeleteNotebooksResponse {
    // API returns empty response or status information
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}
