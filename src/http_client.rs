use std::sync::Arc;
use std::time::{Duration, Instant};

use chrono::Utc;
use reqwest::{header, Client, RequestBuilder};
use tokio::sync::Mutex as AsyncMutex;

use crate::error::{ApiErrorResponse, NombaError, Result};

const LIVE_BASE_URL: &str = "https://api.nomba.com";
const SANDBOX_BASE_URL: &str = "https://sandbox.nomba.com";
const RETRYABLE_STATUS_CODES: &[u16] = &[429, 500, 502, 503, 504];

#[derive(Debug, Clone)]
struct TokenCache {
    access_token: Option<String>,
    expires_at: Option<Instant>,
}

impl TokenCache {
    fn new() -> Self {
        Self {
            access_token: None,
            expires_at: None,
        }
    }

    fn is_valid(&self) -> bool {
        self.access_token.is_some()
            && self.expires_at.map_or(false, |exp| Instant::now() < exp)
    }

    fn invalidate(&mut self) {
        self.access_token = None;
        self.expires_at = None;
    }
}

#[derive(Clone)]
pub struct NombaClientConfig {
    pub client_id: String,
    pub client_secret: String,
    pub account_id: String,
    pub sandbox: bool,
    pub timeout: Duration,
    pub max_retries: u32,
    pub backoff_factor: f64,
}

impl NombaClientConfig {
    pub fn new(client_id: String, client_secret: String, account_id: String) -> Self {
        Self {
            client_id,
            client_secret,
            account_id,
            sandbox: false,
            timeout: Duration::from_secs(30),
            max_retries: 3,
            backoff_factor: 0.5,
        }
    }

    pub fn sandbox(mut self, sandbox: bool) -> Self {
        self.sandbox = sandbox;
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    pub fn backoff_factor(mut self, backoff_factor: f64) -> Self {
        self.backoff_factor = backoff_factor;
        self
    }
}

#[derive(Clone)]
pub struct NombaClient {
    pub config: NombaClientConfig,
    http: Client,
    token_cache: Arc<tokio::sync::Mutex<TokenCache>>,
}

impl NombaClient {
    pub fn new(config: NombaClientConfig) -> Result<Self> {
        let _base_url = if config.sandbox {
            SANDBOX_BASE_URL
        } else {
            LIVE_BASE_URL
        };

        let mut builder = Client::builder()
            .timeout(config.timeout)
            .default_headers({
                let mut headers = header::HeaderMap::new();
                headers.insert(
                    header::CONTENT_TYPE,
                    header::HeaderValue::from_static("application/json"),
                );
                headers
            });

        if config.sandbox {
            builder = builder.danger_accept_invalid_certs(true);
            eprintln!(
                "Warning: Sandbox mode enabled - SSL verification disabled. Do not use in production."
            );
        }

        let http = builder.build()?;

        Ok(Self {
            config,
            http,
            token_cache: Arc::new(tokio::sync::Mutex::new(TokenCache::new())),
        })
    }

    fn base_url(&self) -> &str {
        if self.config.sandbox {
            SANDBOX_BASE_URL
        } else {
            LIVE_BASE_URL
        }
    }

    async fn fetch_token(&self) -> Result<String> {
        let url = format!("{}/v1/auth/token/issue", self.base_url());

        let response = self
            .http
            .post(&url)
            .header("accountId", &self.config.account_id)
            .json(&serde_json::json!({
                "grant_type": "client_credentials",
                "client_id": self.config.client_id,
                "client_secret": self.config.client_secret,
            }))
            .send()
            .await?;

        let status = response.status();
        let body: serde_json::Value = response.json().await?;

        if !status.is_success() {
            let error: ApiErrorResponse = serde_json::from_value(body.clone()).unwrap_or_else(|_| {
                ApiErrorResponse {
                    code: "UNKNOWN".to_string(),
                    description: "Failed to parse error response".to_string(),
                    data: None,
                }
            });
            return Err(NombaError::auth_with_details(
                format!("Failed to obtain access token: {}", error),
                status.as_u16(),
                Some(body),
            ));
        }

        let data = body.get("data").unwrap_or(&body);
        let token = data
            .get("access_token")
            .and_then(|v| v.as_str())
            .ok_or_else(|| NombaError::auth("Nomba auth response did not include an access_token"))?;

        let expires_in = data
            .get("expires_in")
            .and_then(|v| v.as_u64())
            .unwrap_or(3000);

        let mut cache = self.token_cache.lock().await;
        cache.access_token = Some(token.to_string());
        cache.expires_at = Some(Instant::now() + Duration::from_secs(expires_in.saturating_sub(50)));

        Ok(token.to_string())
    }

    async fn ensure_token(&self) -> Result<String> {
        {
            let cache = self.token_cache.lock().await;
            if cache.is_valid() {
                return Ok(cache.access_token.as_ref().unwrap().clone());
            }
        }

        let mut cache = self.token_cache.lock().await;
        if !cache.is_valid() {
            let token = self.fetch_token().await?;
            cache.access_token = Some(token.clone());
            return Ok(token);
        }
        Ok(cache.access_token.as_ref().unwrap().clone())
    }

    fn invalidate_token(&self) {
        let mut cache = self.token_cache.blocking_lock();
        cache.invalidate();
    }

