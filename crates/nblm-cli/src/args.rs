use std::time::Duration;

use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::ops;

#[derive(Parser)]
#[command(
    name = "nblm",
    version,
    about = "NotebookLM Enterprise CLI",
    disable_help_subcommand = true
)]
pub struct Cli {
    #[command(flatten)]
    pub global: GlobalArgs,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Args)]
pub struct GlobalArgs {
    #[arg(long, env = "NBLM_PROJECT_NUMBER")]
    pub project_number: String,

    #[arg(long, env = "NBLM_LOCATION", default_value = "global")]
    pub location: String,

    #[arg(long, env = "NBLM_ENDPOINT_LOCATION", default_value = "us")]
    pub endpoint_location: String,

    #[arg(long, value_enum, default_value_t = AuthMethod::Gcloud)]
    pub auth: AuthMethod,

    #[arg(long)]
    pub token: Option<String>,

    #[arg(long, global = true)]
    pub json: bool,

    #[arg(long, value_name = "DURATION", value_parser = parse_duration)]
    pub timeout: Option<Duration>,

    #[arg(long, env = "NBLM_ACCESS_TOKEN", hide_env_values = true)]
    pub env_token: Option<String>,

    /// (hidden) Override Discovery Engine API base URL. For tests only.
    /// Also configurable via env NBLM_BASE_URL.
    #[arg(long, hide = true, env = "NBLM_BASE_URL")]
    pub base_url: Option<String>,
}

#[derive(Subcommand)]
#[allow(clippy::large_enum_variant)]
pub enum Command {
    #[command(subcommand)]
    Notebooks(ops::notebooks::Command),
    #[command(subcommand)]
    Sources(ops::sources::Command),
    #[command(subcommand)]
    Audio(ops::audio::Command),
    #[command(subcommand)]
    Share(ops::share::Command),
}

#[derive(Copy, Clone, ValueEnum)]
pub enum AuthMethod {
    Gcloud,
    Env,
}

fn parse_duration(input: &str) -> std::result::Result<Duration, String> {
    humantime::parse_duration(input).map_err(|err| err.to_string())
}
