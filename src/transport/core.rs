use std::time::Duration;

use base64::engine::general_purpose;
use base64::Engine;
use bitreq::{post, Client as BitreqClient, Error as BitreqError, RequestExt};
use serde_json::Value;
use thiserror::Error;
use tokio::time::sleep;
use tracing::warn;

/// Errors that can occur during RPC transport operations
#[derive(Debug, Error, serde::Serialize, serde::Deserialize)]
pub enum TransportError {
    /// HTTP communication error
    #[error("HTTP error: {0}")]
    Http(String),
    /// JSON serialization error (request)
    #[error("JSON error: {0}")]
    Json(String),
    /// RPC protocol error
    #[error("RPC error: {0}")]
    Rpc(String),
    /// Network connection error
    #[error("Connection error: {0}")]
    ConnectionError(String),
    /// Redirect error, not retryable
    #[error("HttpRedirect: {0}")]
    HttpRedirect(String),
    /// Error decoding the response
    #[error("Malformed Response: {0}")]
    MalformedResponse(String),
    /// Error parsing RPC response
    #[error("Error parsing rpc response: {0}")]
    Parse(String),
    /// Maximum retries exceeded
    #[error("Max retries {0} exceeded")]
    MaxRetriesExceeded(u8),
}

impl From<BitreqError> for TransportError {
    fn from(value: BitreqError) -> Self {
        match value {
            // Connection errors
            BitreqError::AddressNotFound
            | BitreqError::IoError(_)
            | BitreqError::RustlsCreateConnection(_) =>
                TransportError::ConnectionError(value.to_string()),

            // Redirect errors
            BitreqError::RedirectLocationMissing
            | BitreqError::InfiniteRedirectionLoop
            | BitreqError::TooManyRedirections => TransportError::HttpRedirect(value.to_string()),

            // Size/parsing errors
            BitreqError::HeadersOverflow
            | BitreqError::StatusLineOverflow
            | BitreqError::BodyOverflow
            | BitreqError::MalformedChunkLength
            | BitreqError::MalformedChunkEnd
            | BitreqError::MalformedContentLength
            | BitreqError::InvalidUtf8InResponse
            | BitreqError::InvalidUtf8InBody(_) =>
                TransportError::MalformedResponse(value.to_string()),

            // Other errors
            _ => TransportError::Http(value.to_string()),
        }
    }
}

impl From<serde_json::Error> for TransportError {
    fn from(err: serde_json::Error) -> Self { TransportError::Json(err.to_string()) }
}

impl From<std::io::Error> for TransportError {
    fn from(err: std::io::Error) -> Self { TransportError::Rpc(err.to_string()) }
}

/// Core trait for RPC transport operations
pub trait TransportTrait: Send + Sync {
    /// Send a single RPC request and return the response
    fn send_request<'a>(
        &'a self,
        method: &'a str,
        params: &'a [Value],
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Value, TransportError>> + Send + 'a>,
    >;

    /// Send a **batch** of raw JSON-RPC objects in one HTTP call.
    ///
    /// The `bodies` slice is already serializable JSON-RPC-2.0 frames:
    ///   [ { "jsonrpc":"2.0", "id":0, "method":"foo", "params": [...] }, … ]
    fn send_batch<'a>(
        &'a self,
        bodies: &'a [Value],
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Vec<Value>, TransportError>> + Send + 'a>,
    >;

    /// Get the URL endpoint for this transport
    fn url(&self) -> &str;
}
/// Extended transport trait with type-safe RPC calls
pub trait TransportExt {
    /// Send a type-safe RPC request and deserialize the response
    fn call<'a, T: serde::de::DeserializeOwned>(
        &'a self,
        method: &'a str,
        params: &'a [Value],
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T, TransportError>> + Send + 'a>>;
}

impl<T: TransportTrait> TransportExt for T {
    fn call<'a, T2: serde::de::DeserializeOwned>(
        &'a self,
        method: &'a str,
        params: &'a [Value],
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T2, TransportError>> + Send + 'a>>
    {
        Box::pin(async move {
            let result = self.send_request(method, params).await?;
            Ok(serde_json::from_value(result)?)
        })
    }
}

/// Default HTTP transport implementation for RPC communication
#[derive(Clone)]
pub struct DefaultTransport {
    /// HTTP client for making requests (reuses TCP connections)
    client: BitreqClient,
    /// RPC endpoint URL
    url: String,
    /// Precomputed Basic auth header value, or None
    authorization: Option<String>,
    /// Timeout for requests in seconds
    timeout_secs: u64,
    /// Maximum number of retries per request
    max_retries: u8,
    /// Interval between retries in ms
    retry_interval: u64,
    /// Optional wallet name for Bitcoin Core RPC calls
    wallet_name: Option<String>,
}

