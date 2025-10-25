use serde_json::json;
use wiremock::matchers::{header, header_exists, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[allow(dead_code)]
pub struct MockApi {
    pub server: MockServer,
}

#[allow(dead_code)]
impl MockApi {
    pub async fn start() -> Self {
        let server = MockServer::start().await;
        Self { server }
    }

    pub fn base_url(&self) -> String {
        format!("{}/v1alpha", self.server.uri())
    }

    /// Stub for POST /v1alpha/projects/{project}/locations/{location}/notebooks
    pub async fn stub_notebooks_create(&self, project: &str, location: &str, title: &str) {
        let path_str = format!(
            "/v1alpha/projects/{}/locations/{}/notebooks",
            project, location
        );
        let response = json!({
            "name": format!("projects/{}/locations/{}/notebooks/test-notebook-id", project, location),
            "notebookId": "test-notebook-id",
            "title": title,
        });

        Mock::given(method("POST"))
            .and(path(path_str))
            .and(header("authorization", "Bearer DUMMY_TOKEN"))
            .and(header_exists("user-agent"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response))
            .mount(&self.server)
            .await;
    }

    /// Stub for GET /v1alpha/projects/{project}/locations/{location}/notebooks:listRecentlyViewed
    pub async fn stub_notebooks_recent(&self, project: &str, location: &str) {
        let path_str = format!(
            "/v1alpha/projects/{}/locations/{}/notebooks:listRecentlyViewed",
            project, location
        );
        let response = json!({
            "notebooks": [
                {
                    "name": format!("projects/{}/locations/{}/notebooks/nb1", project, location),
                    "notebookId": "nb1",
                    "title": "Test Notebook 1"
                }
            ],
            "nextPageToken": null
        });

        Mock::given(method("GET"))
            .and(path(path_str))
            .and(header("authorization", "Bearer DUMMY_TOKEN"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response))
            .mount(&self.server)
            .await;
    }

    /// Stub for notebooks:listRecentlyViewed with page size query param
    pub async fn stub_notebooks_recent_with_page_size(
        &self,
        project: &str,
        location: &str,
        page_size: u32,
    ) {
        let path_str = format!(
            "/v1alpha/projects/{}/locations/{}/notebooks:listRecentlyViewed",
            project, location
        );
        let response = json!({
            "notebooks": [],
            "nextPageToken": null
        });

        Mock::given(method("GET"))
            .and(path(path_str))
            .and(query_param("pageSize", page_size.to_string()))
            .and(header("authorization", "Bearer DUMMY_TOKEN"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response))
            .mount(&self.server)
            .await;
    }

    /// Stub for POST /v1alpha/projects/{project}/locations/{location}/notebooks/{notebook_id}/sources:batchCreate
    pub async fn stub_sources_batch_create(
        &self,
        project: &str,
        location: &str,
        notebook_id: &str,
    ) {
        let path_str = format!(
            "/v1alpha/projects/{}/locations/{}/notebooks/{}/sources:batchCreate",
            project, location, notebook_id
        );
        let response = json!({
            "sources": [
                {
                    "name": format!("projects/{}/locations/{}/notebooks/{}/sources/src1", project, location, notebook_id),
                    "displayName": "Test Source"
                }
            ]
        });

        Mock::given(method("POST"))
            .and(path(path_str))
            .and(header("authorization", "Bearer DUMMY_TOKEN"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response))
            .mount(&self.server)
            .await;
    }

    pub async fn stub_sources_upload_file(
        &self,
        project: &str,
        location: &str,
        notebook_id: &str,
        source_id: &str,
    ) {
        let path_str = format!(
            "/upload/v1alpha/projects/{}/locations/{}/notebooks/{}/sources:uploadFile",
            project, location, notebook_id
        );
        let response = json!({
            "sourceId": {
                "id": format!(
                    "projects/{}/locations/{}/notebooks/{}/sources/{}",
                    project, location, notebook_id, source_id
                )
            }
        });

        Mock::given(method("POST"))
            .and(path(path_str))
            .and(query_param("uploadType", "media"))
            .and(header("authorization", "Bearer DUMMY_TOKEN"))
            .and(header("x-goog-upload-protocol", "raw"))
            .and(header_exists("content-type"))
            .and(header_exists("x-goog-upload-file-name"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response))
            .mount(&self.server)
            .await;
    }

    /// Stub for POST /v1alpha/projects/{project}/locations/{location}/notebooks/{notebook_id}:share
    pub async fn stub_notebook_share(&self, project: &str, location: &str, notebook_id: &str) {
        let path_str = format!(
            "/v1alpha/projects/{}/locations/{}/notebooks/{}:share",
            project, location, notebook_id
        );
        let response = json!({
            "granted": 1
        });

        Mock::given(method("POST"))
            .and(path(path_str))
            .and(header("authorization", "Bearer DUMMY_TOKEN"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response))
            .mount(&self.server)
            .await;
    }

    /// Stub for 429 Too Many Requests with Retry-After header, then success
    pub async fn stub_notebooks_recent_429_then_success(
        &self,
        project: &str,
        location: &str,
        retry_count: usize,
    ) {
        let path_str = format!(
            "/v1alpha/projects/{}/locations/{}/notebooks:listRecentlyViewed",
            project, location
        );

        // First N requests return 429
        for _ in 0..retry_count {
            Mock::given(method("GET"))
                .and(path(&path_str))
                .and(header("authorization", "Bearer DUMMY_TOKEN"))
                .respond_with(
                    ResponseTemplate::new(429)
                        .insert_header("Retry-After", "0")
                        .set_body_json(json!({
                            "error": {
                                "message": "Too Many Requests"
                            }
                        })),
                )
                .up_to_n_times(1)
                .mount(&self.server)
                .await;
        }

        // Final request succeeds
        let response = json!({
            "notebooks": [],
            "nextPageToken": null
        });

        Mock::given(method("GET"))
            .and(path(path_str))
            .and(header("authorization", "Bearer DUMMY_TOKEN"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response))
            .mount(&self.server)
            .await;
    }

    /// Stub for 401 Unauthorized then success (token refresh scenario)
    pub async fn stub_notebooks_recent_401_then_success(&self, project: &str, location: &str) {
        let path_str = format!(
            "/v1alpha/projects/{}/locations/{}/notebooks:listRecentlyViewed",
            project, location
        );

        // First request returns 401
        Mock::given(method("GET"))
            .and(path(&path_str))
            .and(header("authorization", "Bearer DUMMY_TOKEN"))
            .respond_with(ResponseTemplate::new(401).set_body_json(json!({
                "error": {
                    "message": "Unauthorized"
                }
            })))
            .up_to_n_times(1)
            .mount(&self.server)
            .await;

        // Subsequent requests succeed
        let response = json!({
            "notebooks": [],
            "nextPageToken": null
        });

        Mock::given(method("GET"))
            .and(path(path_str))
            .and(header("authorization", "Bearer DUMMY_TOKEN"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response))
            .mount(&self.server)
            .await;
    }

    /// Stub for persistent 429 (will exhaust retries)
    pub async fn stub_notebooks_recent_persistent_429(&self, project: &str, location: &str) {
        let path_str = format!(
            "/v1alpha/projects/{}/locations/{}/notebooks:listRecentlyViewed",
            project, location
        );

        Mock::given(method("GET"))
            .and(path(path_str))
            .and(header("authorization", "Bearer DUMMY_TOKEN"))
            .respond_with(
                ResponseTemplate::new(429)
                    .insert_header("Retry-After", "0")
                    .set_body_json(json!({
                        "error": {
                            "message": "Too Many Requests"
                        }
                    })),
            )
            .mount(&self.server)
            .await;
    }

    /// Stub for POST /v1alpha/projects/{project}/locations/{location}/notebooks:batchDelete
    pub async fn stub_notebooks_batch_delete(&self, project: &str, location: &str) {
        let path_str = format!(
            "/v1alpha/projects/{}/locations/{}/notebooks:batchDelete",
            project, location
        );

        Mock::given(method("POST"))
            .and(path(path_str))
            .and(header("authorization", "Bearer DUMMY_TOKEN"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
            .mount(&self.server)
            .await;
    }

    /// Stub for POST /v1alpha/projects/{project}/locations/{location}/notebooks/{notebook_id}/sources:batchDelete
    pub async fn stub_sources_batch_delete(
        &self,
        project: &str,
        location: &str,
        notebook_id: &str,
    ) {
        let path_str = format!(
            "/v1alpha/projects/{}/locations/{}/notebooks/{}/sources:batchDelete",
            project, location, notebook_id
        );

        Mock::given(method("POST"))
            .and(path(path_str))
            .and(header("authorization", "Bearer DUMMY_TOKEN"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
            .mount(&self.server)
            .await;
    }

    /// Stub for DELETE /v1alpha/projects/{project}/locations/{location}/notebooks/{notebook_id}/audioOverviews/default
    pub async fn stub_audio_delete(&self, project: &str, location: &str, notebook_id: &str) {
        let path_str = format!(
            "/v1alpha/projects/{}/locations/{}/notebooks/{}/audioOverviews/default",
            project, location, notebook_id
        );

        Mock::given(method("DELETE"))
            .and(path(path_str))
            .and(header("authorization", "Bearer DUMMY_TOKEN"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
            .mount(&self.server)
            .await;
    }
}


/// Stub for POST /api/v1/audio
    /// This stub responds with a 201 Created status and an AudioOverviewResponse body.
    /// It expects the request body to contain the audio's name.
    pub async fn stub_audio_create(&self, audio_name: &str) -> MockGuard {
        let expected_request_body = json!({
            "name": audio_name,
        });
        let response_body = json!({
            "id": "test-id",
            "name": audio_name,
            "status": "Created"
        });

        Mock::given(method("POST"))
            .and(path("/api/v1/audio"))
            .and(header("authorization", "Bearer DUMMY_TOKEN"))
            .and(header_exists("user-agent"))
            .and(body_json(expected_request_body))
            .respond_with(ResponseTemplate::new(201).set_body_json(response_body))
            .mount_as_scoped(&self.server)
            .await
    }