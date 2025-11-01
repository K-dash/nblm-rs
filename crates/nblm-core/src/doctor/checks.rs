use colored::Colorize;
use std::env;

/// Status of a diagnostic check
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CheckStatus {
    Pass,
    Warning,
    Error,
}

impl CheckStatus {
    /// Convert status to exit code contribution
    pub fn exit_code(&self) -> i32 {
        match self {
            CheckStatus::Pass => 0,
            CheckStatus::Warning => 1,
            CheckStatus::Error => 2,
        }
    }

    /// Convert status to ASCII marker with aligned label
    pub fn as_marker(&self) -> String {
        let label = match self {
            CheckStatus::Pass => "ok",
            CheckStatus::Warning => "warn",
            CheckStatus::Error => "error",
        };
        let total_width = "error".len() + 2; // include brackets
        format!("{:>width$}", format!("[{}]", label), width = total_width)
    }

    /// Convert status to colored marker using the colored crate
    pub fn as_marker_colored(&self) -> String {
        let marker = self.as_marker();
        match self {
            CheckStatus::Pass => marker.green(),
            CheckStatus::Warning => marker.yellow(),
            CheckStatus::Error => marker.red(),
        }
        .to_string()
    }
}

/// Result of a single diagnostic check
#[derive(Debug, Clone)]
pub struct CheckResult {
    pub name: String,
    pub status: CheckStatus,
    pub message: String,
    pub suggestion: Option<String>,
}

impl CheckResult {
    pub fn new(name: impl Into<String>, status: CheckStatus, message: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            status,
            message: message.into(),
            suggestion: None,
        }
    }

    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestion = Some(suggestion.into());
        self
    }

    /// Format check result for display
    pub fn format(&self) -> String {
        self.format_with_marker(self.status.as_marker())
    }

    /// Format check result for display with colored markers
    pub fn format_colored(&self) -> String {
        self.format_with_marker(self.status.as_marker_colored())
    }

    fn format_with_marker(&self, marker: String) -> String {
        let mut output = format!("{} {}", marker, self.message);
        if let Some(suggestion) = &self.suggestion {
            output.push_str(&format!("\n       Suggestion: {}", suggestion));
        }
        output
    }
}

/// Summary of all diagnostic checks
#[derive(Debug)]
pub struct DiagnosticsSummary {
    pub checks: Vec<CheckResult>,
}

impl DiagnosticsSummary {
    pub fn new(checks: Vec<CheckResult>) -> Self {
        Self { checks }
    }

    /// Calculate the overall exit code
    pub fn exit_code(&self) -> i32 {
        self.checks
            .iter()
            .map(|check| check.status.exit_code())
            .max()
            .unwrap_or(0)
    }

    /// Count checks by status
    pub fn count_by_status(&self, status: CheckStatus) -> usize {
        self.checks
            .iter()
            .filter(|check| check.status == status)
            .count()
    }

    /// Format summary for display
    pub fn format_summary(&self) -> String {
        let total = self.checks.len();
        let failed =
            self.count_by_status(CheckStatus::Error) + self.count_by_status(CheckStatus::Warning);

        if failed == 0 {
            format!("\nSummary: All {} checks passed.", total)
        } else {
            format!(
                "\nSummary: {} checks failing out of {}. See above for details.",
                failed, total
            )
        }
    }

    /// Format summary for display with color
    pub fn format_summary_colored(&self) -> String {
        let total = self.checks.len();
        let failed =
            self.count_by_status(CheckStatus::Error) + self.count_by_status(CheckStatus::Warning);

        if failed == 0 {
            format!(
                "\n{}",
                format!("Summary: All {} checks passed.", total).green()
            )
        } else {
            format!(
                "\n{}",
                format!(
                    "Summary: {} checks failing out of {}. See above for details.",
                    failed, total
                )
                .yellow()
            )
        }
    }
}

/// Configuration for an environment variable check
pub struct EnvVarCheck {
    pub name: &'static str,
    pub required: bool,
    pub suggestion: &'static str,
}

