use crate::models::enterprise::{
    audio as domain_audio, notebook as domain_notebook, source as domain_source,
};

use super::models::{
    notebook as wire_notebook,
    requests::{audio as wire_audio_req, notebook as wire_notebook_req, source as wire_source_req},
    responses::{audio as wire_audio_resp, list as wire_list_resp, source as wire_source_resp},
    source as wire_source,
};

// ---------- Audio ----------

impl From<wire_audio_resp::AudioOverviewResponse> for domain_audio::AudioOverviewResponse {
    fn from(value: wire_audio_resp::AudioOverviewResponse) -> Self {
        Self {
            audio_overview_id: value.audio_overview_id,
            name: value.name,
            status: value.status,
            generation_options: value.generation_options,
            extra: value.extra,
        }
    }
}

impl From<domain_audio::AudioOverviewRequest> for wire_audio_req::AudioOverviewRequest {
    fn from(_value: domain_audio::AudioOverviewRequest) -> Self {
        wire_audio_req::AudioOverviewRequest::default()
    }
}

// ---------- Notebook ----------

impl From<wire_notebook::Notebook> for domain_notebook::Notebook {
    fn from(value: wire_notebook::Notebook) -> Self {
        Self {
            name: value.name,
            title: value.title,
            notebook_id: value.notebook_id,
            emoji: value.emoji,
            sources: value.sources.into_iter().map(Into::into).collect(),
            metadata: value.metadata.map(Into::into),
            extra: value.extra,
        }
    }
}

impl From<wire_notebook::NotebookMetadata> for domain_notebook::NotebookMetadata {
    fn from(value: wire_notebook::NotebookMetadata) -> Self {
        Self {
            create_time: value.create_time,
            is_shareable: value.is_shareable,
            is_shared: value.is_shared,
            last_viewed: value.last_viewed,
            extra: value.extra,
        }
    }
}

impl From<wire_notebook::NotebookRef> for domain_notebook::NotebookRef {
    fn from(value: wire_notebook::NotebookRef) -> Self {
        Self {
            notebook_id: value.notebook_id,
            name: value.name,
        }
    }
}

impl From<wire_list_resp::ListRecentlyViewedResponse>
    for domain_notebook::ListRecentlyViewedResponse
{
    fn from(value: wire_list_resp::ListRecentlyViewedResponse) -> Self {
        Self {
            notebooks: value
                .notebooks
                .into_iter()
                .map(domain_notebook::Notebook::from)
                .collect(),
        }
    }
}

impl From<domain_notebook::BatchDeleteNotebooksRequest>
    for wire_notebook_req::BatchDeleteNotebooksRequest
{
    fn from(value: domain_notebook::BatchDeleteNotebooksRequest) -> Self {
        Self { names: value.names }
    }
}

impl From<wire_notebook_req::BatchDeleteNotebooksResponse>
    for domain_notebook::BatchDeleteNotebooksResponse
{
    fn from(value: wire_notebook_req::BatchDeleteNotebooksResponse) -> Self {
        Self { extra: value.extra }
    }
}

// ---------- Source ----------

impl From<wire_source::NotebookSource> for domain_source::NotebookSource {
    fn from(value: wire_source::NotebookSource) -> Self {
        Self {
            metadata: value
                .metadata
                .map(domain_source::NotebookSourceMetadata::from),
            name: value.name,
            settings: value
                .settings
                .map(domain_source::NotebookSourceSettings::from),
            source_id: value.source_id.map(domain_source::NotebookSourceId::from),
            title: value.title,
            extra: value.extra,
        }
    }
}

impl From<wire_source::NotebookSourceMetadata> for domain_source::NotebookSourceMetadata {
    fn from(value: wire_source::NotebookSourceMetadata) -> Self {
        Self {
            source_added_timestamp: value.source_added_timestamp,
            word_count: value.word_count,
            youtube_metadata: value
                .youtube_metadata
                .map(domain_source::NotebookSourceYoutubeMetadata::from),
            extra: value.extra,
        }
    }
}

impl From<wire_source::NotebookSourceYoutubeMetadata>
    for domain_source::NotebookSourceYoutubeMetadata
{
    fn from(value: wire_source::NotebookSourceYoutubeMetadata) -> Self {
        Self {
            channel_name: value.channel_name,
            video_id: value.video_id,
            extra: value.extra,
        }
    }
}

