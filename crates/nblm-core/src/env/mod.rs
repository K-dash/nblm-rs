use crate::error::{Error, Result};

/// API profile types supported by the SDK.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApiProfile {
    Enterprise,
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
}
