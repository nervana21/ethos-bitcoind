//! blockchain RPC method wrappers
//!
//! This module contains transport wrappers for blockchain methods.

use serde_json::{json, Value};

use crate::transport::core::{TransportError, TransportTrait};

/// Write the serialized UTXO set to a file. This can be used in loadtxoutset afterwards if this snapshot height is supported in the chainparams as well.
/// Unless the "latest" type is requested, the node will roll back to the requested height and network activity will be suspended during this process. Because of this it is discouraged to interact with the node in any other way during the execution of this call to avoid inconsistent results and race conditions, particularly RPCs that interact with blockstorage.
/// This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.dumptxoutset(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::dumptxoutset(&transport, ...).await`
/// Calls the `dumptxoutset` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn dump_tx_out_set(
    transport: &dyn TransportTrait,
    path: serde_json::Value,
    r#type: serde_json::Value,
    options: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(path), json!(r#type), json!(options)];
    let raw = transport.send_request("dumptxoutset", &params).await?;
    Ok(raw)
}

/// Returns the hash of the best (tip) block in the most-work fully-validated chain.
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.getbestblockhash(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::getbestblockhash(&transport, ...).await`
/// Calls the `getbestblockhash` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn get_best_block_hash(transport: &dyn TransportTrait) -> Result<Value, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getbestblockhash", &params).await?;
    Ok(raw)
}

/// If verbosity is 0, returns a string that is serialized, hex-encoded data for block \\\'hash\\\'.
/// If verbosity is 1, returns an Object with information about block <hash>.
/// If verbosity is 2, returns an Object with information about block <hash> and information about each transaction.
/// If verbosity is 3, returns an Object with information about block <hash> and information about each transaction, including prevout information for inputs (only for unpruned blocks in the current best chain).
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.getblock(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::getblock(&transport, ...).await`
/// Calls the `getblock` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn get_block(
    transport: &dyn TransportTrait,
    blockhash: serde_json::Value,
    verbosity: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(blockhash), json!(verbosity)];
    let raw = transport.send_request("getblock", &params).await?;
    Ok(raw)
}

/// Returns an object containing various state info regarding blockchain processing.
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.getblockchaininfo(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::getblockchaininfo(&transport, ...).await`
/// Calls the `getblockchaininfo` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn get_blockchain_info(transport: &dyn TransportTrait) -> Result<Value, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getblockchaininfo", &params).await?;
    Ok(raw)
}

/// Returns the height of the most-work fully-validated chain.
/// The genesis block has height 0.
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.getblockcount(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::getblockcount(&transport, ...).await`
/// Calls the `getblockcount` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn get_block_count(transport: &dyn TransportTrait) -> Result<Value, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getblockcount", &params).await?;
    Ok(raw)
}

/// Retrieve a BIP 157 content filter for a particular block.
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.getblockfilter(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::getblockfilter(&transport, ...).await`
/// Calls the `getblockfilter` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn get_block_filter(
    transport: &dyn TransportTrait,
    blockhash: serde_json::Value,
    filtertype: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(blockhash), json!(filtertype)];
    let raw = transport.send_request("getblockfilter", &params).await?;
    Ok(raw)
}

/// Attempt to fetch block from a given peer.
/// We must have the header for this block, e.g. using submitheader.
/// The block will not have any undo data which can limit the usage of the block data in a context where the undo data is needed.
/// Subsequent calls for the same block may cause the response from the previous peer to be ignored.
/// Peers generally ignore requests for a stale block that they never fully verified, or one that is more than a month old.
/// When a peer does not respond with a block, we will disconnect.
/// Note: The block could be re-pruned as soon as it is received.
/// Returns an empty JSON object if the request was successfully scheduled.
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.getblockfrompeer(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::getblockfrompeer(&transport, ...).await`
/// Calls the `getblockfrompeer` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn get_block_from_peer(
    transport: &dyn TransportTrait,
    blockhash: serde_json::Value,
    peer_id: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(blockhash), json!(peer_id)];
    let raw = transport.send_request("getblockfrompeer", &params).await?;
    Ok(raw)
}