/// Static configuration table for environment variable checks
const ENV_VAR_CHECKS: &[EnvVarCheck] = &[
    EnvVarCheck {
        name: "NBLM_PROJECT_NUMBER",
        required: true,
        suggestion: "export NBLM_PROJECT_NUMBER=<your-project-number>",
    },
    EnvVarCheck {
        name: "NBLM_ENDPOINT_LOCATION",
        required: false,
        suggestion: "export NBLM_ENDPOINT_LOCATION=us-central1",
    },
    EnvVarCheck {
        name: "NBLM_LOCATION",
        required: false,
        suggestion: "export NBLM_LOCATION=us-central1",
    },
    EnvVarCheck {
        name: "NBLM_ACCESS_TOKEN",
        required: false,
        suggestion: "export NBLM_ACCESS_TOKEN=$(gcloud auth print-access-token)",
    },
];

/// Check a single environment variable
fn check_env_var(config: &EnvVarCheck) -> CheckResult {
    match env::var(config.name) {
        Ok(value) if !value.is_empty() => CheckResult::new(
            format!("env_var_{}", config.name.to_lowercase()),
            CheckStatus::Pass,
            format!("{}={}", config.name, value),
        ),
        Ok(_) | Err(env::VarError::NotPresent) => {
            let status = if config.required {
                CheckStatus::Error
            } else {
                CheckStatus::Warning
            };
            CheckResult::new(
                format!("env_var_{}", config.name.to_lowercase()),
                status,
                format!("{} missing", config.name),
            )
            .with_suggestion(config.suggestion)
        }
        Err(env::VarError::NotUnicode(_)) => CheckResult::new(
            format!("env_var_{}", config.name.to_lowercase()),
            CheckStatus::Error,
            format!("{} contains invalid UTF-8", config.name),
        ),
    }
}

/// Run all environment variable checks
pub fn check_environment_variables() -> Vec<CheckResult> {
    ENV_VAR_CHECKS.iter().map(check_env_var).collect()
}

/// Configuration for a command availability check
pub struct CommandCheck {
    pub name: &'static str,
    pub command: &'static str,
    pub required: bool,
    pub suggestion: &'static str,
}

/// Static configuration table for command checks
const COMMAND_CHECKS: &[CommandCheck] = &[CommandCheck {
    name: "gcloud",
    command: "gcloud",
    required: false,
    suggestion: "Install Google Cloud CLI: https://cloud.google.com/sdk/docs/install",
}];

/// Check if a command is available in PATH
fn check_command(config: &CommandCheck) -> CheckResult {
    let status = std::process::Command::new(config.command)
        .arg("--version")
        .output();

    match status {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout);
            let version_line = version.lines().next().unwrap_or("").trim();
            CheckResult::new(
                format!("command_{}", config.name),
                CheckStatus::Pass,
                format!("{} is installed ({})", config.name, version_line),
            )
        }
        _ => {
            let status = if config.required {
                CheckStatus::Error
            } else {
                CheckStatus::Warning
            };
            CheckResult::new(
                format!("command_{}", config.name),
                status,
                format!("{} command not found", config.name),
            )
            .with_suggestion(config.suggestion)
        }
    }
}