impl From<domain_source::NotebookSourceYoutubeMetadata>
    for wire_source::NotebookSourceYoutubeMetadata
{
    fn from(value: domain_source::NotebookSourceYoutubeMetadata) -> Self {
        Self {
            channel_name: value.channel_name,
            video_id: value.video_id,
            extra: value.extra,
        }
    }
}

impl From<wire_source::NotebookSourceSettings> for domain_source::NotebookSourceSettings {
    fn from(value: wire_source::NotebookSourceSettings) -> Self {
        Self {
            status: value.status,
            extra: value.extra,
        }
    }
}

impl From<wire_source::NotebookSourceId> for domain_source::NotebookSourceId {
    fn from(value: wire_source::NotebookSourceId) -> Self {
        Self {
            id: value.id,
            extra: value.extra,
        }
    }
}

impl From<wire_source::UserContent> for domain_source::UserContent {
    fn from(value: wire_source::UserContent) -> Self {
        match value {
            wire_source::UserContent::Web { web_content } => domain_source::UserContent::Web {
                web_content: web_content.into(),
            },
            wire_source::UserContent::Text { text_content } => domain_source::UserContent::Text {
                text_content: text_content.into(),
            },
            wire_source::UserContent::GoogleDrive {
                google_drive_content,
            } => domain_source::UserContent::GoogleDrive {
                google_drive_content: google_drive_content.into(),
            },
            wire_source::UserContent::Video { video_content } => {
                domain_source::UserContent::Video {
                    video_content: video_content.into(),
                }
            }
        }
    }
}

impl From<wire_source::WebContent> for domain_source::WebContent {
    fn from(value: wire_source::WebContent) -> Self {
        Self {
            url: value.url,
            source_name: value.source_name,
        }
    }
}

impl From<wire_source::TextContent> for domain_source::TextContent {
    fn from(value: wire_source::TextContent) -> Self {
        Self {
            content: value.content,
            source_name: value.source_name,
        }
    }
}

impl From<wire_source::GoogleDriveContent> for domain_source::GoogleDriveContent {
    fn from(value: wire_source::GoogleDriveContent) -> Self {
        Self {
            document_id: value.document_id,
            mime_type: value.mime_type,
            source_name: value.source_name,
        }
    }
}

impl From<wire_source::VideoContent> for domain_source::VideoContent {
    fn from(value: wire_source::VideoContent) -> Self {
        Self { url: value.url }
    }
}

impl From<domain_source::UserContent> for wire_source::UserContent {
    fn from(value: domain_source::UserContent) -> Self {
        match value {
            domain_source::UserContent::Web { web_content } => wire_source::UserContent::Web {
                web_content: web_content.into(),
            },
            domain_source::UserContent::Text { text_content } => wire_source::UserContent::Text {
                text_content: text_content.into(),
            },
            domain_source::UserContent::GoogleDrive {
                google_drive_content,
            } => wire_source::UserContent::GoogleDrive {
                google_drive_content: google_drive_content.into(),
            },
            domain_source::UserContent::Video { video_content } => {
                wire_source::UserContent::Video {
                    video_content: video_content.into(),
                }
            }
        }
    }
}

impl From<domain_source::WebContent> for wire_source::WebContent {
    fn from(value: domain_source::WebContent) -> Self {
        Self {
            url: value.url,
            source_name: value.source_name,
        }
    }
}

impl From<domain_source::TextContent> for wire_source::TextContent {
    fn from(value: domain_source::TextContent) -> Self {
        Self {
            content: value.content,
            source_name: value.source_name,
        }
    }
}

impl From<domain_source::GoogleDriveContent> for wire_source::GoogleDriveContent {
    fn from(value: domain_source::GoogleDriveContent) -> Self {
        Self {
            document_id: value.document_id,
            mime_type: value.mime_type,
            source_name: value.source_name,
        }
    }
}

impl From<domain_source::VideoContent> for wire_source::VideoContent {
    fn from(value: domain_source::VideoContent) -> Self {
        Self { url: value.url }
    }
}

