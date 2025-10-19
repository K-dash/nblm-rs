use anyhow::Result;
use clap::Parser;

mod app;
mod args;
mod ops;
mod util;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    let cli = args::Cli::parse();
    app::NblmApp::new(cli)?.run().await
}
