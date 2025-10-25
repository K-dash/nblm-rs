use serde::{Deserialize, Serialize};

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
