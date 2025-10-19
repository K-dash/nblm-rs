use std::{
    env,
    io::{self, Read},
    path::PathBuf,
    sync::Arc,
};

use anyhow::{anyhow, Result};
use nblm_core::auth::{
    EnvTokenProvider, GcloudTokenProvider, ServiceAccountTokenProvider, StaticTokenProvider,
    TokenProvider,
};

use crate::args::{AuthMethod, GlobalArgs};

const CLOUD_PLATFORM_SCOPE: &str = "https://www.googleapis.com/auth/cloud-platform";

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
        AuthMethod::Sa => Arc::new(build_service_account_provider(args)?),
    })
}

fn build_gcloud_provider() -> Result<GcloudTokenProvider> {
    let binary = env::var("NBLM_GCLOUD_PATH").unwrap_or_else(|_| "gcloud".to_string());
    Ok(GcloudTokenProvider::new(binary))
}

fn build_service_account_provider(args: &GlobalArgs) -> Result<ServiceAccountTokenProvider> {
    let provider = if let Some(json) = &args.sa_key_json {
        ServiceAccountTokenProvider::from_json(json, scopes())
    } else if args.sa_key_stdin {
        let mut buf = String::new();
        io::stdin()
            .read_to_string(&mut buf)
            .map_err(|err| anyhow!("failed to read key JSON from stdin: {err}"))?;
        ServiceAccountTokenProvider::from_json(&buf, scopes())
    } else if let Some(path) = &args.sa_key {
        ServiceAccountTokenProvider::from_file(path, scopes())
    } else if let Ok(path) = env::var("GOOGLE_APPLICATION_CREDENTIALS") {
        ServiceAccountTokenProvider::from_file(PathBuf::from(path), scopes())
    } else if let Ok(json) = env::var("GOOGLE_APPLICATION_CREDENTIALS_JSON") {
        ServiceAccountTokenProvider::from_json(&json, scopes())
    } else {
        return Err(anyhow!(
            "no service account key specified (--sa-key-json|--sa-key-stdin|--sa-key|GOOGLE_APPLICATION_CREDENTIALS|GOOGLE_APPLICATION_CREDENTIALS_JSON)"
        ));
    }
    .map_err(|err| anyhow!("failed to initialize service account token: {err}"))?;

    Ok(apply_sa_overrides(provider, args))
}

fn apply_sa_overrides(
    mut provider: ServiceAccountTokenProvider,
    args: &GlobalArgs,
) -> ServiceAccountTokenProvider {
    if let Some(leeway) = args.sa_token_leeway {
        provider = provider.with_leeway(leeway);
    }
    if let Some(timeout) = args.sa_http_timeout {
        provider = provider.with_http_timeout(timeout);
    }
    provider
}

fn scopes() -> Vec<String> {
    vec![CLOUD_PLATFORM_SCOPE.to_string()]
}