impl From<wire_source_resp::BatchCreateSourcesResponse>
    for domain_source::BatchCreateSourcesResponse
{
    fn from(value: wire_source_resp::BatchCreateSourcesResponse) -> Self {
        Self {
            sources: value
                .sources
                .into_iter()
                .map(domain_source::NotebookSource::from)
                .collect(),
            error_count: value.error_count,
        }
    }
}

impl From<domain_source::BatchCreateSourcesRequest> for wire_source_req::BatchCreateSourcesRequest {
    fn from(value: domain_source::BatchCreateSourcesRequest) -> Self {
        Self {
            user_contents: value
                .user_contents
                .into_iter()
                .map(wire_source::UserContent::from)
                .collect(),
        }
    }
}

impl From<domain_source::BatchDeleteSourcesRequest> for wire_source_req::BatchDeleteSourcesRequest {
    fn from(value: domain_source::BatchDeleteSourcesRequest) -> Self {
        Self { names: value.names }
    }
}

impl From<wire_source_req::BatchDeleteSourcesResponse>
    for domain_source::BatchDeleteSourcesResponse
{
    fn from(value: wire_source_req::BatchDeleteSourcesResponse) -> Self {
        Self { extra: value.extra }
    }
}

impl From<wire_source_resp::UploadSourceFileResponse> for domain_source::UploadSourceFileResponse {
    fn from(value: wire_source_resp::UploadSourceFileResponse) -> Self {
        Self {
            source_id: value.source_id.map(domain_source::NotebookSourceId::from),
            extra: value.extra,
        }
    }
}

impl From<domain_source::UploadSourceFileResponse> for wire_source_resp::UploadSourceFileResponse {
    fn from(value: domain_source::UploadSourceFileResponse) -> Self {
        Self {
            source_id: value.source_id.map(wire_source::NotebookSourceId::from),
            extra: value.extra,
        }
    }
}

impl From<domain_source::NotebookSourceMetadata> for wire_source::NotebookSourceMetadata {
    fn from(value: domain_source::NotebookSourceMetadata) -> Self {
        Self {
            source_added_timestamp: value.source_added_timestamp,
            word_count: value.word_count,
            youtube_metadata: value.youtube_metadata.map(Into::into),
            extra: value.extra,
        }
    }
}

impl From<domain_source::NotebookSourceSettings> for wire_source::NotebookSourceSettings {
    fn from(value: domain_source::NotebookSourceSettings) -> Self {
        Self {
            status: value.status,
            extra: value.extra,
        }
    }
}

impl From<domain_source::NotebookSourceId> for wire_source::NotebookSourceId {
    fn from(value: domain_source::NotebookSourceId) -> Self {
        Self {
            id: value.id,
            extra: value.extra,
        }
    }
}

impl From<domain_source::NotebookSource> for wire_source::NotebookSource {
    fn from(value: domain_source::NotebookSource) -> Self {
        Self {
            metadata: value
                .metadata
                .map(wire_source::NotebookSourceMetadata::from),
            name: value.name,
            settings: value
                .settings
                .map(wire_source::NotebookSourceSettings::from),
            source_id: value.source_id.map(wire_source::NotebookSourceId::from),
            title: value.title,
            extra: value.extra,
        }
    }
}

// ---------- test ----------
#[cfg(test)]
mod tests {
    use super::*;

    // UserContent round-trip conversion tests
    #[test]
    fn user_content_web_round_trip() {
        let domain = domain_source::UserContent::Web {
            web_content: domain_source::WebContent {
                url: "https://example.com".to_string(),
                source_name: Some("Example".to_string()),
            },
        };

        let wire: wire_source::UserContent = domain.clone().into();
        let back: domain_source::UserContent = wire.into();

        match (domain, back) {
            (
                domain_source::UserContent::Web { web_content: d },
                domain_source::UserContent::Web { web_content: b },
            ) => {
                assert_eq!(d.url, b.url);
                assert_eq!(d.source_name, b.source_name);
            }
            _ => panic!("UserContent variant mismatch"),
        }
    }

