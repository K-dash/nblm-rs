mod _helpers;

use predicates::function;
use predicates::prelude::*;
use rstest::rstest;
use serde_json::json;
use serial_test::serial;
use tokio::runtime::Runtime;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn setup_drive_tokeninfo() -> (Runtime, MockServer, String) {
    let runtime = Runtime::new().expect("runtime");
    let server = runtime.block_on(async {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/tokeninfo"))
            .and(query_param("access_token", "test-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "scope": "https://www.googleapis.com/auth/drive.file"
            })))
            .mount(&server)
            .await;
        server
    });
    let endpoint = format!("{}/tokeninfo", server.uri());
    (runtime, server, endpoint)
}

#[test]
#[serial]
fn doctor_all_env_vars_present() {
    let (_runtime, _server, tokeninfo) = setup_drive_tokeninfo();
    let mut cmd = _helpers::cmd::nblm();
    let common = _helpers::cmd::CommonArgs::default();
    common.apply(&mut cmd);
    cmd.env("NBLM_PROJECT_NUMBER", "224840249322");
    cmd.env("NBLM_ENDPOINT_LOCATION", "us-central1");
    cmd.env("NBLM_LOCATION", "global");
    cmd.env("NBLM_ACCESS_TOKEN", "test-token");
    cmd.env("NBLM_TOKENINFO_ENDPOINT", &tokeninfo);
    cmd.arg("doctor");

    let assert = cmd.assert();
    // Exit code may be 0 or 1 depending on whether gcloud is installed
    // 0 = all checks passed, 1 = warnings present (e.g., gcloud not found)
    assert
        .code(function::function(|code: &i32| *code == 0 || *code == 1))
        .stdout(predicate::str::contains(
            "Running NotebookLM environment diagnostics",
        ))
        .stdout(predicate::str::contains(
            "   [ok] NBLM_PROJECT_NUMBER=224840249322",
        ))
        .stdout(predicate::str::contains(
            "   [ok] NBLM_ENDPOINT_LOCATION=us-central1",
        ))
        .stdout(predicate::str::contains("   [ok] NBLM_LOCATION=global"))
        .stdout(predicate::str::contains(
            "   [ok] NBLM_ACCESS_TOKEN set (value hidden)",
        ))
        .stdout(predicate::str::contains(
            "   [ok] NBLM_ACCESS_TOKEN grants Google Drive access",
        ));
}

enum ProjectNumberValue {
    Missing,
    Empty,
}

#[rstest]
#[case::missing(ProjectNumberValue::Missing)]
#[case::empty(ProjectNumberValue::Empty)]
#[test]
#[serial]
fn doctor_missing_or_empty_required_env_var(#[case] value: ProjectNumberValue) {
    let (_runtime, _server, tokeninfo) = setup_drive_tokeninfo();
    let mut cmd = _helpers::cmd::nblm();
    let common = _helpers::cmd::CommonArgs::default();
    common.apply(&mut cmd);

    match value {
        ProjectNumberValue::Missing => cmd.env_remove("NBLM_PROJECT_NUMBER"),
        ProjectNumberValue::Empty => cmd.env("NBLM_PROJECT_NUMBER", ""),
    };

    cmd.env("NBLM_ENDPOINT_LOCATION", "us-central1");
    cmd.env("NBLM_LOCATION", "global");
    cmd.env("NBLM_ACCESS_TOKEN", "test-token");
    cmd.env("NBLM_TOKENINFO_ENDPOINT", &tokeninfo);
    cmd.arg("doctor");

    let assert = cmd.assert();
    assert
        .code(2) // Error exit code
        .stdout(predicate::str::contains(
            "[error] NBLM_PROJECT_NUMBER missing",
        ))
        .stdout(predicate::str::contains(
            "Suggestion: export NBLM_PROJECT_NUMBER=<your-project-number>",
        ));
}

#[test]
#[serial]
fn doctor_missing_optional_env_vars() {
    let mut cmd = _helpers::cmd::nblm();
    let common = _helpers::cmd::CommonArgs::default();
    common.apply(&mut cmd);
    cmd.env("NBLM_PROJECT_NUMBER", "224840249322");
    cmd.env_remove("NBLM_ENDPOINT_LOCATION");
    cmd.env_remove("NBLM_LOCATION");
    cmd.env_remove("NBLM_ACCESS_TOKEN");
    cmd.arg("doctor");

    let assert = cmd.assert();
    assert
        .code(1) // Warning exit code
        .stdout(predicate::str::contains(
            "   [ok] NBLM_PROJECT_NUMBER=224840249322",
        ))
        .stdout(predicate::str::contains(
            " [warn] NBLM_ENDPOINT_LOCATION missing",
        ))
        .stdout(predicate::str::contains(
            "Suggestion: export NBLM_ENDPOINT_LOCATION=us-central1",
        ))
        .stdout(predicate::str::contains(" [warn] NBLM_LOCATION missing"))
        .stdout(predicate::str::contains(
            "Suggestion: export NBLM_LOCATION=us-central1",
        ))
        .stdout(predicate::str::contains(
            " [warn] NBLM_ACCESS_TOKEN missing",
        ));
}

#[test]
#[serial]
fn doctor_all_env_vars_missing() {
    let mut cmd = _helpers::cmd::nblm();
    let common = _helpers::cmd::CommonArgs::default();
    common.apply(&mut cmd);
    cmd.env_remove("NBLM_PROJECT_NUMBER");
    cmd.env_remove("NBLM_ENDPOINT_LOCATION");
    cmd.env_remove("NBLM_LOCATION");
    cmd.env_remove("NBLM_ACCESS_TOKEN");
    cmd.arg("doctor");

    let assert = cmd.assert();
    assert
        .code(2) // Error exit code (highest priority)
        .stdout(predicate::str::contains(
            "[error] NBLM_PROJECT_NUMBER missing",
        ))
        .stdout(predicate::str::contains(
            " [warn] NBLM_ENDPOINT_LOCATION missing",
        ))
        .stdout(predicate::str::contains(" [warn] NBLM_LOCATION missing"))
        .stdout(predicate::str::contains(
            " [warn] NBLM_ACCESS_TOKEN missing",
        ));
}
