use serde::{Deserialize, Serialize};

use crate::models::enterprise::notebook::Notebook;

/// Response from list recently viewed notebooks API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ListRecentlyViewedResponse {
    #[serde(default)]
    pub notebooks: Vec<Notebook>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_recently_viewed_response_deserializes_correctly() {
        let json = r#"{
            "notebooks": [
                {"title": "Notebook 1", "name": "notebooks/123"},
                {"title": "Notebook 2", "name": "notebooks/456"}
            ]
        }"#;
        let response: ListRecentlyViewedResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.notebooks.len(), 2);
        assert_eq!(response.notebooks[0].title, "Notebook 1");
        assert_eq!(response.notebooks[1].title, "Notebook 2");
    }

    #[test]
    fn list_recently_viewed_response_deserializes_empty() {
        let json = r#"{}"#;
        let response: ListRecentlyViewedResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.notebooks.len(), 0);
    }

    #[test]
    fn list_recently_viewed_response_serializes_correctly() {
        let response = ListRecentlyViewedResponse {
            notebooks: vec![Notebook {
                name: Some("notebooks/123".to_string()),
                title: "Test Notebook".to_string(),
                ..Default::default()
            }],
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("notebooks"));
        assert!(json.contains("Test Notebook"));
    }
}
