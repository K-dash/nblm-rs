use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::models::{NotebookSource, NotebookSourceId};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BatchCreateSourcesResponse {
    #[serde(default)]
    pub sources: Vec<NotebookSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_count: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UploadSourceFileResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<NotebookSourceId>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
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

    #[test]
    fn upload_source_file_response_deserializes() {
        let json = r#"{
            "sourceId": {
                "id": "projects/123/locations/global/notebooks/abc/sources/source-id"
            },
            "requestId": "abc123"
        }"#;
        let response: UploadSourceFileResponse = serde_json::from_str(json).unwrap();
        assert_eq!(
            response.source_id.as_ref().and_then(|id| id.id.as_deref()),
            Some("projects/123/locations/global/notebooks/abc/sources/source-id")
        );
        assert_eq!(
            response
                .extra
                .get("requestId")
                .and_then(|value| value.as_str()),
            Some("abc123")
        );
    }
}
