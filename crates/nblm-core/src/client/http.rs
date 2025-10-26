use std::borrow::Cow;
use std::sync::{Arc, OnceLock};

use bytes::Bytes;
use reqwest::{header::HeaderMap, Client, Method, RequestBuilder, StatusCode, Url};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::auth::TokenProvider;
use crate::error::{Error, Result};

use super::retry::Retryer;

/// HTTP layer implementation for NBLM API requests
pub(crate) struct HttpClient {
    pub(super) client: Client,
    pub(super) token_provider: Arc<dyn TokenProvider>,
    pub(super) retryer: Retryer,
    pub(super) user_project: Option<String>,
}

impl HttpClient {
    pub fn new(
        client: Client,
        token_provider: Arc<dyn TokenProvider>,
        retryer: Retryer,
        user_project: Option<String>,
    ) -> Self {
        Self {
            client,
            token_provider,
            retryer,
            user_project,
        }
    }

    pub async fn request_json<B, R>(&self, method: Method, url: Url, body: Option<&B>) -> Result<R>
    where
        B: Serialize + ?Sized,
        R: DeserializeOwned,
    {
        let body_bytes = match body {
            Some(value) => Some(Bytes::from(serde_json::to_vec(value).map_err(Error::Json)?)),
            None => None,
        };
        let body_bytes = Arc::new(body_bytes);
        let builder_fn = {
            let body_bytes = Arc::clone(&body_bytes);
            move |mut builder: RequestBuilder| -> Result<RequestBuilder> {
                if let Some(bytes) = &*body_bytes {
                    builder = builder
                        .header(reqwest::header::CONTENT_TYPE, "application/json")
                        .body(bytes.clone());
                }
                Ok(builder)
            }
        };

        let method_for_parse = method.clone();
        let url_for_parse = url.clone();
        let response = self.execute_with_builder(method, url, builder_fn).await?;

        parse_json_response(&method_for_parse, &url_for_parse, response).await
    }

    pub async fn request_binary<R>(
        &self,
        method: Method,
        url: Url,
        headers: HeaderMap,
        body: Bytes,
    ) -> Result<R>
    where
        R: DeserializeOwned,
    {
        let header_entries: Vec<(reqwest::header::HeaderName, reqwest::header::HeaderValue)> =
            headers
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect();
        let header_entries = Arc::new(header_entries);
        let body = body;
        let builder_fn = {
            let header_entries = Arc::clone(&header_entries);
            let body = body.clone();
            move |mut builder: RequestBuilder| -> Result<RequestBuilder> {
                for (key, value) in header_entries.iter() {
                    builder = builder.header(key.clone(), value.clone());
                }
                builder = builder.body(body.clone());
                Ok(builder)
            }
        };

        let method_for_parse = method.clone();
        let url_for_parse = url.clone();
        let response = self.execute_with_builder(method, url, builder_fn).await?;

        parse_json_response(&method_for_parse, &url_for_parse, response).await
    }
}

const MAX_BODY_PREVIEW: usize = 2048;

fn debug_http_enabled() -> bool {
    static FLAG: OnceLock<bool> = OnceLock::new();
    *FLAG.get_or_init(|| match std::env::var("NBLM_DEBUG_HTTP") {
        Ok(value) => matches!(value.to_ascii_lowercase().as_str(), "1" | "true" | "yes"),
        Err(_) => false,
    })
}

fn build_body_preview(body: &[u8]) -> Cow<'_, str> {
    match std::str::from_utf8(body) {
        Ok(text) => {
            if text.len() > MAX_BODY_PREVIEW {
                let mut preview = text[..MAX_BODY_PREVIEW].to_string();
                preview.push('…');
                Cow::Owned(preview)
            } else {
                Cow::Borrowed(text)
            }
        }
        Err(_) => Cow::Owned(format!("<non-utf8 body: {} bytes>", body.len())),
    }
}

fn log_http_response(method: &Method, url: &Url, status: StatusCode, body: &[u8]) {
    if !debug_http_enabled() {
        return;
    }

    let preview = build_body_preview(body);
    eprintln!(
        "[nblm::http] method={} status={} url={} body_len={} body={}",
        method,
        status.as_u16(),
        url,
        body.len(),
        preview
    );
}

