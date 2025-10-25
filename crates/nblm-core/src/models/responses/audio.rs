use std::collections::HashMap;

use serde::{Deserialize, Serialize};

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
    fn audio_overview_response_deserializes_correctly() {
        let json = r#"{
            "name": "notebooks/123/audioOverviews/456",
            "state": "COMPLETED"
        }"#;
        let response: AudioOverviewResponse = serde_json::from_str(json).unwrap();
        assert_eq!(
            response.name.as_ref().unwrap(),
            "notebooks/123/audioOverviews/456"
        );
        assert_eq!(response.state.as_ref().unwrap(), "COMPLETED");
    }

    #[test]
    fn audio_overview_response_skips_none_fields_on_serialize() {
        let response = AudioOverviewResponse {
            name: Some("notebooks/123/audioOverviews/456".to_string()),
            state: None,
            extra: HashMap::new(),
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("name"));
        assert!(!json.contains("state"));
    }

    #[test]
    fn audio_overview_response_with_extra_fields() {
        let json = r#"{
            "name": "notebooks/123/audioOverviews/456",
            "state": "PROCESSING",
            "customField": "value"
        }"#;
        let response: AudioOverviewResponse = serde_json::from_str(json).unwrap();
        assert_eq!(
            response.name.as_ref().unwrap(),
            "notebooks/123/audioOverviews/456"
        );
        assert_eq!(response.state.as_ref().unwrap(), "PROCESSING");
        assert_eq!(
            response.extra.get("customField").unwrap().as_str().unwrap(),
            "value"
        );
    }
}
