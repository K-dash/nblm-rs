use std::{env, sync::Arc};

use anyhow::Result;
use nblm_core::auth::{EnvTokenProvider, GcloudTokenProvider, StaticTokenProvider, TokenProvider};
use nblm_core::env::profile_experiment_enabled;

use crate::args::{AuthMethod, GlobalArgs};
use crate::util::oauth_bootstrap::OAuthBootstrapper;

pub fn build_token_provider(args: &GlobalArgs) -> Result<Arc<dyn TokenProvider>> {
    if args.auth.requires_experimental_flag() && !profile_experiment_enabled() {
        anyhow::bail!(
            "auth method '{}' is experimental and not yet available. Set {}=1 to enable experimental auth methods.",
            auth_method_label(args.auth),
            nblm_core::PROFILE_EXPERIMENT_FLAG
        );
    }

    Ok(match args.auth {
        AuthMethod::Gcloud => Arc::new(build_gcloud_provider()?),
        AuthMethod::Env => {
            if let Some(token) = args.token.as_ref().or(args.env_token.as_ref()) {
                Arc::new(StaticTokenProvider::new(token.clone()))
            } else {
                Arc::new(EnvTokenProvider::new("NBLM_ACCESS_TOKEN"))
            }
        }
        AuthMethod::UserOauth => {
            let bootstrapper = OAuthBootstrapper::new()?;
            bootstrapper.bootstrap_provider(args)?
        }
    })
}

fn build_gcloud_provider() -> Result<GcloudTokenProvider> {
    let binary = env::var("NBLM_GCLOUD_PATH").unwrap_or_else(|_| "gcloud".to_string());
    Ok(GcloudTokenProvider::new(binary))
}

