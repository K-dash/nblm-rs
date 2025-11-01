use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Domain-level request for creating an audio overview.
///
/// As of today the API expects an empty object, but fields are kept optional for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AudioOverviewRequest {}

/// Domain-level response for audio overview operations.
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
    pub generation_options: Option<Value>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
