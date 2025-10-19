use anyhow::{bail, Result};
use clap::{Args, Subcommand, ValueEnum};
use nblm_core::{
    models::{AccountRole, ProjectRole},
    NblmClient,
};

use crate::util::io::emit_share;

#[derive(Subcommand)]
pub enum Command {
    Add(AddArgs),
}

#[derive(Args)]
pub struct AddArgs {
    #[arg(long, value_name = "ID")]
    pub notebook_id: String,

    #[arg(long = "email", value_name = "EMAIL", required = true)]
    pub emails: Vec<String>,

    #[arg(long, default_value = "reader")]
    pub role: ShareRole,
}

#[derive(Copy, Clone, ValueEnum)]
pub enum ShareRole {
    Owner,
    Writer,
    Reader,
    #[clap(name = "not-shared")]
    NotShared,
}

impl ShareRole {
    fn account_role(&self, email: String) -> AccountRole {
        AccountRole {
            email,
            role: match self {
                ShareRole::Owner => ProjectRole::ProjectRoleOwner,
                ShareRole::Writer => ProjectRole::ProjectRoleWriter,
                ShareRole::Reader => ProjectRole::ProjectRoleReader,
                ShareRole::NotShared => ProjectRole::ProjectRoleNotShared,
            },
        }
    }
}

pub async fn run(cmd: Command, client: &NblmClient, json_mode: bool) -> Result<()> {
    match cmd {
        Command::Add(args) => {
            if args.emails.is_empty() {
                bail!("provide at least one --email");
            }
            let accounts = args
                .emails
                .iter()
                .map(|email| args.role.account_role(email.clone()))
                .collect();
            let response = client.share_notebook(&args.notebook_id, accounts).await?;
            emit_share(&response, json_mode)?;
        }
    }
    Ok(())
}
