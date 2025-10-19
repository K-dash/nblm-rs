mod _helpers;

use _helpers::{cmd::CommonArgs, mock::MockApi};
use predicates::prelude::*;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn sources_add_web_url() {
    let mock = MockApi::start().await;
    let args = CommonArgs::default();
    let notebook_id = "test-notebook";

    mock.stub_sources_batch_create(&args.project_number, &args.location, notebook_id)
        .await;

    let mut cmd = _helpers::cmd::nblm();
    args.with_base_url(&mut cmd, &mock.base_url());
    cmd.args([
        "sources",
        "add",
        "--notebook-id",
        notebook_id,
        "--web-url",
        "https://example.com",
        "--web-name",
        "Example",
    ]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("sources"));
}
