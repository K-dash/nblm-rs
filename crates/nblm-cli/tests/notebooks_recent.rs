mod _helpers;

use _helpers::{cmd::CommonArgs, mock::MockApi};
use predicates::prelude::*;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn notebooks_recent_success() {
    let mock = MockApi::start().await;
    let args = CommonArgs::default();

    mock.stub_notebooks_recent(&args.project_number, &args.location)
        .await;

    let mut cmd = _helpers::cmd::nblm();
    args.with_base_url(&mut cmd, &mock.base_url());
    cmd.args(["notebooks", "recent"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("notebooks"))
        .stdout(predicate::str::contains("nb1"));
}

#[tokio::test]
#[serial]
async fn notebooks_recent_with_page_size() {
    let mock = MockApi::start().await;
    let args = CommonArgs::default();

    mock.stub_notebooks_recent_with_page_size(&args.project_number, &args.location, 10)
        .await;

    let mut cmd = _helpers::cmd::nblm();
    args.with_base_url(&mut cmd, &mock.base_url());
    cmd.args(["notebooks", "recent", "--page-size", "10"]);

    cmd.assert().success();
}
