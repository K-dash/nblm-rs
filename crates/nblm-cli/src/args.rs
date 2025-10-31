use std::time::Duration;

use clap::{Args, Parser, Subcommand, ValueEnum};

use nblm_core::ApiProfile;

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
    pub project_number: Option<String>,

    #[arg(long, env = "NBLM_LOCATION", default_value = "global")]
    pub location: String,

    #[arg(long, env = "NBLM_ENDPOINT_LOCATION", default_value = "global")]
    pub endpoint_location: String,

    /// (hidden) API profile selector. Defaults to Enterprise until additional SKUs are public.
    #[arg(long, value_enum, default_value_t = ProfileArg::Enterprise, hide = true)]
    pub profile: ProfileArg,

    #[arg(long, value_enum, default_value_t = AuthMethod::Gcloud)]
    pub auth: AuthMethod,

    #[arg(long)]
    pub token: Option<String>,

    #[arg(long, global = true)]
    pub json: bool,

    /// Enable verbose HTTP logging (also available via env NBLM_DEBUG_HTTP=1)
    #[arg(long, global = true)]
    pub debug_http: bool,

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
    Doctor(ops::doctor::DoctorArgs),
}

#[derive(Copy, Clone, ValueEnum)]
pub enum AuthMethod {
    Gcloud,
    Env,
    #[value(name = "user-oauth", hide = true)]
    UserOauth,
}

impl AuthMethod {
    pub fn requires_experimental_flag(self) -> bool {
        matches!(self, AuthMethod::UserOauth)
    }
}

fn parse_duration(input: &str) -> std::result::Result<Duration, String> {
    humantime::parse_duration(input).map_err(|err| err.to_string())
}

#[derive(Copy, Clone, ValueEnum)]
pub enum ProfileArg {
    Enterprise,
    Personal,
    Workspace,
}

impl ProfileArg {
    pub fn requires_experimental_flag(self) -> bool {
        matches!(self, ProfileArg::Personal | ProfileArg::Workspace)
    }
}

impl From<ProfileArg> for ApiProfile {
    fn from(arg: ProfileArg) -> Self {
        match arg {
            ProfileArg::Enterprise => ApiProfile::Enterprise,
            ProfileArg::Personal => ApiProfile::Personal,
            ProfileArg::Workspace => ApiProfile::Workspace,
        }
    }
}
