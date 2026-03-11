//! rawtransaction RPC method wrappers
//!
//! This module contains transport wrappers for rawtransaction methods.

use serde_json::{json, Value};

use crate::transport::core::{TransportError, TransportTrait};

/// Abort private broadcast attempts for a transaction currently being privately broadcast.
/// The transaction will be removed from the private broadcast queue.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.abortprivatebroadcast(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::abortprivatebroadcast(&transport, ...).await`
///
/// Calls the `abortprivatebroadcast` RPC method.
pub async fn abort_private_broadcast(
    transport: &dyn TransportTrait,
    id: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(id)];
    let raw = transport.send_request("abortprivatebroadcast", &params).await?;
    Ok(raw)
}

/// Analyzes and provides information about the current status of a PSBT and its inputs
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.analyzepsbt(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::analyzepsbt(&transport, ...).await`
///
/// Calls the `analyzepsbt` RPC method.
pub async fn analyze_psbt(
    transport: &dyn TransportTrait,
    psbt: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(psbt)];
    let raw = transport.send_request("analyzepsbt", &params).await?;
    Ok(raw)
}

/// Combine multiple partially signed Bitcoin transactions into one transaction.
/// Implements the Combiner role.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.combinepsbt(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::combinepsbt(&transport, ...).await`
///
/// Calls the `combinepsbt` RPC method.
pub async fn combine_psbt(
    transport: &dyn TransportTrait,
    txs: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(txs)];
    let raw = transport.send_request("combinepsbt", &params).await?;
    Ok(raw)
}

/// Combine multiple partially signed transactions into one transaction.
/// The combined transaction may be another partially signed transaction or a
/// fully signed transaction.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.combinerawtransaction(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::combinerawtransaction(&transport, ...).await`
///
/// Calls the `combinerawtransaction` RPC method.
pub async fn combine_raw_transaction(
    transport: &dyn TransportTrait,
    txs: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(txs)];
    let raw = transport.send_request("combinerawtransaction", &params).await?;
    Ok(raw)
}

/// Converts a network serialized transaction to a PSBT. This should be used only with createrawtransaction and fundrawtransaction
/// createpsbt and walletcreatefundedpsbt should be used for new applications.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.converttopsbt(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::converttopsbt(&transport, ...).await`
///
/// Calls the `converttopsbt` RPC method.
pub async fn convert_to_psbt(
    transport: &dyn TransportTrait,
    hex_string: serde_json::Value,
    permit_sig_data: serde_json::Value,
    is_witness: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(hex_string), json!(permit_sig_data), json!(is_witness)];
    let raw = transport.send_request("converttopsbt", &params).await?;
    Ok(raw)
}

/// Creates a transaction in the Partially Signed Transaction format.
/// Implements the Creator role.
/// Note that the transaction's inputs are not signed, and
/// it is not stored in the wallet or transmitted to the network.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.createpsbt(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::createpsbt(&transport, ...).await`
///
/// Calls the `createpsbt` RPC method.
pub async fn create_psbt(
    transport: &dyn TransportTrait,
    inputs: serde_json::Value,
    outputs: serde_json::Value,
    lock_time: serde_json::Value,
    replaceable: serde_json::Value,
    version: serde_json::Value,
) -> Result<Value, TransportError> {
    let params =
        vec![json!(inputs), json!(outputs), json!(lock_time), json!(replaceable), json!(version)];
    let raw = transport.send_request("createpsbt", &params).await?;
    Ok(raw)
}

/// Create a transaction spending the given inputs and creating new outputs.
/// Outputs can be addresses or data.
/// Returns hex-encoded raw transaction.
/// Note that the transaction's inputs are not signed, and
/// it is not stored in the wallet or transmitted to the network.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.createrawtransaction(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::createrawtransaction(&transport, ...).await`
///
/// Calls the `createrawtransaction` RPC method.
pub async fn create_raw_transaction(
    transport: &dyn TransportTrait,
    inputs: serde_json::Value,
    outputs: serde_json::Value,
    lock_time: serde_json::Value,
    replaceable: serde_json::Value,
    version: serde_json::Value,
) -> Result<Value, TransportError> {
    let params =
        vec![json!(inputs), json!(outputs), json!(lock_time), json!(replaceable), json!(version)];
    let raw = transport.send_request("createrawtransaction", &params).await?;
    Ok(raw)
}

/// Return a JSON object representing the serialized, base64-encoded partially signed Bitcoin transaction.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.decodepsbt(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::decodepsbt(&transport, ...).await`
///
/// Calls the `decodepsbt` RPC method.
pub async fn decode_psbt(
    transport: &dyn TransportTrait,
    psbt: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(psbt)];
    let raw = transport.send_request("decodepsbt", &params).await?;
    Ok(raw)
}

/// Return a JSON object representing the serialized, hex-encoded transaction.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.decoderawtransaction(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::decoderawtransaction(&transport, ...).await`
///
/// Calls the `decoderawtransaction` RPC method.
pub async fn decode_raw_transaction(
    transport: &dyn TransportTrait,
    hex_string: serde_json::Value,
    is_witness: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(hex_string), json!(is_witness)];
    let raw = transport.send_request("decoderawtransaction", &params).await?;
    Ok(raw)
}