    fn compute_backoff(&self, attempt: u32, retry_after: Option<&str>) -> Duration {
        if let Some(retry_after) = retry_after {
            if let Ok(seconds) = retry_after.parse::<f64>() {
                return Duration::from_secs_f64(seconds);
            }
        }

        let base = self.config.backoff_factor * 2_f64.powi(attempt as i32);
        let jitter = base * (rand::random::<f64>() * 0.4 - 0.2);
        Duration::from_secs_f64(base + jitter).max(Duration::from_millis(1))
    }

    async fn request_with_retry(
        &self,
        method: reqwest::Method,
        path: &str,
        json: Option<&serde_json::Value>,
        query: Option<Vec<(&str, String)>>,
        extra_headers: Option<&header::HeaderMap>,
        attempt: u32,
        retry_on_auth_failure: bool,
    ) -> Result<serde_json::Value> {
        let token = self.ensure_token().await?;

        let mut request = self
            .http
            .request(method.clone(), format!("{}{}", self.base_url(), path))
            .header("Authorization", format!("Bearer {}", token))
            .header("accountId", &self.config.account_id);

        if let Some(ref q) = query {
            request = request.query(q);
        }

        if let Some(body) = json {
            request = request.json(body);
        }

        if let Some(headers) = extra_headers {
            for (key, value) in headers {
                request = request.header(key, value);
            }
        }

        let response = request.send().await?;

        let status = response.status();

        if status.as_u16() == 401 && retry_on_auth_failure {
            self.invalidate_token();
            return Box::pin(self.request_with_retry(
                method,
                path,
                json,
                query,
                extra_headers,
                attempt,
                false,
            ))
            .await;
        }

        if RETRYABLE_STATUS_CODES.contains(&status.as_u16()) && attempt < self.config.max_retries {
            let retry_after = response
                .headers()
                .get("Retry-After")
                .and_then(|v| v.to_str().ok());
            let delay = self.compute_backoff(attempt, retry_after);
            tokio::time::sleep(delay).await;
            return Box::pin(self.request_with_retry(
                method,
                path,
                json,
                query,
                extra_headers,
                attempt + 1,
                retry_on_auth_failure,
            ))
            .await;
        }

        let body: serde_json::Value = response.json().await?;

        if !status.is_success() {
            let error: ApiErrorResponse = serde_json::from_value(body.clone()).unwrap_or_else(|_| {
                ApiErrorResponse {
                    code: "UNKNOWN".to_string(),
                    description: "Failed to parse error response".to_string(),
                    data: None,
                }
            });
            return Err(NombaError::api_with_details(
                error.description,
                status.as_u16(),
                Some(error.code),
                Some(body),
            ));
        }

        Ok(body)
    }

    pub async fn get(&self, path: &str, query: Option<Vec<(&str, String)>>) -> Result<serde_json::Value> {
        self.request_with_retry(
            reqwest::Method::GET,
            path,
            None,
            query,
            None,
            0,
            true,
        )
        .await
    }

    pub async fn post(
        &self,
        path: &str,
        json: &serde_json::Value,
        query: Option<Vec<(&str, String)>>,
    ) -> Result<serde_json::Value> {
        self.request_with_retry(
            reqwest::Method::POST,
            path,
            Some(json),
            query,
            None,
            0,
            true,
        )
        .await
    }

    pub async fn put(
        &self,
        path: &str,
        json: &serde_json::Value,
        query: Option<Vec<(&str, String)>>,
    ) -> Result<serde_json::Value> {
        self.request_with_retry(
            reqwest::Method::PUT,
            path,
            Some(json),
            query,
            None,
            0,
            true,
        )
        .await
    }

    pub async fn delete(
        &self,
        path: &str,
        query: Option<Vec<(&str, String)>>,
    ) -> Result<serde_json::Value> {
        self.request_with_retry(
            reqwest::Method::DELETE,
            path,
            None,
            query,
            None,
            0,
            true,
        )
        .await
    }
}

#[cfg(feature = "blocking")]
#[derive(Clone)]
pub struct BlockingNombaClient {
    pub inner: NombaClient,
    pub runtime: std::sync::Arc<tokio::runtime::Runtime>,
}

#[cfg(feature = "blocking")]
impl BlockingNombaClient {
    pub fn new(config: NombaClientConfig) -> Result<Self> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;
        let inner = NombaClient::new(config)?;
        Ok(Self { inner, runtime: std::sync::Arc::new(runtime) })
    }

    pub fn get(&self, path: &str, query: Option<Vec<(&str, String)>>) -> Result<serde_json::Value> {
        self.runtime.block_on(self.inner.get(path, query))
    }

    pub fn post(
        &self,
        path: &str,
        json: &serde_json::Value,
        query: Option<Vec<(&str, String)>>,
    ) -> Result<serde_json::Value> {
        self.runtime.block_on(self.inner.post(path, json, query))
    }

    pub fn put(
        &self,
        path: &str,
        json: &serde_json::Value,
        query: Option<Vec<(&str, String)>>,
    ) -> Result<serde_json::Value> {
        self.runtime.block_on(self.inner.put(path, json, query))
    }

    pub fn delete(&self, path: &str, query: Option<Vec<(&str, String)>>) -> Result<serde_json::Value> {
        self.runtime.block_on(self.inner.delete(path, query))
    }
}