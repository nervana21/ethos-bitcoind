//! network RPC method wrappers
//!
//! This module contains transport wrappers for network methods.

use serde_json::{json, Value};

use crate::transport::core::{TransportError, TransportTrait};

/// Attempts to add or remove a node from the addnode list.
/// Or try a connection to a node once.
/// Nodes added using addnode (or -connect) are protected from DoS disconnection and are not required to be
/// full nodes/support SegWit as other outbound peers are (though such peers will not be synced from).
/// Addnode connections are limited to 8 at a time and are counted separately from the -maxconnections limit.
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.addnode(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::addnode(&transport, ...).await`
/// Calls the `addnode` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn add_node(
    transport: &dyn TransportTrait,
    node: serde_json::Value,
    command: serde_json::Value,
    v2transport: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(node), json!(command), json!(v2transport)];
    let raw = transport.send_request("addnode", &params).await?;
    Ok(raw)
}

/// Clear all banned IPs.
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.clearbanned(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::clearbanned(&transport, ...).await`
/// Calls the `clearbanned` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn clear_banned(transport: &dyn TransportTrait) -> Result<Value, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("clearbanned", &params).await?;
    Ok(raw)
}

/// Immediately disconnects from the specified peer node.
/// Strictly one out of \\\'address" and \\\'nodeid" can be provided to identify the node.
/// To disconnect by nodeid, either set \\\'address" to the empty string, or call using the named \\\'nodeid" argument only.
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.disconnectnode(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::disconnectnode(&transport, ...).await`
/// Calls the `disconnectnode` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn disconnect_node(
    transport: &dyn TransportTrait,
    address: serde_json::Value,
    nodeid: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(address), json!(nodeid)];
    let raw = transport.send_request("disconnectnode", &params).await?;
    Ok(raw)
}

/// Returns information about the given added node, or all added nodes
/// (note that onetry addnodes are not listed here)
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.getaddednodeinfo(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::getaddednodeinfo(&transport, ...).await`
/// Calls the `getaddednodeinfo` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn get_added_node_info(
    transport: &dyn TransportTrait,
    node: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(node)];
    let raw = transport.send_request("getaddednodeinfo", &params).await?;
    Ok(raw)
}

/// Provides information about the node\\\'s address manager by returning the number of addresses in the ````new```` and ````tried```` tables and their sum for all networks.
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.getaddrmaninfo(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::getaddrmaninfo(&transport, ...).await`
/// Calls the `getaddrmaninfo` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn get_addr_man_info(transport: &dyn TransportTrait) -> Result<Value, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getaddrmaninfo", &params).await?;
    Ok(raw)
}

/// Returns the number of connections to other nodes.
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.getconnectioncount(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::getconnectioncount(&transport, ...).await`
/// Calls the `getconnectioncount` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn get_connection_count(transport: &dyn TransportTrait) -> Result<Value, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getconnectioncount", &params).await?;
    Ok(raw)
}

/// Returns information about network traffic, including bytes in, bytes out,
/// and current system time.
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.getnettotals(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::getnettotals(&transport, ...).await`
/// Calls the `getnettotals` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn get_net_totals(transport: &dyn TransportTrait) -> Result<Value, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getnettotals", &params).await?;
    Ok(raw)
}

/// Returns an object containing various state info regarding P2P networking.
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.getnetworkinfo(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::getnetworkinfo(&transport, ...).await`
/// Calls the `getnetworkinfo` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn get_network_info(transport: &dyn TransportTrait) -> Result<Value, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getnetworkinfo", &params).await?;
    Ok(raw)
}

/// Return known addresses, after filtering for quality and recency.
/// These can potentially be used to find new peers in the network.
/// The total number of addresses known to the node may be higher.
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.getnodeaddresses(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::getnodeaddresses(&transport, ...).await`
/// Calls the `getnodeaddresses` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn get_node_addresses(
    transport: &dyn TransportTrait,
    count: serde_json::Value,
    network: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(count), json!(network)];
    let raw = transport.send_request("getnodeaddresses", &params).await?;
    Ok(raw)
}

/// Returns data about each connected network peer as a json array of objects.
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.getpeerinfo(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::getpeerinfo(&transport, ...).await`
/// Calls the `getpeerinfo` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn get_peer_info(transport: &dyn TransportTrait) -> Result<Value, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getpeerinfo", &params).await?;
    Ok(raw)
}

/// List all manually banned IPs/Subnets.
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.listbanned(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::listbanned(&transport, ...).await`
/// Calls the `listbanned` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn list_banned(transport: &dyn TransportTrait) -> Result<Value, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("listbanned", &params).await?;
    Ok(raw)
}

/// Requests that a ping be sent to all other nodes, to measure ping time.
/// Results are provided in getpeerinfo.
/// Ping command is handled in queue with all other commands, so it measures processing backlog, not just network ping.
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.ping(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::ping(&transport, ...).await`
/// Calls the `ping` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn ping(transport: &dyn TransportTrait) -> Result<Value, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("ping", &params).await?;
    Ok(raw)
}

/// Attempts to add or remove an IP/Subnet from the banned list.
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.setban(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::setban(&transport, ...).await`
/// Calls the `setban` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn set_ban(
    transport: &dyn TransportTrait,
    subnet: serde_json::Value,
    command: serde_json::Value,
    bantime: serde_json::Value,
    absolute: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(subnet), json!(command), json!(bantime), json!(absolute)];
    let raw = transport.send_request("setban", &params).await?;
    Ok(raw)
}

/// Disable/enable all p2p network activity.
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.setnetworkactive(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::setnetworkactive(&transport, ...).await`
/// Calls the `setnetworkactive` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn set_network_active(
    transport: &dyn TransportTrait,
    state: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(state)];
    let raw = transport.send_request("setnetworkactive", &params).await?;
    Ok(raw)
}
