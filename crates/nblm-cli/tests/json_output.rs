mod _helpers;

use _helpers::{cmd::CommonArgs, mock::MockApi};
use serial_test::serial;

#[tokio::test]
#[serial]
async fn notebooks_create_json_output() {
    let mock = MockApi::start().await;
    let args = CommonArgs::default();

    mock.stub_notebooks_create(&args.project_number, &args.location, "JSON Test")
        .await;

    let mut cmd = _helpers::cmd::nblm();
    args.with_base_url(&mut cmd, &mock.base_url());
    cmd.args(["--json", "notebooks", "create", "--title", "JSON Test"]);

    let output = cmd.assert().success().get_output().stdout.clone();
    let json_output: serde_json::Value =
        serde_json::from_slice(&output).expect("valid JSON output");

    insta::assert_json_snapshot!(json_output, @r###"
    {
      "notebook": {
        "name": "projects/123456/locations/global/notebooks/test-notebook-id",
        "notebookId": "test-notebook-id",
        "title": "JSON Test"
      },
      "notebook_id": "test-notebook-id"
    }
    "###);
}

#[tokio::test]
#[serial]
async fn notebooks_recent_json_output() {
    let mock = MockApi::start().await;
    let args = CommonArgs::default();

    mock.stub_notebooks_recent(&args.project_number, &args.location)
        .await;

    let mut cmd = _helpers::cmd::nblm();
    args.with_base_url(&mut cmd, &mock.base_url());
    cmd.args(["--json", "notebooks", "recent"]);

    let output = cmd.assert().success().get_output().stdout.clone();
    let json_output: serde_json::Value =
        serde_json::from_slice(&output).expect("valid JSON output");

    insta::assert_json_snapshot!(json_output, @r###"
    {
      "notebooks": [
        {
          "name": "projects/123456/locations/global/notebooks/nb1",
          "notebookId": "nb1",
          "title": "Test Notebook 1"
        }
      ]
    }
    "###);
}

#[tokio::test]
#[serial]
async fn sources_add_json_output() {
    let mock = MockApi::start().await;
    let args = CommonArgs::default();
    let notebook_id = "test-notebook";

    mock.stub_sources_batch_create(&args.project_number, &args.location, notebook_id)
        .await;

    let mut cmd = _helpers::cmd::nblm();
    args.with_base_url(&mut cmd, &mock.base_url());
    cmd.args([
        "--json",
        "sources",
        "add",
        "--notebook-id",
        notebook_id,
        "--web-url",
        "https://example.com",
        "--web-name",
        "Example",
    ]);

    let output = cmd.assert().success().get_output().stdout.clone();
    let json_output: serde_json::Value =
        serde_json::from_slice(&output).expect("valid JSON output");

    insta::assert_json_snapshot!(json_output, @r#"
    {
      "error_count": null,
      "notebook_id": "test-notebook",
      "sources": [
        {
          "displayName": "Test Source",
          "name": "projects/123456/locations/global/notebooks/test-notebook/sources/src1"
        }
      ]
    }
    "#);
}
