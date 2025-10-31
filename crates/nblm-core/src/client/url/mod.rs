mod enterprise;

use std::sync::Arc;

use reqwest::Url;

use crate::env::ApiProfile;
use crate::error::Result;

pub(crate) use enterprise::EnterpriseUrlBuilder;

/// Profile-aware URL builder interface.
pub(crate) trait UrlBuilder: Send + Sync {
    fn notebooks_collection(&self) -> String;
    fn notebook_path(&self, notebook_id: &str) -> String;
    fn build_url(&self, path: &str) -> Result<Url>;
    fn build_upload_url(&self, path: &str) -> Result<Url>;
}

pub(crate) fn new_url_builder(
    profile: ApiProfile,
    base: String,
    parent: String,
) -> Arc<dyn UrlBuilder> {
    // TODO(profile-support): add profile-specific builders when new SKUs become available.
    match profile {
        ApiProfile::Enterprise => Arc::new(EnterpriseUrlBuilder::new(base, parent)),
        ApiProfile::Personal | ApiProfile::Workspace => {
            unimplemented!(
                "UrlBuilder for profile '{}' is not implemented",
                profile.as_str()
            )
        }
    }
}
