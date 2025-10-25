mod _helpers;

use _helpers::{cmd::CommonArgs, mock::MockApi};
use predicates::prelude::*;
use serial_test::serial;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
#[serial]
async fn sources_get_success() {
    let mock = MockApi::start().await;
    let args = CommonArgs::default();
    let notebook_id = "nb123";
    let source_id = "src456";

    // Mock response for get source
    let source_response = serde_json::json!({
        "name": format!(
            "projects/{}/locations/{}/notebooks/{}/sources/{}",
            args.project_number, args.location, notebook_id, source_id
        ),
        "title": "Test Source",
        "sourceId": {
            "id": source_id
        },
        "metadata": {
            "sourceAddedTimestamp": "2025-10-25T10:00:00Z",
            "wordCount": 1500
        },
        "settings": {
            "status": "ACTIVE"
        }
    });

    let path_str = format!(
        "/v1alpha/projects/{}/locations/{}/notebooks/{}/sources/{}",
        args.project_number, args.location, notebook_id, source_id
    );

    Mock::given(method("GET"))
        .and(path(path_str))
        .and(header("authorization", format!("Bearer {}", args.token)))
        .respond_with(ResponseTemplate::new(200).set_body_json(source_response))
        .mount(&mock.server)
        .await;

    let mut cmd = _helpers::cmd::nblm();
    args.with_base_url(&mut cmd, &mock.base_url());
    cmd.args([
        "sources",
        "get",
        "--notebook-id",
        notebook_id,
        "--source-id",
        source_id,
    ]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Source Details:"))
        .stdout(predicate::str::contains(format!(
            "Name: projects/{}/locations/{}/notebooks/{}/sources/{}",
            args.project_number, args.location, notebook_id, source_id
        )))
        .stdout(predicate::str::contains("Title: Test Source"))
        .stdout(predicate::str::contains(format!(
            "Source ID: {}",
            source_id
        )))
        .stdout(predicate::str::contains("Added: 2025-10-25T10:00:00Z"))
        .stdout(predicate::str::contains("Word Count: 1500"))
        .stdout(predicate::str::contains("Status: ACTIVE"));
}

#[tokio::test]
#[serial]
async fn sources_get_json_output() {
    let mock = MockApi::start().await;
    let args = CommonArgs::default();
    let notebook_id = "nb123";
    let source_id = "src456";

    let source_response = serde_json::json!({
        "name": format!(
            "projects/{}/locations/{}/notebooks/{}/sources/{}",
            args.project_number, args.location, notebook_id, source_id
        ),
        "title": "Test Source",
        "sourceId": {
            "id": source_id
        }
    });

    let path_str = format!(
        "/v1alpha/projects/{}/locations/{}/notebooks/{}/sources/{}",
        args.project_number, args.location, notebook_id, source_id
    );

    Mock::given(method("GET"))
        .and(path(path_str))
        .and(header("authorization", format!("Bearer {}", args.token)))
        .respond_with(ResponseTemplate::new(200).set_body_json(source_response))
        .mount(&mock.server)
        .await;

    let mut cmd = _helpers::cmd::nblm();
    args.with_base_url(&mut cmd, &mock.base_url());
    cmd.args([
        "--json",
        "sources",
        "get",
        "--notebook-id",
        notebook_id,
        "--source-id",
        source_id,
    ]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(r#""title": "Test Source""#))
        .stdout(predicate::str::contains(format!(r#""id": "{}"#, source_id)));
}

#[tokio::test]
#[serial]
async fn sources_get_not_found() {
    let mock = MockApi::start().await;
    let args = CommonArgs::default();
    let notebook_id = "nb123";
    let source_id = "nonexistent";

    let path_str = format!(
        "/v1alpha/projects/{}/locations/{}/notebooks/{}/sources/{}",
        args.project_number, args.location, notebook_id, source_id
    );

    Mock::given(method("GET"))
        .and(path(path_str))
        .and(header("authorization", format!("Bearer {}", args.token)))
        .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
            "error": {"code": 404, "message": "Source not found"}
        })))
        .mount(&mock.server)
        .await;

    let mut cmd = _helpers::cmd::nblm();
    args.with_base_url(&mut cmd, &mock.base_url());
    cmd.args([
        "sources",
        "get",
        "--notebook-id",
        notebook_id,
        "--source-id",
        source_id,
    ]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("404"));
}

#[test]
fn sources_get_missing_notebook_id() {
    let args = CommonArgs::default();
    let mut cmd = _helpers::cmd::nblm();
    cmd.args([
        "--project-number",
        &args.project_number,
        "--auth",
        &args.auth,
        "--token",
        &args.token,
    ]);
    cmd.args(["sources", "get", "--source-id", "src123"]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("required arguments"));
}

#[test]
fn sources_get_missing_source_id() {
    let args = CommonArgs::default();
    let mut cmd = _helpers::cmd::nblm();
    cmd.args([
        "--project-number",
        &args.project_number,
        "--auth",
        &args.auth,
        "--token",
        &args.token,
    ]);
    cmd.args(["sources", "get", "--notebook-id", "nb123"]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("required arguments"));
}
