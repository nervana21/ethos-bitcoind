//! signer RPC method wrappers
//!
//! This module contains transport wrappers for signer methods.

use serde_json::Value;

use crate::transport::core::{TransportError, TransportTrait};

/// Returns a list of external signers from -signer. Signers with duplicate master key fingerprints are skipped.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.enumeratesigners(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::enumeratesigners(&transport, ...).await`
///
/// Calls the `enumeratesigners` RPC method.
pub async fn enumerate_signers(transport: &dyn TransportTrait) -> Result<Value, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("enumeratesigners", &params).await?;
    Ok(raw)
}
