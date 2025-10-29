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
    if args.len() > 1 && args[1] == "doctor" {
        return ops::doctor::run(ops::doctor::DoctorArgs {}).await;
    }

    let cli = args::Cli::parse();
    app::NblmApp::new(cli)?.run().await
}
