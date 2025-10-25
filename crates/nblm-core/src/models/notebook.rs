use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::source::NotebookSource;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Notebook {
    pub name: Option<String>,
    pub title: String,
    #[serde(rename = "notebookId", skip_serializing_if = "Option::is_none")]
    pub notebook_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sources: Vec<NotebookSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<NotebookMetadata>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotebookRef {
    pub notebook_id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NotebookMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_shareable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_shared: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_viewed: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn notebook_skips_notebook_id_when_none() {
        let notebook = Notebook {
            name: Some("test".to_string()),
            title: "Test Notebook".to_string(),
            notebook_id: None,
            emoji: None,
            metadata: None,
            sources: Vec::new(),
            extra: Default::default(),
        };
        let json = serde_json::to_string(&notebook).unwrap();
        assert!(!json.contains("notebookId"));
    }

    #[test]
    fn notebook_includes_notebook_id_when_some() {
        let notebook = Notebook {
            name: Some("test".to_string()),
            title: "Test Notebook".to_string(),
            notebook_id: Some("nb123".to_string()),
            emoji: None,
            metadata: None,
            sources: Vec::new(),
            extra: Default::default(),
        };
        let json = serde_json::to_string(&notebook).unwrap();
        assert!(json.contains("notebookId"));
        assert!(json.contains("nb123"));
    }
}
