use anyhow::{bail, Result};
use clap::{Args, Subcommand};
use nblm_core::models::{GoogleDriveContent, TextContent, UserContent, VideoContent, WebContent};
use nblm_core::NblmClient;

use crate::util::{
    io::emit_sources,
    validate::{pair_with_names, validate_url},
};

#[derive(Subcommand)]
pub enum Command {
    Add(AddArgs),
}

#[derive(Args)]
pub struct AddArgs {
    #[arg(long, value_name = "ID")]
    pub notebook_id: String,

    #[arg(long = "web-url", value_name = "URL", alias = "url")]
    pub web_urls: Vec<String>,
    #[arg(long = "web-name", value_name = "DISPLAY", alias = "name")]
    pub web_names: Vec<String>,

    #[arg(long = "text", value_name = "TEXT")]
    pub texts: Vec<String>,
    #[arg(long = "text-name", value_name = "DISPLAY")]
    pub text_names: Vec<String>,

    #[arg(long = "drive-resource", value_name = "RESOURCE")]
    pub drive_resources: Vec<String>,
    #[arg(long = "drive-name", value_name = "DISPLAY")]
    pub drive_names: Vec<String>,

    #[arg(long = "video-url", value_name = "URL")]
    pub video_urls: Vec<String>,
    #[arg(long = "video-name", value_name = "DISPLAY")]
    pub video_names: Vec<String>,

    #[arg(long)]
    pub client_token: Option<String>,

    #[arg(long)]
    pub dry_run: bool,
}

pub async fn run(cmd: Command, client: &NblmClient, json_mode: bool) -> Result<()> {
    match cmd {
        Command::Add(args) => {
            let mut contents = Vec::<UserContent>::new();

            for (url, name) in pair_with_names(&args.web_urls, &args.web_names, "--web-name")? {
                validate_url(&url)?;
                contents.push(UserContent::Web {
                    web_content: WebContent {
                        url,
                        source_name: name,
                    },
                });
            }

            for (text, name) in pair_with_names(&args.texts, &args.text_names, "--text-name")? {
                if text.trim().is_empty() {
                    bail!("--text cannot be empty");
                }
                contents.push(UserContent::Text {
                    text_content: TextContent {
                        text,
                        source_name: name,
                    },
                });
            }

            let drive_entries =
                pair_with_names(&args.drive_resources, &args.drive_names, "--drive-name")?;
            let includes_drive = !drive_entries.is_empty();
            for (resource, name) in drive_entries {
                if resource.trim().is_empty() {
                    bail!("--drive-resource cannot be empty");
                }
                contents.push(UserContent::GoogleDrive {
                    google_drive_content: GoogleDriveContent {
                        resource_name: resource,
                        source_name: name,
                    },
                });
            }

            for (url, name) in pair_with_names(&args.video_urls, &args.video_names, "--video-name")?
            {
                validate_url(&url)?;
                contents.push(UserContent::Video {
                    video_content: VideoContent {
                        url,
                        source_name: name,
                    },
                });
            }

            if contents.is_empty() {
                bail!(
                    "at least one source must be specified (--url/--web-url/--text/--drive-resource/--video-url)"
                );
            }

            let response = client
                .add_sources(
                    &args.notebook_id,
                    contents,
                    args.client_token.clone(),
                    args.dry_run,
                )
                .await?;
            emit_sources(&args.notebook_id, &response, json_mode)?;
            if includes_drive {
                eprintln!(
                    "NOTE: To add Google Drive sources, run `gcloud auth login --enable-gdrive-access` first."
                );
            }
        }
    }
    Ok(())
}
