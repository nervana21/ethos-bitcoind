use serde_json::Value;
use thiserror::Error;
use {reqwest, serde};

/// Errors that can occur during RPC transport operations
#[derive(Debug, Error, serde::Serialize, serde::Deserialize)]
pub enum TransportError {
    /// HTTP communication error
    #[error("HTTP error: {0}")]
    Http(String),
    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    Json(String),
    /// RPC protocol error
    #[error("RPC error: {0}")]
    Rpc(String),
    /// Network connection error
    #[error("Connection error: {0}")]
    ConnectionError(String),
}

impl From<reqwest::Error> for TransportError {
    fn from(err: reqwest::Error) -> Self { TransportError::Http(err.to_string()) }
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
#[derive(Clone, Debug)]
pub struct DefaultTransport {
    /// HTTP client for making requests
    client: reqwest::Client,
    /// RPC endpoint URL
    url: String,
    /// Optional authentication credentials (username, password)
    auth: Option<(String, String)>,
    /// Optional wallet name for Bitcoin Core RPC calls
    wallet_name: Option<String>,
}

impl DefaultTransport {
    /// Create a new default transport with the given URL and optional authentication.
    ///
    /// # Arguments
    /// * `url` - The RPC endpoint URL
    /// * `auth` - Optional (username, password) tuple for authentication
    pub fn new(url: impl Into<String>, auth: Option<(String, String)>) -> Self {
        Self { client: reqwest::Client::new(), url: url.into(), auth, wallet_name: None }
    }

    /// Configure this transport to use a specific wallet for RPC calls.
    ///
    /// # Arguments
    /// * `wallet_name` - The name of the wallet to use for RPC calls
    pub fn with_wallet(mut self, wallet_name: impl Into<String>) -> Self {
        self.wallet_name = Some(wallet_name.into());
        self
    }
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
        let auth = self.auth.clone();
        let wallet_name = self.wallet_name.clone();
        Box::pin(async move {
            let request = serde_json::json!({
                "jsonrpc": "2.0", "id": "1", "method": method, "params": params
            });

            // If a wallet is configured, prefer wallet endpoint; fallback to base URL on -32601 (method not found)
            if let Some(wallet) = &wallet_name {
                let wallet_url = format!("{}/wallet/{}", url.trim_end_matches('/'), wallet);

                // Try wallet endpoint first
                let mut req = client.post(&wallet_url).json(&request);
                if let Some((username, password)) = &auth {
                    req = req.basic_auth(username, Some(password));
                }
                let response = match req.send().await {
                    Ok(resp) => resp,
                    Err(e) => return Err(TransportError::Http(e.to_string())),
                };

                let text =
                    response.text().await.map_err(|e| TransportError::Http(e.to_string()))?;
                let json: Value =
                    serde_json::from_str(&text).map_err(|e| TransportError::Json(e.to_string()))?;

                if let Some(error) = json.get("error") {
                    // Fallback only for -32601 (Method not found)
                    if error.get("code").and_then(|c| c.as_i64()) == Some(-32601) {
                        let mut req = client.post(&url).json(&request);
                        if let Some((username, password)) = &auth {
                            req = req.basic_auth(username, Some(password));
                        }
                        let response = match req.send().await {
                            Ok(resp) => resp,
                            Err(e) => return Err(TransportError::Http(e.to_string())),
                        };
                        let text = response
                            .text()
                            .await
                            .map_err(|e| TransportError::Http(e.to_string()))?;
                        let json: Value = serde_json::from_str(&text)
                            .map_err(|e| TransportError::Json(e.to_string()))?;
                        if let Some(error) = json.get("error") {
                            return Err(TransportError::Rpc(error.to_string()));
                        }
                        return json
                            .get("result")
                            .cloned()
                            .ok_or_else(|| TransportError::Rpc("No result field".to_string()));
                    } else {
                        return Err(TransportError::Rpc(error.to_string()));
                    }
                }

                return json
                    .get("result")
                    .cloned()
                    .ok_or_else(|| TransportError::Rpc("No result field".to_string()));
            }

            // No wallet configured → base URL
            let mut req = client.post(&url).json(&request);
            if let Some((username, password)) = &auth {
                req = req.basic_auth(username, Some(password));
            }
            let response = match req.send().await {
                Ok(resp) => resp,
                Err(e) => return Err(TransportError::Http(e.to_string())),
            };
            let text = response.text().await.map_err(|e| TransportError::Http(e.to_string()))?;
            let json: Value =
                serde_json::from_str(&text).map_err(|e| TransportError::Json(e.to_string()))?;
            if let Some(error) = json.get("error") {
                return Err(TransportError::Rpc(error.to_string()));
            }
            json.get("result")
                .cloned()
                .ok_or_else(|| TransportError::Rpc("No result field".to_string()))
        })
    }

    fn send_batch<'a>(
        &'a self,
        bodies: &'a [Value],
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Vec<Value>, TransportError>> + Send + 'a>,
    > {
        let client = self.client.clone();
        let url = self.url.clone();
        let auth = self.auth.clone();
        Box::pin(async move {
            let mut req = client.post(&url).json(bodies);
            if let Some((username, password)) = &auth {
                req = req.basic_auth(username, Some(password));
            }
            let response = match req.send().await {
                Ok(resp) => resp,
                Err(e) => return Err(TransportError::Http(e.to_string())),
            };
            let text = response.text().await.map_err(|e| TransportError::Http(e.to_string()))?;
            let v: Vec<Value> =
                serde_json::from_str(&text).map_err(|e| TransportError::Json(e.to_string()))?;
            Ok(v)
        })
    }

    fn url(&self) -> &str { &self.url }
}