/// The default capacity for the HTTP client connection pool.
const DEFAULT_HTTP_CLIENT_CAPACITY: usize = 10;
/// Timeout for a request in seconds.
const DEFAULT_TIMEOUT_SECONDS: u64 = 30;
/// Maximum number of retries for a request.
const DEFAULT_MAX_RETRIES: u8 = 3;
/// Interval between retries in ms.
const DEFAULT_RETRY_INTERVAL_MS: u64 = 1_000;

impl DefaultTransport {
    /// Create a new default transport with the given URL and optional authentication.
    ///
    /// # Arguments
    /// * `url` - The RPC endpoint URL
    /// * `auth` - Optional (username, password) tuple for authentication
    pub fn new(url: impl Into<String>, auth: Option<(String, String)>) -> Self {
        let authorization = auth.as_ref().map(|(u, p)| {
            format!("Basic {}", general_purpose::STANDARD.encode(format!("{}:{}", u, p)))
        });
        Self {
            client: BitreqClient::new(DEFAULT_HTTP_CLIENT_CAPACITY),
            url: url.into(),
            authorization,
            timeout_secs: DEFAULT_TIMEOUT_SECONDS,
            max_retries: DEFAULT_MAX_RETRIES,
            retry_interval: DEFAULT_RETRY_INTERVAL_MS,
            wallet_name: None,
        }
    }

    /// Configure this transport to use a specific wallet for RPC calls.
    ///
    /// # Arguments
    /// * `wallet_name` - The name of the wallet to use for RPC calls
    pub fn with_wallet(mut self, wallet_name: impl Into<String>) -> Self {
        self.wallet_name = Some(wallet_name.into());
        self
    }

    /// Returns `true` if the error is potentially recoverable and should be retried.
    fn is_bitreq_error_recoverable(err: &BitreqError) -> bool {
        match err {
            // Connection/network errors - might be recoverable
            BitreqError::AddressNotFound
            | BitreqError::IoError(_)
            | BitreqError::RustlsCreateConnection(_) => {
                warn!(err = %err, "connection error, retrying...");
                true
            }

            // Redirect errors - not retryable
            BitreqError::RedirectLocationMissing => false,
            BitreqError::InfiniteRedirectionLoop => false,
            BitreqError::TooManyRedirections => false,

            // Size limit errors - not retryable
            BitreqError::HeadersOverflow => false,
            BitreqError::StatusLineOverflow => false,
            BitreqError::BodyOverflow => false,

            // Protocol/parsing errors - might be recoverable
            BitreqError::MalformedChunkLength
            | BitreqError::MalformedChunkEnd
            | BitreqError::MalformedContentLength
            | BitreqError::InvalidUtf8InResponse => {
                warn!(err = %err, "malformed response, retrying...");
                true
            }

            // UTF-8 in body - not retryable
            BitreqError::InvalidUtf8InBody(_) => false,

            // HTTPS not enabled - not retryable
            BitreqError::HttpsFeatureNotEnabled => false,

            // Other errors - not retryable
            BitreqError::Other(_) => false,

            // Non-exhaustive match fallback
            _ => false,
        }
    }
}

impl std::fmt::Debug for DefaultTransport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DefaultTransport")
            .field("url", &self.url)
            .field("timeout_secs", &self.timeout_secs)
            .field("max_retries", &self.max_retries)
            .field("retry_interval", &self.retry_interval)
            .field("wallet_name", &self.wallet_name)
            .finish_non_exhaustive()
    }
}

/// Internal error type for `do_request` to distinguish network errors from other transport errors.
/// This allows the retry logic to check recoverability on raw `BitreqError` variants.
enum DoRequestError {
    /// Network error from bitreq - check recoverability before converting
    Network(BitreqError),
    /// Other transport error - not recoverable via network retry
    Transport(TransportError),
}

