use std::collections::HashMap;

use serde::{Deserialize, Serialize};

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
    pub extra: HashMap<String, serde_json::Value>,
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
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NotebookSourceYoutubeMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_id: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NotebookSourceSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NotebookSourceId {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
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

/// Google Drive content for adding sources.
///
/// # Known Issues
///
/// **WARNING**: As of 2025-10-19, the NotebookLM API returns HTTP 500 Internal Server Error
/// when attempting to add Google Drive sources. This functionality is currently unavailable.
/// The error occurs even with proper authentication (`gcloud auth login --enable-gdrive-access`)
/// and correct IAM permissions. No detailed error information is provided in API responses or
/// Google Cloud logs, indicating a server-side issue with the NotebookLM API.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_content_untagged_web() {
        let json = r#"{"webContent":{"url":"https://example.com"}}"#;
        let content: UserContent = serde_json::from_str(json).unwrap();
        match content {
            UserContent::Web { web_content } => {
                assert_eq!(web_content.url, "https://example.com");
            }
            _ => panic!("expected Web variant"),
        }
    }

    #[test]
    fn user_content_untagged_text() {
        let json = r#"{"textContent":{"content":"sample text"}}"#;
        let content: UserContent = serde_json::from_str(json).unwrap();
        match content {
            UserContent::Text { text_content } => {
                assert_eq!(text_content.content, "sample text");
            }
            _ => panic!("expected Text variant"),
        }
    }

    #[test]
    fn user_content_untagged_google_drive() {
        let json = r#"{"googleDriveContent":{"documentId":"123","mimeType":"application/vnd.google-apps.document"}}"#;
        let content: UserContent = serde_json::from_str(json).unwrap();
        match content {
            UserContent::GoogleDrive {
                google_drive_content,
            } => {
                assert_eq!(google_drive_content.document_id, "123");
                assert_eq!(
                    google_drive_content.mime_type,
                    "application/vnd.google-apps.document"
                );
            }
            _ => panic!("expected GoogleDrive variant"),
        }
    }

    #[test]
    fn user_content_untagged_video() {
        let json = r#"{"videoContent":{"youtubeUrl":"https://youtube.com/watch?v=123"}}"#;
        let content: UserContent = serde_json::from_str(json).unwrap();
        match content {
            UserContent::Video { video_content } => {
                assert_eq!(video_content.url, "https://youtube.com/watch?v=123");
            }
            _ => panic!("expected Video variant"),
        }
    }

    #[test]
    fn user_content_video_serializes_correctly() {
        let content = UserContent::Video {
            video_content: VideoContent {
                url: "https://youtube.com/watch?v=123".to_string(),
            },
        };
        let json = serde_json::to_string(&content).unwrap();
        assert!(
            json.contains("videoContent"),
            "JSON should contain videoContent, got: {}",
            json
        );
        assert!(
            json.contains(r#""youtubeUrl":"https://youtube.com/watch?v=123""#),
            "JSON should contain youtubeUrl field, got: {}",
            json
        );
    }
}
