use anyhow::Result;
use clap::Parser;

mod app;
mod args;
mod ops;
mod util;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    // Check if this is the doctor command before requiring global args
    let args: Vec<String> = std::env::args().collect();
    let has_doctor = args.iter().any(|arg| arg == "doctor");
    let has_json = args.iter().any(|arg| arg == "--json");

    // If both doctor and --json are present, bail immediately
    if has_doctor && has_json {
        anyhow::bail!("The --json flag is not supported for the 'doctor' command");
    }

    // Check for special commands that need to bypass NblmApp initialization
    if let Some(cmd) = args::parse_pre_command(&args) {
        match cmd {
            args::SpecialCommand::Doctor(args) => return ops::doctor::run(args).await,
            args::SpecialCommand::Auth(cmd) => return ops::auth::run(cmd).await,
        }
    }

    let cli = args::Cli::parse();
    app::NblmApp::new(cli)?.run().await
}
