use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AccountRole {
    pub email: String,
    pub role: ProjectRole,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granted: Option<i32>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
