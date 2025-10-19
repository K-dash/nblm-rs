mod _helpers;

use _helpers::{cmd::CommonArgs, mock::MockApi};
use predicates::prelude::*;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn share_add_email() {
    let mock = MockApi::start().await;
    let args = CommonArgs::default();
    let notebook_id = "test-notebook";

    mock.stub_notebook_share(&args.project_number, &args.location, notebook_id)
        .await;

    let mut cmd = _helpers::cmd::nblm();
    args.with_base_url(&mut cmd, &mock.base_url());
    cmd.args([
        "share",
        "add",
        "--notebook-id",
        notebook_id,
        "--email",
        "user@example.com",
    ]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("granted"));
}