/// Returns hash of block in best-block-chain at height provided.
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.getblockhash(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::getblockhash(&transport, ...).await`
/// Calls the `getblockhash` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn get_block_hash(
    transport: &dyn TransportTrait,
    height: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(height)];
    let raw = transport.send_request("getblockhash", &params).await?;
    Ok(raw)
}

/// If verbose is false, returns a string that is serialized, hex-encoded data for blockheader \\\'hash\\\'.
/// If verbose is true, returns an Object with information about blockheader <hash>.
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.getblockheader(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::getblockheader(&transport, ...).await`
/// Calls the `getblockheader` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn get_block_header(
    transport: &dyn TransportTrait,
    blockhash: serde_json::Value,
    verbose: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(blockhash), json!(verbose)];
    let raw = transport.send_request("getblockheader", &params).await?;
    Ok(raw)
}

/// Compute per block statistics for a given window. All amounts are in satoshis.
/// It won\\\'t work for some heights with pruning.
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.getblockstats(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::getblockstats(&transport, ...).await`
/// Calls the `getblockstats` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn get_block_stats(
    transport: &dyn TransportTrait,
    hash_or_height: serde_json::Value,
    stats: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(hash_or_height), json!(stats)];
    let raw = transport.send_request("getblockstats", &params).await?;
    Ok(raw)
}

/// Return information about chainstates.
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.getchainstates(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::getchainstates(&transport, ...).await`
/// Calls the `getchainstates` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn get_chain_states(transport: &dyn TransportTrait) -> Result<Value, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getchainstates", &params).await?;
    Ok(raw)
}

/// Return information about all known tips in the block tree, including the main chain as well as orphaned branches.
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.getchaintips(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::getchaintips(&transport, ...).await`
/// Calls the `getchaintips` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn get_chain_tips(transport: &dyn TransportTrait) -> Result<Value, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getchaintips", &params).await?;
    Ok(raw)
}

/// Compute statistics about the total number and rate of transactions in the chain.
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.getchaintxstats(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::getchaintxstats(&transport, ...).await`
/// Calls the `getchaintxstats` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn get_chain_tx_stats(
    transport: &dyn TransportTrait,
    nblocks: serde_json::Value,
    blockhash: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(nblocks), json!(blockhash)];
    let raw = transport.send_request("getchaintxstats", &params).await?;
    Ok(raw)
}

/// Returns an object containing various state info regarding deployments of consensus changes.
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.getdeploymentinfo(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::getdeploymentinfo(&transport, ...).await`
/// Calls the `getdeploymentinfo` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn get_deployment_info(
    transport: &dyn TransportTrait,
    blockhash: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(blockhash)];
    let raw = transport.send_request("getdeploymentinfo", &params).await?;
    Ok(raw)
}

/// Get spend and receive activity associated with a set of descriptors for a set of blocks. This command pairs well with the ````relevant_blocks```` output of ````scanblocks()````.
/// This call may take several minutes. If you encounter timeouts, try specifying no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.getdescriptoractivity(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::getdescriptoractivity(&transport, ...).await`
/// Calls the `getdescriptoractivity` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn get_descriptor_activity(
    transport: &dyn TransportTrait,
    blockhashes: serde_json::Value,
    scanobjects: serde_json::Value,
    include_mempool: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(blockhashes), json!(scanobjects), json!(include_mempool)];
    let raw = transport.send_request("getdescriptoractivity", &params).await?;
    Ok(raw)
}

/// Returns the proof-of-work difficulty as a multiple of the minimum difficulty.
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.getdifficulty(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::getdifficulty(&transport, ...).await`
/// Calls the `getdifficulty` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn get_difficulty(transport: &dyn TransportTrait) -> Result<Value, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getdifficulty", &params).await?;
    Ok(raw)
}

/// If txid is in the mempool, returns all in-mempool ancestors.
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.getmempoolancestors(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::getmempoolancestors(&transport, ...).await`
/// Calls the `getmempoolancestors` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn get_mempool_ancestors(
    transport: &dyn TransportTrait,
    txid: serde_json::Value,
    verbose: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(txid), json!(verbose)];
    let raw = transport.send_request("getmempoolancestors", &params).await?;
    Ok(raw)
}

