use std::time::Duration;

use anyhow::{anyhow, bail, Result};
use tracing_subscriber::EnvFilter;

use nblm_core::{
    ApiProfile, EnvironmentConfig, NblmClient, ProfileParams, RetryConfig, PROFILE_EXPERIMENT_FLAG,
};

use crate::args::{Cli, Command, GlobalArgs};
use crate::ops::{audio, doctor, notebooks, share, sources};
use crate::util::auth::build_token_provider;

pub struct NblmApp {
    cli: Cli,
    client: NblmClient,
}

impl NblmApp {
    pub fn new(cli: Cli) -> Result<Self> {
        init_logging();

        if cli.global.debug_http {
            std::env::set_var("NBLM_DEBUG_HTTP", "1");
        }

        let provider = build_token_provider(&cli.global)?;
        let profile: ApiProfile = cli.global.profile.into();
        if profile.requires_experimental_flag() && !profile_experiment_enabled() {
            bail!(
                "profile '{}' is experimental and not yet available. Set {}=1 to enable experimental profile support.",
                profile.as_str(),
                PROFILE_EXPERIMENT_FLAG
            );
        }

        // TODO(profile-docs): Document profile selection once additional SKUs are available publicly.
        let params = resolve_profile_params(&cli.global, profile)?;
        let environment = EnvironmentConfig::from_profile(profile, params)?;
        let mut client = NblmClient::new(provider, environment)?;

        if let Some(timeout) = cli.global.timeout {
            client = client.with_timeout(timeout);
        }

        // Use fast retry config for tests to avoid slow retries
        let retry_config = if std::env::var_os("NBLM_RETRY_FAST").is_some() {
            RetryConfig::default()
                .with_min_delay(Duration::from_millis(5))
                .with_max_delay(Duration::from_millis(20))
                .with_max_retries(2)
        } else {
            RetryConfig::default()
        };
        client = client.with_retry_config(retry_config);

        if let Some(base) = &cli.global.base_url {
            client = client.with_base_url(base)?;
        }

        Ok(Self { cli, client })
    }

    pub async fn run(self) -> Result<()> {
        let json_mode = self.cli.global.json;
        match self.cli.command {
            Command::Notebooks(cmd) => notebooks::run(cmd, &self.client, json_mode).await,
            Command::Sources(cmd) => sources::run(cmd, &self.client, json_mode).await,
            Command::Audio(cmd) => audio::run(cmd, &self.client, json_mode).await,
            Command::Share(cmd) => share::run(cmd, &self.client, json_mode).await,
            Command::Doctor(cmd) => doctor::run(cmd).await,
        }
    }
}

fn resolve_profile_params(args: &GlobalArgs, profile: ApiProfile) -> Result<ProfileParams> {
    match profile {
        ApiProfile::Enterprise => {
            let project_number = args
                .project_number
                .as_ref()
                .map(|value| value.trim())
                .filter(|value| !value.is_empty())
                .ok_or_else(|| {
                    anyhow!(
                        "enterprise profile requires --project-number or the NBLM_PROJECT_NUMBER environment variable"
                    )
                })?
                .to_string();

            let location = if args.location.trim().is_empty() {
                "global".to_string()
            } else {
                args.location.clone()
            };

            let endpoint_location = if args.endpoint_location.trim().is_empty() {
                "global".to_string()
            } else {
                args.endpoint_location.clone()
            };

            Ok(ProfileParams::enterprise(
                project_number,
                location,
                endpoint_location,
            ))
        }
        ApiProfile::Personal => Ok(ProfileParams::personal::<String>(None)),
        ApiProfile::Workspace => Ok(ProfileParams::workspace::<String, String>(None, None)),
    }
}

fn profile_experiment_enabled() -> bool {
    match std::env::var(PROFILE_EXPERIMENT_FLAG) {
        Ok(value) => matches!(value.as_str(), "1" | "true" | "TRUE" | "yes" | "YES"),
        Err(_) => false,
    }
}

