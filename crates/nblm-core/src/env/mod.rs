use crate::error::{Error, Result};

/// API profile types supported by the SDK.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApiProfile {
    Enterprise,
}

impl ApiProfile {
    pub fn as_str(&self) -> &'static str {
        match self {
            ApiProfile::Enterprise => "enterprise",
        }
    }

    pub fn parse(input: &str) -> Result<Self> {
        match input.trim().to_ascii_lowercase().as_str() {
            "enterprise" => Ok(ApiProfile::Enterprise),
            other => Err(Error::Endpoint(format!("unsupported API profile: {other}"))),
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
        let endpoint = normalize_endpoint_location(endpoint_location.into())?;
        let project_number = project_number.into();
        let location = location.into();
        let base_url = format!("https://{}discoveryengine.googleapis.com/v1alpha", endpoint);
        let parent_path = format!("projects/{}/locations/{}", project_number, location);
        Ok(Self {
            profile: ApiProfile::Enterprise,
            base_url,
            parent_path,
        })
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

    pub fn for_profile(
        profile: ApiProfile,
        project_number: impl Into<String>,
        location: impl Into<String>,
        endpoint_location: impl Into<String>,
    ) -> Result<Self> {
        match profile {
            ApiProfile::Enterprise => Self::enterprise(project_number, location, endpoint_location),
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
    fn api_profile_parse_accepts_enterprise() {
        let profile = ApiProfile::parse("enterprise").unwrap();
        assert_eq!(profile, ApiProfile::Enterprise);
        assert_eq!(profile.as_str(), "enterprise");
    }
}