/// Returns mempool data for given cluster
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.getmempoolcluster(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::getmempoolcluster(&transport, ...).await`
/// Calls the `getmempoolcluster` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn get_mempool_cluster(
    transport: &dyn TransportTrait,
    txid: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(txid)];
    let raw = transport.send_request("getmempoolcluster", &params).await?;
    Ok(raw)
}

/// If txid is in the mempool, returns all in-mempool descendants.
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.getmempooldescendants(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::getmempooldescendants(&transport, ...).await`
/// Calls the `getmempooldescendants` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn get_mempool_descendants(
    transport: &dyn TransportTrait,
    txid: serde_json::Value,
    verbose: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(txid), json!(verbose)];
    let raw = transport.send_request("getmempooldescendants", &params).await?;
    Ok(raw)
}

/// Returns mempool data for given transaction
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.getmempoolentry(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::getmempoolentry(&transport, ...).await`
/// Calls the `getmempoolentry` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn get_mempool_entry(
    transport: &dyn TransportTrait,
    txid: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(txid)];
    let raw = transport.send_request("getmempoolentry", &params).await?;
    Ok(raw)
}

/// Returns details on the active state of the TX memory pool.
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.getmempoolinfo(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::getmempoolinfo(&transport, ...).await`
/// Calls the `getmempoolinfo` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn get_mempool_info(transport: &dyn TransportTrait) -> Result<Value, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getmempoolinfo", &params).await?;
    Ok(raw)
}

/// Returns all transaction ids in memory pool as a json array of string transaction ids.
/// Hint: use getmempoolentry to fetch a specific transaction from the mempool.
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.getrawmempool(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::getrawmempool(&transport, ...).await`
/// Calls the `getrawmempool` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn get_raw_mempool(
    transport: &dyn TransportTrait,
    verbose: serde_json::Value,
    mempool_sequence: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(verbose), json!(mempool_sequence)];
    let raw = transport.send_request("getrawmempool", &params).await?;
    Ok(raw)
}

/// Returns details about an unspent transaction output.
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.gettxout(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::gettxout(&transport, ...).await`
/// Calls the `gettxout` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn get_tx_out(
    transport: &dyn TransportTrait,
    txid: serde_json::Value,
    n: serde_json::Value,
    include_mempool: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(txid), json!(n), json!(include_mempool)];
    let raw = transport.send_request("gettxout", &params).await?;
    Ok(raw)
}

/// Returns a hex-encoded proof that "txid" was included in a block.
/// NOTE: By default this function only works sometimes. This is when there is an
/// unspent output in the utxo for this transaction. To make it always work,
/// you need to maintain a transaction index, using the -txindex command line option or
/// specify the block in which the transaction is included manually (by blockhash).
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.gettxoutproof(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::gettxoutproof(&transport, ...).await`
/// Calls the `gettxoutproof` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn get_tx_out_proof(
    transport: &dyn TransportTrait,
    txids: serde_json::Value,
    blockhash: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(txids), json!(blockhash)];
    let raw = transport.send_request("gettxoutproof", &params).await?;
    Ok(raw)
}

/// Returns statistics about the unspent transaction output set.
/// Note this call may take some time if you are not using coinstatsindex.
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.gettxoutsetinfo(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::gettxoutsetinfo(&transport, ...).await`
/// Calls the `gettxoutsetinfo` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn get_tx_out_set_info(
    transport: &dyn TransportTrait,
    hash_type: serde_json::Value,
    hash_or_height: serde_json::Value,
    use_index: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(hash_type), json!(hash_or_height), json!(use_index)];
    let raw = transport.send_request("gettxoutsetinfo", &params).await?;
    Ok(raw)
}

