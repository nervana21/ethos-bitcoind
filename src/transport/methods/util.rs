//! util RPC method wrappers
//!
//! This module contains transport wrappers for util methods.

use serde_json::{json, Value};

use crate::transport::core::{TransportError, TransportTrait};

/// Creates a multi-signature address with n signatures of m keys required.
/// It returns a json object with the address and redeemScript.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.createmultisig(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::createmultisig(&transport, ...).await`
///
/// Calls the `createmultisig` RPC method.
pub async fn create_multisig(
    transport: &dyn TransportTrait,
    nrequired: serde_json::Value,
    keys: serde_json::Value,
    address_type: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(nrequired), json!(keys), json!(address_type)];
    let raw = transport.send_request("createmultisig", &params).await?;
    Ok(raw)
}

/// Derives one or more addresses corresponding to an output descriptor.
/// Examples of output descriptors are:
/// pkh(&lt;pubkey&gt;)                                     P2PKH outputs for the given pubkey
/// wpkh(&lt;pubkey&gt;)                                    Native segwit P2PKH outputs for the given pubkey
/// sh(multi(&lt;n&gt;,&lt;pubkey&gt;,&lt;pubkey&gt;,...))              P2SH-multisig outputs for the given threshold and pubkeys
/// raw(&lt;hex script&gt;)                                 Outputs whose output script equals the specified hex-encoded bytes
/// tr(&lt;pubkey&gt;,multi_a(&lt;n&gt;,&lt;pubkey&gt;,&lt;pubkey&gt;,...))   P2TR-multisig outputs for the given threshold and pubkeys
/// In the above, &lt;pubkey&gt; either refers to a fixed public key in hexadecimal notation, or to an xpub/xprv optionally followed by one
/// or more path elements separated by "/", where "h" represents a hardened child key.
/// For more information on output descriptors, see the documentation in the doc/descriptors.md file.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.deriveaddresses(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::deriveaddresses(&transport, ...).await`
///
/// Calls the `deriveaddresses` RPC method.
pub async fn derive_addresses(
    transport: &dyn TransportTrait,
    descriptor: serde_json::Value,
    range: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(descriptor), json!(range)];
    let raw = transport.send_request("deriveaddresses", &params).await?;
    Ok(raw)
}

/// Estimates the approximate fee per kilobyte needed for a transaction to begin
/// confirmation within conf_target blocks if possible and return the number of blocks
/// for which the estimate is valid. Uses virtual transaction size as defined
/// in BIP 141 (witness data is discounted).
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.estimatesmartfee(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::estimatesmartfee(&transport, ...).await`
///
/// Calls the `estimatesmartfee` RPC method.
pub async fn estimate_smart_fee(
    transport: &dyn TransportTrait,
    conf_target: serde_json::Value,
    estimate_mode: serde_json::Value,
    options: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(conf_target), json!(estimate_mode), json!(options)];
    let raw = transport.send_request("estimatesmartfee", &params).await?;
    Ok(raw)
}

/// Analyses a descriptor.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.getdescriptorinfo(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::getdescriptorinfo(&transport, ...).await`
///
/// Calls the `getdescriptorinfo` RPC method.
pub async fn get_descriptor_info(
    transport: &dyn TransportTrait,
    descriptor: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(descriptor)];
    let raw = transport.send_request("getdescriptorinfo", &params).await?;
    Ok(raw)
}

/// Returns the status of one or all available indices currently running in the node.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.getindexinfo(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::getindexinfo(&transport, ...).await`
///
/// Calls the `getindexinfo` RPC method.
pub async fn get_index_info(
    transport: &dyn TransportTrait,
    index_name: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(index_name)];
    let raw = transport.send_request("getindexinfo", &params).await?;
    Ok(raw)
}

/// Sign a message with the private key of an address
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.signmessagewithprivkey(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::signmessagewithprivkey(&transport, ...).await`
///
/// Calls the `signmessagewithprivkey` RPC method.
pub async fn sign_message_with_priv_key(
    transport: &dyn TransportTrait,
    priv_key: serde_json::Value,
    message: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(priv_key), json!(message)];
    let raw = transport
        .send_request("signmessagewithprivkey", &params)
        .await?;
    Ok(raw)
}

/// Return information about the given bitcoin address.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.validateaddress(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::validateaddress(&transport, ...).await`
///
/// Calls the `validateaddress` RPC method.
pub async fn validate_address(
    transport: &dyn TransportTrait,
    address: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(address)];
    let raw = transport.send_request("validateaddress", &params).await?;
    Ok(raw)
}

/// Verify a signed message.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.verifymessage(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::verifymessage(&transport, ...).await`
///
/// Calls the `verifymessage` RPC method.
pub async fn verify_message(
    transport: &dyn TransportTrait,
    address: serde_json::Value,
    signature: serde_json::Value,
    message: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(address), json!(signature), json!(message)];
    let raw = transport.send_request("verifymessage", &params).await?;
    Ok(raw)
}
