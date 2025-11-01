use anyhow::Result;
use clap::Args;
use colored::Colorize;
use nblm_core::doctor::{check_commands, check_environment_variables, DiagnosticsSummary};
use std::io::IsTerminal;

#[derive(Args)]
pub struct DoctorArgs {}

pub async fn run(_args: DoctorArgs) -> Result<()> {
    println!("Running NotebookLM environment diagnostics...\n");

    let use_color = should_use_color();

    // Run all checks
    let mut all_checks = Vec::new();
    all_checks.extend(check_environment_variables());
    all_checks.extend(check_commands());

    // Print individual check results
    for check in &all_checks {
        if use_color {
            println!("{}", check.format_colored());
        } else {
            println!("{}", check.format());
        }
    }

    // Print summary
    let summary = DiagnosticsSummary::new(all_checks);
    if use_color {
        println!("{}", summary.format_summary_colored());
    } else {
        println!("{}", summary.format_summary());
    }

    // Determine exit behavior
    let exit_code = summary.exit_code();
    if exit_code == 0 {
        if use_color {
            println!(
                "\n{}",
                "All critical checks passed. You're ready to use nblm.".green()
            );
        } else {
            println!("\nAll critical checks passed. You're ready to use nblm.");
        }
    }

    std::process::exit(exit_code);
}

fn should_use_color() -> bool {
    if std::env::var_os("NO_COLOR").is_some() {
        return false;
    }

    if let Ok(force) = std::env::var("CLICOLOR_FORCE") {
        if force != "0" {
            return true;
        }
    }

    if let Ok(choice) = std::env::var("CLICOLOR") {
        if choice == "0" {
            return false;
        }
    }

    std::io::stdout().is_terminal()
}
