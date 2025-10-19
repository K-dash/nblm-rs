mod _helpers;

use _helpers::{cmd::CommonArgs, mock::MockApi};
use predicates::prelude::*;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn retry_429_then_success() {
    let mock = MockApi::start().await;
    let args = CommonArgs::default();

    // Configure mock to return 429 twice, then 200
    mock.stub_notebooks_recent_429_then_success(&args.project_number, &args.location, 2)
        .await;

    let mut cmd = _helpers::cmd::nblm();
    args.with_base_url(&mut cmd, &mock.base_url());
    cmd.args(["notebooks", "recent"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("notebooks"));
}

#[tokio::test]
#[serial]
async fn retry_429_exhausted() {
    let mock = MockApi::start().await;
    let args = CommonArgs::default();

    // Configure mock to always return 429
    mock.stub_notebooks_recent_persistent_429(&args.project_number, &args.location)
        .await;

    let mut cmd = _helpers::cmd::nblm();
    args.with_base_url(&mut cmd, &mock.base_url());
    cmd.args(["notebooks", "recent"]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Too Many Requests"));
}

#[tokio::test]
#[serial]
async fn retry_401_then_success() {
    let mock = MockApi::start().await;
    let args = CommonArgs::default();

    // Configure mock to return 401 once, then 200 (simulates token refresh)
    mock.stub_notebooks_recent_401_then_success(&args.project_number, &args.location)
        .await;

    let mut cmd = _helpers::cmd::nblm();
    args.with_base_url(&mut cmd, &mock.base_url());
    cmd.args(["notebooks", "recent"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("notebooks"));
}
