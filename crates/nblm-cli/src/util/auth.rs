use std::{env, sync::Arc};

use anyhow::Result;
use nblm_core::auth::{EnvTokenProvider, GcloudTokenProvider, StaticTokenProvider, TokenProvider};

use crate::args::{AuthMethod, GlobalArgs};

pub fn build_token_provider(args: &GlobalArgs) -> Result<Arc<dyn TokenProvider>> {
    Ok(match args.auth {
        AuthMethod::Gcloud => Arc::new(build_gcloud_provider()?),
        AuthMethod::Env => {
            if let Some(token) = args.token.as_ref().or(args.env_token.as_ref()) {
                Arc::new(StaticTokenProvider::new(token.clone()))
            } else {
                Arc::new(EnvTokenProvider::new("NBLM_ACCESS_TOKEN"))
            }
        }
    })
}

fn build_gcloud_provider() -> Result<GcloudTokenProvider> {
    let binary = env::var("NBLM_GCLOUD_PATH").unwrap_or_else(|_| "gcloud".to_string());
    Ok(GcloudTokenProvider::new(binary))
}
