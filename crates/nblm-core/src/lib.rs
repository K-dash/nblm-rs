pub mod auth;
pub mod client;
pub mod doctor;
pub mod env;
mod error;
pub mod models;

pub use auth::{EnvTokenProvider, GcloudTokenProvider, StaticTokenProvider, TokenProvider};
pub use client::{NblmClient, RetryConfig, Retryer};
pub use env::{ApiProfile, EnvironmentConfig};
pub use error::{Error, Result};

use std::sync::Arc;

pub type DynTokenProvider = Arc<dyn TokenProvider>;
