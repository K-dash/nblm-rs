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
    /// Manage authentication using Google Cloud SDK (gcloud)
    Auth(AuthCommand),
    Doctor(ops::doctor::DoctorArgs),
}

#[derive(Args)]
pub struct AuthCommand {
    #[command(subcommand)]
    pub command: AuthSubcommand,
}

#[derive(Subcommand)]
pub enum AuthSubcommand {
    /// Log in via Google Cloud SDK (gcloud auth login)
    Login(LoginArgs),
    /// Check current authentication status
    Status,
}

#[derive(Args)]
pub struct LoginArgs {
    /// Request Google Drive access (adds --enable-gdrive-access to gcloud)
    #[arg(long)]
    pub drive_access: bool,
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

pub enum SpecialCommand {
    Doctor(crate::ops::doctor::DoctorArgs),
    Auth(AuthCommand),
}

pub fn parse_pre_command(args: &[String]) -> Option<SpecialCommand> {
    if args.len() <= 1 {
        return None;
    }

    match args[1].as_str() {
        "doctor" => {
            #[derive(Parser)]
            #[command(name = "nblm")]
            struct DoctorCli {
                #[command(subcommand)]
                command: DoctorCommand,
            }

            #[derive(Subcommand)]
            enum DoctorCommand {
                Doctor(crate::ops::doctor::DoctorArgs),
            }

            // We use try_parse_from to avoid exiting the process on error/help
            // But for main logic we might want to just parse.
            // Here we are replicating main.rs logic which assumes if "doctor" is present
            // we treat it as doctor command.
            // However, main.rs used `parse()` which exits.
            // To keep it testable, we should probably use `try_parse_from`.
            // But `main.rs` logic was: if args[1] == "doctor", parse as DoctorCli.

            // If parsing fails (e.g. --help), we might want to let main handle it or return None?
            // In main.rs, it called `parse()`, so it would exit.
            // For exact behavior preservation:
            let cli = DoctorCli::parse_from(args);
            let DoctorCommand::Doctor(args) = cli.command;
            Some(SpecialCommand::Doctor(args))
        }
        "auth" => {
            #[derive(Parser)]
            #[command(name = "nblm")]
            struct AuthCli {
                #[command(subcommand)]
                command: AuthCommandWrapper,
            }

            #[derive(Subcommand)]
            enum AuthCommandWrapper {
                Auth(AuthCommand),
            }

            let cli = AuthCli::parse_from(args);
            let AuthCommandWrapper::Auth(cmd) = cli.command;
            Some(SpecialCommand::Auth(cmd))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pre_command() {
        // Test doctor
        let args = vec!["nblm".to_string(), "doctor".to_string()];
        match parse_pre_command(&args) {
            Some(SpecialCommand::Doctor(_)) => {}
            _ => panic!("expected Doctor command"),
        }

        // Test auth
        let args = vec!["nblm".to_string(), "auth".to_string(), "login".to_string()];
        match parse_pre_command(&args) {
            Some(SpecialCommand::Auth(cmd)) => match cmd.command {
                AuthSubcommand::Login(_) => {}
                _ => panic!("expected Login subcommand"),
            },
            _ => panic!("expected Auth command"),
        }

        // Test normal command
        let args = vec!["nblm".to_string(), "notebooks".to_string()];
        assert!(parse_pre_command(&args).is_none());
    }

    #[test]
    fn parse_auth_command() {
        let args = Cli::parse_from(["nblm", "auth", "login"]);
        match args.command {
            Command::Auth(cmd) => match cmd.command {
                AuthSubcommand::Login(_) => {}
                _ => panic!("expected Login subcommand"),
            },
            _ => panic!("expected Auth command"),
        }

        let args = Cli::parse_from(["nblm", "auth", "login", "--drive-access"]);
        match args.command {
            Command::Auth(cmd) => match cmd.command {
                AuthSubcommand::Login(args) => assert!(args.drive_access),
                _ => panic!("expected Login subcommand"),
            },
            _ => panic!("expected Auth command"),
        }

        let args = Cli::parse_from(["nblm", "auth", "status"]);
        match args.command {
            Command::Auth(cmd) => match cmd.command {
                AuthSubcommand::Status => {}
                _ => panic!("expected Status subcommand"),
            },
            _ => panic!("expected Auth command"),
        }
    }
}
