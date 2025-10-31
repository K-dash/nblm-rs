#[cfg(test)]
mod tests {
    use super::*;
    use crate::args::{AuthMethod, GlobalArgs, ProfileArg};
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
    fn user_oauth_returns_placeholder_when_flag_enabled() {
        let _guard = EnvGuard::new(nblm_core::PROFILE_EXPERIMENT_FLAG);
        env::set_var(nblm_core::PROFILE_EXPERIMENT_FLAG, "1");
        let args = make_args(AuthMethod::UserOauth);
        let provider = build_token_provider(&args).expect("expected provider");
        assert_eq!(provider.kind(), ProviderKind::UserOauth);
        let rt = tokio::runtime::Runtime::new().expect("runtime");
        let err = rt.block_on(provider.access_token()).unwrap_err();
        assert!(format!("{err}").contains("not implemented"));
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
    fn pending_oauth_provider_returns_user_oauth_kind() {
        let provider = PendingOAuthProvider::new();
        assert_eq!(provider.kind(), ProviderKind::UserOauth);
    }
}
use std::{env, sync::Arc};

use anyhow::Result;
use async_trait::async_trait;
use nblm_core::auth::{
    EnvTokenProvider, GcloudTokenProvider, ProviderKind, StaticTokenProvider, TokenProvider,
};
use nblm_core::{Error as CoreError, Result as CoreResult};

use crate::args::{AuthMethod, GlobalArgs};

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
        AuthMethod::UserOauth => Arc::new(PendingOAuthProvider::new()),
    })
}

fn build_gcloud_provider() -> Result<GcloudTokenProvider> {
    let binary = env::var("NBLM_GCLOUD_PATH").unwrap_or_else(|_| "gcloud".to_string());
    Ok(GcloudTokenProvider::new(binary))
}

fn profile_experiment_enabled() -> bool {
    match env::var(nblm_core::PROFILE_EXPERIMENT_FLAG) {
        Ok(value) => matches!(value.as_str(), "1" | "true" | "TRUE" | "yes" | "YES"),
        Err(_) => false,
    }
}

fn auth_method_label(method: AuthMethod) -> &'static str {
    match method {
        AuthMethod::Gcloud => "gcloud",
        AuthMethod::Env => "env",
        AuthMethod::UserOauth => "user-oauth",
    }
}

#[derive(Debug, Clone)]
struct PendingOAuthProvider;

impl PendingOAuthProvider {
    fn new() -> Self {
        Self
    }
}

#[async_trait]
impl TokenProvider for PendingOAuthProvider {
    async fn access_token(&self) -> CoreResult<String> {
        Err(CoreError::TokenProvider(
            "user OAuth flow is not implemented yet".to_string(),
        ))
    }

    fn kind(&self) -> ProviderKind {
        ProviderKind::UserOauth
    }
}