async fn parse_json_response<R>(
    method: &Method,
    url: &Url,
    response: reqwest::Response,
) -> Result<R>
where
    R: DeserializeOwned,
{
    let status = response.status();
    let body = response.bytes().await.map_err(Error::Request)?;
    log_http_response(method, url, status, &body);

    if !status.is_success() {
        let text = String::from_utf8_lossy(&body).into_owned();
        return Err(Error::http(status, text));
    }

    let parsed = serde_json::from_slice::<R>(&body)?;
    Ok(parsed)
}

impl HttpClient {
    async fn execute_with_builder<F>(
        &self,
        method: Method,
        url: Url,
        builder_fn: F,
    ) -> Result<reqwest::Response>
    where
        F: Fn(RequestBuilder) -> Result<RequestBuilder> + Send + Sync + 'static,
    {
        let client = self.client.clone();
        let method_clone = method.clone();
        let url_clone = url.clone();
        let provider = Arc::clone(&self.token_provider);
        let user_project = self.user_project.clone();
        let builder_fn = Arc::new(builder_fn);

        let run = {
            let client = client.clone();
            let method = method_clone.clone();
            let url = url_clone.clone();
            let provider = Arc::clone(&provider);
            let user_project = user_project.clone();
            let builder_fn = Arc::clone(&builder_fn);
            move || {
                let client = client.clone();
                let method = method.clone();
                let url = url.clone();
                let provider = Arc::clone(&provider);
                let user_project = user_project.clone();
                let builder_fn = Arc::clone(&builder_fn);
                async move {
                    let token = provider.access_token().await?;
                    let mut builder = client.request(method, url).bearer_auth(token);
                    if let Some(project) = &user_project {
                        builder = builder.header("x-goog-user-project", project);
                    }
                    builder = builder_fn(builder)?;
                    let request = builder.build().map_err(Error::Request)?;
                    let response = client.execute(request).await.map_err(Error::Request)?;
                    Ok(response)
                }
            }
        };

        let mut response = self.retryer.run_with_retry(run).await?;

        if response.status() == StatusCode::UNAUTHORIZED {
            let status = response.status();
            let body = response.bytes().await.map_err(Error::Request)?;
            log_http_response(&method, &url, status, &body);
            let run_refresh = {
                let client = client.clone();
                let method = method_clone.clone();
                let url = url_clone.clone();
                let provider = Arc::clone(&provider);
                let user_project = user_project.clone();
                let builder_fn = Arc::clone(&builder_fn);
                move || {
                    let client = client.clone();
                    let method = method.clone();
                    let url = url.clone();
                    let provider = Arc::clone(&provider);
                    let user_project = user_project.clone();
                    let builder_fn = Arc::clone(&builder_fn);
                    async move {
                        let token = provider.refresh_token().await?;
                        let mut builder = client.request(method, url).bearer_auth(token);
                        if let Some(project) = &user_project {
                            builder = builder.header("x-goog-user-project", project);
                        }
                        builder = builder_fn(builder)?;
                        let request = builder.build().map_err(Error::Request)?;
                        let response = client.execute(request).await.map_err(Error::Request)?;
                        Ok(response)
                    }
                }
            };
            response = self.retryer.run_with_retry(run_refresh).await?;
        }

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_body_preview_returns_borrowed_for_short_utf8() {
        let input = b"short body";
        let preview = build_body_preview(input);
        assert!(matches!(preview, Cow::Borrowed("short body")));
    }

    #[test]
    fn build_body_preview_truncates_long_utf8() {
        let long_text = "x".repeat(MAX_BODY_PREVIEW + 10);
        let preview = build_body_preview(long_text.as_bytes());
        let expected = format!("{}…", "x".repeat(MAX_BODY_PREVIEW));
        match preview {
            Cow::Owned(truncated) => assert_eq!(truncated, expected),
            _ => panic!("expected owned truncated preview"),
        }
    }

    #[test]
    fn build_body_preview_handles_non_utf8() {
        let binary = [0xffu8, 0x00, 0xfe];
        let preview = build_body_preview(&binary);
        let expected = format!("<non-utf8 body: {} bytes>", binary.len());
        match preview {
            Cow::Owned(msg) => assert_eq!(msg, expected),
            _ => panic!("expected owned message for non-utf8 body"),
        }
    }
}
