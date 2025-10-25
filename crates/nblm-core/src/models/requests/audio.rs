use serde::{Deserialize, Serialize};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn audio_overview_request_serializes_to_empty_object() {
        let request = AudioOverviewRequest::default();
        let json = serde_json::to_string(&request).unwrap();
        assert_eq!(json, "{}");
    }

    #[test]
    fn audio_overview_request_deserializes_from_empty_object() {
        let json = r#"{}"#;
        let request: AudioOverviewRequest = serde_json::from_str(json).unwrap();
        let _ = request; // Verify it deserializes successfully
    }
}
