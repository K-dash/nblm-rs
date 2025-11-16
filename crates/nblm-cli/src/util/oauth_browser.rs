use std::net::TcpListener as StdTcpListener;
use std::sync::Arc;

use anyhow::{anyhow, bail, Result};
use reqwest::Client;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener as AsyncTcpListener;
use tokio::time::Duration as TokioDuration;
use url::Url;

use nblm_core::auth::oauth::{self, AuthorizeParams, OAuthConfig, OAuthFlow, OAuthTokens};

/// Handles the interactive browser OAuth2 flow via loopback redirection.
pub struct OAuthBrowserFlow {
    config: OAuthConfig,
    http_client: Arc<Client>,
}

impl OAuthBrowserFlow {
    pub fn new(config: OAuthConfig, http_client: Arc<Client>) -> Self {
        Self {
            config,
            http_client,
        }
    }

    pub async fn run(&self) -> Result<OAuthTokens> {
        let mut config = self.config.clone();
        let mut listener: Option<AsyncTcpListener> = None;

        if std::env::var("NBLM_OAUTH_REDIRECT_URI").is_err()
            && config.redirect_uri == OAuthConfig::DEFAULT_REDIRECT_URI
        {
            let loopback = oauth::loopback::bind_loopback_listener(None)
                .map_err(|e| anyhow!("failed to bind loopback listener: {}", e))?;
            let port = loopback.port();
            config.redirect_uri = oauth::loopback::build_redirect_uri(port);
            let std_listener = loopback.into_std();
            let async_listener = AsyncTcpListener::from_std(std_listener)
                .map_err(|e| anyhow!("failed to create async listener: {}", e))?;
            listener = Some(async_listener);
        }

        if listener.is_none() {
            let redirect_url = Url::parse(&config.redirect_uri)
                .map_err(|e| anyhow!("invalid redirect_uri: {}", e))?;
            let addrs = redirect_url
                .socket_addrs(|| None)
                .map_err(|e| anyhow!("failed to parse redirect host: {}", e))?;
            let addr = addrs
                .into_iter()
                .find(|addr| addr.ip().is_loopback())
                .ok_or_else(|| anyhow!("redirect_uri must resolve to a loopback address"))?;
            let std_listener = StdTcpListener::bind(addr)
                .map_err(|e| anyhow!("failed to bind {}: {}", addr, e))?;
            std_listener
                .set_nonblocking(true)
                .map_err(|e| anyhow!("failed to configure listener: {}", e))?;
            let async_listener = AsyncTcpListener::from_std(std_listener)
                .map_err(|e| anyhow!("failed to create async listener: {}", e))?;
            listener = Some(async_listener);
        }

        let listener = listener.expect("listener must be initialized");
        let flow = OAuthFlow::new(config, Arc::clone(&self.http_client))
            .map_err(|e| anyhow!("failed to create OAuth flow: {}", e))?;

        let auth_context = flow.build_authorize_url(&AuthorizeParams {
            state: None,
            code_challenge: None,
            code_challenge_method: None,
        });

        eprintln!("Opening browser for authentication...");
        eprintln!(
            "If the browser doesn't open, please visit:\n{}",
            auth_context.url
        );
        if let Err(err) = webbrowser::open(&auth_context.url) {
            eprintln!("Warning: Failed to open browser: {}", err);
            eprintln!("Please manually visit the URL above");
        }

        let callback = listen_for_callback(listener).await?;
        if callback.state != auth_context.state {
            bail!("OAuth state mismatch - possible CSRF attack");
        }

        let tokens = flow
            .exchange_code(&auth_context, &callback.code)
            .await
            .map_err(|e| anyhow!("failed to exchange authorization code: {}", e))?;

        Ok(tokens)
    }
}

struct CallbackResult {
    code: String,
    state: String,
}

async fn listen_for_callback(listener: AsyncTcpListener) -> Result<CallbackResult> {
    if let Ok(addr) = listener.local_addr() {
        eprintln!("Listening for OAuth callback on {}", addr);
    }
    handle_callback(listener).await
}

async fn handle_callback(listener: AsyncTcpListener) -> Result<CallbackResult> {
    const TIMEOUT: TokioDuration = TokioDuration::from_secs(600);

    let result = tokio::time::timeout(TIMEOUT, async {
        let (mut stream, _) = listener.accept().await?;
        let mut buffer = vec![0u8; 4096];
        let n = stream.read(&mut buffer).await?;
        let request = String::from_utf8_lossy(&buffer[..n]);

        let mut code = None;
        let mut state = None;
        let mut error = None;

        if let Some(query_start) = request.find('?') {
            let query = &request[query_start + 1..];
            if let Some(http_end) = query.find(" HTTP/") {
                let query = &query[..http_end];
                for param in query.split('&') {
                    if let Some((key, value)) = param.split_once('=') {
                        let value = urlencoding::decode(value)
                            .map(|v| v.to_string())
                            .unwrap_or_else(|_| value.to_string());
                        match key {
                            "code" => code = Some(value),
                            "state" => state = Some(value),
                            "error" => error = Some(value),
                            _ => {}
                        }
                    }
                }
            }
        }

        let response = if let Some(error) = error {
            format!(
                "HTTP/1.1 400 Bad Request\r\nContent-Type: text/html\r\n\r\n<html><body><h1>Authentication failed</h1><p>Error: {error}</p></body></html>"
            )
        } else if code.is_some() && state.is_some() {
            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n<html><body><h1>Authentication successful!</h1><p>You can close this window.</p></body></html>"
                .to_string()
        } else {
            "HTTP/1.1 400 Bad Request\r\nContent-Type: text/html\r\n\r\n<html><body><h1>Invalid request</h1></body></html>"
                .to_string()
        };

        stream.write_all(response.as_bytes()).await?;
        stream.flush().await?;

        Ok::<CallbackResult, anyhow::Error>(CallbackResult {
            code: code.ok_or_else(|| anyhow!("no code parameter"))?,
            state: state.ok_or_else(|| anyhow!("no state parameter"))?,
        })
    })
    .await;

    match result {
        Ok(Ok(callback)) => Ok(callback),
        Ok(Err(e)) => Err(e),
        Err(_) => bail!("OAuth callback timeout after 10 minutes"),
    }
}
