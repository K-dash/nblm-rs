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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn project_role_serializes_correctly() {
        let owner = ProjectRole::ProjectRoleOwner;
        let json = serde_json::to_string(&owner).unwrap();
        assert_eq!(json, r#""PROJECT_ROLE_OWNER""#);

        let writer = ProjectRole::ProjectRoleWriter;
        let json = serde_json::to_string(&writer).unwrap();
        assert_eq!(json, r#""PROJECT_ROLE_WRITER""#);

        let reader = ProjectRole::ProjectRoleReader;
        let json = serde_json::to_string(&reader).unwrap();
        assert_eq!(json, r#""PROJECT_ROLE_READER""#);
    }

    #[test]
    fn project_role_deserializes_correctly() {
        let json = r#""PROJECT_ROLE_OWNER""#;
        let role: ProjectRole = serde_json::from_str(json).unwrap();
        assert!(matches!(role, ProjectRole::ProjectRoleOwner));

        let json = r#""PROJECT_ROLE_READER""#;
        let role: ProjectRole = serde_json::from_str(json).unwrap();
        assert!(matches!(role, ProjectRole::ProjectRoleReader));
    }

    #[test]
    fn share_request_serializes_correctly() {
        let request = ShareRequest {
            account_and_roles: vec![
                AccountRole {
                    email: "user1@example.com".to_string(),
                    role: ProjectRole::ProjectRoleOwner,
                },
                AccountRole {
                    email: "user2@example.com".to_string(),
                    role: ProjectRole::ProjectRoleReader,
                },
            ],
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("accountAndRoles"));
        assert!(json.contains("user1@example.com"));
        assert!(json.contains("PROJECT_ROLE_OWNER"));
        assert!(json.contains("user2@example.com"));
        assert!(json.contains("PROJECT_ROLE_READER"));
    }

    #[test]
    fn account_role_default_is_reader() {
        let default_role = ProjectRole::default();
        assert!(matches!(default_role, ProjectRole::ProjectRoleReader));
    }
}
