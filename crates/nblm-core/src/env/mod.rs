use crate::error::{Error, Result};

const PROFILE_NAME_ENTERPRISE: &str = "enterprise";
const PROFILE_NAME_PERSONAL: &str = "personal";
const PROFILE_NAME_WORKSPACE: &str = "workspace";

/// API profile types supported by the SDK.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ApiProfile {
    Enterprise,
    Personal,
    Workspace,
}

impl ApiProfile {
    pub fn as_str(&self) -> &'static str {
        match self {
            ApiProfile::Enterprise => PROFILE_NAME_ENTERPRISE,
            ApiProfile::Personal => PROFILE_NAME_PERSONAL,
            ApiProfile::Workspace => PROFILE_NAME_WORKSPACE,
        }
    }

    pub fn parse(input: &str) -> Result<Self> {
        match input.trim().to_ascii_lowercase().as_str() {
            PROFILE_NAME_ENTERPRISE => Ok(ApiProfile::Enterprise),
            PROFILE_NAME_PERSONAL => Ok(ApiProfile::Personal),
            PROFILE_NAME_WORKSPACE => Ok(ApiProfile::Workspace),
            other => Err(Error::Endpoint(format!("unsupported API profile: {other}"))),
        }
    }

    pub fn requires_experimental_flag(&self) -> bool {
        matches!(self, ApiProfile::Personal | ApiProfile::Workspace)
    }
}

pub const PROFILE_EXPERIMENT_FLAG: &str = "NBLM_PROFILE_EXPERIMENT";

#[derive(Debug, Clone)]
pub enum ProfileParams {
    Enterprise {
        project_number: String,
        location: String,
        endpoint_location: String,
    },
    Personal {
        user_email: Option<String>,
    },
    Workspace {
        customer_id: Option<String>,
        admin_email: Option<String>,
    },
}

impl ProfileParams {
    pub fn enterprise(
        project_number: impl Into<String>,
        location: impl Into<String>,
        endpoint_location: impl Into<String>,
    ) -> Self {
        Self::Enterprise {
            project_number: project_number.into(),
            location: location.into(),
            endpoint_location: endpoint_location.into(),
        }
    }

    pub fn personal<T: Into<String>>(user_email: Option<T>) -> Self {
        Self::Personal {
            user_email: user_email.map(|email| email.into()),
        }
    }

    pub fn workspace<T: Into<String>, U: Into<String>>(
        customer_id: Option<T>,
        admin_email: Option<U>,
    ) -> Self {
        Self::Workspace {
            customer_id: customer_id.map(|value| value.into()),
            admin_email: admin_email.map(|value| value.into()),
        }
    }

    pub fn expected_profile(&self) -> ApiProfile {
        match self {
            ProfileParams::Enterprise { .. } => ApiProfile::Enterprise,
            ProfileParams::Personal { .. } => ApiProfile::Personal,
            ProfileParams::Workspace { .. } => ApiProfile::Workspace,
        }
    }
}

/// Runtime configuration describing the API environment.
#[derive(Debug, Clone)]
pub struct EnvironmentConfig {
    profile: ApiProfile,
    base_url: String,
    parent_path: String,
}

impl EnvironmentConfig {
    /// Construct the environment config for the Enterprise SKU.
    pub fn enterprise(
        project_number: impl Into<String>,
        location: impl Into<String>,
        endpoint_location: impl Into<String>,
    ) -> Result<Self> {
        Self::from_profile(
            ApiProfile::Enterprise,
            ProfileParams::enterprise(project_number, location, endpoint_location),
        )
    }