/// Scans the mempool to find transactions spending any of the given outputs
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.gettxspendingprevout(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::gettxspendingprevout(&transport, ...).await`
/// Calls the `gettxspendingprevout` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn get_tx_spending_prev_out(
    transport: &dyn TransportTrait,
    outputs: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(outputs)];
    let raw = transport.send_request("gettxspendingprevout", &params).await?;
    Ok(raw)
}

/// Import a mempool.dat file and attempt to add its contents to the mempool.
/// Warning: Importing untrusted files is dangerous, especially if metadata from the file is taken over.
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.importmempool(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::importmempool(&transport, ...).await`
/// Calls the `importmempool` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn import_mempool(
    transport: &dyn TransportTrait,
    filepath: serde_json::Value,
    options: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(filepath), json!(options)];
    let raw = transport.send_request("importmempool", &params).await?;
    Ok(raw)
}

/// Load the serialized UTXO set from a file.
/// Once this snapshot is loaded, its contents will be deserialized into a second chainstate data structure, which is then used to sync to the network\\\'s tip. Meanwhile, the original chainstate will complete the initial block download process in the background, eventually validating up to the block that the snapshot is based upon.
/// The result is a usable bitcoind instance that is current with the network tip in a matter of minutes rather than hours. UTXO snapshot are typically obtained from third-party sources (HTTP, torrent, etc.) which is reasonable since their contents are always checked by hash.
/// You can find more information on this process in the ````assumeutxo```` design document (<https://github.com/bitcoin/bitcoin/blob/master/doc/design/assumeutxo.md>).
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.loadtxoutset(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::loadtxoutset(&transport, ...).await`
/// Calls the `loadtxoutset` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn load_tx_out_set(
    transport: &dyn TransportTrait,
    path: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(path)];
    let raw = transport.send_request("loadtxoutset", &params).await?;
    Ok(raw)
}

/// Treats a block as if it were received before others with the same work.
/// A later preciousblock call can override the effect of an earlier one.
/// The effects of preciousblock are not retained across restarts.
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.preciousblock(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::preciousblock(&transport, ...).await`
/// Calls the `preciousblock` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn precious_block(
    transport: &dyn TransportTrait,
    blockhash: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(blockhash)];
    let raw = transport.send_request("preciousblock", &params).await?;
    Ok(raw)
}

/// Attempts to delete block and undo data up to a specified height or timestamp, if eligible for pruning.
/// Requires ````-prune```` to be enabled at startup. While pruned data may be re-fetched in some cases (e.g., via ````getblockfrompeer````), local deletion is irreversible.
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.pruneblockchain(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::pruneblockchain(&transport, ...).await`
/// Calls the `pruneblockchain` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn prune_blockchain(
    transport: &dyn TransportTrait,
    height: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(height)];
    let raw = transport.send_request("pruneblockchain", &params).await?;
    Ok(raw)
}

/// Dumps the mempool to disk. It will fail until the previous dump is fully loaded.
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.savemempool(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::savemempool(&transport, ...).await`
/// Calls the `savemempool` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn save_mempool(transport: &dyn TransportTrait) -> Result<Value, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("savemempool", &params).await?;
    Ok(raw)
}

/// Return relevant blockhashes for given descriptors (requires blockfilterindex).
/// This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.scanblocks(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::scanblocks(&transport, ...).await`
/// Calls the `scanblocks` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn scan_blocks(
    transport: &dyn TransportTrait,
    action: serde_json::Value,
    scanobjects: serde_json::Value,
    start_height: serde_json::Value,
    stop_height: serde_json::Value,
    filtertype: serde_json::Value,
    options: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![
        json!(action),
        json!(scanobjects),
        json!(start_height),
        json!(stop_height),
        json!(filtertype),
        json!(options),
    ];
    let raw = transport.send_request("scanblocks", &params).await?;
    Ok(raw)
}

