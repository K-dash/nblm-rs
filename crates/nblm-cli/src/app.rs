use std::time::Duration;

use anyhow::Result;
use tracing_subscriber::EnvFilter;

use nblm_core::{ApiProfile, EnvironmentConfig, NblmClient, ProfileParams, RetryConfig};

use crate::args::{Cli, Command};
use crate::ops::{audio, doctor, notebooks, share, sources};
use crate::util::auth::build_token_provider;

pub struct NblmApp {
    cli: Cli,
    client: NblmClient,
}

impl NblmApp {
    pub fn new(cli: Cli) -> Result<Self> {
        init_logging();

        if cli.global.debug_http {
            std::env::set_var("NBLM_DEBUG_HTTP", "1");
        }

        let provider = build_token_provider(&cli.global)?;
        let profile: ApiProfile = cli.global.profile.into();
        // TODO(profile-docs): Document profile selection once additional SKUs are available publicly.
        let environment = EnvironmentConfig::from_profile(
            profile,
            ProfileParams::enterprise(
                cli.global.project_number.clone(),
                cli.global.location.clone(),
                cli.global.endpoint_location.clone(),
            ),
        )?;
        let mut client = NblmClient::new(provider, environment)?;

        if let Some(timeout) = cli.global.timeout {
            client = client.with_timeout(timeout);
        }

        // Use fast retry config for tests to avoid slow retries
        let retry_config = if std::env::var_os("NBLM_RETRY_FAST").is_some() {
            RetryConfig::default()
                .with_min_delay(Duration::from_millis(5))
                .with_max_delay(Duration::from_millis(20))
                .with_max_retries(2)
        } else {
            RetryConfig::default()
        };
        client = client.with_retry_config(retry_config);

        if let Some(base) = &cli.global.base_url {
            client = client.with_base_url(base)?;
        }

        Ok(Self { cli, client })
    }

    pub async fn run(self) -> Result<()> {
        let json_mode = self.cli.global.json;
        match self.cli.command {
            Command::Notebooks(cmd) => notebooks::run(cmd, &self.client, json_mode).await,
            Command::Sources(cmd) => sources::run(cmd, &self.client, json_mode).await,
            Command::Audio(cmd) => audio::run(cmd, &self.client, json_mode).await,
            Command::Share(cmd) => share::run(cmd, &self.client, json_mode).await,
            Command::Doctor(cmd) => doctor::run(cmd).await,
        }
    }
}

fn init_logging() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let _ = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .with_writer(std::io::stderr)
        .try_init();
}
