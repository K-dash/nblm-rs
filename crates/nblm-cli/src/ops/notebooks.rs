use anyhow::Result;
use clap::{Args, Subcommand};
use nblm_core::NblmClient;

use crate::util::io::{emit_notebook, emit_recent};

#[derive(Subcommand)]
pub enum Command {
    Create(CreateArgs),
    Recent(RecentArgs),
    Delete(DeleteArgs),
}

#[derive(Args)]
pub struct CreateArgs {
    #[arg(long)]
    pub title: String,
}

#[derive(Args)]
pub struct RecentArgs {
    /// Page size for pagination (1-500). Note: NotebookLM API currently ignores this parameter and returns all notebooks.
    #[arg(long)]
    pub page_size: Option<u32>,

    /// Page token for pagination. Note: NotebookLM API does not currently implement pagination tokens.
    #[arg(long)]
    pub page_token: Option<String>,
}

#[derive(Args)]
pub struct DeleteArgs {
    /// Full notebook resource name (e.g., projects/PROJECT_NUMBER/locations/LOCATION/notebooks/NOTEBOOK_ID).
    /// Can be specified multiple times. Note: API limitation requires sequential deletion (one at a time).
    #[arg(long = "notebook-name", value_name = "NAME", required = true)]
    pub notebook_names: Vec<String>,
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
        Command::Delete(args) => {
            let response = client.delete_notebooks(args.notebook_names.clone()).await?;
            if !json_mode {
                println!(
                    "Deleted {} notebook(s) successfully",
                    args.notebook_names.len()
                );
            } else {
                use serde_json::json;
                crate::util::io::emit_json(
                    json!({
                        "status": "deleted",
                        "count": args.notebook_names.len(),
                        "response": response
                    }),
                    json_mode,
                );
            }
        }
    }
    Ok(())
}
