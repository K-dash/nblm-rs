mod _helpers;

use _helpers::{cmd::CommonArgs, mock::MockApi};
use predicates::prelude::*;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn notebooks_delete_single() {
    let mock = MockApi::start().await;
    let args = CommonArgs::default();

    mock.stub_notebooks_batch_delete(&args.project_number, &args.location)
        .await;

    let notebook_name = format!(
        "projects/{}/locations/{}/notebooks/test-nb-123",
        args.project_number, args.location
    );

    let mut cmd = _helpers::cmd::nblm();
    args.with_base_url(&mut cmd, &mock.base_url());
    cmd.args([
        "notebooks",
        "delete",
        "--notebook-name",
        &notebook_name,
    ]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Deleted 1 notebook(s) successfully"));
}

#[tokio::test]
#[serial]
async fn notebooks_delete_multiple() {
    let mock = MockApi::start().await;
    let args = CommonArgs::default();

    // Mock will be called twice (sequential deletion due to API limitation)
    mock.stub_notebooks_batch_delete(&args.project_number, &args.location)
        .await;

    let notebook_name1 = format!(
        "projects/{}/locations/{}/notebooks/test-nb-1",
        args.project_number, args.location
    );
    let notebook_name2 = format!(
        "projects/{}/locations/{}/notebooks/test-nb-2",
        args.project_number, args.location
    );

    let mut cmd = _helpers::cmd::nblm();
    args.with_base_url(&mut cmd, &mock.base_url());
    cmd.args([
        "notebooks",
        "delete",
        "--notebook-name",
        &notebook_name1,
        "--notebook-name",
        &notebook_name2,
    ]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Deleted 2 notebook(s) successfully"));
}
