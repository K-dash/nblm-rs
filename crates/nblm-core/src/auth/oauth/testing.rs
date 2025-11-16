#[cfg(test)]
pub mod fake {
    use serde_json::json;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    /// Helper struct that spins up a fake OAuth server for tests.
    pub struct FakeOAuthServer {
        mock_server: MockServer,
    }

    impl FakeOAuthServer {
        /// Start the fake server with default token and revocation endpoints.
        pub async fn start() -> Self {
            let mock_server = MockServer::start().await;

            Mock::given(method("POST"))
                .and(path("/token"))
                .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                    "access_token": "fake_access_token",
                    "refresh_token": "fake_refresh_token",
                    "token_type": "Bearer",
                    "expires_in": 3600
                })))
                .mount(&mock_server)
                .await;

            Mock::given(method("POST"))
                .and(path("/revoke"))
                .respond_with(ResponseTemplate::new(200))
                .mount(&mock_server)
                .await;

            Self { mock_server }
        }

        pub fn token_endpoint(&self) -> String {
            format!("{}/token", self.mock_server.uri())
        }

        pub fn revoke_endpoint(&self) -> String {
            format!("{}/revoke", self.mock_server.uri())
        }

        pub fn base_uri(&self) -> String {
            self.mock_server.uri()
        }
    }
}
