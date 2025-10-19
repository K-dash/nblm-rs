mod _helpers;

use _helpers::{cmd::CommonArgs, mock::MockApi};
use predicates::prelude::*;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn audio_delete_success() {
    let mock = MockApi::start().await;
    let args = CommonArgs::default();
    let notebook_id = "test-notebook-id";

    mock.stub_audio_delete(&args.project_number, &args.location, notebook_id)
        .await;

    let mut cmd = _helpers::cmd::nblm();
    args.with_base_url(&mut cmd, &mock.base_url());
    cmd.args(["audio", "delete", "--notebook-id", notebook_id]);

    cmd.assert().success().stdout(predicate::str::contains(
        "Audio overview deleted successfully",
    ));
}