/// Run all command availability checks
pub fn check_commands() -> Vec<CheckResult> {
    COMMAND_CHECKS.iter().map(check_command).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_status_markers() {
        assert_eq!(CheckStatus::Pass.as_marker(), "   [ok]");
        assert_eq!(CheckStatus::Warning.as_marker(), " [warn]");
        assert_eq!(CheckStatus::Error.as_marker(), "[error]");
    }

    #[test]
    fn test_check_status_colored_markers() {
        // Force colored output in tests
        colored::control::set_override(true);

        // Verify colored markers contain ANSI escape codes
        let ok = CheckStatus::Pass.as_marker_colored();
        assert!(ok.contains("\x1b["));
        assert!(ok.contains("[ok]"));

        let warn = CheckStatus::Warning.as_marker_colored();
        assert!(warn.contains("\x1b["));
        assert!(warn.contains("[warn]"));

        let err = CheckStatus::Error.as_marker_colored();
        assert!(err.contains("\x1b["));
        assert!(err.contains("[error]"));

        // Reset override
        colored::control::unset_override();
    }

    #[test]
    fn test_check_status_exit_codes() {
        assert_eq!(CheckStatus::Pass.exit_code(), 0);
        assert_eq!(CheckStatus::Warning.exit_code(), 1);
        assert_eq!(CheckStatus::Error.exit_code(), 2);
    }

    #[test]
    fn test_check_result_format() {
        let result = CheckResult::new("test", CheckStatus::Pass, "Test passed");
        assert_eq!(result.format(), "   [ok] Test passed");

        let result_with_suggestion = CheckResult::new("test", CheckStatus::Warning, "Test warning")
            .with_suggestion("Try this fix");
        assert!(result_with_suggestion.format().contains("Suggestion:"));
    }

    #[test]
    fn test_check_result_format_colored() {
        // Force colored output in tests
        colored::control::set_override(true);

        let result = CheckResult::new("test", CheckStatus::Pass, "Test passed");
        let colored = result.format_colored();
        assert!(colored.contains("\x1b["));
        assert!(colored.contains("Test passed"));
        assert!(colored.ends_with("Test passed"));

        // Reset override
        colored::control::unset_override();
    }

    #[test]
    fn test_diagnostics_summary_exit_code() {
        let summary = DiagnosticsSummary::new(vec![
            CheckResult::new("test1", CheckStatus::Pass, "Pass"),
            CheckResult::new("test2", CheckStatus::Pass, "Pass"),
        ]);
        assert_eq!(summary.exit_code(), 0);

        let summary = DiagnosticsSummary::new(vec![
            CheckResult::new("test1", CheckStatus::Pass, "Pass"),
            CheckResult::new("test2", CheckStatus::Warning, "Warning"),
        ]);
        assert_eq!(summary.exit_code(), 1);

        let summary = DiagnosticsSummary::new(vec![
            CheckResult::new("test1", CheckStatus::Pass, "Pass"),
            CheckResult::new("test2", CheckStatus::Error, "Error"),
        ]);
        assert_eq!(summary.exit_code(), 2);
    }

    #[test]
    fn test_check_env_var_present() {
        env::set_var("TEST_VAR", "test_value");
        let config = EnvVarCheck {
            name: "TEST_VAR",
            required: true,
            suggestion: "export TEST_VAR=value",
        };
        let result = check_env_var(&config);
        assert_eq!(result.status, CheckStatus::Pass);
        assert!(result.message.contains("test_value"));
        env::remove_var("TEST_VAR");
    }

    #[test]
    fn test_check_env_var_missing_required() {
        env::remove_var("MISSING_VAR");
        let config = EnvVarCheck {
            name: "MISSING_VAR",
            required: true,
            suggestion: "export MISSING_VAR=value",
        };
        let result = check_env_var(&config);
        assert_eq!(result.status, CheckStatus::Error);
        assert!(result.message.contains("missing"));
        assert!(result.suggestion.is_some());
    }

    #[test]
    fn test_check_env_var_missing_optional() {
        env::remove_var("OPTIONAL_VAR");
        let config = EnvVarCheck {
            name: "OPTIONAL_VAR",
            required: false,
            suggestion: "export OPTIONAL_VAR=value",
        };
        let result = check_env_var(&config);
        assert_eq!(result.status, CheckStatus::Warning);
        assert!(result.message.contains("missing"));
    }

    #[test]
    fn test_check_command_not_found() {
        let config = CommandCheck {
            name: "nonexistent_command_xyz",
            command: "nonexistent_command_xyz",
            required: false,
            suggestion: "Install the command",
        };
        let result = check_command(&config);
        assert_eq!(result.status, CheckStatus::Warning);
        assert!(result.message.contains("not found"));
        assert!(result.suggestion.is_some());
    }

    #[test]
    fn test_check_command_required_not_found() {
        let config = CommandCheck {
            name: "nonexistent_required",
            command: "nonexistent_required",
            required: true,
            suggestion: "Install the command",
        };
        let result = check_command(&config);
        assert_eq!(result.status, CheckStatus::Error);
        assert!(result.message.contains("not found"));
    }
}