/// Scans the unspent transaction output set for entries that match certain output descriptors.
/// Examples of output descriptors are:
/// addr(<address>)                      Outputs whose output script corresponds to the specified address (does not include P2PK)
/// raw(<hex script>)                    Outputs whose output script equals the specified hex-encoded bytes
/// combo(<pubkey>)                      P2PK, P2PKH, P2WPKH, and P2SH-P2WPKH outputs for the given pubkey
/// pkh(<pubkey>)                        P2PKH outputs for the given pubkey
/// sh(multi(<n>,<pubkey>,<pubkey>,...)) P2SH-multisig outputs for the given threshold and pubkeys
/// tr(<pubkey>)                         P2TR
/// tr(<pubkey>,{pk(<pubkey>)})          P2TR with single fallback pubkey in tapscript
/// rawtr(<pubkey>)                      P2TR with the specified key as output key rather than inner
/// wsh(and_v(v:pk(<pubkey>),after(2)))  P2WSH miniscript with mandatory pubkey and a timelock
/// In the above, <pubkey> either refers to a fixed public key in hexadecimal notation, or to an xpub/xprv optionally followed by one
/// or more path elements separated by "/", and optionally ending in "/*" (unhardened), or "/*\\\'" or "/*h" (hardened) to specify all
/// unhardened or hardened child keys.
/// In the latter case, a range needs to be specified by below if different from 1000.
/// For more information on output descriptors, see the documentation in the doc/descriptors.md file.
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.scantxoutset(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::scantxoutset(&transport, ...).await`
/// Calls the `scantxoutset` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn scan_tx_out_set(
    transport: &dyn TransportTrait,
    action: serde_json::Value,
    scanobjects: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(action), json!(scanobjects)];
    let raw = transport.send_request("scantxoutset", &params).await?;
    Ok(raw)
}

/// Verifies blockchain database.
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.verifychain(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::verifychain(&transport, ...).await`
/// Calls the `verifychain` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn verify_chain(
    transport: &dyn TransportTrait,
    checklevel: serde_json::Value,
    nblocks: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(checklevel), json!(nblocks)];
    let raw = transport.send_request("verifychain", &params).await?;
    Ok(raw)
}

/// Verifies that a proof points to a transaction in a block, returning the transaction it commits to
/// and throwing an RPC error if the block is not in our best chain
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.verifytxoutproof(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::verifytxoutproof(&transport, ...).await`
/// Calls the `verifytxoutproof` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn verify_tx_out_proof(
    transport: &dyn TransportTrait,
    proof: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(proof)];
    let raw = transport.send_request("verifytxoutproof", &params).await?;
    Ok(raw)
}

/// Waits for a specific new block and returns useful info about it.
/// Returns the current block on timeout or exit.
/// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.waitforblock(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::waitforblock(&transport, ...).await`
/// Calls the `waitforblock` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn wait_for_block(
    transport: &dyn TransportTrait,
    blockhash: serde_json::Value,
    timeout: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(blockhash), json!(timeout)];
    let raw = transport.send_request("waitforblock", &params).await?;
    Ok(raw)
}

/// Waits for (at least) block height and returns the height and hash
/// of the current tip.
/// Returns the current block on timeout or exit.
/// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.waitforblockheight(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::waitforblockheight(&transport, ...).await`
/// Calls the `waitforblockheight` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn wait_for_block_height(
    transport: &dyn TransportTrait,
    height: serde_json::Value,
    timeout: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(height), json!(timeout)];
    let raw = transport.send_request("waitforblockheight", &params).await?;
    Ok(raw)
}

/// Waits for any new block and returns useful info about it.
/// Returns the current block on timeout or exit.
/// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.waitfornewblock(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::waitfornewblock(&transport, ...).await`
/// Calls the `waitfornewblock` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn wait_for_new_block(
    transport: &dyn TransportTrait,
    timeout: serde_json::Value,
    current_tip: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(timeout), json!(current_tip)];
    let raw = transport.send_request("waitfornewblock", &params).await?;
    Ok(raw)
}