impl TransportTrait for DefaultTransport {
    fn send_request<'a>(
        &'a self,
        method: &'a str,
        params: &'a [Value],
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Value, TransportError>> + Send + 'a>,
    > {
        let client = self.client.clone();
        let url = self.url.clone();
        let authorization = self.authorization.clone();
        let wallet_name = self.wallet_name.clone();
        let timeout_secs = self.timeout_secs;
        let max_retries = self.max_retries;
        let retry_interval = self.retry_interval;

        async fn do_request(
            client: &BitreqClient,
            url: &str,
            authorization: &Option<String>,
            request: &serde_json::Value,
            timeout_secs: u64,
        ) -> Result<Value, DoRequestError> {
            let body = serde_json::to_vec(request)
                .map_err(|e| DoRequestError::Transport(TransportError::Json(e.to_string())))?;
            let mut req = post(url)
                .with_header("Content-Type", "application/json")
                .with_body(body)
                .with_timeout(timeout_secs);
            if let Some(ref h) = authorization {
                req = req.with_header("Authorization", h);
            }
            let response =
                req.send_async_with_client(client).await.map_err(DoRequestError::Network)?;
            let status_code = response.status_code;
            if !(200..300).contains(&status_code) {
                return Err(DoRequestError::Transport(TransportError::Http(format!(
                    "{} {}",
                    status_code, response.reason_phrase
                ))));
            }
            let raw = response.as_str().map_err(|e: BitreqError| {
                DoRequestError::Transport(TransportError::Parse(e.to_string()))
            })?;
            let json: Value = serde_json::from_str(raw)
                .map_err(|e| DoRequestError::Transport(TransportError::Parse(e.to_string())))?;
            if let Some(error) = json.get("error") {
                if !error.is_null() {
                    return Err(DoRequestError::Transport(TransportError::Rpc(error.to_string())));
                }
            }
            json.get("result").cloned().ok_or_else(|| {
                DoRequestError::Transport(TransportError::Rpc("No result field".to_string()))
            })
        }

        Box::pin(async move {
            let request = serde_json::json!({
                "jsonrpc": "2.0", "id": "1", "method": method, "params": params
            });
            let mut retries = 0u8;
            loop {
                let target_url = if let Some(ref wallet) = wallet_name {
                    format!("{}/wallet/{}", url.trim_end_matches('/'), wallet)
                } else {
                    url.clone()
                };
                match do_request(&client, &target_url, &authorization, &request, timeout_secs).await
                {
                    Ok(v) => return Ok(v),
                    Err(DoRequestError::Transport(TransportError::Rpc(ref msg)))
                        if wallet_name.is_some() && msg.contains("\"code\":-32601") =>
                        match do_request(&client, &url, &authorization, &request, timeout_secs)
                            .await
                        {
                            Ok(v) => return Ok(v),
                            Err(DoRequestError::Network(e)) => return Err(TransportError::from(e)),
                            Err(DoRequestError::Transport(e)) => return Err(e),
                        },
                    Err(DoRequestError::Network(bitreq_err)) => {
                        if !Self::is_bitreq_error_recoverable(&bitreq_err) {
                            return Err(TransportError::from(bitreq_err));
                        }
                        // Error is recoverable, will retry after incrementing counter
                    }
                    Err(DoRequestError::Transport(err)) => {
                        // Non-network transport errors are not recoverable
                        return Err(err);
                    }
                }
                retries += 1;
                if retries >= max_retries {
                    return Err(TransportError::MaxRetriesExceeded(max_retries));
                }
                sleep(Duration::from_millis(retry_interval)).await;
            }
        })
    }

    // Note: Batch requests do not retry on failure. Callers should implement
    // their own retry logic if needed.
    fn send_batch<'a>(
        &'a self,
        bodies: &'a [Value],
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Vec<Value>, TransportError>> + Send + 'a>,
    > {
        let client = self.client.clone();
        let url = self.url.clone();
        let authorization = self.authorization.clone();
        let timeout_secs = self.timeout_secs;
        Box::pin(async move {
            let bodies_vec: Vec<Value> = bodies.to_vec();
            let body =
                serde_json::to_vec(&bodies_vec).map_err(|e| TransportError::Json(e.to_string()))?;
            let mut req = post(&url)
                .with_header("Content-Type", "application/json")
                .with_body(body)
                .with_timeout(timeout_secs);
            if let Some(ref h) = authorization {
                req = req.with_header("Authorization", h);
            }
            let response = req.send_async_with_client(&client).await?;
            let status_code = response.status_code;
            if !(200..300).contains(&status_code) {
                return Err(TransportError::Http(format!("HTTP {}", status_code)));
            }
            let raw =
                response.as_str().map_err(|e: BitreqError| TransportError::Parse(e.to_string()))?;
            let v: Vec<Value> =
                serde_json::from_str(raw).map_err(|e| TransportError::Parse(e.to_string()))?;
            Ok(v)
        })
    }

    fn url(&self) -> &str { &self.url }
}
