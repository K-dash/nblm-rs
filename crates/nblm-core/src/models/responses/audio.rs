use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Wrapper for the API response that contains audioOverview
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AudioOverviewApiResponse {
    pub audio_overview: AudioOverviewResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AudioOverviewResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_overview_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_options: Option<serde_json::Value>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn audio_overview_response_deserializes_correctly() {
        let json = r#"{
            "audioOverviewId": "c825b865-ad95-42d7-8abb-fb4ed46f4cc9",
            "name": "projects/224840249322/locations/global/notebooks/123/audioOverviews/456",
            "status": "AUDIO_OVERVIEW_STATUS_IN_PROGRESS"
        }"#;
        let response: AudioOverviewResponse = serde_json::from_str(json).unwrap();
        assert_eq!(
            response.audio_overview_id.as_ref().unwrap(),
            "c825b865-ad95-42d7-8abb-fb4ed46f4cc9"
        );
        assert_eq!(
            response.name.as_ref().unwrap(),
            "projects/224840249322/locations/global/notebooks/123/audioOverviews/456"
        );
        assert_eq!(
            response.status.as_ref().unwrap(),
            "AUDIO_OVERVIEW_STATUS_IN_PROGRESS"
        );
    }

    #[test]
    fn audio_overview_response_skips_none_fields_on_serialize() {
        let response = AudioOverviewResponse {
            audio_overview_id: Some("test-id".to_string()),
            name: Some("notebooks/123/audioOverviews/456".to_string()),
            status: None,
            generation_options: None,
            extra: HashMap::new(),
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("audioOverviewId"));
        assert!(json.contains("name"));
        assert!(!json.contains("status"));
        assert!(!json.contains("generationOptions"));
    }

    #[test]
    fn audio_overview_response_with_extra_fields() {
        let json = r#"{
            "audioOverviewId": "test-id",
            "name": "notebooks/123/audioOverviews/456",
            "status": "AUDIO_OVERVIEW_STATUS_IN_PROGRESS",
            "generationOptions": {},
            "customField": "value"
        }"#;
        let response: AudioOverviewResponse = serde_json::from_str(json).unwrap();
        assert_eq!(
            response.name.as_ref().unwrap(),
            "notebooks/123/audioOverviews/456"
        );
        assert_eq!(
            response.status.as_ref().unwrap(),
            "AUDIO_OVERVIEW_STATUS_IN_PROGRESS"
        );
        assert_eq!(
            response.extra.get("customField").unwrap().as_str().unwrap(),
            "value"
        );
    }

    #[test]
    fn audio_overview_api_response_deserializes_correctly() {
        let json = r#"{
            "audioOverview": {
                "audioOverviewId": "c825b865-ad95-42d7-8abb-fb4ed46f4cc9",
                "name": "projects/224840249322/locations/global/notebooks/123/audioOverviews/456",
                "status": "AUDIO_OVERVIEW_STATUS_IN_PROGRESS",
                "generationOptions": {}
            }
        }"#;
        let api_response: AudioOverviewApiResponse = serde_json::from_str(json).unwrap();
        let response = api_response.audio_overview;
        assert_eq!(
            response.audio_overview_id.as_ref().unwrap(),
            "c825b865-ad95-42d7-8abb-fb4ed46f4cc9"
        );
        assert_eq!(
            response.name.as_ref().unwrap(),
            "projects/224840249322/locations/global/notebooks/123/audioOverviews/456"
        );
        assert_eq!(
            response.status.as_ref().unwrap(),
            "AUDIO_OVERVIEW_STATUS_IN_PROGRESS"
        );
    }
}
