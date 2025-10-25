use serde::{Deserialize, Serialize};

use crate::models::NotebookSource;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BatchCreateSourcesResponse {
    #[serde(default)]
    pub sources: Vec<NotebookSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_count: Option<i32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn batch_create_sources_response_deserializes_correctly() {
        let json = r#"{
            "sources": [
                {
                    "name": "projects/123/locations/global/notebooks/abc/sources/123",
                    "title": "Test Source",
                    "metadata": {
                        "wordCount": 100
                    }
                }
            ],
            "errorCount": 0
        }"#;
        let response: BatchCreateSourcesResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.sources.len(), 1);
        assert_eq!(response.error_count, Some(0));
        assert_eq!(
            response.sources[0].name,
            "projects/123/locations/global/notebooks/abc/sources/123"
        );
        assert_eq!(response.sources[0].title.as_ref().unwrap(), "Test Source");
    }
}
