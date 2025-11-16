use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeJsonError;
use thiserror::Error;

/// Errors that can occur during OAuth2 operations.
#[derive(Error, Debug)]
pub enum OAuthError {
    #[error("OAuth client configuration error: {0}")]
    Config(String),

    #[error("OAuth authorization flow failed: {0}")]
    Flow(String),

    #[error("Token refresh failed: {0}")]
    Refresh(String),

    #[error("Token revocation failed: {0}")]
    Revocation(String),

    #[error("Token storage error: {0}")]
    Storage(#[from] std::io::Error),

    #[error("CSRF state mismatch: expected {expected}, got {actual}")]
    StateMismatch { expected: String, actual: String },

    #[error("Missing required environment variable: {0}")]
    MissingEnvVar(&'static str),

    #[error("HTTP request failed: {0}")]
    Http(#[from] ReqwestError),

    #[error("Invalid token response: {0}")]
    InvalidResponse(String),

    #[error("JSON serialization error: {0}")]
    Json(#[from] SerdeJsonError),
}

pub type Result<T> = std::result::Result<T, OAuthError>;
