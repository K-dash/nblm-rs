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
    pub notebook: Notebook,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BatchCreateSourcesRequest {
    #[serde(rename = "userContents")]
    pub user_contents: Vec<UserContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "validateOnly", skip_serializing_if = "Option::is_none")]
    pub validate_only: Option<bool>,
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
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GoogleDriveContent {
    #[serde(rename = "resourceName")]
    pub resource_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct VideoContent {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ListRecentlyViewedResponse {
    #[serde(default)]
    pub notebooks: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AudioOverviewRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<serde_json::Value>,
}

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
        let json = r#"{"textContent":{"text":"sample text"}}"#;
        let content: UserContent = serde_json::from_str(json).unwrap();
        match content {
            UserContent::Text { text_content } => {
                assert_eq!(text_content.text, "sample text");
            }
            _ => panic!("expected Text variant"),
        }
    }

    #[test]
    fn user_content_untagged_google_drive() {
        let json = r#"{"googleDriveContent":{"resourceName":"drive://file/123"}}"#;
        let content: UserContent = serde_json::from_str(json).unwrap();
        match content {
            UserContent::GoogleDrive {
                google_drive_content,
            } => {
                assert_eq!(google_drive_content.resource_name, "drive://file/123");
            }
            _ => panic!("expected GoogleDrive variant"),
        }
    }

    #[test]
    fn user_content_untagged_video() {
        let json = r#"{"videoContent":{"url":"https://youtube.com/watch?v=123"}}"#;
        let content: UserContent = serde_json::from_str(json).unwrap();
        match content {
            UserContent::Video { video_content } => {
                assert_eq!(video_content.url, "https://youtube.com/watch?v=123");
            }
            _ => panic!("expected Video variant"),
        }
    }

    #[test]
    fn batch_create_sources_request_skips_validate_only_when_none() {
        let request = BatchCreateSourcesRequest {
            user_contents: vec![],
            client_token: None,
            validate_only: None,
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(!json.contains("validateOnly"));
    }

    #[test]
    fn batch_create_sources_request_includes_validate_only_when_some() {
        let request = BatchCreateSourcesRequest {
            user_contents: vec![],
            client_token: None,
            validate_only: Some(true),
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("validateOnly"));
        assert!(json.contains("true"));
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
