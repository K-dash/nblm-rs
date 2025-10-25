use serde::{Deserialize, Serialize};

use crate::models::notebook::Notebook;

/// Response from list recently viewed notebooks API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ListRecentlyViewedResponse {
    #[serde(default)]
    pub notebooks: Vec<Notebook>,
}