    pub fn profile(&self) -> ApiProfile {
        self.profile
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub fn parent_path(&self) -> &str {
        &self.parent_path
    }

    /// Return a copy with a different base URL (useful for tests or overrides).
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    pub fn from_profile(profile: ApiProfile, params: ProfileParams) -> Result<Self> {
        let params_profile = params.expected_profile();
        if profile != params_profile {
            return Err(profile_params_mismatch_error(profile, params_profile));
        }

        match profile {
            ApiProfile::Enterprise => match params {
                ProfileParams::Enterprise {
                    project_number,
                    location,
                    endpoint_location,
                } => {
                    let endpoint = normalize_endpoint_location(endpoint_location)?;
                    let base_url =
                        format!("https://{}discoveryengine.googleapis.com/v1alpha", endpoint);
                    let parent_path = format!("projects/{}/locations/{}", project_number, location);
                    Ok(Self {
                        profile: ApiProfile::Enterprise,
                        base_url,
                        parent_path,
                    })
                }
                _ => unreachable!("profile/params mismatch should already be validated"),
            },
            ApiProfile::Personal | ApiProfile::Workspace => Err(unsupported_profile_error(profile)),
        }
    }
}

/// Normalize endpoint location strings to the canonical discovery engine prefix.
pub fn normalize_endpoint_location(input: String) -> Result<String> {
    let trimmed = input.trim().trim_end_matches('-').to_lowercase();
    let normalized = match trimmed.as_str() {
        "us" => "us-",
        "eu" => "eu-",
        "global" => "global-",
        other => {
            return Err(Error::Endpoint(format!(
                "unsupported endpoint location: {other}"
            )))
        }
    };
    Ok(normalized.to_string())
}

fn unsupported_profile_error(profile: ApiProfile) -> Error {
    Error::Endpoint(format!(
        "API profile '{}' is not available yet",
        profile.as_str()
    ))
}

fn profile_params_mismatch_error(expected: ApiProfile, provided: ApiProfile) -> Error {
    Error::Endpoint(format!(
        "profile '{}' expects parameters for '{}', but '{}' parameters were provided",
        expected.as_str(),
        expected.as_str(),
        provided.as_str()
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enterprise_constructor_builds_expected_urls() {
        let env = EnvironmentConfig::enterprise("123", "global", "us").unwrap();
        assert_eq!(env.profile(), ApiProfile::Enterprise);
        assert_eq!(
            env.base_url(),
            "https://us-discoveryengine.googleapis.com/v1alpha"
        );
        assert_eq!(env.parent_path(), "projects/123/locations/global");
    }

    #[test]
    fn normalize_endpoint_location_variants() {
        assert_eq!(
            normalize_endpoint_location("us".into()).unwrap(),
            "us-".to_string()
        );
        assert_eq!(
            normalize_endpoint_location("eu-".into()).unwrap(),
            "eu-".to_string()
        );
        assert_eq!(
            normalize_endpoint_location(" global ".into()).unwrap(),
            "global-".to_string()
        );
    }

    #[test]
    fn normalize_endpoint_location_invalid() {
        let err = normalize_endpoint_location("asia".into()).unwrap_err();
        assert!(format!("{err}").contains("unsupported endpoint location"));
    }

    #[test]
    fn with_base_url_overrides_base_url() {
        let env = EnvironmentConfig::enterprise("123", "global", "us")
            .unwrap()
            .with_base_url("http://localhost:8080/v1alpha");
        assert_eq!(env.base_url(), "http://localhost:8080/v1alpha");
        assert_eq!(env.parent_path(), "projects/123/locations/global");
    }

    #[test]
    fn api_profile_parse_accepts_all_known_variants() {
        let enterprise = ApiProfile::parse("enterprise").unwrap();
        assert_eq!(enterprise, ApiProfile::Enterprise);
        assert_eq!(enterprise.as_str(), "enterprise");

        let personal = ApiProfile::parse("personal").unwrap();
        assert_eq!(personal, ApiProfile::Personal);
        assert_eq!(personal.as_str(), "personal");

        let workspace = ApiProfile::parse("workspace").unwrap();
        assert_eq!(workspace, ApiProfile::Workspace);
        assert_eq!(workspace.as_str(), "workspace");
    }

    #[test]
    fn from_profile_rejects_mismatched_params() {
        let err = EnvironmentConfig::from_profile(
            ApiProfile::Enterprise,
            ProfileParams::personal::<String>(None),
        )
        .unwrap_err();
        let msg = format!("{err}");
        assert!(msg.contains("expects parameters for 'enterprise'"));
    }

    #[test]
    fn personal_profile_not_available_yet() {
        let err = EnvironmentConfig::from_profile(
            ApiProfile::Personal,
            ProfileParams::personal(Some("user@example.com")),
        )
        .unwrap_err();
        let msg = format!("{err}");
        assert!(msg.contains("not available yet"));
    }

    #[test]
    fn workspace_profile_not_available_yet() {
        let err = EnvironmentConfig::from_profile(
            ApiProfile::Workspace,
            ProfileParams::workspace::<String, String>(None, None),
        )
        .unwrap_err();
        let msg = format!("{err}");
        assert!(msg.contains("not available yet"));
    }

    #[test]
    fn profile_params_expected_profile_returns_correct_variant() {
        let enterprise = ProfileParams::enterprise("123", "global", "us");
        assert_eq!(enterprise.expected_profile(), ApiProfile::Enterprise);

        let personal = ProfileParams::personal(Some("user@example.com"));
        assert_eq!(personal.expected_profile(), ApiProfile::Personal);

        let workspace = ProfileParams::workspace(Some("customer123"), Some("admin@example.com"));
        assert_eq!(workspace.expected_profile(), ApiProfile::Workspace);
    }

    #[test]
    fn from_profile_succeeds_with_matching_enterprise_params() {
        let env = EnvironmentConfig::from_profile(
            ApiProfile::Enterprise,
            ProfileParams::enterprise("456", "us-central1", "us"),
        )
        .unwrap();
        assert_eq!(env.profile(), ApiProfile::Enterprise);
        assert_eq!(
            env.base_url(),
            "https://us-discoveryengine.googleapis.com/v1alpha"
        );
        assert_eq!(env.parent_path(), "projects/456/locations/us-central1");
    }

    #[test]
    fn from_profile_rejects_personal_profile_with_enterprise_params() {
        let err = EnvironmentConfig::from_profile(
            ApiProfile::Personal,
            ProfileParams::enterprise("123", "global", "us"),
        )
        .unwrap_err();
        let msg = format!("{err}");
        assert!(msg.contains("expects parameters for 'personal'"));
        assert!(msg.contains("'enterprise' parameters were provided"));
    }

    #[test]
    fn from_profile_rejects_workspace_profile_with_enterprise_params() {
        let err = EnvironmentConfig::from_profile(
            ApiProfile::Workspace,
            ProfileParams::enterprise("123", "global", "us"),
        )
        .unwrap_err();
        let msg = format!("{err}");
        assert!(msg.contains("expects parameters for 'workspace'"));
        assert!(msg.contains("'enterprise' parameters were provided"));
    }

    #[test]
    fn profile_params_personal_builder_handles_optional_email() {
        let with_email = ProfileParams::personal(Some("user@example.com"));
        assert_eq!(with_email.expected_profile(), ApiProfile::Personal);

        let without_email = ProfileParams::personal::<String>(None);
        assert_eq!(without_email.expected_profile(), ApiProfile::Personal);
    }

    #[test]
    fn profile_params_workspace_builder_handles_optional_fields() {
        let with_both = ProfileParams::workspace(Some("customer123"), Some("admin@example.com"));
        assert_eq!(with_both.expected_profile(), ApiProfile::Workspace);

        let with_customer_only = ProfileParams::workspace(Some("customer123"), None::<String>);
        assert_eq!(with_customer_only.expected_profile(), ApiProfile::Workspace);

        let with_admin_only = ProfileParams::workspace(None::<String>, Some("admin@example.com"));
        assert_eq!(with_admin_only.expected_profile(), ApiProfile::Workspace);

        let with_neither = ProfileParams::workspace::<String, String>(None, None);
        assert_eq!(with_neither.expected_profile(), ApiProfile::Workspace);
    }

    #[test]
    fn requires_experimental_flag_returns_correct_values() {
        assert!(!ApiProfile::Enterprise.requires_experimental_flag());
        assert!(ApiProfile::Personal.requires_experimental_flag());
        assert!(ApiProfile::Workspace.requires_experimental_flag());
    }
}