fn init_logging() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let _ = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .with_writer(std::io::stderr)
        .try_init();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::args::ProfileArg;
    use serial_test::serial;

    // Test helpers
    struct TestEnvGuard {
        key: &'static str,
        original: Option<String>,
    }

    impl TestEnvGuard {
        fn new(key: &'static str) -> Self {
            let original = std::env::var(key).ok();
            Self { key, original }
        }
    }

    impl Drop for TestEnvGuard {
        fn drop(&mut self) {
            match &self.original {
                Some(value) => std::env::set_var(self.key, value),
                None => std::env::remove_var(self.key),
            }
        }
    }

    fn make_test_args(
        project_number: Option<String>,
        location: &str,
        endpoint_location: &str,
        profile: ProfileArg,
    ) -> GlobalArgs {
        GlobalArgs {
            project_number,
            location: location.to_string(),
            endpoint_location: endpoint_location.to_string(),
            profile,
            auth: crate::args::AuthMethod::Gcloud,
            token: None,
            json: false,
            debug_http: false,
            timeout: None,
            env_token: None,
            base_url: None,
        }
    }

    // Tests for profile_experiment_enabled()
    #[test]
    #[serial]
    fn profile_experiment_enabled_recognizes_truthy_values() {
        let _guard = TestEnvGuard::new(PROFILE_EXPERIMENT_FLAG);
        for value in ["1", "true", "TRUE", "yes", "YES"] {
            std::env::set_var(PROFILE_EXPERIMENT_FLAG, value);
            assert!(
                profile_experiment_enabled(),
                "expected '{}' to enable experiment",
                value
            );
        }
    }

    #[test]
    #[serial]
    fn profile_experiment_enabled_rejects_other_values() {
        let _guard = TestEnvGuard::new(PROFILE_EXPERIMENT_FLAG);
        for value in ["0", "false", "no", "maybe", ""] {
            std::env::set_var(PROFILE_EXPERIMENT_FLAG, value);
            assert!(
                !profile_experiment_enabled(),
                "expected '{}' to not enable experiment",
                value
            );
        }
        std::env::remove_var(PROFILE_EXPERIMENT_FLAG);
        assert!(!profile_experiment_enabled());
    }

    // Tests for ApiProfile::requires_experimental_flag()
    #[test]
    #[serial]
    fn experimental_profiles_require_flag_personal() {
        let _guard = TestEnvGuard::new(PROFILE_EXPERIMENT_FLAG);
        std::env::remove_var(PROFILE_EXPERIMENT_FLAG);

        assert!(ApiProfile::Personal.requires_experimental_flag());
        assert!(!profile_experiment_enabled());
    }

    #[test]
    #[serial]
    fn experimental_profiles_require_flag_workspace() {
        let _guard = TestEnvGuard::new(PROFILE_EXPERIMENT_FLAG);
        std::env::remove_var(PROFILE_EXPERIMENT_FLAG);

        assert!(ApiProfile::Workspace.requires_experimental_flag());
        assert!(!profile_experiment_enabled());
    }

    #[test]
    #[serial]
    fn enterprise_profile_does_not_require_flag() {
        let _guard = TestEnvGuard::new(PROFILE_EXPERIMENT_FLAG);
        std::env::remove_var(PROFILE_EXPERIMENT_FLAG);

        assert!(!ApiProfile::Enterprise.requires_experimental_flag());
    }

    // Tests for resolve_profile_params() - Enterprise profile
    #[test]
    fn resolve_profile_params_enterprise_requires_project_number() {
        let args = make_test_args(None, "global", "us", ProfileArg::Enterprise);
        let err = resolve_profile_params(&args, ApiProfile::Enterprise).unwrap_err();
        assert!(format!("{err}").contains("requires --project-number"));
    }

    #[test]
    fn resolve_profile_params_enterprise_rejects_empty_project_number() {
        let args = make_test_args(
            Some("  ".to_string()),
            "global",
            "us",
            ProfileArg::Enterprise,
        );
        let err = resolve_profile_params(&args, ApiProfile::Enterprise).unwrap_err();
        assert!(format!("{err}").contains("requires --project-number"));
    }

    #[test]
    fn resolve_profile_params_enterprise_accepts_valid_project_number() {
        let args = make_test_args(
            Some("123456".to_string()),
            "us-central1",
            "us",
            ProfileArg::Enterprise,
        );
        let params = resolve_profile_params(&args, ApiProfile::Enterprise).unwrap();
        assert_eq!(params.expected_profile(), ApiProfile::Enterprise);
    }

    #[test]
    fn resolve_profile_params_enterprise_defaults_empty_location_to_global() {
        let args = make_test_args(
            Some("123456".to_string()),
            "  ",
            "us",
            ProfileArg::Enterprise,
        );
        let params = resolve_profile_params(&args, ApiProfile::Enterprise).unwrap();
        // We can verify this by checking that EnvironmentConfig accepts it
        let env = EnvironmentConfig::from_profile(ApiProfile::Enterprise, params).unwrap();
        assert!(env.parent_path().contains("locations/global"));
    }

    #[test]
    fn resolve_profile_params_enterprise_defaults_empty_endpoint_to_global() {
        let args = make_test_args(
            Some("123456".to_string()),
            "global",
            "  ",
            ProfileArg::Enterprise,
        );
        let params = resolve_profile_params(&args, ApiProfile::Enterprise).unwrap();
        let env = EnvironmentConfig::from_profile(ApiProfile::Enterprise, params).unwrap();
        assert!(env.base_url().contains("global-discoveryengine"));
    }

    // Tests for resolve_profile_params() - Personal and Workspace profiles
    #[test]
    fn resolve_profile_params_personal_returns_personal_params() {
        let args = make_test_args(None, "global", "us", ProfileArg::Personal);
        let params = resolve_profile_params(&args, ApiProfile::Personal).unwrap();
        assert_eq!(params.expected_profile(), ApiProfile::Personal);
    }

    #[test]
    fn resolve_profile_params_workspace_returns_workspace_params() {
        let args = make_test_args(None, "global", "us", ProfileArg::Workspace);
        let params = resolve_profile_params(&args, ApiProfile::Workspace).unwrap();
        assert_eq!(params.expected_profile(), ApiProfile::Workspace);
    }
}
