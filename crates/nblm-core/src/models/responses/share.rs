use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ShareResponse {
    #[serde(default)]
    pub granted: Option<i32>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn share_response_deserializes_correctly() {
        let json = r#"{"granted": 2}"#;
        let response: ShareResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.granted, Some(2));
        assert!(response.extra.is_empty());
    }

    #[test]
    fn share_response_deserializes_with_extra_fields() {
        let json = r#"{"granted": 1, "customField": "value"}"#;
        let response: ShareResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.granted, Some(1));
        assert_eq!(
            response.extra.get("customField").unwrap().as_str().unwrap(),
            "value"
        );
    }

    #[test]
    fn share_response_deserializes_without_granted() {
        let json = r#"{}"#;
        let response: ShareResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.granted, None);
    }
}
