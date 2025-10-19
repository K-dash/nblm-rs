use anyhow::Result;
use clap::{Args, Subcommand};
use nblm_core::NblmClient;

use crate::util::io::{emit_notebook, emit_recent};

#[derive(Subcommand)]
pub enum Command {
    Create(CreateArgs),
    Recent(RecentArgs),
}

#[derive(Args)]
pub struct CreateArgs {
    #[arg(long)]
    pub title: String,
}

#[derive(Args)]
pub struct RecentArgs {
    #[arg(long)]
    pub page_size: Option<u32>,

    #[arg(long)]
    pub page_token: Option<String>,
}

pub async fn run(cmd: Command, client: &NblmClient, json_mode: bool) -> Result<()> {
    match cmd {
        Command::Create(args) => {
            let notebook = client.create_notebook(args.title).await?;
            emit_notebook(&notebook, json_mode);
        }
        Command::Recent(args) => {
            let response = client
                .list_recently_viewed(args.page_size, args.page_token.as_deref())
                .await?;
            emit_recent(&response, json_mode)?;
        }
    }
    Ok(())
}
