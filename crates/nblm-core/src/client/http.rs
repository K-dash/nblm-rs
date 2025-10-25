use std::sync::Arc;

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
            let _ = response.bytes().await;
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
            if !response.status().is_success() {
                let status = response.status();
                let body = response.text().await.unwrap_or_default();
                return Err(Error::http(status, body));
            }
            return Ok(response.json::<R>().await?);
        }

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(Error::http(status, body));
        }

        Ok(response.json::<R>().await?)
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
            let _ = response.bytes().await;
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
            if !response.status().is_success() {
                let status = response.status();
                let body = response.text().await.unwrap_or_default();
                return Err(Error::http(status, body));
            }
            return Ok(response.json::<R>().await?);
        }

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(Error::http(status, body));
        }

        Ok(response.json::<R>().await?)
    }
}
