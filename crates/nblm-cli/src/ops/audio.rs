use anyhow::{Context, Result};
use clap::{Args, Subcommand};
use nblm_core::{models::AudioOverviewRequest, NblmClient};
use serde_json::json;

use crate::util::io::emit_json;

#[derive(Subcommand)]
pub enum Command {
    Create(CreateArgs),
}

#[derive(Args)]
pub struct CreateArgs {
    #[arg(long, value_name = "ID")]
    pub notebook_id: String,

    #[arg(long, value_name = "JSON")]
    pub config: Option<String>,
}

pub async fn run(cmd: Command, client: &NblmClient, json_mode: bool) -> Result<()> {
    match cmd {
        Command::Create(args) => {
            let config = match args.config {
                Some(raw) => {
                    Some(serde_json::from_str(&raw).context("failed to parse --config JSON")?)
                }
                None => None,
            };
            let request = AudioOverviewRequest { config };
            let response = client
                .create_audio_overview(&args.notebook_id, request)
                .await?;
            emit_json(json!(response), json_mode);
        }
    }
    Ok(())
}
