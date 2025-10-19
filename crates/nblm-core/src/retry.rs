use std::time::{Duration, SystemTime};

use backon::{BackoffBuilder, ExponentialBuilder};
use httpdate::parse_http_date;
use reqwest::{header::RETRY_AFTER, StatusCode};
use tokio::time::sleep;
use tracing::warn;

use crate::error::{Error, Result};

const DEFAULT_RETRY_MIN_DELAY_MS: u64 = 500;
const DEFAULT_RETRY_MAX_DELAY_SECS: u64 = 5;
const DEFAULT_RETRY_MAX_RETRIES: usize = 3;

#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Minimum backoff delay between retry attempts.
    pub min_delay: Duration,
    /// Maximum backoff delay between retry attempts.
    pub max_delay: Duration,
    /// Maximum number of retry attempts after the initial request.
    pub max_retries: usize,
    pub jitter: bool,
}

impl RetryConfig {
    pub fn with_min_delay(mut self, delay: Duration) -> Self {
        self.min_delay = delay;
        self
    }

    pub fn with_max_delay(mut self, delay: Duration) -> Self {
        self.max_delay = delay;
        self
    }

    pub fn with_max_retries(mut self, retries: usize) -> Self {
        self.max_retries = retries;
        self
    }

    pub fn with_jitter(mut self, jitter: bool) -> Self {
        self.jitter = jitter;
        self
    }
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            min_delay: Duration::from_millis(DEFAULT_RETRY_MIN_DELAY_MS),
            max_delay: Duration::from_secs(DEFAULT_RETRY_MAX_DELAY_SECS),
            max_retries: DEFAULT_RETRY_MAX_RETRIES,
            jitter: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Retryer {
    config: RetryConfig,
}

impl Retryer {
    pub fn new(config: RetryConfig) -> Self {
        Self { config }
    }

    pub async fn run_with_retry<F, Fut>(&self, mut operation: F) -> Result<reqwest::Response>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = std::result::Result<reqwest::Response, Error>>,
    {
        let mut builder = ExponentialBuilder::default()
            .with_min_delay(self.config.min_delay)
            .with_max_delay(self.config.max_delay)
            .with_max_times(self.config.max_retries);
        if self.config.jitter {
            builder = builder.with_jitter();
        }
        let mut backoff = builder.build();
        let mut attempts = 0usize;

        loop {
            match operation().await {
                Ok(response) => {
                    if should_retry_status(response.status()) {
                        let status = response.status();
                        let retry_after = retry_after_delay(&response);
                        if attempts >= self.config.max_retries {
                            let body = response.text().await.unwrap_or_default();
                            return Err(Error::http(status, body));
                        }
                        attempts += 1;
                        let max_delay = self.config.max_delay;
                        let backoff_delay = backoff.next().map(|d| d.min(max_delay));
                        let delay = retry_after
                            .map(|d| d.min(max_delay))
                            .or(backoff_delay)
                            .unwrap_or(Duration::from_millis(0));
                        let _ = response.bytes().await;
                        warn!(
                            %status,
                            attempt = attempts,
                            max_retries = self.config.max_retries,
                            retry_after = ?delay,
                            "retrying HTTP request due to status"
                        );
                        sleep(delay).await;
                        continue;
                    }
                    return Ok(response);
                }
                Err(err) => {
                    if is_retryable_error(&err) {
                        if attempts >= self.config.max_retries {
                            return Err(err);
                        }
                        attempts += 1;
                        if let Some(delay) = backoff.next().map(|d| d.min(self.config.max_delay)) {
                            warn!(
                                ?err,
                                attempt = attempts,
                                max_retries = self.config.max_retries,
                                retry_after = ?delay,
                                "retrying HTTP request due to error"
                            );
                            sleep(delay).await;
                            continue;
                        }
                    }
                    return Err(err);
                }
            }
        }
    }
}

fn should_retry_status(status: StatusCode) -> bool {
    matches!(
        status,
        StatusCode::TOO_MANY_REQUESTS | StatusCode::REQUEST_TIMEOUT
    ) || status.is_server_error()
}

fn retry_after_delay(response: &reqwest::Response) -> Option<Duration> {
    response
        .headers()
        .get(RETRY_AFTER)
        .and_then(|value| parse_retry_after(value.to_str().ok()?, SystemTime::now()))
}

fn is_retryable_error(err: &Error) -> bool {
    match err {
        Error::Request(req_err) => req_err.is_connect() || req_err.is_timeout(),
        Error::Http { status, .. } => should_retry_status(*status),
        _ => false,
    }
}

fn parse_retry_after(value: &str, now: SystemTime) -> Option<Duration> {
    if let Ok(seconds) = value.parse::<u64>() {
        return Some(Duration::from_secs(seconds));
    }
    if let Ok(date) = parse_http_date(value) {
        if let Ok(dur) = date.duration_since(now) {
            return Some(dur);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_retry_after_seconds() {
        let now = SystemTime::now();
        let delay = parse_retry_after("5", now).unwrap();
        assert_eq!(delay, Duration::from_secs(5));
    }

    #[test]
    fn parse_retry_after_http_date() {
        let future = SystemTime::now() + Duration::from_secs(3);
        let header = httpdate::fmt_http_date(future);
        let delay = parse_retry_after(&header, SystemTime::now()).unwrap();
        assert!(delay <= Duration::from_secs(3));
    }

    #[test]
    fn parse_retry_after_invalid() {
        let now = SystemTime::now();
        assert!(parse_retry_after("invalid", now).is_none());
    }
}