/// Decode a hex-encoded script.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.decodescript(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::decodescript(&transport, ...).await`
///
/// Calls the `decodescript` RPC method.
pub async fn decode_script(
    transport: &dyn TransportTrait,
    hex_string: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(hex_string)];
    let raw = transport.send_request("decodescript", &params).await?;
    Ok(raw)
}

/// Update all segwit inputs in a PSBT with information from output descriptors, the UTXO set or the mempool.
/// Then, sign the inputs we are able to with information from the output descriptors.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.descriptorprocesspsbt(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::descriptorprocesspsbt(&transport, ...).await`
///
/// Calls the `descriptorprocesspsbt` RPC method.
pub async fn descriptor_process_psbt(
    transport: &dyn TransportTrait,
    psbt: serde_json::Value,
    descriptors: serde_json::Value,
    sighash_type: serde_json::Value,
    bip32_derivs: serde_json::Value,
    finalize: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![
        json!(psbt),
        json!(descriptors),
        json!(sighash_type),
        json!(bip32_derivs),
        json!(finalize),
    ];
    let raw = transport.send_request("descriptorprocesspsbt", &params).await?;
    Ok(raw)
}

/// Finalize the inputs of a PSBT. If the transaction is fully signed, it will produce a
/// network serialized transaction which can be broadcast with sendrawtransaction. Otherwise a PSBT will be
/// created which has the final_scriptSig and final_scriptwitness fields filled for inputs that are complete.
/// Implements the Finalizer and Extractor roles.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.finalizepsbt(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::finalizepsbt(&transport, ...).await`
///
/// Calls the `finalizepsbt` RPC method.
pub async fn finalize_psbt(
    transport: &dyn TransportTrait,
    psbt: serde_json::Value,
    extract: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(psbt), json!(extract)];
    let raw = transport.send_request("finalizepsbt", &params).await?;
    Ok(raw)
}

/// If the transaction has no inputs, they will be automatically selected to meet its out value.
/// It will add at most one change output to the outputs.
/// No existing outputs will be modified unless "subtractFeeFromOutputs" is specified.
/// Note that inputs which were signed may need to be resigned after completion since in/outputs have been added.
/// The inputs added will not be signed, use signrawtransactionwithkey
/// or signrawtransactionwithwallet for that.
/// All existing inputs must either have their previous output transaction be in the wallet
/// or be in the UTXO set. Solving data must be provided for non-wallet inputs.
/// Note that all inputs selected must be of standard form and P2SH scripts must be
/// in the wallet using importdescriptors (to calculate fees).
/// You can see whether this is the case by checking the "solvable" field in the listunspent output.
/// Note that if specifying an exact fee rate, the resulting transaction may have a higher fee rate
/// if the transaction has unconfirmed inputs. This is because the wallet will attempt to make the
/// entire package have the given fee rate, not the resulting transaction.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.fundrawtransaction(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::fundrawtransaction(&transport, ...).await`
///
/// Calls the `fundrawtransaction` RPC method.
pub async fn fund_raw_transaction(
    transport: &dyn TransportTrait,
    hex_string: serde_json::Value,
    options: serde_json::Value,
    is_witness: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(hex_string), json!(options), json!(is_witness)];
    let raw = transport.send_request("fundrawtransaction", &params).await?;
    Ok(raw)
}

/// Returns information about transactions that are currently being privately broadcast.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.getprivatebroadcastinfo(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::getprivatebroadcastinfo(&transport, ...).await`
///
/// Calls the `getprivatebroadcastinfo` RPC method.
pub async fn get_private_broadcast_info(
    transport: &dyn TransportTrait,
) -> Result<Value, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getprivatebroadcastinfo", &params).await?;
    Ok(raw)
}

/// By default, this call only returns a transaction if it is in the mempool. If -txindex is enabled
/// and no blockhash argument is passed, it will return the transaction if it is in the mempool or any block.
/// If a blockhash argument is passed, it will return the transaction if
/// the specified block is available and the transaction is in that block.
/// Hint: Use gettransaction for wallet transactions.
/// If verbosity is 0 or omitted, returns the serialized transaction as a hex-encoded string.
/// If verbosity is 1, returns a JSON Object with information about the transaction.
/// If verbosity is 2, returns a JSON Object with information about the transaction, including fee and prevout information.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.getrawtransaction(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::getrawtransaction(&transport, ...).await`
///
/// Calls the `getrawtransaction` RPC method.
pub async fn get_raw_transaction(
    transport: &dyn TransportTrait,
    txid: serde_json::Value,
    verbosity: serde_json::Value,
    block_hash: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(txid), json!(verbosity), json!(block_hash)];
    let raw = transport.send_request("getrawtransaction", &params).await?;
    Ok(raw)
}

