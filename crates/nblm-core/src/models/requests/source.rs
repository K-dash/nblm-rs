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
