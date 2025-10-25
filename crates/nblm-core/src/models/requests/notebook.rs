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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_notebook_request_serializes_correctly() {
        let request = CreateNotebookRequest {
            title: "Test Notebook".to_string(),
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains(r#""title":"Test Notebook""#));
    }

    #[test]
    fn batch_delete_notebooks_request_serializes_correctly() {
        let request = BatchDeleteNotebooksRequest {
            names: vec!["notebooks/123".to_string(), "notebooks/456".to_string()],
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains(r#""names""#));
        assert!(json.contains("notebooks/123"));
        assert!(json.contains("notebooks/456"));
    }

    #[test]
    fn batch_delete_notebooks_response_deserializes_empty() {
        let json = r#"{}"#;
        let response: BatchDeleteNotebooksResponse = serde_json::from_str(json).unwrap();
        assert!(response.extra.is_empty());
    }

    #[test]
    fn batch_delete_notebooks_response_deserializes_with_extra() {
        let json = r#"{"customField":"value"}"#;
        let response: BatchDeleteNotebooksResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.extra.len(), 1);
        assert_eq!(
            response.extra.get("customField").unwrap().as_str().unwrap(),
            "value"
        );
    }
}
