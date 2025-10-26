use std::borrow::Cow;
use std::sync::{Arc, OnceLock};

use bytes::Bytes;
use reqwest::{header::HeaderMap, Client, Method, StatusCode, Url};
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
        let client = self.client.clone();
        let method_clone = method.clone();
        let url_clone = url.clone();
        let body_ref = body;
        let provider = Arc::clone(&self.token_provider);
        let user_project = self.user_project.clone();

        let run = || {
            let client = client.clone();
            let method = method_clone.clone();
            let url = url_clone.clone();
            let provider = Arc::clone(&provider);
            let user_project = user_project.clone();
            async move {
                let token = provider.access_token().await?;
                let mut builder = client.request(method, url).bearer_auth(token);
                if let Some(project) = &user_project {
                    builder = builder.header("x-goog-user-project", project);
                }
                if let Some(body) = body_ref {
                    builder = builder.json(body);
                }
                let request = builder.build().map_err(Error::Request)?;
                let response = client.execute(request).await.map_err(Error::Request)?;
                Ok(response)
            }
        };

        let mut response = self.retryer.run_with_retry(run).await?;

        if response.status() == StatusCode::UNAUTHORIZED {
            let status = response.status();
            let body = response.bytes().await.map_err(Error::Request)?;
            log_http_response(&method, &url, status, &body);
            let run_refresh = || {
                let client = client.clone();
                let method = method_clone.clone();
                let url = url_clone.clone();
                let provider = Arc::clone(&provider);
                let user_project = user_project.clone();
                async move {
                    let token = provider.refresh_token().await?;
                    let mut builder = client.request(method, url).bearer_auth(token);
                    if let Some(project) = &user_project {
                        builder = builder.header("x-goog-user-project", project);
                    }
                    if let Some(body) = body_ref {
                        builder = builder.json(body);
                    }
                    let request = builder.build().map_err(Error::Request)?;
                    let response = client.execute(request).await.map_err(Error::Request)?;
                    Ok(response)
                }
            };
            response = self.retryer.run_with_retry(run_refresh).await?;
            return parse_json_response::<R>(&method, &url, response).await;
        }

        parse_json_response(&method, &url, response).await
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
        let client = self.client.clone();
        let method_clone = method.clone();
        let url_clone = url.clone();
        let provider = Arc::clone(&self.token_provider);
        let user_project = self.user_project.clone();
        let headers = Arc::new(headers);
        let body = body;

        let run = || {
            let client = client.clone();
            let method = method_clone.clone();
            let url = url_clone.clone();
            let provider = Arc::clone(&provider);
            let user_project = user_project.clone();
            let headers = Arc::clone(&headers);
            let body = body.clone();
            async move {
                let token = provider.access_token().await?;
                let mut builder = client.request(method, url).bearer_auth(token);
                if let Some(project) = &user_project {
                    builder = builder.header("x-goog-user-project", project);
                }
                for (key, value) in headers.iter() {
                    builder = builder.header(key.clone(), value.clone());
                }
                builder = builder.body(body.clone());
                let request = builder.build().map_err(Error::Request)?;
                let response = client.execute(request).await.map_err(Error::Request)?;
                Ok(response)
            }
        };

        let mut response = self.retryer.run_with_retry(run).await?;

        if response.status() == StatusCode::UNAUTHORIZED {
            let status = response.status();
            let body = response.bytes().await.map_err(Error::Request)?;
            log_http_response(&method, &url, status, &body);
            let run_refresh = || {
                let client = client.clone();
                let method = method_clone.clone();
                let url = url_clone.clone();
                let provider = Arc::clone(&provider);
                let user_project = user_project.clone();
                let headers = Arc::clone(&headers);
                let body = body.clone();
                async move {
                    let token = provider.refresh_token().await?;
                    let mut builder = client.request(method, url).bearer_auth(token);
                    if let Some(project) = &user_project {
                        builder = builder.header("x-goog-user-project", project);
                    }
                    for (key, value) in headers.iter() {
                        builder = builder.header(key.clone(), value.clone());
                    }
                    builder = builder.body(body.clone());
                    let request = builder.build().map_err(Error::Request)?;
                    let response = client.execute(request).await.map_err(Error::Request)?;
                    Ok(response)
                }
            };
            response = self.retryer.run_with_retry(run_refresh).await?;
            return parse_json_response::<R>(&method, &url, response).await;
        }

        parse_json_response(&method, &url, response).await
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
