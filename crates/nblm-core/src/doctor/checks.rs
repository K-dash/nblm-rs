use colored::Colorize;
use std::env;

use crate::auth::{ensure_drive_scope, EnvTokenProvider};
use crate::error::Error;

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
    pub show_value: bool,
}

/// Static configuration table for environment variable checks
const ENV_VAR_CHECKS: &[EnvVarCheck] = &[
    EnvVarCheck {
        name: "NBLM_PROJECT_NUMBER",
        required: true,
        suggestion: "export NBLM_PROJECT_NUMBER=<your-project-number>",
        show_value: true,
    },
    EnvVarCheck {
        name: "NBLM_ENDPOINT_LOCATION",
        required: false,
        suggestion: "export NBLM_ENDPOINT_LOCATION=us  # or 'eu' or 'global'",
        show_value: true,
    },
    EnvVarCheck {
        name: "NBLM_LOCATION",
        required: false,
        suggestion: "export NBLM_LOCATION=global",
        show_value: true,
    },
    EnvVarCheck {
        name: "NBLM_ACCESS_TOKEN",
        required: false,
        suggestion: "export NBLM_ACCESS_TOKEN=$(gcloud auth print-access-token)",
        show_value: false,
    },
];

/// Check a single environment variable
fn check_env_var(config: &EnvVarCheck) -> CheckResult {
    match env::var(config.name) {
        Ok(value) if !value.is_empty() => {
            let message = if config.show_value {
                format!("{}={}", config.name, value)
            } else {
                format!("{} set (value hidden)", config.name)
            };
            CheckResult::new(
                format!("env_var_{}", config.name.to_lowercase()),
                CheckStatus::Pass,
                message,
            )
        }
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

/// Validate that `NBLM_ACCESS_TOKEN`, when present, grants Google Drive access.
pub async fn check_drive_access_token() -> Vec<CheckResult> {
    match env::var("NBLM_ACCESS_TOKEN") {
        Ok(value) if !value.trim().is_empty() => {
            let provider = EnvTokenProvider::new("NBLM_ACCESS_TOKEN");
            match ensure_drive_scope(&provider).await {
                Ok(_) => vec![CheckResult::new(
                    "drive_scope_nblm_access_token",
                    CheckStatus::Pass,
                    "NBLM_ACCESS_TOKEN grants Google Drive access",
                )],
                Err(Error::TokenProvider(message)) => {
                    if message.contains("missing the required drive.file scope") {
                        vec![CheckResult::new(
                            "drive_scope_nblm_access_token",
                            CheckStatus::Warning,
                            "NBLM_ACCESS_TOKEN lacks Google Drive scope",
                        )
                        .with_suggestion(
                            "Run `gcloud auth login --enable-gdrive-access` and refresh NBLM_ACCESS_TOKEN",
                        )]
                    } else {
                        vec![CheckResult::new(
                            "drive_scope_nblm_access_token",
                            CheckStatus::Warning,
                            format!(
                                "Could not confirm Google Drive scope for NBLM_ACCESS_TOKEN: {}",
                                message
                            ),
                        )]
                    }
                }
                Err(err) => vec![CheckResult::new(
                    "drive_scope_nblm_access_token",
                    CheckStatus::Warning,
                    format!(
                        "Could not confirm Google Drive scope for NBLM_ACCESS_TOKEN: {}",
                        err
                    ),
                )],
            }
        }
        _ => Vec::new(),
    }
}

/// Check NotebookLM API connectivity by calling list_recently_viewed
pub async fn check_api_connectivity() -> Vec<CheckResult> {
    use crate::auth::GcloudTokenProvider;
    use crate::client::NblmClient;
    use crate::env::EnvironmentConfig;
    use std::sync::Arc;

    // Skip if required environment variables are missing
    let project_number = match env::var("NBLM_PROJECT_NUMBER") {
        Ok(val) if !val.is_empty() => val,
        _ => {
            // Don't report error here - env var check already handles this
            return Vec::new();
        }
    };

    let location = env::var("NBLM_LOCATION").unwrap_or_else(|_| "global".to_string());
    let endpoint_location =
        env::var("NBLM_ENDPOINT_LOCATION").unwrap_or_else(|_| "global".to_string());

    // Try to construct environment config
    let env_config =
        match EnvironmentConfig::enterprise(project_number, location, endpoint_location) {
            Ok(config) => config,
            Err(err) => {
                return vec![CheckResult::new(
                "api_connectivity",
                CheckStatus::Error,
                format!("Cannot construct environment config: {}", err),
            )
            .with_suggestion(
                "Ensure NBLM_PROJECT_NUMBER, NBLM_LOCATION, and NBLM_ENDPOINT_LOCATION are valid",
            )];
            }
        };

    // Create token provider - only use gcloud if NBLM_ACCESS_TOKEN is not set
    let token_provider: Arc<dyn crate::auth::TokenProvider> =
        match env::var("NBLM_ACCESS_TOKEN").ok().filter(|s| !s.is_empty()) {
            Some(_) => Arc::new(crate::auth::EnvTokenProvider::new("NBLM_ACCESS_TOKEN")),
            None => {
                // Skip API check if gcloud is not available to avoid interactive prompts
                if !is_gcloud_available() {
                    return Vec::new();
                }
                Arc::new(GcloudTokenProvider::new("gcloud"))
            }
        };

    // Try to create client
    let client = match NblmClient::new(token_provider, env_config) {
        Ok(client) => client,
        Err(err) => {
            return vec![CheckResult::new(
                "api_connectivity",
                CheckStatus::Error,
                format!("Failed to create API client: {}", err),
            )
            .with_suggestion("Check your environment configuration and credentials")];
        }
    };

    // Try to call list_recently_viewed
    match client.list_recently_viewed(Some(1)).await {
        Ok(_) => vec![CheckResult::new(
            "api_connectivity",
            CheckStatus::Pass,
            "Successfully connected to NotebookLM API",
        )],
        Err(err) => {
            let err_string = err.to_string();
            let (status, message, suggestion) = categorize_api_error(&err_string);

            vec![CheckResult::new("api_connectivity", status, message).with_suggestion(suggestion)]
        }
    }
}

/// Check if gcloud command is available
fn is_gcloud_available() -> bool {
    std::process::Command::new("gcloud")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Categorize API errors and provide actionable suggestions
fn categorize_api_error(error: &str) -> (CheckStatus, String, &'static str) {
    let error_lower = error.to_lowercase();

    match () {
        _ if error_lower.contains("401") || error_lower.contains("unauthorized") => (
            CheckStatus::Error,
            "Authentication failed (401 Unauthorized)".to_string(),
            "Run `gcloud auth login` or `gcloud auth application-default login`",
        ),
        _ if error_lower.contains("403") || error_lower.contains("permission denied") => (
            CheckStatus::Error,
            "Permission denied (403 Forbidden)".to_string(),
            "Ensure your account has NotebookLM API access and required IAM roles (e.g., aiplatform.user)",
        ),
        _ if error_lower.contains("404") || error_lower.contains("not found") => (
            CheckStatus::Error,
            "Resource not found (404)".to_string(),
            "Verify NBLM_PROJECT_NUMBER is correct and the project has NotebookLM enabled",
        ),
        _
            if error_lower.contains("timeout")
                || error_lower.contains("connection")
                || error_lower.contains("network") =>
        {
            (
                CheckStatus::Error,
                format!("Network error: {}", error),
                "Check your internet connection and firewall settings",
            )
        }
        _ => (
            CheckStatus::Error,
            format!("API error: {}", error),
            "Check the error message above and your configuration",
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    struct EnvGuard {
        key: &'static str,
        original: Option<String>,
    }

    impl EnvGuard {
        fn new(key: &'static str) -> Self {
            let original = env::var(key).ok();
            Self { key, original }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            if let Some(value) = &self.original {
                env::set_var(self.key, value);
            } else {
                env::remove_var(self.key);
            }
        }
    }

    #[test]
    fn test_check_status_markers() {
        assert_eq!(CheckStatus::Pass.as_marker(), "   [ok]");
        assert_eq!(CheckStatus::Warning.as_marker(), " [warn]");
        assert_eq!(CheckStatus::Error.as_marker(), "[error]");
    }

    #[test]
    fn test_check_status_colored_markers() {
        // Force colored output in tests (OK if target ignores it)
        colored::control::set_override(true);

        // Verify colored markers still include status labels
        let ok = CheckStatus::Pass.as_marker_colored();
        assert!(ok.contains("[ok]"));

        let warn = CheckStatus::Warning.as_marker_colored();
        assert!(warn.contains("[warn]"));

        let err = CheckStatus::Error.as_marker_colored();
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
            show_value: true,
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
            show_value: true,
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
            show_value: true,
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

    #[tokio::test]
    #[serial]
    async fn test_drive_access_check_passes_with_valid_scope() {
        let token_guard = EnvGuard::new("NBLM_ACCESS_TOKEN");
        let endpoint_guard = EnvGuard::new("NBLM_TOKENINFO_ENDPOINT");

        env::set_var("NBLM_ACCESS_TOKEN", "test-token");

        let server = MockServer::start().await;
        let tokeninfo_url = format!("{}/tokeninfo", server.uri());
        env::set_var("NBLM_TOKENINFO_ENDPOINT", &tokeninfo_url);

        Mock::given(method("GET"))
            .and(path("/tokeninfo"))
            .and(query_param("access_token", "test-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "scope": "https://www.googleapis.com/auth/drive.file"
            })))
            .expect(1)
            .mount(&server)
            .await;

        let results = check_drive_access_token().await;
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].status, CheckStatus::Pass);
        assert!(results[0].message.contains("grants Google Drive access"));

        drop(token_guard);
        drop(endpoint_guard);
    }

    #[tokio::test]
    #[serial]
    async fn test_drive_access_check_reports_missing_scope() {
        let token_guard = EnvGuard::new("NBLM_ACCESS_TOKEN");
        let endpoint_guard = EnvGuard::new("NBLM_TOKENINFO_ENDPOINT");

        env::set_var("NBLM_ACCESS_TOKEN", "test-token");

        let server = MockServer::start().await;
        let tokeninfo_url = format!("{}/tokeninfo", server.uri());
        env::set_var("NBLM_TOKENINFO_ENDPOINT", &tokeninfo_url);

        Mock::given(method("GET"))
            .and(path("/tokeninfo"))
            .and(query_param("access_token", "test-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "scope": "https://www.googleapis.com/auth/cloud-platform"
            })))
            .expect(1)
            .mount(&server)
            .await;

        let results = check_drive_access_token().await;
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].status, CheckStatus::Warning);
        assert!(results[0].message.contains("lacks Google Drive scope"));

        drop(token_guard);
        drop(endpoint_guard);
    }

    #[test]
    fn test_categorize_api_error_401() {
        let (status, message, suggestion) = categorize_api_error("401 Unauthorized");
        assert_eq!(status, CheckStatus::Error);
        assert!(message.contains("Authentication failed"));
        assert!(suggestion.contains("gcloud auth login"));
    }

    #[test]
    fn test_categorize_api_error_403() {
        let (status, message, suggestion) = categorize_api_error("403 Permission denied");
        assert_eq!(status, CheckStatus::Error);
        assert!(message.contains("Permission denied"));
        assert!(suggestion.contains("IAM roles"));
    }

    #[test]
    fn test_categorize_api_error_404() {
        let (status, message, suggestion) = categorize_api_error("404 Not found");
        assert_eq!(status, CheckStatus::Error);
        assert!(message.contains("Resource not found"));
        assert!(suggestion.contains("NBLM_PROJECT_NUMBER"));
    }

    #[test]
    fn test_categorize_api_error_timeout() {
        let (status, message, suggestion) = categorize_api_error("Connection timeout");
        assert_eq!(status, CheckStatus::Error);
        assert!(message.contains("Network error"));
        assert!(suggestion.contains("internet connection"));
    }

    #[test]
    fn test_categorize_api_error_generic() {
        let (status, message, suggestion) = categorize_api_error("Some random error");
        assert_eq!(status, CheckStatus::Error);
        assert!(message.contains("API error"));
        assert!(suggestion.contains("configuration"));
    }

    #[tokio::test]
    #[serial]
    async fn test_check_api_connectivity_missing_project_number() {
        let _guard = EnvGuard::new("NBLM_PROJECT_NUMBER");
        env::remove_var("NBLM_PROJECT_NUMBER");

        let results = check_api_connectivity().await;
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_is_gcloud_available() {
        // This test just verifies that is_gcloud_available() doesn't panic
        // The actual result depends on whether gcloud is installed
        let _ = is_gcloud_available();
    }

    #[test]
    fn test_diagnostics_summary_count_by_status() {
        let summary = DiagnosticsSummary::new(vec![
            CheckResult::new("test1", CheckStatus::Pass, "Pass"),
            CheckResult::new("test2", CheckStatus::Warning, "Warning"),
            CheckResult::new("test3", CheckStatus::Error, "Error"),
            CheckResult::new("test4", CheckStatus::Pass, "Pass"),
        ]);

        assert_eq!(summary.count_by_status(CheckStatus::Pass), 2);
        assert_eq!(summary.count_by_status(CheckStatus::Warning), 1);
        assert_eq!(summary.count_by_status(CheckStatus::Error), 1);
    }

    #[test]
    fn test_diagnostics_summary_format() {
        let summary = DiagnosticsSummary::new(vec![
            CheckResult::new("test1", CheckStatus::Pass, "Pass"),
            CheckResult::new("test2", CheckStatus::Pass, "Pass"),
        ]);
        let formatted = summary.format_summary();
        assert!(formatted.contains("All 2 checks passed"));

        let summary_with_failures = DiagnosticsSummary::new(vec![
            CheckResult::new("test1", CheckStatus::Pass, "Pass"),
            CheckResult::new("test2", CheckStatus::Warning, "Warning"),
        ]);
        let formatted_fail = summary_with_failures.format_summary();
        assert!(formatted_fail.contains("1 checks failing out of 2"));
    }

    #[test]
    fn test_check_result_with_suggestion() {
        let result = CheckResult::new("test", CheckStatus::Warning, "Something wrong")
            .with_suggestion("Fix it this way");

        assert_eq!(result.suggestion, Some("Fix it this way".to_string()));
        assert!(result.format().contains("Suggestion: Fix it this way"));
    }

    #[test]
    fn test_check_env_var_hidden_value() {
        env::set_var("SECRET_VAR", "secret_value");
        let config = EnvVarCheck {
            name: "SECRET_VAR",
            required: true,
            suggestion: "export SECRET_VAR=value",
            show_value: false,
        };
        let result = check_env_var(&config);
        assert_eq!(result.status, CheckStatus::Pass);
        assert!(result.message.contains("value hidden"));
        assert!(!result.message.contains("secret_value"));
        env::remove_var("SECRET_VAR");
    }

    #[test]
    fn test_check_environment_variables_integration() {
        let results = check_environment_variables();
        // Should return results for all configured env vars
        assert_eq!(results.len(), ENV_VAR_CHECKS.len());
    }

    #[test]
    fn test_check_commands_integration() {
        let results = check_commands();
        // Should return results for all configured commands
        assert_eq!(results.len(), COMMAND_CHECKS.len());
    }
}
