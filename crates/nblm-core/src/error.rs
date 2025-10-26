use reqwest::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("token provider error: {0}")]
    TokenProvider(String),
    #[error("invalid endpoint configuration: {0}")]
    Endpoint(String),
    #[error("request error: {0}")]
    Request(#[from] reqwest::Error),
    #[error("http error {status}: {message}")]
    Http {
        status: StatusCode,
        message: String,
        body: String,
    },
    #[error("json deserialize error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("url parse error: {0}")]
    Url(#[from] url::ParseError),
    #[error("validation error: {0}")]
    Validation(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    pub fn http(status: StatusCode, body: impl Into<String>) -> Self {
        let body = body.into();
        let message = extract_error_message(&body).unwrap_or_else(|| body.clone());
        Self::Http {
            status,
            message,
            body,
        }
    }

    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation(message.into())
    }
}

fn extract_error_message(body: &str) -> Option<String> {
    let json: serde_json::Value = serde_json::from_str(body).ok()?;
    json.get("error")
        .and_then(|err| err.get("message"))
        .and_then(|msg| msg.as_str())
        .map(|s| s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_error_message_from_gcp_response() {
        let body = r#"{"error":{"message":"Not Found"}}"#;
        assert_eq!(extract_error_message(body), Some("Not Found".to_string()));
    }

    #[test]
    fn extract_error_message_missing() {
        assert_eq!(extract_error_message("{}"), None);
        assert_eq!(extract_error_message("invalid"), None);
    }

    #[test]
    fn http_error_uses_message_field_if_available() {
        let e = Error::http(
            StatusCode::TOO_MANY_REQUESTS,
            r#"{"error":{"message":"Too Many Requests"}}"#,
        );
        match e {
            Error::Http {
                message,
                body,
                status,
            } => {
                assert_eq!(status, StatusCode::TOO_MANY_REQUESTS);
                assert_eq!(message, "Too Many Requests");
                assert!(body.contains("Too Many Requests"));
            }
            _ => panic!("expected Error::Http"),
        }
    }

    #[test]
    fn http_error_uses_body_when_no_message() {
        let e = Error::http(StatusCode::BAD_REQUEST, "plain error text");
        match e {
            Error::Http { message, body, .. } => {
                assert_eq!(message, "plain error text");
                assert_eq!(body, "plain error text");
            }
            _ => panic!("expected Error::Http"),
        }
    }
}