/// Joins multiple distinct PSBTs with different inputs and outputs into one PSBT with inputs and outputs from all of the PSBTs
/// No input in any of the PSBTs can be in more than one of the PSBTs.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.joinpsbts(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::joinpsbts(&transport, ...).await`
///
/// Calls the `joinpsbts` RPC method.
pub async fn join_psbts(
    transport: &dyn TransportTrait,
    txs: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(txs)];
    let raw = transport.send_request("joinpsbts", &params).await?;
    Ok(raw)
}

/// Submit a raw transaction (serialized, hex-encoded) to the network.
/// If -privatebroadcast is disabled, then the transaction will be put into the
/// local mempool of the node and will be sent unconditionally to all currently
/// connected peers, so using sendrawtransaction for manual rebroadcast will degrade
/// privacy by leaking the transaction's origin, as nodes will normally not
/// rebroadcast non-wallet transactions already in their mempool.
/// If -privatebroadcast is enabled, then the transaction will be sent only via
/// dedicated, short-lived connections to Tor or I2P peers or IPv4/IPv6 peers
/// via the Tor network. This conceals the transaction's origin. The transaction
/// will only enter the local mempool when it is received back from the network.
/// A specific exception, RPC_TRANSACTION_ALREADY_IN_UTXO_SET, may throw if the transaction cannot be added to the mempool.
/// Related RPCs: createrawtransaction, signrawtransactionwithkey
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.sendrawtransaction(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::sendrawtransaction(&transport, ...).await`
///
/// Calls the `sendrawtransaction` RPC method.
pub async fn send_raw_transaction(
    transport: &dyn TransportTrait,
    hex_string: serde_json::Value,
    max_fee_rate: serde_json::Value,
    max_burn_amount: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(hex_string), json!(max_fee_rate), json!(max_burn_amount)];
    let raw = transport.send_request("sendrawtransaction", &params).await?;
    Ok(raw)
}

/// Sign inputs for raw transaction (serialized, hex-encoded).
/// The second argument is an array of base58-encoded private
/// keys that will be the only keys used to sign the transaction.
/// The third optional argument (may be null) is an array of previous transaction outputs that
/// this transaction depends on but may not yet be in the block chain.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.signrawtransactionwithkey(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::signrawtransactionwithkey(&transport, ...).await`
///
/// Calls the `signrawtransactionwithkey` RPC method.
pub async fn sign_raw_transaction_with_key(
    transport: &dyn TransportTrait,
    hex_string: serde_json::Value,
    priv_keys: serde_json::Value,
    prev_txs: serde_json::Value,
    sighash_type: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(hex_string), json!(priv_keys), json!(prev_txs), json!(sighash_type)];
    let raw = transport.send_request("signrawtransactionwithkey", &params).await?;
    Ok(raw)
}

/// Submit a package of raw transactions (serialized, hex-encoded) to local node.
/// The package will be validated according to consensus and mempool policy rules. If any transaction passes, it will be accepted to mempool.
/// This RPC is experimental and the interface may be unstable. Refer to doc/policy/packages.md for documentation on package policies.
/// Warning: successful submission does not mean the transactions will propagate throughout the network.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.submitpackage(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::submitpackage(&transport, ...).await`
///
/// Calls the `submitpackage` RPC method.
pub async fn submit_package(
    transport: &dyn TransportTrait,
    package: serde_json::Value,
    max_fee_rate: serde_json::Value,
    max_burn_amount: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(package), json!(max_fee_rate), json!(max_burn_amount)];
    let raw = transport.send_request("submitpackage", &params).await?;
    Ok(raw)
}

/// Returns result of mempool acceptance tests indicating if raw transaction(s) (serialized, hex-encoded) would be accepted by mempool.
/// If multiple transactions are passed in, parents must come before children and package policies apply: the transactions cannot conflict with any mempool transactions or each other.
/// If one transaction fails, other transactions may not be fully validated (the 'allowed' key will be blank).
/// The maximum number of transactions allowed is 25.
/// This checks if transactions violate the consensus or policy rules.
/// See sendrawtransaction call.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.testmempoolaccept(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::testmempoolaccept(&transport, ...).await`
///
/// Calls the `testmempoolaccept` RPC method.
pub async fn test_mempool_accept(
    transport: &dyn TransportTrait,
    rawtxs: serde_json::Value,
    max_fee_rate: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(rawtxs), json!(max_fee_rate)];
    let raw = transport.send_request("testmempoolaccept", &params).await?;
    Ok(raw)
}

/// Updates all segwit inputs and outputs in a PSBT with data from output descriptors, the UTXO set, txindex, or the mempool.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.utxoupdatepsbt(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::utxoupdatepsbt(&transport, ...).await`
///
/// Calls the `utxoupdatepsbt` RPC method.
pub async fn utxo_update_psbt(
    transport: &dyn TransportTrait,
    psbt: serde_json::Value,
    descriptors: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(psbt), json!(descriptors)];
    let raw = transport.send_request("utxoupdatepsbt", &params).await?;
    Ok(raw)
}
