//! mining RPC method wrappers
//!
//! This module contains transport wrappers for mining methods.

use serde_json::{json, Value};

use crate::transport::core::{TransportError, TransportTrait};

/// If the request parameters include a 'mode' key, that is used to explicitly select between the default 'template' request or a 'proposal'.
/// It returns data needed to construct a block to work on.
/// For full specification, see BIPs 22, 23, 9, and 145:
/// <https://github.com/bitcoin/bips/blob/master/bip-0022.mediawiki>
/// <https://github.com/bitcoin/bips/blob/master/bip-0023.mediawiki>
/// <https://github.com/bitcoin/bips/blob/master/bip-0009.mediawiki#getblocktemplate_changes>
/// <https://github.com/bitcoin/bips/blob/master/bip-0145.mediawiki>
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.getblocktemplate(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::getblocktemplate(&transport, ...).await`
///
/// Calls the `getblocktemplate` RPC method.
pub async fn get_block_template(
    transport: &dyn TransportTrait,
    template_request: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(template_request)];
    let raw = transport.send_request("getblocktemplate", &params).await?;
    Ok(raw)
}

/// Returns a json object containing mining-related information.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.getmininginfo(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::getmininginfo(&transport, ...).await`
///
/// Calls the `getmininginfo` RPC method.
pub async fn get_mining_info(transport: &dyn TransportTrait) -> Result<Value, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getmininginfo", &params).await?;
    Ok(raw)
}

/// Returns the estimated network hashes per second based on the last n blocks.
/// Pass in \[blocks\] to override # of blocks, -1 specifies since last difficulty change.
/// Pass in \[height\] to estimate the network speed at the time when a certain block was found.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.getnetworkhashps(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::getnetworkhashps(&transport, ...).await`
///
/// Calls the `getnetworkhashps` RPC method.
pub async fn get_network_hashps(
    transport: &dyn TransportTrait,
    nblocks: serde_json::Value,
    height: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(nblocks), json!(height)];
    let raw = transport.send_request("getnetworkhashps", &params).await?;
    Ok(raw)
}

/// Returns a map of all user-created (see prioritisetransaction) fee deltas by txid, and whether the tx is present in mempool.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.getprioritisedtransactions(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::getprioritisedtransactions(&transport, ...).await`
///
/// Calls the `getprioritisedtransactions` RPC method.
pub async fn get_prioritised_transactions(
    transport: &dyn TransportTrait,
) -> Result<Value, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport
        .send_request("getprioritisedtransactions", &params)
        .await?;
    Ok(raw)
}

/// Accepts the transaction into mined blocks at a higher (or lower) priority
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.prioritisetransaction(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::prioritisetransaction(&transport, ...).await`
///
/// Calls the `prioritisetransaction` RPC method.
pub async fn prioritise_transaction(
    transport: &dyn TransportTrait,
    txid: serde_json::Value,
    dummy: serde_json::Value,
    fee_delta: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(txid), json!(dummy), json!(fee_delta)];
    let raw = transport
        .send_request("prioritisetransaction", &params)
        .await?;
    Ok(raw)
}

/// Attempts to submit new block to network.
/// See <https://en.bitcoin.it/wiki/BIP_0022> for full specification.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.submitblock(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::submitblock(&transport, ...).await`
///
/// Calls the `submitblock` RPC method.
pub async fn submit_block(
    transport: &dyn TransportTrait,
    hexdata: serde_json::Value,
    dummy: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(hexdata), json!(dummy)];
    let raw = transport.send_request("submitblock", &params).await?;
    Ok(raw)
}

/// Decode the given hexdata as a header and submit it as a candidate chain tip if valid.
/// Throws when the header is invalid.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.submitheader(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::submitheader(&transport, ...).await`
///
/// Calls the `submitheader` RPC method.
pub async fn submit_header(
    transport: &dyn TransportTrait,
    hexdata: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(hexdata)];
    let raw = transport.send_request("submitheader", &params).await?;
    Ok(raw)
}
