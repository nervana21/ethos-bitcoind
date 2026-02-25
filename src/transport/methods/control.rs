//! control RPC method wrappers
//!
//! This module contains transport wrappers for control methods.

use serde_json::{json, Value};

use crate::transport::core::{TransportError, TransportTrait};

/// Returns an object containing information about memory usage.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.getmemoryinfo(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::getmemoryinfo(&transport, ...).await`
///
/// Calls the `getmemoryinfo` RPC method.
pub async fn get_memory_info(
    transport: &dyn TransportTrait,
    mode: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(mode)];
    let raw = transport.send_request("getmemoryinfo", &params).await?;
    Ok(raw)
}

/// Returns details of the RPC server.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.getrpcinfo(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::getrpcinfo(&transport, ...).await`
///
/// Calls the `getrpcinfo` RPC method.
pub async fn get_rpc_info(transport: &dyn TransportTrait) -> Result<Value, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getrpcinfo", &params).await?;
    Ok(raw)
}

/// List all commands, or get help for a specified command.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.help(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::help(&transport, ...).await`
///
/// Calls the `help` RPC method.
pub async fn help(
    transport: &dyn TransportTrait,
    command: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(command)];
    let raw = transport.send_request("help", &params).await?;
    Ok(raw)
}

/// Gets and sets the logging configuration.
/// When called without an argument, returns the list of categories with status that are currently being debug logged or not.
/// When called with arguments, adds or removes categories from debug logging and return the lists above.
/// The arguments are evaluated in order "include", "exclude".
/// If an item is both included and excluded, it will thus end up being excluded.
/// The valid logging categories are: addrman, bench, blockstorage, cmpctblock, coindb, estimatefee, http, i2p, ipc, kernel, leveldb, libevent, mempool, mempoolrej, net, privatebroadcast, proxy, prune, qt, rand, reindex, rpc, scan, selectcoins, tor, txpackages, txreconciliation, validation, walletdb, zmq
/// In addition, the following are available as category names with special meanings:
/// - "all",  "1" : represent all logging categories.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.logging(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::logging(&transport, ...).await`
///
/// Calls the `logging` RPC method.
pub async fn logging(
    transport: &dyn TransportTrait,
    include: serde_json::Value,
    exclude: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(include), json!(exclude)];
    let raw = transport.send_request("logging", &params).await?;
    Ok(raw)
}

/// Return RPC command JSON Schema descriptions.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.schema(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::schema(&transport, ...).await`
///
/// Calls the `schema` RPC method.
pub async fn schema(transport: &dyn TransportTrait) -> Result<Value, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("schema", &params).await?;
    Ok(raw)
}

/// Request a graceful shutdown of Bitcoin Core.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.stop(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::stop(&transport, ...).await`
///
/// Calls the `stop` RPC method.
pub async fn stop(
    transport: &dyn TransportTrait,
    wait: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(wait)];
    let raw = transport.send_request("stop", &params).await?;
    Ok(raw)
}

/// Returns the total uptime of the server.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.uptime(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::uptime(&transport, ...).await`
///
/// Calls the `uptime` RPC method.
pub async fn uptime(transport: &dyn TransportTrait) -> Result<Value, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("uptime", &params).await?;
    Ok(raw)
}
