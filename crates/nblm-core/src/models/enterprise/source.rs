use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NotebookSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<NotebookSourceMetadata>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<NotebookSourceSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<NotebookSourceId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NotebookSourceMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_added_timestamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub word_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub youtube_metadata: Option<NotebookSourceYoutubeMetadata>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NotebookSourceYoutubeMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_id: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NotebookSourceSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NotebookSourceId {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserContent {
    Web {
        #[serde(rename = "webContent")]
        web_content: WebContent,
    },
    Text {
        #[serde(rename = "textContent")]
        text_content: TextContent,
    },
    GoogleDrive {
        #[serde(rename = "googleDriveContent")]
        google_drive_content: GoogleDriveContent,
    },
    Video {
        #[serde(rename = "videoContent")]
        video_content: VideoContent,
    },
}

impl UserContent {
    pub fn web(url: String, source_name: Option<String>) -> Self {
        Self::Web {
            web_content: WebContent { url, source_name },
        }
    }

    pub fn text(content: String, source_name: Option<String>) -> Self {
        Self::Text {
            text_content: TextContent {
                content,
                source_name,
            },
        }
    }

    pub fn google_drive(
        document_id: String,
        mime_type: String,
        source_name: Option<String>,
    ) -> Self {
        Self::GoogleDrive {
            google_drive_content: GoogleDriveContent {
                document_id,
                mime_type,
                source_name,
            },
        }
    }

    pub fn video(url: String) -> Self {
        Self::Video {
            video_content: VideoContent { url },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WebContent {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TextContent {
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GoogleDriveContent {
    pub document_id: String,
    pub mime_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VideoContent {
    #[serde(rename = "youtubeUrl")]
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BatchCreateSourcesRequest {
    #[serde(rename = "userContents")]
    pub user_contents: Vec<UserContent>,
}

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
pub struct BatchDeleteSourcesRequest {
    pub names: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BatchDeleteSourcesResponse {
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UploadSourceFileResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<NotebookSourceId>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test 1: UserContent constructor methods
    #[test]
    fn test_user_content_web_constructor() {
        let url = "https://example.com".to_string();
        let source_name = Some("Test Web Source".to_string());

        let content = UserContent::web(url.clone(), source_name.clone());

        match content {
            UserContent::Web { web_content } => {
                assert_eq!(web_content.url, url);
                assert_eq!(web_content.source_name, source_name);
            }
            _ => panic!("Expected UserContent::Web variant"),
        }
    }

    #[test]
    fn test_user_content_web_constructor_without_name() {
        let url = "https://example.com".to_string();

        let content = UserContent::web(url.clone(), None);

        match content {
            UserContent::Web { web_content } => {
                assert_eq!(web_content.url, url);
                assert_eq!(web_content.source_name, None);
            }
            _ => panic!("Expected UserContent::Web variant"),
        }
    }

    #[test]
    fn test_user_content_text_constructor() {
        let text = "Sample text content".to_string();
        let source_name = Some("Test Text".to_string());

        let content = UserContent::text(text.clone(), source_name.clone());

        match content {
            UserContent::Text { text_content } => {
                assert_eq!(text_content.content, text);
                assert_eq!(text_content.source_name, source_name);
            }
            _ => panic!("Expected UserContent::Text variant"),
        }
    }

    #[test]
    fn test_user_content_google_drive_constructor() {
        let document_id = "doc-12345".to_string();
        let mime_type = "application/pdf".to_string();
        let source_name = Some("Drive Document".to_string());

        let content =
            UserContent::google_drive(document_id.clone(), mime_type.clone(), source_name.clone());

        match content {
            UserContent::GoogleDrive {
                google_drive_content,
            } => {
                assert_eq!(google_drive_content.document_id, document_id);
                assert_eq!(google_drive_content.mime_type, mime_type);
                assert_eq!(google_drive_content.source_name, source_name);
            }
            _ => panic!("Expected UserContent::GoogleDrive variant"),
        }
    }

    #[test]
    fn test_user_content_video_constructor() {
        let url = "https://youtube.com/watch?v=abc123".to_string();

        let content = UserContent::video(url.clone());

        match content {
            UserContent::Video { video_content } => {
                assert_eq!(video_content.url, url);
            }
            _ => panic!("Expected UserContent::Video variant"),
        }
    }

    // Test 2: Serialization/Deserialization correctness
    #[test]
    fn test_user_content_web_serialization() {
        let content = UserContent::web(
            "https://example.com".to_string(),
            Some("Web Source".to_string()),
        );

        let json = serde_json::to_value(&content).unwrap();

        assert_eq!(json["webContent"]["url"], "https://example.com");
        assert_eq!(json["webContent"]["sourceName"], "Web Source");
    }

    #[test]
    fn test_user_content_web_deserialization() {
        let json = serde_json::json!({
            "webContent": {
                "url": "https://example.com",
                "sourceName": "Web Source"
            }
        });

        let content: UserContent = serde_json::from_value(json).unwrap();

        match content {
            UserContent::Web { web_content } => {
                assert_eq!(web_content.url, "https://example.com");
                assert_eq!(web_content.source_name, Some("Web Source".to_string()));
            }
            _ => panic!("Expected UserContent::Web variant"),
        }
    }

    #[test]
    fn test_user_content_text_serialization() {
        let content = UserContent::text("Sample text".to_string(), Some("Text Source".to_string()));

        let json = serde_json::to_value(&content).unwrap();

        assert_eq!(json["textContent"]["content"], "Sample text");
        assert_eq!(json["textContent"]["sourceName"], "Text Source");
    }

    #[test]
    fn test_user_content_text_deserialization() {
        let json = serde_json::json!({
            "textContent": {
                "content": "Sample text",
                "sourceName": "Text Source"
            }
        });

        let content: UserContent = serde_json::from_value(json).unwrap();

        match content {
            UserContent::Text { text_content } => {
                assert_eq!(text_content.content, "Sample text");
                assert_eq!(text_content.source_name, Some("Text Source".to_string()));
            }
            _ => panic!("Expected UserContent::Text variant"),
        }
    }

    #[test]
    fn test_user_content_google_drive_serialization() {
        let content = UserContent::google_drive(
            "doc-123".to_string(),
            "application/pdf".to_string(),
            Some("Drive Doc".to_string()),
        );

        let json = serde_json::to_value(&content).unwrap();

        assert_eq!(json["googleDriveContent"]["documentId"], "doc-123");
        assert_eq!(json["googleDriveContent"]["mimeType"], "application/pdf");
        assert_eq!(json["googleDriveContent"]["sourceName"], "Drive Doc");
    }

    #[test]
    fn test_user_content_google_drive_deserialization() {
        let json = serde_json::json!({
            "googleDriveContent": {
                "documentId": "doc-123",
                "mimeType": "application/pdf",
                "sourceName": "Drive Doc"
            }
        });

        let content: UserContent = serde_json::from_value(json).unwrap();

        match content {
            UserContent::GoogleDrive {
                google_drive_content,
            } => {
                assert_eq!(google_drive_content.document_id, "doc-123");
                assert_eq!(google_drive_content.mime_type, "application/pdf");
                assert_eq!(
                    google_drive_content.source_name,
                    Some("Drive Doc".to_string())
                );
            }
            _ => panic!("Expected UserContent::GoogleDrive variant"),
        }
    }

    #[test]
    fn test_user_content_video_serialization() {
        let content = UserContent::video("https://youtube.com/watch?v=abc".to_string());

        let json = serde_json::to_value(&content).unwrap();

        assert_eq!(
            json["videoContent"]["youtubeUrl"],
            "https://youtube.com/watch?v=abc"
        );
    }

    #[test]
    fn test_user_content_video_deserialization() {
        let json = serde_json::json!({
            "videoContent": {
                "youtubeUrl": "https://youtube.com/watch?v=abc"
            }
        });

        let content: UserContent = serde_json::from_value(json).unwrap();

        match content {
            UserContent::Video { video_content } => {
                assert_eq!(video_content.url, "https://youtube.com/watch?v=abc");
            }
            _ => panic!("Expected UserContent::Video variant"),
        }
    }

    // Test 3: skip_serializing_if behavior (None fields are omitted)
    #[test]
    fn test_web_content_omits_none_source_name() {
        let content = WebContent {
            url: "https://example.com".to_string(),
            source_name: None,
        };

        let json = serde_json::to_value(&content).unwrap();
        let obj = json.as_object().unwrap();

        // sourceName should not be present in JSON
        assert!(!obj.contains_key("sourceName"));
        assert_eq!(obj.get("url").unwrap(), "https://example.com");
    }

    #[test]
    fn test_web_content_includes_some_source_name() {
        let content = WebContent {
            url: "https://example.com".to_string(),
            source_name: Some("My Source".to_string()),
        };

        let json = serde_json::to_value(&content).unwrap();
        let obj = json.as_object().unwrap();

        // sourceName should be present
        assert!(obj.contains_key("sourceName"));
        assert_eq!(obj.get("sourceName").unwrap(), "My Source");
    }

    #[test]
    fn test_text_content_omits_none_source_name() {
        let content = TextContent {
            content: "text".to_string(),
            source_name: None,
        };

        let json = serde_json::to_value(&content).unwrap();
        let obj = json.as_object().unwrap();

        assert!(!obj.contains_key("sourceName"));
        assert_eq!(obj.get("content").unwrap(), "text");
    }

    #[test]
    fn test_google_drive_content_omits_none_source_name() {
        let content = GoogleDriveContent {
            document_id: "doc-123".to_string(),
            mime_type: "application/pdf".to_string(),
            source_name: None,
        };

        let json = serde_json::to_value(&content).unwrap();
        let obj = json.as_object().unwrap();

        assert!(!obj.contains_key("sourceName"));
        assert_eq!(obj.get("documentId").unwrap(), "doc-123");
        assert_eq!(obj.get("mimeType").unwrap(), "application/pdf");
    }

    #[test]
    fn test_notebook_source_omits_none_fields() {
        let source = NotebookSource {
            name: "projects/123/notebooks/456/sources/789".to_string(),
            metadata: None,
            settings: None,
            source_id: None,
            title: None,
            extra: HashMap::new(),
        };

        let json = serde_json::to_value(&source).unwrap();
        let obj = json.as_object().unwrap();

        // Only 'name' should be present
        assert!(obj.contains_key("name"));
        assert!(!obj.contains_key("metadata"));
        assert!(!obj.contains_key("settings"));
        assert!(!obj.contains_key("sourceId"));
        assert!(!obj.contains_key("title"));
    }

    #[test]
    fn test_notebook_source_includes_some_fields() {
        let source = NotebookSource {
            name: "projects/123/notebooks/456/sources/789".to_string(),
            metadata: Some(NotebookSourceMetadata {
                word_count: Some(1000),
                ..Default::default()
            }),
            settings: Some(NotebookSourceSettings {
                status: Some("ACTIVE".to_string()),
                ..Default::default()
            }),
            source_id: Some(NotebookSourceId {
                id: Some("source-123".to_string()),
                ..Default::default()
            }),
            title: Some("My Document".to_string()),
            extra: HashMap::new(),
        };

        let json = serde_json::to_value(&source).unwrap();
        let obj = json.as_object().unwrap();

        assert!(obj.contains_key("name"));
        assert!(obj.contains_key("metadata"));
        assert!(obj.contains_key("settings"));
        assert!(obj.contains_key("sourceId"));
        assert!(obj.contains_key("title"));
    }

    #[test]
    fn test_batch_create_sources_request_serialization() {
        let request = BatchCreateSourcesRequest {
            user_contents: vec![
                UserContent::web("https://example.com".to_string(), None),
                UserContent::text("Sample text".to_string(), Some("Text".to_string())),
            ],
        };

        let json = serde_json::to_value(&request).unwrap();

        assert!(json["userContents"].is_array());
        assert_eq!(json["userContents"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn test_batch_create_sources_response_deserialization() {
        let json = serde_json::json!({
            "sources": [
                {
                    "name": "projects/123/notebooks/456/sources/789"
                }
            ],
            "errorCount": 0
        });

        let response: BatchCreateSourcesResponse = serde_json::from_value(json).unwrap();

        assert_eq!(response.sources.len(), 1);
        assert_eq!(response.error_count, Some(0));
    }

    #[test]
    fn test_batch_create_sources_response_empty_sources() {
        let json = serde_json::json!({
            "sources": []
        });

        let response: BatchCreateSourcesResponse = serde_json::from_value(json).unwrap();

        assert_eq!(response.sources.len(), 0);
        assert_eq!(response.error_count, None);
    }

    #[test]
    fn test_notebook_source_metadata_youtube() {
        let metadata = NotebookSourceMetadata {
            source_added_timestamp: Some("2024-01-01T00:00:00Z".to_string()),
            word_count: Some(5000),
            youtube_metadata: Some(NotebookSourceYoutubeMetadata {
                channel_name: Some("Test Channel".to_string()),
                video_id: Some("abc123".to_string()),
                extra: HashMap::new(),
            }),
            extra: HashMap::new(),
        };

        let json = serde_json::to_value(&metadata).unwrap();

        assert_eq!(json["sourceAddedTimestamp"], "2024-01-01T00:00:00Z");
        assert_eq!(json["wordCount"], 5000);
        assert_eq!(json["youtubeMetadata"]["channelName"], "Test Channel");
        assert_eq!(json["youtubeMetadata"]["videoId"], "abc123");
    }

    #[test]
    fn test_upload_source_file_response_omits_none_source_id() {
        let response = UploadSourceFileResponse {
            source_id: None,
            extra: HashMap::new(),
        };

        let json = serde_json::to_value(&response).unwrap();
        let obj = json.as_object().unwrap();

        assert!(!obj.contains_key("sourceId"));
    }

    #[test]
    fn test_camel_case_serialization() {
        // Verify that snake_case fields are converted to camelCase
        let metadata = NotebookSourceMetadata {
            source_added_timestamp: Some("2024-01-01T00:00:00Z".to_string()),
            word_count: Some(100),
            youtube_metadata: None,
            extra: HashMap::new(),
        };

        let json = serde_json::to_string(&metadata).unwrap();

        // Should contain camelCase keys
        assert!(json.contains("sourceAddedTimestamp"));
        assert!(json.contains("wordCount"));
        // Should NOT contain snake_case keys
        assert!(!json.contains("source_added_timestamp"));
        assert!(!json.contains("word_count"));
    }
}
