mod _helpers;

use _helpers::{cmd::CommonArgs, mock::MockApi};
use predicates::prelude::*;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn sources_delete_single() {
    let mock = MockApi::start().await;
    let args = CommonArgs::default();
    let notebook_id = "test-notebook-id";

    mock.stub_sources_batch_delete(&args.project_number, &args.location, notebook_id)
        .await;

    let source_name = format!(
        "projects/{}/locations/{}/notebooks/{}/sources/src-123",
        args.project_number, args.location, notebook_id
    );

    let mut cmd = _helpers::cmd::nblm();
    args.with_base_url(&mut cmd, &mock.base_url());
    cmd.args([
        "sources",
        "delete",
        "--notebook-id",
        notebook_id,
        "--source-name",
        &source_name,
    ]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Deleted 1 source(s) successfully"));
}

#[tokio::test]
#[serial]
async fn sources_delete_multiple() {
    let mock = MockApi::start().await;
    let args = CommonArgs::default();
    let notebook_id = "test-notebook-id";

    mock.stub_sources_batch_delete(&args.project_number, &args.location, notebook_id)
        .await;

    let source_name1 = format!(
        "projects/{}/locations/{}/notebooks/{}/sources/src-1",
        args.project_number, args.location, notebook_id
    );
    let source_name2 = format!(
        "projects/{}/locations/{}/notebooks/{}/sources/src-2",
        args.project_number, args.location, notebook_id
    );

    let mut cmd = _helpers::cmd::nblm();
    args.with_base_url(&mut cmd, &mock.base_url());
    cmd.args([
        "sources",
        "delete",
        "--notebook-id",
        notebook_id,
        "--source-name",
        &source_name1,
        "--source-name",
        &source_name2,
    ]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Deleted 2 source(s) successfully"));
}
