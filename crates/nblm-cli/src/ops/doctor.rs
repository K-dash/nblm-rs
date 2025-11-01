use anyhow::Result;
use clap::Args;
use colored::Colorize;
use nblm_core::doctor::{check_commands, check_environment_variables, DiagnosticsSummary};

#[derive(Args)]
pub struct DoctorArgs {}

pub async fn run(_args: DoctorArgs) -> Result<()> {
    println!("Running NotebookLM environment diagnostics...\n");

    // Run all checks
    let mut all_checks = Vec::new();
    all_checks.extend(check_environment_variables());
    all_checks.extend(check_commands());

    // Print individual check results
    for check in &all_checks {
        println!("{}", check.format_colored());
    }

    // Print summary
    let summary = DiagnosticsSummary::new(all_checks);
    println!("{}", summary.format_summary_colored());

    // Determine exit behavior
    let exit_code = summary.exit_code();
    if exit_code == 0 {
        println!(
            "\n{}",
            "All critical checks passed. You're ready to use nblm.".green()
        );
    }

    std::process::exit(exit_code);
}
