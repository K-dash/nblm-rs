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
    #[error("url parse error: {0}")]
    Url(#[from] url::ParseError),
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
}

fn extract_error_message(body: &str) -> Option<String> {
    let json: serde_json::Value = serde_json::from_str(body).ok()?;
    json.get("error")
        .and_then(|err| err.get("message"))
        .and_then(|msg| msg.as_str())
        .map(|s| s.to_string())
}
