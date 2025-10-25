use anyhow::Result;
use clap::{Args, Subcommand};
use nblm_core::{models::AudioOverviewRequest, NblmClient};
use serde_json::json;

use crate::util::io::emit_json;

#[derive(Subcommand)]
pub enum Command {
    Create(CreateArgs),
    Delete(DeleteArgs),
}

#[derive(Args)]
pub struct CreateArgs {
    #[arg(long, value_name = "ID")]
    pub notebook_id: String,
    // TODO: Uncomment when API supports these fields (as of 2025-10-19, they return "Unknown name" errors)
    // /// Source IDs to include in the audio overview
    // #[arg(long = "source-id", value_name = "SOURCE_ID")]
    // pub source_ids: Vec<String>,
    //
    // /// Focus topic for the episode
    // #[arg(long, value_name = "TEXT")]
    // pub episode_focus: Option<String>,
    //
    // /// Language code (e.g., ja-JP, en-US)
    // #[arg(long, value_name = "CODE")]
    // pub language_code: Option<String>,
}

#[derive(Args)]
pub struct DeleteArgs {
    #[arg(long, value_name = "ID")]
    pub notebook_id: String,
}

pub async fn run(cmd: Command, client: &NblmClient, json_mode: bool) -> Result<()> {
    match cmd {
        Command::Create(args) => {
            // TODO: Uncomment when API supports configuration fields
            // let source_ids = if args.source_ids.is_empty() {
            //     None
            // } else {
            //     Some(
            //         args.source_ids
            //             .into_iter()
            //             .map(|id| SourceId { id })
            //             .collect(),
            //     )
            // };
            //
            // let request = AudioOverviewRequest {
            //     source_ids,
            //     episode_focus: args.episode_focus,
            //     language_code: args.language_code,
            // };

            let request = AudioOverviewRequest::default();

            let response = client
                .create_audio_overview(&args.notebook_id, request)
                .await?;

            if json_mode {
                // In CLI json mode, wrap with audioOverview to match original format
                emit_json(json!({"audioOverview": response}), json_mode);
            } else {
                println!("Audio overview created successfully:");
                if let Some(id) = &response.audio_overview_id {
                    println!("  Audio Overview ID: {}", id);
                }
                if let Some(name) = &response.name {
                    println!("  Name: {}", name);
                }
                if let Some(status) = &response.status {
                    println!("  Status: {}", status);
                }
            }
        }
        Command::Delete(args) => {
            client.delete_audio_overview(&args.notebook_id).await?;
            if !json_mode {
                println!("Audio overview deleted successfully");
            } else {
                emit_json(json!({"status": "deleted"}), json_mode);
            }
        }
    }
    Ok(())
}