fn auth_method_label(method: AuthMethod) -> &'static str {
    match method {
        AuthMethod::Gcloud => "gcloud",
        AuthMethod::Env => "env",
        AuthMethod::UserOauth => "user-oauth",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::args::{AuthMethod, GlobalArgs, ProfileArg};
    use nblm_core::auth::ProviderKind;
    use serial_test::serial;

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

    fn make_args(auth: AuthMethod) -> GlobalArgs {
        GlobalArgs {
            project_number: Some("123456".to_string()),
            location: "global".to_string(),
            endpoint_location: "global".to_string(),
            profile: ProfileArg::Enterprise,
            auth,
            token: Some("token".to_string()),
            json: false,
            debug_http: false,
            timeout: None,
            env_token: Some("token".to_string()),
            base_url: None,
        }
    }

    #[test]
    #[serial]
    fn user_oauth_requires_experiment_flag() {
        let _guard = EnvGuard::new(nblm_core::PROFILE_EXPERIMENT_FLAG);
        env::remove_var(nblm_core::PROFILE_EXPERIMENT_FLAG);
        let args = make_args(AuthMethod::UserOauth);
        let err = match build_token_provider(&args) {
            Ok(_) => panic!("expected experimental auth to fail"),
            Err(err) => err,
        };
        assert!(format!("{err}").contains("experimental"));
    }

    #[test]
    #[serial]
    fn user_oauth_requires_project_number() {
        let _guard_bootstrap = EnvGuard::new("NBLM_OAUTH_DISABLE_BOOTSTRAP");
        let _guard = EnvGuard::new(nblm_core::PROFILE_EXPERIMENT_FLAG);
        env::set_var(nblm_core::PROFILE_EXPERIMENT_FLAG, "1");
        env::set_var("NBLM_OAUTH_CLIENT_ID", "test-client-id");
        env::set_var("NBLM_OAUTH_DISABLE_BOOTSTRAP", "1");
        let mut args = make_args(AuthMethod::UserOauth);
        args.project_number = None;
        match build_token_provider(&args) {
            Ok(_) => panic!("expected error for missing project-number"),
            Err(err) => assert!(format!("{err}").contains("project-number")),
        }
        env::remove_var("NBLM_OAUTH_CLIENT_ID");
    }

    #[test]
    #[serial]
    fn user_oauth_requires_client_id() {
        let _guard_bootstrap = EnvGuard::new("NBLM_OAUTH_DISABLE_BOOTSTRAP");
        let _guard = EnvGuard::new(nblm_core::PROFILE_EXPERIMENT_FLAG);
        env::set_var(nblm_core::PROFILE_EXPERIMENT_FLAG, "1");
        env::remove_var("NBLM_OAUTH_CLIENT_ID");
        env::set_var("NBLM_OAUTH_DISABLE_BOOTSTRAP", "1");
        let args = make_args(AuthMethod::UserOauth);
        match build_token_provider(&args) {
            Ok(_) => panic!("expected error for missing NBLM_OAUTH_CLIENT_ID"),
            Err(err) => assert!(format!("{err}").contains("NBLM_OAUTH_CLIENT_ID")),
        }
    }

    #[test]
    #[serial]
    fn user_oauth_builds_provider_when_configured() {
        let _guard_bootstrap = EnvGuard::new("NBLM_OAUTH_DISABLE_BOOTSTRAP");
        let _guard = EnvGuard::new(nblm_core::PROFILE_EXPERIMENT_FLAG);
        env::set_var(nblm_core::PROFILE_EXPERIMENT_FLAG, "1");
        env::set_var("NBLM_OAUTH_CLIENT_ID", "test-client-id");
        env::set_var("NBLM_OAUTH_DISABLE_BOOTSTRAP", "1");
        let args = make_args(AuthMethod::UserOauth);
        let provider = build_token_provider(&args).expect("expected provider");
        assert_eq!(provider.kind(), ProviderKind::UserOauth);
        env::remove_var("NBLM_OAUTH_CLIENT_ID");
    }

    #[test]
    fn auth_method_label_returns_correct_labels() {
        assert_eq!(auth_method_label(AuthMethod::Gcloud), "gcloud");
        assert_eq!(auth_method_label(AuthMethod::Env), "env");
        assert_eq!(auth_method_label(AuthMethod::UserOauth), "user-oauth");
    }

    #[test]
    fn auth_method_requires_experimental_flag_only_for_user_oauth() {
        assert!(!AuthMethod::Gcloud.requires_experimental_flag());
        assert!(!AuthMethod::Env.requires_experimental_flag());
        assert!(AuthMethod::UserOauth.requires_experimental_flag());
    }

    #[test]
    #[serial_test::serial]
    fn gcloud_auth_works_without_experiment_flag() {
        let _guard = EnvGuard::new(nblm_core::PROFILE_EXPERIMENT_FLAG);
        env::remove_var(nblm_core::PROFILE_EXPERIMENT_FLAG);
        let args = make_args(AuthMethod::Gcloud);
        let provider = build_token_provider(&args).expect("expected provider");
        assert_eq!(provider.kind(), ProviderKind::GcloudOauth);
    }

    #[test]
    #[serial]
    fn env_auth_works_without_experiment_flag() {
        let _guard = EnvGuard::new(nblm_core::PROFILE_EXPERIMENT_FLAG);
        env::remove_var(nblm_core::PROFILE_EXPERIMENT_FLAG);
        env::set_var("NBLM_ACCESS_TOKEN", "test-token");
        let mut args = make_args(AuthMethod::Env);
        args.token = None;
        args.env_token = None;
        let provider = build_token_provider(&args).expect("expected provider");
        assert_eq!(provider.kind(), ProviderKind::EnvAccessToken);
        env::remove_var("NBLM_ACCESS_TOKEN");
    }

    #[test]
    #[serial]
    fn env_auth_with_token_arg_uses_static_provider() {
        let _guard = EnvGuard::new(nblm_core::PROFILE_EXPERIMENT_FLAG);
        env::remove_var(nblm_core::PROFILE_EXPERIMENT_FLAG);
        let args = make_args(AuthMethod::Env);
        let provider = build_token_provider(&args).expect("expected provider");
        assert_eq!(provider.kind(), ProviderKind::StaticToken);
    }

    #[test]
    #[serial]
    fn oauth_bootstrapper_respects_disable_bootstrap_flag() {
        let _guard_client = EnvGuard::new("NBLM_OAUTH_CLIENT_ID");
        let _guard_bootstrap = EnvGuard::new("NBLM_OAUTH_DISABLE_BOOTSTRAP");
        let _guard_flag = EnvGuard::new(nblm_core::PROFILE_EXPERIMENT_FLAG);

        env::set_var(nblm_core::PROFILE_EXPERIMENT_FLAG, "1");
        env::set_var("NBLM_OAUTH_CLIENT_ID", "test-client-id");
        env::set_var("NBLM_OAUTH_DISABLE_BOOTSTRAP", "1");

        let args = make_args(AuthMethod::UserOauth);
        let result = build_token_provider(&args);

        // Should succeed without trying to start browser flow
        assert!(result.is_ok());
    }
}
