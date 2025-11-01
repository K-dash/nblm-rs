mod _helpers;

use predicates::function;
use predicates::prelude::*;
use serial_test::serial;

#[test]
#[serial]
fn doctor_all_env_vars_present() {
    let mut cmd = _helpers::cmd::nblm();
    let common = _helpers::cmd::CommonArgs::default();
    common.apply(&mut cmd);
    cmd.env("NBLM_PROJECT_NUMBER", "224840249322");
    cmd.env("NBLM_ENDPOINT_LOCATION", "us-central1");
    cmd.env("NBLM_LOCATION", "global");
    cmd.env("NBLM_ACCESS_TOKEN", "test-token");
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
        .stdout(predicate::str::contains("   [ok] NBLM_LOCATION=global"));
}

#[test]
#[serial]
fn doctor_missing_required_env_var() {
    let mut cmd = _helpers::cmd::nblm();
    let common = _helpers::cmd::CommonArgs::default();
    common.apply(&mut cmd);
    cmd.env_remove("NBLM_PROJECT_NUMBER");
    cmd.env("NBLM_ENDPOINT_LOCATION", "us-central1");
    cmd.env("NBLM_LOCATION", "global");
    cmd.env("NBLM_ACCESS_TOKEN", "test-token");
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

#[test]
#[serial]
fn doctor_empty_env_var_treated_as_missing() {
    let mut cmd = _helpers::cmd::nblm();
    let common = _helpers::cmd::CommonArgs::default();
    common.apply(&mut cmd);
    cmd.env("NBLM_PROJECT_NUMBER", ""); // Empty string should be treated as missing
    cmd.env("NBLM_ENDPOINT_LOCATION", "us-central1");
    cmd.env("NBLM_LOCATION", "global");
    cmd.env("NBLM_ACCESS_TOKEN", "test-token");
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
