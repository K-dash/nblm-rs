mod _helpers;

use _helpers::{cmd::CommonArgs, mock::MockApi};
use predicates::prelude::*;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn notebooks_create_with_base_url() {
    let mock = MockApi::start().await;
    let args = CommonArgs::default();

    mock.stub_notebooks_create(&args.project_number, &args.location, "Hello")
        .await;

    let mut cmd = _helpers::cmd::nblm();
    args.with_base_url(&mut cmd, &mock.base_url());
    cmd.args(["notebooks", "create", "--title", "Hello"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test-notebook-id"))
        .stdout(predicate::str::contains("Hello"));
}

#[tokio::test]
#[serial]
async fn notebooks_create_with_env_base_url() {
    let mock = MockApi::start().await;
    let args = CommonArgs::default();

    mock.stub_notebooks_create(&args.project_number, &args.location, "World")
        .await;

    let mut cmd = _helpers::cmd::nblm();
    cmd.env("NBLM_BASE_URL", mock.base_url());
    args.apply(&mut cmd);
    cmd.args(["notebooks", "create", "--title", "World"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test-notebook-id"));
}
