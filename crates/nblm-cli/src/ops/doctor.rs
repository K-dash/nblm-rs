use anyhow::Result;
use clap::Args;
use nblm_core::doctor::{check_environment_variables, DiagnosticsSummary};

#[derive(Args)]
pub struct DoctorArgs {}

pub async fn run(_args: DoctorArgs) -> Result<()> {
    println!("Running NotebookLM environment diagnostics...\n");

    // Run environment variable checks
    let env_checks = check_environment_variables();

    // Print individual check results
    for check in &env_checks {
        println!("{}", check.format());
    }

    // Print summary
    let summary = DiagnosticsSummary::new(env_checks);
    println!("{}", summary.format_summary());

    // Determine exit behavior
    let exit_code = summary.exit_code();
    if exit_code == 0 {
        println!("\nAll critical checks passed. You're ready to use nblm.");
    }

    std::process::exit(exit_code);
}
