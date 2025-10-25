use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BatchCreateSourcesResponse {
    #[serde(default)]
    pub sources: Vec<SourceResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_count: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SourceResult {
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
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
                {"url": "https://example.com", "name": "sources/123", "status": "SUCCESS"},
                {"url": "https://example2.com", "name": "sources/456", "status": "PENDING"}
            ],
            "errorCount": 0
        }"#;
        let response: BatchCreateSourcesResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.sources.len(), 2);
        assert_eq!(response.error_count, Some(0));
        assert_eq!(
            response.sources[0].url.as_ref().unwrap(),
            "https://example.com"
        );
        assert_eq!(response.sources[0].name.as_ref().unwrap(), "sources/123");
        assert_eq!(response.sources[0].status.as_ref().unwrap(), "SUCCESS");
    }

    #[test]
    fn source_result_skips_none_fields() {
        let result = SourceResult {
            url: Some("https://example.com".to_string()),
            name: None,
            status: Some("SUCCESS".to_string()),
            extra: HashMap::new(),
        };
        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("url"));
        assert!(json.contains("status"));
        assert!(!json.contains("name"));
    }

    #[test]
    fn source_result_with_extra_fields() {
        let json = r#"{
            "url": "https://example.com",
            "name": "sources/123",
            "customField": "customValue"
        }"#;
        let result: SourceResult = serde_json::from_str(json).unwrap();
        assert_eq!(result.url.as_ref().unwrap(), "https://example.com");
        assert_eq!(
            result.extra.get("customField").unwrap().as_str().unwrap(),
            "customValue"
        );
    }
}
