use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::models::source::UserContent;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BatchCreateSourcesRequest {
    #[serde(rename = "userContents")]
    pub user_contents: Vec<UserContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchDeleteSourcesRequest {
    pub names: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchDeleteSourcesResponse {
    // API may return empty response or status information
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::source::{TextContent, WebContent};

    #[test]
    fn batch_create_sources_request_serializes_with_user_contents() {
        let request = BatchCreateSourcesRequest {
            user_contents: vec![
                UserContent::Web {
                    web_content: WebContent {
                        url: "https://example.com".to_string(),
                        source_name: None,
                    },
                },
                UserContent::Text {
                    text_content: TextContent {
                        content: "Sample text".to_string(),
                        source_name: Some("My text".to_string()),
                    },
                },
            ],
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("userContents"));
        assert!(json.contains("https://example.com"));
        assert!(json.contains("Sample text"));
    }

    #[test]
    fn batch_delete_sources_request_serializes_correctly() {
        let request = BatchDeleteSourcesRequest {
            names: vec!["sources/123".to_string()],
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains(r#""names""#));
        assert!(json.contains("sources/123"));
    }

    #[test]
    fn batch_delete_sources_response_deserializes_empty() {
        let json = r#"{}"#;
        let response: BatchDeleteSourcesResponse = serde_json::from_str(json).unwrap();
        assert!(response.extra.is_empty());
    }
}
