use reqwest::Url;

use super::UrlBuilder;
use crate::error::{Error, Result};

/// Enterprise-specific URL builder.
#[derive(Clone)]
pub(crate) struct EnterpriseUrlBuilder {
    base: String,
    parent: String,
}

impl EnterpriseUrlBuilder {
    pub fn new(base: String, parent: String) -> Self {
        Self { base, parent }
    }
}

impl UrlBuilder for EnterpriseUrlBuilder {
    fn notebooks_collection(&self) -> String {
        format!("{}/notebooks", self.parent)
    }

    fn notebook_path(&self, notebook_id: &str) -> String {
        format!("{}/notebooks/{}", self.parent, notebook_id)
    }

    fn build_url(&self, path: &str) -> Result<Url> {
        let path = path.trim_start_matches('/');
        Url::parse(&format!("{}/{}", self.base, path)).map_err(Error::from)
    }

    fn build_upload_url(&self, path: &str) -> Result<Url> {
        let base = self.base.trim_end_matches('/');
        let trimmed_path = path.trim_start_matches('/');
        let upload_base = if let Some((prefix, _)) = base.rsplit_once("/v1alpha") {
            format!("{}/upload/v1alpha/{}", prefix, trimmed_path)
        } else {
            format!("{}/upload/{}", base, trimmed_path)
        };
        Url::parse(&upload_base).map_err(Error::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_url_combines_base_and_path_correctly() {
        let builder = EnterpriseUrlBuilder::new(
            "http://example.com/v1alpha".to_string(),
            "projects/123/locations/global".to_string(),
        );

        // Test with leading slash
        let url = builder.build_url("/projects/123/notebooks").unwrap();
        assert_eq!(
            url.as_str(),
            "http://example.com/v1alpha/projects/123/notebooks"
        );

        // Test without leading slash
        let url = builder.build_url("projects/123/notebooks").unwrap();
        assert_eq!(
            url.as_str(),
            "http://example.com/v1alpha/projects/123/notebooks"
        );
    }

    #[test]
    fn build_upload_url_handles_v1alpha_correctly() {
        let builder = EnterpriseUrlBuilder::new(
            "https://us-discoveryengine.googleapis.com/v1alpha".to_string(),
            "projects/123/locations/global".to_string(),
        );

        let url = builder
            .build_upload_url("/projects/123/notebooks/abc/sources:uploadFile")
            .unwrap();
        assert_eq!(
            url.as_str(),
            "https://us-discoveryengine.googleapis.com/upload/v1alpha/projects/123/notebooks/abc/sources:uploadFile"
        );
    }
}
