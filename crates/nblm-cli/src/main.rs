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

    if args.len() > 1 && args[1] == "doctor" {
        // Parse doctor-specific arguments
        use clap::Parser;
        #[derive(Parser)]
        #[command(name = "nblm")]
        struct DoctorCli {
            #[command(subcommand)]
            command: DoctorCommand,
        }

        #[derive(clap::Subcommand)]
        enum DoctorCommand {
            Doctor(ops::doctor::DoctorArgs),
        }

        let doctor_cli = DoctorCli::parse();
        let DoctorCommand::Doctor(doctor_args) = doctor_cli.command;
        return ops::doctor::run(doctor_args).await;
    }

    let cli = args::Cli::parse();
    app::NblmApp::new(cli)?.run().await
}
