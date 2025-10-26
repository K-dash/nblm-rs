use std::{fs, path::PathBuf};

use anyhow::{anyhow, bail, Context, Result};
use clap::{Args, Subcommand};
use nblm_core::models::{GoogleDriveContent, TextContent, UserContent, VideoContent, WebContent};
use nblm_core::NblmClient;

use crate::util::{
    io::{emit_source, emit_sources, emit_uploaded_source},
    validate::{pair_with_names, validate_url},
};

#[derive(Subcommand)]
pub enum Command {
    Add(AddArgs),
    Delete(DeleteArgs),
    Upload(UploadArgs),
    Get(GetArgs),
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

    /// Google Drive document ID.
    #[arg(long = "drive-document-id", value_name = "DOCUMENT_ID")]
    pub drive_document_ids: Vec<String>,
    /// Google Drive MIME type.
    #[arg(long = "drive-mime-type", value_name = "MIME_TYPE")]
    pub drive_mime_types: Vec<String>,
    #[arg(long = "drive-name", value_name = "DISPLAY")]
    pub drive_names: Vec<String>,

    #[arg(long = "video-url", value_name = "URL")]
    pub video_urls: Vec<String>,
}

#[derive(Args)]
pub struct DeleteArgs {
    #[arg(long, value_name = "ID")]
    pub notebook_id: String,

    #[arg(long = "source-name", value_name = "NAME", required = true)]
    pub source_names: Vec<String>,
}

#[derive(Args)]
pub struct UploadArgs {
    #[arg(long, value_name = "ID")]
    pub notebook_id: String,

    #[arg(long, value_name = "PATH")]
    pub file: PathBuf,

    #[arg(long = "content-type", value_name = "MEDIA_TYPE")]
    pub content_type: Option<String>,

    /// NOTE: As of 2025-10-25 the NotebookLM API rejects custom display names (HTTP 400).
    /// This flag is kept for forward compatibility but currently non-functional.
    #[arg(long = "display-name", value_name = "NAME")]
    pub display_name: Option<String>,
}

#[derive(Args)]
pub struct GetArgs {
    #[arg(long, value_name = "ID", help = "Notebook ID containing the source")]
    pub notebook_id: String,

    #[arg(long, value_name = "ID", help = "Source ID to retrieve")]
    pub source_id: String,
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
                        content: text,
                        source_name: name,
                    },
                });
            }

            let includes_drive = !args.drive_document_ids.is_empty();
            if args.drive_document_ids.len() != args.drive_mime_types.len() {
                bail!(
                    "--drive-document-id and --drive-mime-type must be specified in pairs (got {} document IDs and {} mime types)",
                    args.drive_document_ids.len(),
                    args.drive_mime_types.len()
                );
            }
            if args.drive_names.len() > args.drive_document_ids.len() {
                bail!("--drive-name count exceeds number of document IDs");
            }
            for (idx, (document_id, mime_type)) in args
                .drive_document_ids
                .iter()
                .zip(&args.drive_mime_types)
                .enumerate()
            {
                if document_id.trim().is_empty() {
                    bail!("--drive-document-id cannot be empty");
                }
                let source_name = args.drive_names.get(idx).and_then(|s| {
                    let trimmed = s.trim();
                    if trimmed.is_empty() {
                        None
                    } else {
                        Some(trimmed.to_string())
                    }
                });
                contents.push(UserContent::GoogleDrive {
                    google_drive_content: GoogleDriveContent {
                        document_id: document_id.clone(),
                        mime_type: mime_type.clone(),
                        source_name,
                    },
                });
            }

            for url in &args.video_urls {
                validate_url(url)?;
                contents.push(UserContent::Video {
                    video_content: VideoContent { url: url.clone() },
                });
            }

            if contents.is_empty() {
                bail!(
                    "at least one source must be specified (--web-url/--text/--drive-document-id/--video-url)"
                );
            }

            let response = client.add_sources(&args.notebook_id, contents).await?;
            emit_sources(&args.notebook_id, &response, json_mode)?;
            if includes_drive {
                eprintln!("NOTE: Google Drive sources require `gcloud auth login --enable-gdrive-access` and that the authenticated account has view access to the document.");
            }
        }
        Command::Delete(args) => {
            let response = client
                .delete_sources(&args.notebook_id, args.source_names.clone())
                .await?;
            if !json_mode {
                println!("Deleted {} source(s) successfully", args.source_names.len());
            } else {
                use serde_json::json;
                crate::util::io::emit_json(
                    json!({
                        "status": "deleted",
                        "count": args.source_names.len(),
                        "response": response
                    }),
                    json_mode,
                );
            }
        }
        Command::Upload(args) => {
            if !args.file.exists() {
                bail!("file not found: {}", args.file.display());
            }
            if !args.file.is_file() {
                bail!("path is not a file: {}", args.file.display());
            }

            let data = fs::read(&args.file)
                .with_context(|| format!("failed to read {}", args.file.display()))?;
            if data.is_empty() {
                bail!("cannot upload empty files");
            }

            let content_type = args
                .content_type
                .map(|value| value.trim().to_string())
                .filter(|value| !value.is_empty())
                .unwrap_or_else(|| {
                    mime_guess::from_path(&args.file)
                        .first_or_octet_stream()
                        .essence_str()
                        .to_string()
                });

            let inferred_name = args
                .display_name
                .as_ref()
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .or_else(|| {
                    args.file
                        .file_name()
                        .and_then(|name| name.to_str())
                        .map(|s| s.to_string())
                })
                .ok_or_else(|| anyhow!("could not determine file name; use --display-name"))?;

            if args.display_name.is_some() {
                eprintln!(
                    "WARNING: NotebookLM API rejects custom display names as of 2025-10-25 (HTTP 400)."
                );
                eprintln!("The uploaded source will use the original file name instead.");
            }

            let response = client
                .upload_source_file(&args.notebook_id, &inferred_name, &content_type, data)
                .await?;

            emit_uploaded_source(
                &args.notebook_id,
                &inferred_name,
                &content_type,
                &response,
                json_mode,
            )?;
        }
        Command::Get(args) => {
            let source = client
                .get_source(&args.notebook_id, &args.source_id)
                .await?;

            if json_mode {
                crate::util::io::emit_json(serde_json::json!(&source), json_mode);
            } else {
                emit_source(&source);
            }
        }
    }
    Ok(())
}
