use assert_cmd::Command;

/// Create a Command for the nblm CLI binary with common setup
pub fn nblm() -> Command {
    Command::cargo_bin("nblm").unwrap()
}

/// Common arguments for all CLI tests
pub struct CommonArgs {
    pub project_number: String,
    pub location: String,
    pub endpoint_location: String,
    pub auth: String,
    pub token: String,
}

impl Default for CommonArgs {
    fn default() -> Self {
        Self {
            project_number: "123456".to_string(),
            location: "global".to_string(),
            endpoint_location: "us".to_string(),
            auth: "env".to_string(),
            token: "DUMMY_TOKEN".to_string(),
        }
    }
}

impl CommonArgs {
    pub fn apply(&self, cmd: &mut Command) {
        cmd.args([
            "--project-number",
            &self.project_number,
            "--location",
            &self.location,
            "--endpoint-location",
            &self.endpoint_location,
            "--auth",
            &self.auth,
            "--token",
            &self.token,
        ]);
    }

    pub fn with_base_url(&self, cmd: &mut Command, base_url: &str) {
        self.apply(cmd);
        cmd.args(["--base-url", base_url]);
        // Enable fast retry for tests
        cmd.env("NBLM_RETRY_FAST", "1");
    }
}
