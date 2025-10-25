use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Notebook {
    pub name: Option<String>,
    pub title: String,
    #[serde(rename = "notebookId", skip_serializing_if = "Option::is_none")]
    pub notebook_id: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotebookRef {
    pub notebook_id: String,
    pub name: String,
}

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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ShareRequest {
    pub account_and_roles: Vec<AccountRole>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AccountRole {
    pub email: String,
    pub role: ProjectRole,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProjectRole {
    ProjectRoleOwner,
    ProjectRoleWriter,
    #[default]
    ProjectRoleReader,
    ProjectRoleNotShared,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ShareResponse {
    #[serde(default)]
    pub granted: Option<i32>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Response from list recently viewed notebooks API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ListRecentlyViewedResponse {
    #[serde(default)]
    pub notebooks: Vec<Notebook>,
}

/// Audio Overview creation request.
///
/// # Known Issues (as of 2025-10-19)
///
/// Despite the API documentation mentioning fields like `sourceIds`, `episodeFocus`,
/// and `languageCode`, the actual API only accepts an empty request body `{}`.
/// Any fields sent result in "Unknown name" errors.
/// These configuration options are likely set through the NotebookLM UI after creation.
///
/// The fields below are commented out but kept for future compatibility if the API
/// implements them.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AudioOverviewRequest {
    // TODO: Uncomment when API supports these fields
    // #[serde(skip_serializing_if = "Option::is_none", rename = "sourceIds")]
    // pub source_ids: Option<Vec<SourceId>>,
    // #[serde(skip_serializing_if = "Option::is_none", rename = "episodeFocus")]
    // pub episode_focus: Option<String>,
    // #[serde(skip_serializing_if = "Option::is_none", rename = "languageCode")]
    // pub language_code: Option<String>,
}

// TODO: Uncomment when API supports sourceIds field
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct SourceId {
//     pub id: String,
// }

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AudioOverviewResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
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

    #[test]
    fn notebook_skips_notebook_id_when_none() {
        let notebook = Notebook {
            name: Some("test".to_string()),
            title: "Test Notebook".to_string(),
            notebook_id: None,
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
            extra: Default::default(),
        };
        let json = serde_json::to_string(&notebook).unwrap();
        assert!(json.contains("notebookId"));
        assert!(json.contains("nb123"));
    }
}
