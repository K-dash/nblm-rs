mod _helpers;

use std::io::Write;

use _helpers::{cmd::CommonArgs, mock::MockApi};
use predicates::prelude::*;
use serial_test::serial;
use tempfile::NamedTempFile;

#[tokio::test]
#[serial]
async fn sources_upload_file() {
    let mock = MockApi::start().await;
    let args = CommonArgs::default();
    let notebook_id = "notebook-upload";

    mock.stub_sources_upload_file(
        &args.project_number,
        &args.location,
        notebook_id,
        "source-upload",
    )
    .await;

    let mut temp_file = NamedTempFile::new().expect("temp file");
    writeln!(temp_file, "hello world").expect("write temp file");
    let file_path = temp_file.into_temp_path();
    let file_str = file_path.to_str().expect("path to str").to_string();

    let mut cmd = _helpers::cmd::nblm();
    args.with_base_url(&mut cmd, &mock.base_url());
    cmd.args([
        "sources",
        "upload",
        "--notebook-id",
        notebook_id,
        "--file",
        &file_str,
        "--content-type",
        "text/plain",
        "--display-name",
        "Sample.txt",
    ]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Created source:"));
}
