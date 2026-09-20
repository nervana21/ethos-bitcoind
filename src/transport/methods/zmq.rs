//! zmq RPC method wrappers
//!
//! This module contains transport wrappers for zmq methods.

use serde_json::Value;

use crate::transport::core::{TransportError, TransportTrait};

/// Returns information about the active ZeroMQ notifications.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.getzmqnotifications(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::getzmqnotifications(&transport, ...).await`
///
/// Calls the `getzmqnotifications` RPC method.
pub async fn get_zmq_notifications(
    transport: &dyn TransportTrait,
) -> Result<Value, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport
        .send_request("getzmqnotifications", &params)
        .await?;
    Ok(raw)
}