    #[test]
    fn user_content_text_round_trip() {
        let domain = domain_source::UserContent::Text {
            text_content: domain_source::TextContent {
                content: "Test content".to_string(),
                source_name: Some("Test".to_string()),
            },
        };

        let wire: wire_source::UserContent = domain.clone().into();
        let back: domain_source::UserContent = wire.into();

        match (domain, back) {
            (
                domain_source::UserContent::Text { text_content: d },
                domain_source::UserContent::Text { text_content: b },
            ) => {
                assert_eq!(d.content, b.content);
                assert_eq!(d.source_name, b.source_name);
            }
            _ => panic!("UserContent variant mismatch"),
        }
    }

    #[test]
    fn user_content_google_drive_round_trip() {
        let domain = domain_source::UserContent::GoogleDrive {
            google_drive_content: domain_source::GoogleDriveContent {
                document_id: "doc123".to_string(),
                mime_type: "application/pdf".to_string(),
                source_name: Some("My Doc".to_string()),
            },
        };

        let wire: wire_source::UserContent = domain.clone().into();
        let back: domain_source::UserContent = wire.into();

        match (domain, back) {
            (
                domain_source::UserContent::GoogleDrive {
                    google_drive_content: d,
                },
                domain_source::UserContent::GoogleDrive {
                    google_drive_content: b,
                },
            ) => {
                assert_eq!(d.document_id, b.document_id);
                assert_eq!(d.mime_type, b.mime_type);
                assert_eq!(d.source_name, b.source_name);
            }
            _ => panic!("UserContent variant mismatch"),
        }
    }

    #[test]
    fn user_content_video_round_trip() {
        let domain = domain_source::UserContent::Video {
            video_content: domain_source::VideoContent {
                url: "https://youtube.com/watch?v=abc".to_string(),
            },
        };

        let wire: wire_source::UserContent = domain.clone().into();
        let back: domain_source::UserContent = wire.into();

        match (domain, back) {
            (
                domain_source::UserContent::Video { video_content: d },
                domain_source::UserContent::Video { video_content: b },
            ) => {
                assert_eq!(d.url, b.url);
            }
            _ => panic!("UserContent variant mismatch"),
        }
    }

    // NotebookSource nested structure conversion test
    #[test]
    fn notebook_source_with_all_fields() {
        use std::collections::HashMap;

        let domain = domain_source::NotebookSource {
            metadata: Some(domain_source::NotebookSourceMetadata {
                source_added_timestamp: Some("2024-01-01T00:00:00Z".to_string()),
                word_count: Some(1000),
                youtube_metadata: Some(domain_source::NotebookSourceYoutubeMetadata {
                    channel_name: Some("Test Channel".to_string()),
                    video_id: Some("abc123".to_string()),
                    extra: HashMap::new(),
                }),
                extra: HashMap::new(),
            }),
            name: "test-source".to_string(),
            settings: Some(domain_source::NotebookSourceSettings {
                status: Some("ACTIVE".to_string()),
                extra: HashMap::new(),
            }),
            source_id: Some(domain_source::NotebookSourceId {
                id: Some("source-123".to_string()),
                extra: HashMap::new(),
            }),
            title: Some("Test Source".to_string()),
            extra: HashMap::new(),
        };

        let wire: wire_source::NotebookSource = domain.clone().into();
        let back: domain_source::NotebookSource = wire.into();

        assert_eq!(domain.name, back.name);
        assert_eq!(domain.title, back.title);
        assert_eq!(
            domain.metadata.as_ref().unwrap().word_count,
            back.metadata.as_ref().unwrap().word_count
        );
        assert_eq!(
            domain.settings.as_ref().unwrap().status,
            back.settings.as_ref().unwrap().status
        );
        assert_eq!(
            domain.source_id.as_ref().unwrap().id,
            back.source_id.as_ref().unwrap().id
        );
    }

    #[test]
    fn notebook_source_with_none_fields() {
        use std::collections::HashMap;

        let domain = domain_source::NotebookSource {
            metadata: None,
            name: "minimal-source".to_string(),
            settings: None,
            source_id: None,
            title: None,
            extra: HashMap::new(),
        };

        let wire: wire_source::NotebookSource = domain.clone().into();
        let back: domain_source::NotebookSource = wire.into();

        assert_eq!(domain.name, back.name);
        assert_eq!(domain.title, back.title);
        assert!(back.metadata.is_none());
        assert!(back.settings.is_none());
        assert!(back.source_id.is_none());
    }
}
