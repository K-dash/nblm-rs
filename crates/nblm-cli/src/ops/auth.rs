use anyhow::{Context, Result};
use colored::Colorize;
use std::process::Stdio;
use tokio::process::Command;

use crate::args::{AuthCommand, AuthSubcommand};

pub async fn run(cmd: AuthCommand) -> Result<()> {
    match cmd.command {
        AuthSubcommand::Login(args) => login(args).await,
        AuthSubcommand::Status => status().await,
    }
}

async fn login(args: crate::args::LoginArgs) -> Result<()> {
    println!("{}", "Starting Google Cloud authentication...".cyan());
    println!("This will open your browser to authenticate with Google.");

    let mut command = Command::new("gcloud");
    command.arg("auth").arg("login");

    if args.drive_access {
        println!("(Requesting Google Drive access)");
        command.arg("--enable-gdrive-access");
    }

    println!("(Executing: {:?})\n", command);

    let status = command
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .await
        .context("Failed to execute 'gcloud'. Please ensure Google Cloud SDK is installed and in your PATH.")?;

    if status.success() {
        println!("\n{}", "Authentication successful!".green().bold());
        println!("You can now use nblm commands.");
    } else {
        println!("\n{}", "Authentication failed.".red().bold());
        if let Some(code) = status.code() {
            println!("gcloud exited with code: {}", code);
        }
        anyhow::bail!("gcloud auth login failed");
    }

    Ok(())
}

async fn status() -> Result<()> {
    // Check if we can get a token
    let output = Command::new("gcloud")
        .arg("auth")
        .arg("print-access-token")
        .output()
        .await
        .context("Failed to execute 'gcloud'. Please ensure Google Cloud SDK is installed.")?;

    if !output.status.success() {
        println!("{}", "Not authenticated.".yellow());
        println!("Run '{}' to log in.", "nblm auth login".bold());
        anyhow::bail!("Not authenticated");
    }

    // Try to get the current account email for better status info
    let account_output = Command::new("gcloud")
        .arg("config")
        .arg("get-value")
        .arg("account")
        .output()
        .await;

    let account = if let Ok(out) = account_output {
        String::from_utf8_lossy(&out.stdout).trim().to_string()
    } else {
        "Unknown account".to_string()
    };

    println!("{}", "Authenticated".green().bold());
    if !account.is_empty() {
        println!("Account: {}", account.cyan());
    }
    println!("Backend: gcloud");

    Ok(())
}
