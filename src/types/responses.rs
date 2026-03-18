//! Generated version-specific RPC response types
//!
//! Generated for Bitcoin Core v30.2
//!
//! These types are version-specific and may not match other versions.
use std::collections::HashMap;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AnalyzePsbtInput {
    /// Whether a UTXO is provided
    pub has_utxo: bool,
    /// Whether the input is finalized
    pub is_final: bool,
    /// Things that are missing that are required to complete this input
    pub missing: Option<serde_json::Value>,
    /// Role of the next person that this input needs to go to
    pub next: Option<String>,
}

/// Things that are missing that are required to complete this input
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AnalyzePsbtMissing {
    #[serde(rename = "pubkeys")]
    pub pub_keys: Option<Vec<String>>,
    pub signatures: Option<Vec<String>>,
    /// Hash160 of the redeem script that is missing
    #[serde(rename = "redeemscript")]
    pub redeem_script: Option<bitcoin::ScriptBuf>,
    /// SHA256 of the witness script that is missing
    #[serde(rename = "witnessscript")]
    pub witness_script: Option<bitcoin::ScriptBuf>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodePsbtBip32Derivs {
    /// The public key with the derivation path as the value.
    pub pubkey: String,
    /// The fingerprint of the master key
    pub master_fingerprint: String,
    /// The path
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodePsbtFinalScriptSig {
    /// Disassembly of the final signature script
    pub asm: String,
    /// The raw final signature script bytes, hex-encoded
    pub hex: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodePsbtGlobalXpubs {
    /// The extended public key this path corresponds to
    pub xpub: String,
    /// The fingerprint of the master key
    pub master_fingerprint: String,
    /// The path
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodePsbtHash160Preimages {
    /// The hash and preimage that corresponds to it.
    pub hash: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodePsbtHash256Preimages {
    /// The hash and preimage that corresponds to it.
    pub hash: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodePsbtInput {
    /// Decoded network transaction for non-witness UTXOs
    pub non_witness_utxo: Option<serde_json::Value>,
    /// Transaction output for witness UTXOs
    pub witness_utxo: Option<serde_json::Value>,
    pub partial_signatures: Option<serde_json::Value>,
    /// The sighash type to be used
    #[serde(rename = "sighash")]
    pub sig_hash: Option<String>,
    pub redeem_script: Option<serde_json::Value>,
    pub witness_script: Option<serde_json::Value>,
    pub bip32_derivs: Option<serde_json::Value>,
    #[serde(rename = "final_scriptSig")]
    pub final_script_sig: Option<serde_json::Value>,
    #[serde(rename = "final_scriptwitness")]
    pub final_script_witness: Option<Vec<String>>,
    pub ripemd160_preimages: Option<serde_json::Value>,
    pub sha256_preimages: Option<serde_json::Value>,
    pub hash160_preimages: Option<serde_json::Value>,
    pub hash256_preimages: Option<serde_json::Value>,
    /// hex-encoded signature for the Taproot key path spend
    pub taproot_key_path_sig: Option<String>,
    pub taproot_script_path_sigs: Option<serde_json::Value>,
    pub taproot_scripts: Option<serde_json::Value>,
    pub taproot_bip32_derivs: Option<serde_json::Value>,
    /// The hex-encoded Taproot x-only internal key
    pub taproot_internal_key: Option<String>,
    /// The hex-encoded Taproot merkle root
    pub taproot_merkle_root: Option<String>,
    pub musig2_participant_pubkeys: Option<serde_json::Value>,
    pub musig2_pubnonces: Option<serde_json::Value>,
    pub musig2_partial_sigs: Option<serde_json::Value>,
    /// The unknown input fields
    pub unknown: Option<serde_json::Value>,
    /// The input proprietary map
    pub proprietary: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodePsbtMusig2PartialSigs {
    /// The compressed public key of the participant that created this partial signature.
    pub participant_pubkey: String,
    /// The compressed aggregate public key for which this partial signature is for.
    pub aggregate_pubkey: String,
    /// The hash of the leaf script that contains the aggregate pubkey being signed for. Omitted when signing for the internal key.
    pub leaf_hash: Option<String>,
    /// The partial signature itself.
    pub partial_sig: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodePsbtMusig2ParticipantPubkeys {
    /// The compressed aggregate public key for which the participants create.
    pub aggregate_pubkey: String,
    pub participant_pubkeys: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodePsbtMusig2Pubnonces {
    /// The compressed public key of the participant that created this pubnonce.
    pub participant_pubkey: String,
    /// The compressed aggregate public key for which this pubnonce is for.
    pub aggregate_pubkey: String,
    /// The hash of the leaf script that contains the aggregate pubkey being signed for. Omitted when signing for the internal key.
    pub leaf_hash: Option<String>,
    /// The public nonce itself.
    #[serde(rename = "pubnonce")]
    pub pub_nonce: String,
}

/// Decoded network transaction for non-witness UTXOs
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodePsbtNonWitnessUtxo {}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodePsbtOutput {
    pub redeem_script: Option<serde_json::Value>,
    pub witness_script: Option<serde_json::Value>,
    pub bip32_derivs: Option<serde_json::Value>,
    /// The hex-encoded Taproot x-only internal key
    pub taproot_internal_key: Option<String>,
    /// The tuples that make up the Taproot tree, in depth first search order
    pub taproot_tree: Option<serde_json::Value>,
    pub taproot_bip32_derivs: Option<serde_json::Value>,
    pub musig2_participant_pubkeys: Option<serde_json::Value>,
    /// The unknown output fields
    pub unknown: Option<serde_json::Value>,
    /// The output proprietary map
    pub proprietary: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodePsbtPartialSignatures {
    /// The public key and signature that corresponds to it.
    pub pubkey: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodePsbtProprietary {
    /// The hex string for the proprietary identifier
    pub identifier: String,
    /// The number for the subtype
    #[serde(rename = "subtype")]
    pub sub_type: u64,
    /// The hex for the key
    pub key: String,
    /// The hex for the value
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodePsbtRedeemScript {
    /// Disassembly of the redeem script
    pub asm: String,
    /// The raw redeem script bytes, hex-encoded
    pub hex: String,
    /// The type, eg 'pubkeyhash'
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodePsbtRipemd160Preimages {
    /// The hash and preimage that corresponds to it.
    pub hash: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodePsbtSha256Preimages {
    /// The hash and preimage that corresponds to it.
    pub hash: String,
}

/// The signature for the pubkey and leaf hash combination
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodePsbtSignature {
    /// The x-only pubkey for this signature
    pub pubkey: String,
    /// The leaf hash for this signature
    pub leaf_hash: String,
    /// The signature itself
    pub sig: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodePsbtTaprootBip32Derivs {
    /// The x-only public key this path corresponds to
    pub pubkey: String,
    /// The fingerprint of the master key
    pub master_fingerprint: String,
    /// The path
    pub path: String,
    /// The hashes of the leaves this pubkey appears in
    pub leaf_hashes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodePsbtTaprootScripts {
    /// A leaf script
    pub script: bitcoin::ScriptBuf,
    /// The version number for the leaf script
    pub leaf_ver: u64,
    /// The control blocks for this script
    pub control_blocks: Vec<String>,
}

/// A single leaf script in the taproot tree
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodePsbtTuple {
    /// The depth of this element in the tree
    pub depth: u64,
    /// The version of this leaf
    pub leaf_ver: u64,
    /// The hex-encoded script itself
    pub script: bitcoin::ScriptBuf,
}

/// The decoded network-serialized unsigned transaction.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodePsbtTx {}

/// The unknown global fields
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodePsbtUnknown {
    /// (key-value pair) An unknown key-value pair
    pub key: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodePsbtWitnessScript {
    /// Disassembly of the witness script
    pub asm: String,
    /// The raw witness script bytes, hex-encoded
    pub hex: String,
    /// The type, eg 'pubkeyhash'
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Transaction output for witness UTXOs
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodePsbtWitnessUtxo {
    /// The value in BTC
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub amount: bitcoin::Amount,
    #[serde(rename = "scriptPubKey")]
    pub script_pubkey: serde_json::Value,
}

/// Result of a witness output script wrapping this redeem script (not returned for types that should not be wrapped)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodeScriptSegwit {
    /// Disassembly of the output script
    pub asm: String,
    /// The raw output script bytes, hex-encoded
    pub hex: String,
    /// The type of the output script (e.g. witness_v0_keyhash or witness_v0_scripthash)
    #[serde(rename = "type")]
    pub r#type: String,
    /// The Bitcoin address (only if a well-defined address exists)
    pub address: Option<String>,
    /// Inferred descriptor for the script
    pub desc: String,
    /// address of the P2SH script wrapping this witness redeem script
    #[serde(rename = "p2sh-segwit")]
    pub p2sh_segwit: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodedScriptPubKey {
    /// Disassembly of the output script
    pub asm: String,
    /// Inferred descriptor for the output
    pub desc: String,
    /// The raw output script bytes, hex-encoded
    pub hex: String,
    /// The type, eg 'pubkeyhash'
    #[serde(rename = "type")]
    pub r#type: String,
    /// The Bitcoin address (only if a well-defined address exists)
    pub address: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct EnumerateSignersSigners {
    /// Master key fingerprint
    pub fingerprint: String,
    /// Device name
    pub name: String,
}

/// information about the highest range of feerates to fail to meet the threshold
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct EstimateRawFeeFail {}

/// estimate for long time horizon
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct EstimateRawFeeLong {}

/// estimate for medium time horizon
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct EstimateRawFeeMedium {}

/// information about the lowest range of feerates to succeed in meeting the threshold
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct EstimateRawFeePass {
    /// start of feerate range
    #[serde(rename = "startrange")]
    pub start_range: u64,
    /// end of feerate range
    #[serde(rename = "endrange")]
    pub end_range: u64,
    /// number of txs over history horizon in the feerate range that were confirmed within target
    #[serde(rename = "withintarget")]
    pub within_target: u64,
    /// number of txs over history horizon in the feerate range that were confirmed at any point
    #[serde(rename = "totalconfirmed")]
    pub total_confirmed: u64,
    /// current number of txs in mempool in the feerate range unconfirmed for at least target blocks
    #[serde(rename = "inmempool")]
    pub in_mempool: u64,
    /// number of txs over history horizon in the feerate range that left mempool unconfirmed after target
    #[serde(rename = "leftmempool")]
    pub left_mempool: u64,
}

/// estimate for short time horizon
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct EstimateRawFeeShort {
    /// estimate fee rate in BTC/kvB
    #[serde(rename = "feerate")]
    pub fee_rate: Option<f64>,
    /// exponential decay (per block) for historical moving average of confirmation data
    pub decay: u64,
    /// The resolution of confirmation targets at this time horizon
    pub scale: u64,
    /// information about the lowest range of feerates to succeed in meeting the threshold
    pub pass: Option<serde_json::Value>,
    /// information about the highest range of feerates to fail to meet the threshold
    pub fail: Option<serde_json::Value>,
    /// Errors encountered during processing (if there are any)
    pub errors: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetAddedNodeInfoAddresses {
    /// The bitcoin server IP and port we're connected to
    pub address: String,
    /// connection, inbound or outbound
    pub connected: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetAddedNodeInfoElement {
    /// The node IP address or name (as provided to addnode)
    #[serde(rename = "addednode")]
    pub added_node: String,
    /// If connected
    pub connected: bool,
    /// Only when connected = true
    pub addresses: serde_json::Value,
}

/// the network (ipv4, ipv6, onion, i2p, cjdns, all_networks)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetAddrManInfoNetwork {
    /// number of addresses in the new table, which represent potential peers the node has discovered but hasn't yet successfully connected to.
    pub new: u64,
    /// number of addresses in the tried table, which represent peers the node has successfully connected to in the past.
    pub tried: u64,
    /// total number of addresses in both new/tried tables
    pub total: u64,
}

/// Information about the address embedded in P2SH or P2WSH, if relevant and known.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetAddressInfoEmbedded {}

/// json object with information about address
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetAddressesByLabelAddress {
    /// Purpose of address ("send" for sending address, "receive" for receiving address)
    pub purpose: String,
}

/// hash and height of the block this information was generated on
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetBalancesLastprocessedblock {
    /// hash of the block this information was generated on
    pub hash: String,
    /// height of the block this information was generated on
    pub height: u64,
}

/// balances from outputs that the wallet can sign
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetBalancesMine {
    /// trusted balance (outputs created by the wallet or confirmed outputs)
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub trusted: bitcoin::Amount,
    /// untrusted pending balance (outputs created by others that are in the mempool)
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub untrusted_pending: bitcoin::Amount,
    /// balance from immature coinbase outputs
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub immature: bitcoin::Amount,
    /// (only present if avoid_reuse is set) balance from coins sent to addresses that were previously spent from (potentially privacy violating)
    #[serde(deserialize_with = "option_amount_from_btc_float")]
    pub used: Option<bitcoin::Amount>,
}

/// Type alias for GetBlockCoinbase
pub type GetBlockCoinbase = String;

/// Type alias for GetBlockStatsFeerate
pub type GetBlockStatsFeerate = String;

/// data that should be included in the coinbase's scriptSig content
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetBlockTemplateCoinbaseaux {
    /// values must be in the coinbase (keys may be ignored)
    pub key: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetBlockTemplateTransactions {
    /// transaction data encoded in hexadecimal (byte-for-byte)
    pub data: String,
    /// transaction hash excluding witness data, shown in byte-reversed hex
    pub txid: bitcoin::Txid,
    /// transaction hash including witness data, shown in byte-reversed hex
    pub hash: String,
    /// array of numbers
    pub depends: Vec<String>,
    /// difference in value between transaction inputs and outputs (in satoshis); for coinbase transactions, this is a negative Number of the total collected block fees (ie, not including the block subsidy); if key is not present, fee is unknown and clients MUST NOT assume there isn't one
    pub fee: f64,
    /// total SigOps cost, as counted for purposes of block limits; if key is not present, sigop cost is unknown and clients MUST NOT assume it is zero
    #[serde(rename = "sigops")]
    pub sig_ops: u64,
    /// total transaction weight, as counted for purposes of block limits
    pub weight: u64,
}

/// set of pending, supported versionbit (BIP 9) softfork deployments
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetBlockTemplateVbavailable {
    /// identifies the bit number as indicating acceptance and readiness for the named softfork rule
    #[serde(rename = "rulename")]
    pub rule_name: u64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetBlockTx {
    /// The transaction fee in BTC, omitted if block undo data is not available
    pub fee: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetChainStatesChainstates {
    /// number of blocks in this chainstate
    pub blocks: u64,
    /// blockhash of the tip
    #[serde(rename = "bestblockhash")]
    pub best_block_hash: String,
    /// nBits: compact representation of the block difficulty target
    pub bits: String,
    /// The difficulty target
    pub target: String,
    /// difficulty of the tip
    pub difficulty: f64,
    /// progress towards the network tip
    #[serde(rename = "verificationprogress")]
    pub verification_progress: f64,
    /// the base block of the snapshot this chainstate is based on, if any
    pub snapshot_blockhash: Option<String>,
    /// size of the coinsdb cache
    pub coins_db_cache_bytes: u64,
    /// size of the coinstip cache
    pub coins_tip_cache_bytes: u64,
    /// whether the chainstate is fully validated. True if all blocks in the chainstate were validated, false if the chain is based on a snapshot and the snapshot has not yet been validated.
    pub validated: bool,
}

/// status of bip9 softforks (only for "bip9" type)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetDeploymentInfoBip9 {
    /// the bit (0-28) in the block version field used to signal this softfork (only for "started" and "locked_in" status)
    pub bit: Option<u64>,
    /// the minimum median time past of a block at which the bit gains its meaning
    pub start_time: u64,
    /// the median time past of a block at which the deployment is considered failed if not yet locked in
    pub timeout: u64,
    /// minimum height of blocks for which the rules may be enforced
    pub min_activation_height: u64,
    /// status of deployment at specified block (one of "defined", "started", "locked_in", "active", "failed")
    pub status: String,
    /// height of the first block to which the status applies
    pub since: u64,
    /// status of deployment at the next block
    pub status_next: String,
    /// numeric statistics about signalling for a softfork (only for "started" and "locked_in" status)
    pub statistics: Option<serde_json::Value>,
    /// indicates blocks that signalled with a # and blocks that did not with a -
    pub signalling: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetDeploymentInfoDeployments {
    /// name of the deployment
    pub xxxx: serde_json::Value,
}

/// numeric statistics about signalling for a softfork (only for "started" and "locked_in" status)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetDeploymentInfoStatistics {
    /// the length in blocks of the signalling period
    pub period: u64,
    /// the number of blocks with the version bit set required to activate the feature (only for "started" status)
    pub threshold: Option<u64>,
    /// the number of blocks elapsed since the beginning of the current period
    pub elapsed: u64,
    /// the number of blocks with the version bit set in the current period
    pub count: u64,
    /// returns false if there are not enough blocks left in this period to pass activation threshold (only for "started" status)
    pub possible: Option<bool>,
}

/// name of the deployment
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetDeploymentInfoXxxx {
    /// one of "buried", "bip9"
    #[serde(rename = "type")]
    pub r#type: String,
    /// height of the first block which the rules are or will be enforced (only for "buried" type, or "bip9" type with "active" status)
    pub height: Option<u64>,
    /// true if the rules are enforced for the mempool and the next block
    pub active: bool,
    /// status of bip9 softforks (only for "bip9" type)
    pub bip9: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetDescriptorActivityActivity {
    /// always 'spend'
    #[serde(rename = "type")]
    pub r#type: String,
    /// The total amount in BTC of the spent output
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub amount: bitcoin::Amount,
    /// The blockhash this spend appears in (omitted if unconfirmed)
    #[serde(rename = "blockhash")]
    pub block_hash: Option<bitcoin::BlockHash>,
    /// Height of the spend (omitted if unconfirmed)
    pub height: Option<u64>,
    /// The txid of the spending transaction
    pub spend_txid: String,
    /// The input index of the spend
    pub spend_vin: u64,
    /// The txid of the prevout
    pub prevout_txid: String,
    /// The vout of the prevout
    pub prevout_vout: u64,
    pub prevout_spk: serde_json::Value,
}

/// Type alias for GetDescriptorActivityOutput
pub type GetDescriptorActivityOutput = String;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetDescriptorActivityPrevoutSpk {
    /// Disassembly of the output script
    pub asm: String,
    /// Inferred descriptor for the output
    pub desc: String,
    /// The raw output script bytes, hex-encoded
    pub hex: String,
    /// The Bitcoin address (only if a well-defined address exists)
    pub address: Option<String>,
    /// The type (one of: nonstandard, anchor, pubkey, pubkeyhash, scripthash, multisig, nulldata, witness_v0_scripthash, witness_v0_keyhash, witness_v1_taproot, witness_unknown)
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetHdKeysDescriptors {
    /// Descriptor string representation
    pub desc: String,
    /// Whether this descriptor is currently used to generate new addresses
    pub active: bool,
}

/// The name of the index
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetIndexInfoName {
    /// Whether the index is synced or not
    pub synced: bool,
    /// The block height to which the index is synced
    pub best_block_height: u64,
}

/// Information about locked memory manager
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetMemoryInfoLocked {
    /// Number of bytes used
    pub used: u64,
    /// Number of bytes available in current arenas
    pub free: u64,
    /// Total number of bytes managed
    pub total: u64,
    /// Amount of bytes that succeeded locking. If this number is smaller than total, locking pages failed at some point and key data could be swapped to disk.
    pub locked: u64,
    /// Number allocated chunks
    pub chunks_used: u64,
    /// Number unused chunks
    pub chunks_free: u64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetMempoolAncestorsFees {
    /// transaction fee, denominated in BTC
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub base: bitcoin::Amount,
    /// transaction fee with fee deltas used for mining priority, denominated in BTC
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub modified: bitcoin::Amount,
    /// transaction fees of in-mempool ancestors (including this one) with fee deltas used for mining priority, denominated in BTC
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub ancestor: bitcoin::Amount,
    /// transaction fees of in-mempool descendants (including this one) with fee deltas used for mining priority, denominated in BTC
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub descendant: bitcoin::Amount,
    /// transaction fees of chunk, denominated in BTC
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub chunk: bitcoin::Amount,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetMempoolAncestorsTransactionid {
    /// virtual transaction size as defined in BIP 141. This is different from actual serialized size for witness transactions as witness data is discounted.
    #[serde(rename = "vsize")]
    pub v_size: u64,
    /// transaction weight as defined in BIP 141.
    pub weight: u64,
    /// local time transaction entered pool in seconds since 1 Jan 1970 GMT
    pub time: u64,
    /// block height when transaction entered pool
    pub height: u64,
    /// number of in-mempool descendant transactions (including this one)
    #[serde(rename = "descendantcount")]
    pub descendant_count: u64,
    /// virtual transaction size of in-mempool descendants (including this one)
    #[serde(rename = "descendantsize")]
    pub descendant_size: u64,
    /// number of in-mempool ancestor transactions (including this one)
    #[serde(rename = "ancestorcount")]
    pub ancestor_count: u64,
    /// virtual transaction size of in-mempool ancestors (including this one)
    #[serde(rename = "ancestorsize")]
    pub ancestor_size: u64,
    /// sigops-adjusted weight (as defined in BIP 141 and modified by '-bytespersigop') of this transaction's chunk
    #[serde(rename = "chunkweight")]
    pub chunk_weight: u64,
    /// hash of serialized transaction, including witness data
    #[serde(rename = "wtxid")]
    pub w_txid: String,
    pub fees: serde_json::Value,
    /// unconfirmed transactions used as inputs for this transaction
    pub depends: Vec<String>,
    /// unconfirmed transactions spending outputs from this transaction
    #[serde(rename = "spentby")]
    pub spent_by: Vec<String>,
    /// Whether this transaction signals BIP125 replaceability or has an unconfirmed ancestor signaling BIP125 replaceability. (DEPRECATED)
    #[serde(rename = "bip125-replaceable")]
    pub bip125_replaceable: bool,
    /// Whether this transaction is currently unbroadcast (initial broadcast not yet acknowledged by any peers)
    pub unbroadcast: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetMempoolDescendantsFees {
    /// transaction fee, denominated in BTC
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub base: bitcoin::Amount,
    /// transaction fee with fee deltas used for mining priority, denominated in BTC
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub modified: bitcoin::Amount,
    /// transaction fees of in-mempool ancestors (including this one) with fee deltas used for mining priority, denominated in BTC
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub ancestor: bitcoin::Amount,
    /// transaction fees of in-mempool descendants (including this one) with fee deltas used for mining priority, denominated in BTC
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub descendant: bitcoin::Amount,
    /// transaction fees of chunk, denominated in BTC
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub chunk: bitcoin::Amount,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetMempoolDescendantsTransactionid {
    /// virtual transaction size as defined in BIP 141. This is different from actual serialized size for witness transactions as witness data is discounted.
    #[serde(rename = "vsize")]
    pub v_size: u64,
    /// transaction weight as defined in BIP 141.
    pub weight: u64,
    /// local time transaction entered pool in seconds since 1 Jan 1970 GMT
    pub time: u64,
    /// block height when transaction entered pool
    pub height: u64,
    /// number of in-mempool descendant transactions (including this one)
    #[serde(rename = "descendantcount")]
    pub descendant_count: u64,
    /// virtual transaction size of in-mempool descendants (including this one)
    #[serde(rename = "descendantsize")]
    pub descendant_size: u64,
    /// number of in-mempool ancestor transactions (including this one)
    #[serde(rename = "ancestorcount")]
    pub ancestor_count: u64,
    /// virtual transaction size of in-mempool ancestors (including this one)
    #[serde(rename = "ancestorsize")]
    pub ancestor_size: u64,
    /// sigops-adjusted weight (as defined in BIP 141 and modified by '-bytespersigop') of this transaction's chunk
    #[serde(rename = "chunkweight")]
    pub chunk_weight: u64,
    /// hash of serialized transaction, including witness data
    #[serde(rename = "wtxid")]
    pub w_txid: String,
    pub fees: serde_json::Value,
    /// unconfirmed transactions used as inputs for this transaction
    pub depends: Vec<String>,
    /// unconfirmed transactions spending outputs from this transaction
    #[serde(rename = "spentby")]
    pub spent_by: Vec<String>,
    /// Whether this transaction signals BIP125 replaceability or has an unconfirmed ancestor signaling BIP125 replaceability. (DEPRECATED)
    #[serde(rename = "bip125-replaceable")]
    pub bip125_replaceable: bool,
    /// Whether this transaction is currently unbroadcast (initial broadcast not yet acknowledged by any peers)
    pub unbroadcast: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetMempoolEntryFees {
    /// transaction fee, denominated in BTC
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub base: bitcoin::Amount,
    /// transaction fee with fee deltas used for mining priority, denominated in BTC
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub modified: bitcoin::Amount,
    /// transaction fees of in-mempool ancestors (including this one) with fee deltas used for mining priority, denominated in BTC
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub ancestor: bitcoin::Amount,
    /// transaction fees of in-mempool descendants (including this one) with fee deltas used for mining priority, denominated in BTC
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub descendant: bitcoin::Amount,
    /// transaction fees of chunk, denominated in BTC
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub chunk: bitcoin::Amount,
}

/// The next block
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetMiningInfoNext {
    /// The next height
    pub height: u64,
    /// The next target nBits
    pub bits: String,
    /// The next difficulty
    pub difficulty: f64,
    /// The next target
    pub target: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetNetTotalsUploadTarget {
    /// Length of the measuring timeframe in seconds
    pub timeframe: u64,
    /// Target in bytes
    pub target: u64,
    /// True if target is reached
    pub target_reached: bool,
    /// True if serving historical blocks
    pub serve_historical_blocks: bool,
    /// Bytes left in current time cycle
    pub bytes_left_in_cycle: u64,
    /// Seconds left in current time cycle
    pub time_left_in_cycle: u64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetNetworkInfoLocalAddresses {
    /// network address
    pub address: String,
    /// network port
    pub port: u16,
    /// relative score
    pub score: u64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetNetworkInfoNetworks {
    /// network (ipv4, ipv6, onion, i2p, cjdns)
    pub name: String,
    /// is the network limited using -onlynet?
    pub limited: bool,
    /// is the network reachable?
    pub reachable: bool,
    /// ("host:port") the proxy that is used for this network, or empty if none
    pub proxy: String,
    /// Whether randomized credentials are used
    pub proxy_randomize_credentials: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetNodeAddressesElement {
    /// The UNIX epoch time when the node was last seen
    pub time: u64,
    /// The services offered by the node
    pub services: u64,
    /// The address of the node
    pub address: String,
    /// The port number of the node
    pub port: u16,
    /// The network (ipv4, ipv6, onion, i2p, cjdns) the node connected through
    pub network: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetPeerInfoBytesRecvPerMsg {
    /// The total bytes received aggregated by message type
    /// When a message type is not listed in this json object, the bytes received are 0.
    /// Only known message types can appear as keys in the object and all bytes received
    /// of unknown message types are listed under '*other*'.
    pub msg: u64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetPeerInfoBytesSentPerMsg {
    /// The total bytes sent aggregated by message type
    /// When a message type is not listed in this json object, the bytes sent are 0.
    /// Only known message types can appear as keys in the object.
    pub msg: u64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetPeerInfoElement {
    /// Peer index
    pub id: u64,
    /// (host:port) The IP address/hostname optionally followed by :port of the peer
    pub addr: String,
    /// (ip:port) Bind address of the connection to the peer
    #[serde(rename = "addrbind")]
    pub addr_bind: Option<String>,
    /// (ip:port) Local address as reported by the peer
    #[serde(rename = "addrlocal")]
    pub addr_local: Option<String>,
    /// Network (ipv4, ipv6, onion, i2p, cjdns, not_publicly_routable)
    pub network: String,
    /// Mapped AS (Autonomous System) number at the end of the BGP route to the peer, used for diversifying
    /// peer selection (only displayed if the -asmap config option is set)
    pub mapped_as: Option<u64>,
    /// The services offered
    pub services: String,
    /// the services offered, in human-readable form
    #[serde(rename = "servicesnames")]
    pub services_names: Vec<String>,
    /// Whether we relay transactions to this peer
    #[serde(rename = "relaytxes")]
    pub relay_txes: bool,
    /// Mempool sequence number of this peer's last INV
    pub last_inv_sequence: u64,
    /// How many txs we have queued to announce to this peer
    pub inv_to_send: u64,
    /// The UNIX epoch time of the last send
    #[serde(rename = "lastsend")]
    pub last_send: u64,
    /// The UNIX epoch time of the last receive
    #[serde(rename = "lastrecv")]
    pub last_recv: u64,
    /// The UNIX epoch time of the last valid transaction received from this peer
    pub last_transaction: u64,
    /// The UNIX epoch time of the last block received from this peer
    pub last_block: u64,
    /// The total bytes sent
    #[serde(rename = "bytessent")]
    pub bytes_sent: u64,
    /// The total bytes received
    #[serde(rename = "bytesrecv")]
    pub bytes_recv: u64,
    /// The UNIX epoch time of the connection
    #[serde(rename = "conntime")]
    pub conn_time: u64,
    /// The time offset in seconds
    #[serde(rename = "timeoffset")]
    pub time_offset: u64,
    /// The last ping time in seconds, if any
    #[serde(rename = "pingtime")]
    pub ping_time: Option<u64>,
    /// The minimum observed ping time in seconds, if any
    #[serde(rename = "minping")]
    pub min_ping: Option<u64>,
    /// The duration in seconds of an outstanding ping (if non-zero)
    #[serde(rename = "pingwait")]
    pub ping_wait: Option<u64>,
    /// The peer version, such as 70001
    pub version: u32,
    /// The string version
    pub subver: String,
    /// Inbound (true) or Outbound (false)
    pub inbound: bool,
    /// Whether we selected peer as (compact blocks) high-bandwidth peer
    pub bip152_hb_to: bool,
    /// Whether peer selected us as (compact blocks) high-bandwidth peer
    pub bip152_hb_from: bool,
    /// (DEPRECATED, returned only if config option -deprecatedrpc=startingheight is passed) The starting height (block) of the peer
    #[serde(rename = "startingheight")]
    pub starting_height: Option<u64>,
    /// The current height of header pre-synchronization with this peer, or -1 if no low-work sync is in progress
    pub presynced_headers: u64,
    /// The last header we have in common with this peer
    pub synced_headers: u64,
    /// The last block we have in common with this peer
    pub synced_blocks: u64,
    #[serde(rename = "inflight")]
    pub in_flight: Vec<String>,
    /// Whether we participate in address relay with this peer
    pub addr_relay_enabled: bool,
    /// The total number of addresses processed, excluding those dropped due to rate limiting
    pub addr_processed: u64,
    /// The total number of addresses dropped due to rate limiting
    pub addr_rate_limited: u64,
    /// Any special permissions that have been granted to this peer
    pub permissions: Vec<String>,
    /// The minimum fee rate for transactions this peer accepts
    #[serde(rename = "minfeefilter")]
    pub min_fee_filter: u64,
    #[serde(rename = "bytessent_per_msg")]
    pub bytes_sent_per_msg: std::collections::HashMap<String, u64>,
    #[serde(rename = "bytesrecv_per_msg")]
    pub bytes_recv_per_msg: std::collections::HashMap<String, u64>,
    /// Type of connection:
    /// outbound-full-relay (default automatic connections),
    /// block-relay-only (does not relay transactions or addresses),
    /// inbound (initiated by the peer),
    /// manual (added via addnode RPC or -addnode/-connect configuration options),
    /// addr-fetch (short-lived automatic connection for soliciting addresses),
    /// feeler (short-lived automatic connection for testing addresses),
    /// private-broadcast (short-lived automatic connection for broadcasting privacy-sensitive transactions).
    /// Please note this output is unlikely to be stable in upcoming releases as we iterate to
    /// best capture connection behaviors.
    pub connection_type: String,
    /// Type of transport protocol:
    /// detecting (peer could be v1 or v2),
    /// v1 (plaintext transport protocol),
    /// v2 (BIP324 encrypted transport protocol).
    pub transport_protocol_type: String,
    /// The session ID for this connection, or "" if there is none ("v2" transport protocol only).
    pub session_id: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetPrioritisedTransactionsTransactionId {
    /// transaction fee delta in satoshis
    pub fee_delta: u64,
    /// whether this transaction is currently in mempool
    pub in_mempool: bool,
    /// modified fee in satoshis. Only returned if in_mempool=true
    pub modified_fee: Option<u64>,
}

/// Type alias for GetRawAddrManBucket
pub type GetRawAddrManBucket = String;

/// buckets with addresses in the address manager table ( new, tried )
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetRawAddrManTable {
    /// the location in the address manager table (&lt;bucket&gt;/&lt;position&gt;)
    #[serde(rename = "bucket/position")]
    pub bucket_position: serde_json::Value,
}

/// Type alias for GetRpcInfoActive
pub type GetRpcInfoActive = String;

/// The decoded transaction (only present when `verbose` is passed)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetTransactionDecoded {
    /// The transaction id
    pub txid: bitcoin::Txid,
    /// The transaction hash (differs from txid for witness transactions)
    pub hash: String,
    /// The serialized transaction size
    pub size: u64,
    /// The virtual transaction size (differs from size for witness transactions)
    #[serde(rename = "vsize")]
    pub v_size: u64,
    /// The transaction's weight (between vsize*4-3 and vsize*4)
    pub weight: u64,
    /// The version
    pub version: u32,
    /// The lock time
    #[serde(rename = "locktime")]
    pub lock_time: u64,
    pub vin: Vec<DecodedVin>,
    pub vout: Vec<DecodedVout>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetTransactionDetails {
    /// The bitcoin address involved in the transaction.
    pub address: Option<String>,
    /// The transaction category.
    /// "send"                  Transactions sent.
    /// "receive"               Non-coinbase transactions received.
    /// "generate"              Coinbase transactions received with more than 100 confirmations.
    /// "immature"              Coinbase transactions received with 100 or fewer confirmations.
    /// "orphan"                Orphaned coinbase transactions received.
    pub category: String,
    /// The amount in BTC
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub amount: bitcoin::Amount,
    /// A comment for the address/transaction, if any
    pub label: Option<String>,
    /// the vout value
    pub vout: u64,
    /// The amount of the fee in BTC. This is negative and only available for the
    /// 'send' category of transactions.
    #[serde(deserialize_with = "option_amount_from_btc_float")]
    pub fee: Option<bitcoin::Amount>,
    /// 'true' if the transaction has been abandoned (inputs are respendable).
    pub abandoned: bool,
    /// Only if 'category' is 'received'. List of parent descriptors for the output script of this coin.
    pub parent_descs: Option<Vec<String>>,
}

/// hash and height of the block this information was generated on
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetTransactionLastprocessedblock {
    /// hash of the block this information was generated on
    pub hash: String,
    /// height of the block this information was generated on
    pub height: u64,
}

/// Type alias for GetTxOutSetInfoBlock
pub type GetTxOutSetInfoBlock = String;

/// Detailed view of the unspendable categories
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetTxOutSetInfoUnspendables {
    /// The unspendable amount of the Genesis block subsidy
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub genesis_block: bitcoin::Amount,
    /// Transactions overridden by duplicates (no longer possible with BIP30)
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub bip30: bitcoin::Amount,
    /// Amounts sent to scripts that are unspendable (for example OP_RETURN outputs)
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub scripts: bitcoin::Amount,
    /// Fee rewards that miners did not claim in their coinbase transaction
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub unclaimed_rewards: bitcoin::Amount,
}

/// hash and height of the block this information was generated on
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetWalletInfoLastprocessedblock {
    /// hash of the block this information was generated on
    pub hash: String,
    /// height of the block this information was generated on
    pub height: u64,
}

/// current scanning details, or false if no scan is in progress
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetWalletInfoScanning {
    /// elapsed seconds since scan start
    pub duration: u64,
    /// scanning progress percentage \[0.0, 1.0\]
    pub progress: u64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ImportDescriptorsError {}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ListBannedElement {
    /// The IP/Subnet of the banned node
    pub address: String,
    /// The UNIX epoch time the ban was created
    pub ban_created: u64,
    /// The UNIX epoch time the ban expires
    pub banned_until: u64,
    /// The ban duration, in seconds
    pub ban_duration: u64,
    /// The time remaining until the ban expires, in seconds
    pub time_remaining: u64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ListDescriptorsDescriptors {
    /// Descriptor string representation
    pub desc: String,
    /// The creation time of the descriptor
    pub timestamp: u64,
    /// Whether this descriptor is currently used to generate new addresses
    pub active: bool,
    /// True if this descriptor is used to generate change addresses. False if this descriptor is used to generate receiving addresses; defined only for active descriptors
    pub internal: Option<bool>,
    /// Defined only for ranged descriptors
    pub range: Option<serde_json::Value>,
    /// Same as next_index field. Kept for compatibility reason.
    pub next: Option<u64>,
    /// The next index to generate addresses from; defined only for ranged descriptors
    pub next_index: Option<u64>,
}

/// Defined only for ranged descriptors
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ListDescriptorsRange {
    /// Range start inclusive
    pub field_0: u64,
    /// Range end inclusive
    pub field_1: u64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ListLockUnspentElement {
    /// The transaction id locked
    pub txid: bitcoin::Txid,
    /// The vout value
    pub vout: u64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ListReceivedByAddressElement {
    /// The receiving address
    pub address: String,
    /// The total amount in BTC received by the address
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub amount: bitcoin::Amount,
    /// The number of confirmations of the most recent transaction included
    pub confirmations: i64,
    /// The label of the receiving address. The default label is ""
    pub label: String,
    pub txids: Vec<bitcoin::Txid>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ListReceivedByLabelElement {
    /// The total amount received by addresses with this label
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub amount: bitcoin::Amount,
    /// The number of confirmations of the most recent transaction included
    pub confirmations: i64,
    /// The label of the receiving address. The default label is ""
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ListSinceBlockTransactions {
    /// The bitcoin address of the transaction (not returned if the output does not have an address, e.g. OP_RETURN null data).
    pub address: Option<String>,
    /// The transaction category.
    /// "send"                  Transactions sent.
    /// "receive"               Non-coinbase transactions received.
    /// "generate"              Coinbase transactions received with more than 100 confirmations.
    /// "immature"              Coinbase transactions received with 100 or fewer confirmations.
    /// "orphan"                Orphaned coinbase transactions received.
    pub category: String,
    /// The amount in BTC. This is negative for the 'send' category, and is positive
    /// for all other categories
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub amount: bitcoin::Amount,
    /// the vout value
    pub vout: u64,
    /// The amount of the fee in BTC. This is negative and only available for the
    /// 'send' category of transactions.
    #[serde(deserialize_with = "option_amount_from_btc_float")]
    pub fee: Option<bitcoin::Amount>,
    /// The number of confirmations for the transaction. Negative confirmations means the
    /// transaction conflicted that many blocks ago.
    pub confirmations: i64,
    /// Only present if the transaction's only input is a coinbase one.
    pub generated: Option<bool>,
    /// Whether we consider the transaction to be trusted and safe to spend from.
    /// Only present when the transaction has 0 confirmations (or negative confirmations, if conflicted).
    pub trusted: Option<bool>,
    /// The block hash containing the transaction.
    #[serde(rename = "blockhash")]
    pub block_hash: Option<bitcoin::BlockHash>,
    /// The block height containing the transaction.
    #[serde(rename = "blockheight")]
    pub block_height: Option<u64>,
    /// The index of the transaction in the block that includes it.
    #[serde(rename = "blockindex")]
    pub block_index: Option<u64>,
    /// The block time expressed in UNIX epoch time.
    #[serde(rename = "blocktime")]
    pub block_time: Option<u64>,
    /// The transaction id.
    pub txid: bitcoin::Txid,
    /// The hash of serialized transaction, including witness data.
    #[serde(rename = "wtxid")]
    pub w_txid: String,
    /// Confirmed transactions that have been detected by the wallet to conflict with this transaction.
    #[serde(rename = "walletconflicts")]
    pub wallet_conflicts: Vec<String>,
    /// Only if 'category' is 'send'. The txid if this tx was replaced.
    pub replaced_by_txid: Option<String>,
    /// Only if 'category' is 'send'. The txid if this tx replaces another.
    pub replaces_txid: Option<String>,
    /// Transactions in the mempool that directly conflict with either this transaction or an ancestor transaction
    #[serde(rename = "mempoolconflicts")]
    pub mempool_conflicts: Vec<String>,
    /// If a comment to is associated with the transaction.
    pub to: Option<String>,
    /// The transaction time expressed in UNIX epoch time.
    pub time: u64,
    /// The time received expressed in UNIX epoch time.
    #[serde(rename = "timereceived")]
    pub time_received: u64,
    /// If a comment is associated with the transaction, only present if not empty.
    pub comment: Option<String>,
    /// ("yes|no|unknown") Whether this transaction signals BIP125 replaceability or has an unconfirmed ancestor signaling BIP125 replaceability.
    /// May be unknown for unconfirmed transactions not in the mempool because their unconfirmed ancestors are unknown.
    #[serde(rename = "bip125-replaceable")]
    pub bip125_replaceable: String,
    /// Only if 'category' is 'received'. List of parent descriptors for the output script of this coin.
    pub parent_descs: Option<Vec<String>>,
    /// 'true' if the transaction has been abandoned (inputs are respendable).
    pub abandoned: bool,
    /// A comment for the address/transaction, if any
    pub label: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ListTransactionsElement {
    /// The bitcoin address of the transaction (not returned if the output does not have an address, e.g. OP_RETURN null data).
    pub address: Option<String>,
    /// The transaction category.
    /// "send"                  Transactions sent.
    /// "receive"               Non-coinbase transactions received.
    /// "generate"              Coinbase transactions received with more than 100 confirmations.
    /// "immature"              Coinbase transactions received with 100 or fewer confirmations.
    /// "orphan"                Orphaned coinbase transactions received.
    pub category: String,
    /// The amount in BTC. This is negative for the 'send' category, and is positive
    /// for all other categories
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub amount: bitcoin::Amount,
    /// A comment for the address/transaction, if any
    pub label: Option<String>,
    /// the vout value
    pub vout: u64,
    /// The amount of the fee in BTC. This is negative and only available for the
    /// 'send' category of transactions.
    #[serde(deserialize_with = "option_amount_from_btc_float")]
    pub fee: Option<bitcoin::Amount>,
    /// The number of confirmations for the transaction. Negative confirmations means the
    /// transaction conflicted that many blocks ago.
    pub confirmations: i64,
    /// Only present if the transaction's only input is a coinbase one.
    pub generated: Option<bool>,
    /// Whether we consider the transaction to be trusted and safe to spend from.
    /// Only present when the transaction has 0 confirmations (or negative confirmations, if conflicted).
    pub trusted: Option<bool>,
    /// The block hash containing the transaction.
    #[serde(rename = "blockhash")]
    pub block_hash: Option<bitcoin::BlockHash>,
    /// The block height containing the transaction.
    #[serde(rename = "blockheight")]
    pub block_height: Option<u64>,
    /// The index of the transaction in the block that includes it.
    #[serde(rename = "blockindex")]
    pub block_index: Option<u64>,
    /// The block time expressed in UNIX epoch time.
    #[serde(rename = "blocktime")]
    pub block_time: Option<u64>,
    /// The transaction id.
    pub txid: bitcoin::Txid,
    /// The hash of serialized transaction, including witness data.
    #[serde(rename = "wtxid")]
    pub w_txid: String,
    /// Confirmed transactions that have been detected by the wallet to conflict with this transaction.
    #[serde(rename = "walletconflicts")]
    pub wallet_conflicts: Vec<String>,
    /// Only if 'category' is 'send'. The txid if this tx was replaced.
    pub replaced_by_txid: Option<String>,
    /// Only if 'category' is 'send'. The txid if this tx replaces another.
    pub replaces_txid: Option<String>,
    /// Transactions in the mempool that directly conflict with either this transaction or an ancestor transaction
    #[serde(rename = "mempoolconflicts")]
    pub mempool_conflicts: Vec<String>,
    /// If a comment to is associated with the transaction.
    pub to: Option<String>,
    /// The transaction time expressed in UNIX epoch time.
    pub time: u64,
    /// The time received expressed in UNIX epoch time.
    #[serde(rename = "timereceived")]
    pub time_received: u64,
    /// If a comment is associated with the transaction, only present if not empty.
    pub comment: Option<String>,
    /// ("yes|no|unknown") Whether this transaction signals BIP125 replaceability or has an unconfirmed ancestor signaling BIP125 replaceability.
    /// May be unknown for unconfirmed transactions not in the mempool because their unconfirmed ancestors are unknown.
    #[serde(rename = "bip125-replaceable")]
    pub bip125_replaceable: String,
    /// Only if 'category' is 'received'. List of parent descriptors for the output script of this coin.
    pub parent_descs: Option<Vec<String>>,
    /// 'true' if the transaction has been abandoned (inputs are respendable).
    pub abandoned: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ListUnspentElement {
    /// the transaction id
    pub txid: bitcoin::Txid,
    /// the vout value
    pub vout: u64,
    /// the bitcoin address
    pub address: Option<String>,
    /// The associated label, or "" for the default label
    pub label: Option<String>,
    /// the output script
    #[serde(rename = "scriptPubKey")]
    pub script_pubkey: bitcoin::ScriptBuf,
    /// the transaction output amount in BTC
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub amount: bitcoin::Amount,
    /// The number of confirmations
    pub confirmations: i64,
    /// The number of in-mempool ancestor transactions, including this one (if transaction is in the mempool)
    #[serde(rename = "ancestorcount")]
    pub ancestor_count: Option<u64>,
    /// The virtual transaction size of in-mempool ancestors, including this one (if transaction is in the mempool)
    #[serde(rename = "ancestorsize")]
    pub ancestor_size: Option<u64>,
    /// The total fees of in-mempool ancestors (including this one) with fee deltas used for mining priority in sat (if transaction is in the mempool)
    #[serde(rename = "ancestorfees")]
    #[serde(deserialize_with = "option_amount_from_btc_float")]
    pub ancestor_fees: Option<bitcoin::Amount>,
    /// The redeem script if the output script is P2SH
    #[serde(rename = "redeemScript")]
    pub redeem_script: Option<bitcoin::ScriptBuf>,
    /// witness script if the output script is P2WSH or P2SH-P2WSH
    #[serde(rename = "witnessScript")]
    pub witness_script: Option<bitcoin::ScriptBuf>,
    /// (DEPRECATED) Always true
    pub spendable: bool,
    /// Whether we know how to spend this output, ignoring the lack of keys
    pub solvable: bool,
    /// (only present if avoid_reuse is set) Whether this output is reused/dirty (sent to an address that was previously spent from)
    pub reused: Option<bool>,
    /// (only when solvable) A descriptor for spending this output
    pub desc: Option<String>,
    /// List of parent descriptors for the output script of this coin.
    pub parent_descs: Vec<String>,
    /// Whether this output is considered safe to spend. Unconfirmed transactions
    /// from outside keys and unconfirmed replacement transactions are considered unsafe
    /// and are not eligible for spending by fundrawtransaction and sendtoaddress.
    pub safe: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ListWalletDirWallets {
    /// The wallet name
    pub name: String,
    /// Warning messages, if any, related to loading the wallet.
    pub warnings: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ScanTxOutSetUnspents {
    /// The transaction id
    pub txid: bitcoin::Txid,
    /// The vout value
    pub vout: u64,
    /// The output script
    #[serde(rename = "scriptPubKey")]
    pub script_pubkey: bitcoin::ScriptBuf,
    /// A specialized descriptor for the matched output script
    pub desc: String,
    /// The total amount in BTC of the unspent output
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub amount: bitcoin::Amount,
    /// Whether this is a coinbase output
    pub coinbase: bool,
    /// Height of the unspent transaction output
    pub height: u64,
    /// Blockhash of the unspent transaction output
    #[serde(rename = "blockhash")]
    pub block_hash: bitcoin::BlockHash,
    /// Number of confirmations of the unspent transaction output when the scan was done
    pub confirmations: i64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SignRawTransactionWithKeyErrors {
    /// The hash of the referenced, previous transaction
    pub txid: bitcoin::Txid,
    /// The index of the output to spent and used as input
    pub vout: u64,
    pub witness: Vec<String>,
    /// The hex-encoded signature script
    #[serde(rename = "scriptSig")]
    pub script_sig: String,
    /// Script sequence number
    pub sequence: u64,
    /// Verification or signing error related to the input
    pub error: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SignRawTransactionWithWalletErrors {
    /// The hash of the referenced, previous transaction
    pub txid: bitcoin::Txid,
    /// The index of the output to spent and used as input
    pub vout: u64,
    pub witness: Vec<String>,
    /// The hex-encoded signature script
    #[serde(rename = "scriptSig")]
    pub script_sig: String,
    /// Script sequence number
    pub sequence: u64,
    /// Verification or signing error related to the input
    pub error: String,
}

/// Transaction fees
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SubmitPackageFees {
    /// transaction fee in BTC
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub base: bitcoin::Amount,
    /// if the transaction was not already in the mempool, the effective feerate in BTC per KvB. For example, the package feerate and/or feerate with modified fees from prioritisetransaction.
    #[serde(rename = "effective-feerate")]
    #[serde(deserialize_with = "option_amount_from_btc_float")]
    pub effective_feerate: Option<bitcoin::Amount>,
    /// if effective-feerate is provided, the wtxids of the transactions whose fees and vsizes are included in effective-feerate.
    #[serde(rename = "effective-includes")]
    pub effective_includes: Option<Vec<String>>,
}

/// The transaction results keyed by wtxid. An entry is returned for every submitted wtxid.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SubmitPackageTxResults {
    /// transaction wtxid
    #[serde(rename = "wtxid")]
    pub w_txid: serde_json::Value,
}

/// transaction wtxid
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SubmitPackageWtxid {
    /// The transaction hash in hex
    pub txid: bitcoin::Txid,
    /// The wtxid of a different transaction with the same txid but different witness found in the mempool. This means the submitted transaction was ignored.
    #[serde(rename = "other-wtxid")]
    pub other_wtxid: Option<String>,
    /// Sigops-adjusted virtual transaction size.
    #[serde(rename = "vsize")]
    pub v_size: Option<u64>,
    /// Transaction fees
    pub fees: Option<serde_json::Value>,
    /// Error string if rejected from mempool, or "package-not-validated" when the package aborts before any per-tx processing.
    pub error: Option<String>,
}

/// Transaction fees (only present if 'allowed' is true)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct TestMempoolAcceptFees {
    /// transaction fee in BTC
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub base: bitcoin::Amount,
    /// the effective feerate in BTC per KvB. May differ from the base feerate if, for example, there are modified fees from prioritisetransaction or a package feerate was used.
    #[serde(rename = "effective-feerate")]
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub effective_feerate: bitcoin::Amount,
    /// transactions whose fees and vsizes are included in effective-feerate.
    #[serde(rename = "effective-includes")]
    pub effective_includes: Vec<String>,
}

/// Script sig in decoded tx input.
/// See: <https://github.com/bitcoin/bitcoin/blob/744d47fcee0d32a71154292699bfdecf954a6065/src/core_io.cpp#L458-L461>
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodedScriptSig {
    /// scriptSig in human-readable assembly form.
    pub asm: String,
    /// scriptSig serialized as hex.
    pub hex: String,
}

/// Previous output (prevout) in decoded tx input; present for getblock verbosity 3.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodedPrevout {
    /// True if the prevout was created by a coinbase transaction.
    pub generated: bool,
    /// Block height where the prevout was created.
    pub height: i64,
    /// Decoded script pubkey of the prevout output.
    #[serde(rename = "scriptPubKey")]
    pub script_pubkey: DecodedScriptPubKey,
    /// Value of the prevout output in BTC.
    pub value: f64,
}

/// Transaction input in decoded tx; prevout is None for getblock verbosity 2, Some for verbosity 3.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodedVin {
    /// Transaction id of the previous output being spent.
    pub txid: String,
    /// Index of the previous output being spent.
    pub vout: u32,
    /// Decoded scriptSig for this input, when present.
    #[serde(rename = "scriptSig", default, skip_serializing_if = "Option::is_none")]
    pub script_sig: Option<DecodedScriptSig>,
    /// Input sequence number.
    pub sequence: u64,
    /// Witness stack items for this input (if any).
    #[serde(rename = "txinwitness", default, skip_serializing_if = "Option::is_none")]
    pub tx_in_witness: Option<Vec<String>>,
    /// Decoded details of the previous output when verbosity includes prevout.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prevout: Option<DecodedPrevout>,
}

/// Transaction output in decoded tx; mirrors Core vout object.
/// See: <https://github.com/bitcoin/bitcoin/blob/744d47fcee0d32a71154292699bfdecf954a6065/src/core_io.cpp#L495-L519>
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodedVout {
    /// Value in BTC of this output.
    pub value: f64,
    /// Index of this output within the transaction.
    pub n: u32,
    /// Decoded script pubkey of this output.
    #[serde(rename = "scriptPubKey")]
    pub script_pubkey: DecodedScriptPubKey,
}

/// Decoded transaction details (getblock verbosity 2/3 and getrawtransaction verbose).
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodedTxDetails {
    /// Transaction id.
    pub txid: String,
    /// Witness transaction id (wtxid).
    pub hash: String,
    /// Transaction version.
    pub version: i32,
    /// Total serialized size of the transaction in bytes.
    pub size: u32,
    /// Virtual transaction size (vsize) as defined in BIP 141.
    pub vsize: u32,
    /// Transaction weight as defined in BIP 141.
    pub weight: u32,
    /// Transaction locktime.
    pub locktime: u32,
    /// List of transaction inputs.
    pub vin: Vec<DecodedVin>,
    /// List of transaction outputs.
    pub vout: Vec<DecodedVout>,
    /// Fee paid by the transaction, when undo data is available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fee: Option<f64>,
    /// Raw transaction serialized as hex (consistent with getrawtransaction verbose output).
    pub hex: String,
}

/// Hex-encoded block data returned by getblock with verbosity 0.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetBlockV0 {
    /// Serialized block as a hex string.
    pub hex: String,
}

/// Verbose block view with decoded transactions (built from getblock verbosities 1 and 2).
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetBlockWithTxsResponse {
    /// Block header and summary information from getblock verbosity 1.
    pub base: GetBlockResponse,
    /// Fully decoded transactions in the block, matching getblock verbosity 2.
    pub decoded_txs: Vec<DecodedTxDetails>,
}

/// Verbose block view with decoded transactions and prevout metadata (getblock verbosity 3).
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetBlockWithPrevoutResponse {
    /// Verbose block view with prevout-rich inputs; wraps the verbosity-2 representation.
    pub inner: GetBlockWithTxsResponse,
}

/// One transaction entry in getblocktemplate "transactions" array (BIP 22/23/145).
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetBlockTemplateTransaction {
    /// Transaction data encoded in hexadecimal (byte-for-byte).
    pub data: String,
    /// 1-based indexes of transactions in the 'transactions' list that must be present before this one.
    pub depends: Vec<i64>,
    /// Difference in value between inputs and outputs (satoshis); absent when unknown.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fee: Option<i64>,
    /// Transaction hash including witness data (byte-reversed hex).
    pub hash: String,
    /// Total SigOps cost for block limits; absent when unknown.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sigops: Option<i64>,
    /// Transaction hash excluding witness data (byte-reversed hex).
    pub txid: String,
    /// Total transaction weight for block limits.
    pub weight: i64,
}

/// Response for the `AbandonTransaction` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct AbandonTransactionResponse {
    /// Wrapped primitive value
    pub value: (),
}

impl<'de> serde::Deserialize<'de> for AbandonTransactionResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = AbandonTransactionResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(AbandonTransactionResponse { value: () })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(AbandonTransactionResponse { value: () })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(AbandonTransactionResponse { value: () })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(AbandonTransactionResponse { value: () })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(AbandonTransactionResponse { value: () })
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(AbandonTransactionResponse { value: () })
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(AbandonTransactionResponse { value: () })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value::<()>()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(AbandonTransactionResponse { value: () })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for AbandonTransactionResponse {
    type Target = ();
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for AbandonTransactionResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<()> for AbandonTransactionResponse {
    fn as_ref(&self) -> &() { &self.value }
}

impl From<()> for AbandonTransactionResponse {
    fn from(value: ()) -> Self { Self { value } }
}

impl From<AbandonTransactionResponse> for () {
    fn from(wrapper: AbandonTransactionResponse) -> Self { wrapper.value }
}

/// Response for the `AbortRescan` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct AbortRescanResponse {
    /// Wrapped primitive value
    pub value: bool,
}

impl<'de> serde::Deserialize<'de> for AbortRescanResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = AbortRescanResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(AbortRescanResponse { value: v != 0 })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(AbortRescanResponse { value: v != 0 })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(AbortRescanResponse { value: v != 0.0 })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let value = v.parse::<bool>().map_err(de::Error::custom)?;
                Ok(AbortRescanResponse { value })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(AbortRescanResponse { value: v })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(AbortRescanResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for AbortRescanResponse {
    type Target = bool;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for AbortRescanResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<bool> for AbortRescanResponse {
    fn as_ref(&self) -> &bool { &self.value }
}

impl From<bool> for AbortRescanResponse {
    fn from(value: bool) -> Self { Self { value } }
}

impl From<AbortRescanResponse> for bool {
    fn from(wrapper: AbortRescanResponse) -> Self { wrapper.value }
}

/// Response for the `AddConnection` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct AddConnectionResponse {
    /// Address of newly added connection.
    pub address: String,
    /// Type of connection opened.
    pub connection_type: String,
}

/// Response for the `AddNode` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct AddNodeResponse {
    /// Wrapped primitive value
    pub value: (),
}

impl<'de> serde::Deserialize<'de> for AddNodeResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = AddNodeResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(AddNodeResponse { value: () })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(AddNodeResponse { value: () })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(AddNodeResponse { value: () })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(AddNodeResponse { value: () })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(AddNodeResponse { value: () })
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(AddNodeResponse { value: () })
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(AddNodeResponse { value: () })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value::<()>()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(AddNodeResponse { value: () })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for AddNodeResponse {
    type Target = ();
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for AddNodeResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<()> for AddNodeResponse {
    fn as_ref(&self) -> &() { &self.value }
}

impl From<()> for AddNodeResponse {
    fn from(value: ()) -> Self { Self { value } }
}

impl From<AddNodeResponse> for () {
    fn from(wrapper: AddNodeResponse) -> Self { wrapper.value }
}

/// Response for the `AddPeerAddress` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct AddPeerAddressResponse {
    /// whether the peer address was successfully added to the address manager table
    pub success: bool,
    /// error description, if the address could not be added
    pub error: Option<String>,
}

/// Response for the `AnalyzePsbt` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct AnalyzePsbtResponse {
    pub inputs: Option<serde_json::Value>,
    /// Estimated vsize of the final signed transaction
    pub estimated_vsize: Option<u64>,
    /// Estimated feerate of the final signed transaction in BTC/kvB. Shown only if all UTXO slots in the PSBT have been filled
    pub estimated_feerate: Option<f64>,
    /// The transaction fee paid. Shown only if all UTXO slots in the PSBT have been filled
    #[serde(default)]
    #[serde(deserialize_with = "option_amount_from_btc_float")]
    pub fee: Option<bitcoin::Amount>,
    /// Role of the next person that this psbt needs to go to
    pub next: String,
    /// Error message (if there is one)
    pub error: Option<String>,
}

/// Response for the `BackupWallet` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct BackupWalletResponse {
    /// Wrapped primitive value
    pub value: (),
}

impl<'de> serde::Deserialize<'de> for BackupWalletResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = BackupWalletResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(BackupWalletResponse { value: () })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(BackupWalletResponse { value: () })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(BackupWalletResponse { value: () })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(BackupWalletResponse { value: () })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(BackupWalletResponse { value: () })
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(BackupWalletResponse { value: () })
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(BackupWalletResponse { value: () })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value::<()>()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(BackupWalletResponse { value: () })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for BackupWalletResponse {
    type Target = ();
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for BackupWalletResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<()> for BackupWalletResponse {
    fn as_ref(&self) -> &() { &self.value }
}

impl From<()> for BackupWalletResponse {
    fn from(value: ()) -> Self { Self { value } }
}

impl From<BackupWalletResponse> for () {
    fn from(wrapper: BackupWalletResponse) -> Self { wrapper.value }
}

/// Response for the `BumpFee` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct BumpFeeResponse {
    /// The id of the new transaction.
    pub txid: bitcoin::Txid,
    /// The fee of the replaced transaction.
    #[serde(rename = "origfee")]
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub orig_fee: bitcoin::Amount,
    /// The fee of the new transaction.
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub fee: bitcoin::Amount,
    /// Errors encountered during processing (may be empty).
    pub errors: Vec<String>,
}

/// Response for the `ClearBanned` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ClearBannedResponse {
    /// Wrapped primitive value
    pub value: (),
}

impl<'de> serde::Deserialize<'de> for ClearBannedResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = ClearBannedResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ClearBannedResponse { value: () })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ClearBannedResponse { value: () })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ClearBannedResponse { value: () })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ClearBannedResponse { value: () })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ClearBannedResponse { value: () })
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ClearBannedResponse { value: () })
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ClearBannedResponse { value: () })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value::<()>()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(ClearBannedResponse { value: () })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for ClearBannedResponse {
    type Target = ();
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for ClearBannedResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<()> for ClearBannedResponse {
    fn as_ref(&self) -> &() { &self.value }
}

impl From<()> for ClearBannedResponse {
    fn from(value: ()) -> Self { Self { value } }
}

impl From<ClearBannedResponse> for () {
    fn from(wrapper: ClearBannedResponse) -> Self { wrapper.value }
}

/// Response for the `CombinePsbt` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CombinePsbtResponse {
    /// Wrapped primitive value
    pub value: String,
}

impl<'de> serde::Deserialize<'de> for CombinePsbtResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = CombinePsbtResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CombinePsbtResponse { value: v.to_string() })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CombinePsbtResponse { value: v.to_string() })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CombinePsbtResponse { value: v.to_string() })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CombinePsbtResponse { value: v.to_string() })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CombinePsbtResponse { value: v.to_string() })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(CombinePsbtResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for CombinePsbtResponse {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for CombinePsbtResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<String> for CombinePsbtResponse {
    fn as_ref(&self) -> &String { &self.value }
}

impl From<String> for CombinePsbtResponse {
    fn from(value: String) -> Self { Self { value } }
}

impl From<CombinePsbtResponse> for String {
    fn from(wrapper: CombinePsbtResponse) -> Self { wrapper.value }
}

/// Response for the `CombineRawTransaction` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CombineRawTransactionResponse {
    /// Wrapped primitive value
    pub value: String,
}

impl<'de> serde::Deserialize<'de> for CombineRawTransactionResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = CombineRawTransactionResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CombineRawTransactionResponse { value: v.to_string() })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CombineRawTransactionResponse { value: v.to_string() })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CombineRawTransactionResponse { value: v.to_string() })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CombineRawTransactionResponse { value: v.to_string() })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CombineRawTransactionResponse { value: v.to_string() })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(CombineRawTransactionResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for CombineRawTransactionResponse {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for CombineRawTransactionResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<String> for CombineRawTransactionResponse {
    fn as_ref(&self) -> &String { &self.value }
}

impl From<String> for CombineRawTransactionResponse {
    fn from(value: String) -> Self { Self { value } }
}

impl From<CombineRawTransactionResponse> for String {
    fn from(wrapper: CombineRawTransactionResponse) -> Self { wrapper.value }
}

/// Response for the `ConvertToPsbt` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ConvertToPsbtResponse {
    /// Wrapped primitive value
    pub value: String,
}

impl<'de> serde::Deserialize<'de> for ConvertToPsbtResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = ConvertToPsbtResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ConvertToPsbtResponse { value: v.to_string() })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ConvertToPsbtResponse { value: v.to_string() })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ConvertToPsbtResponse { value: v.to_string() })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ConvertToPsbtResponse { value: v.to_string() })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ConvertToPsbtResponse { value: v.to_string() })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(ConvertToPsbtResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for ConvertToPsbtResponse {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for ConvertToPsbtResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<String> for ConvertToPsbtResponse {
    fn as_ref(&self) -> &String { &self.value }
}

impl From<String> for ConvertToPsbtResponse {
    fn from(value: String) -> Self { Self { value } }
}

impl From<ConvertToPsbtResponse> for String {
    fn from(wrapper: ConvertToPsbtResponse) -> Self { wrapper.value }
}

/// Response for the `CreateMultisig` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct CreateMultisigResponse {
    /// The value of the new multisig address.
    pub address: String,
    /// The string value of the hex-encoded redemption script.
    #[serde(rename = "redeemScript")]
    pub redeem_script: bitcoin::ScriptBuf,
    /// The descriptor for this multisig
    pub descriptor: String,
    /// Any warnings resulting from the creation of this multisig
    pub warnings: Option<Vec<String>>,
}

/// Response for the `CreatePsbt` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CreatePsbtResponse {
    /// Wrapped primitive value
    pub value: String,
}

impl<'de> serde::Deserialize<'de> for CreatePsbtResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = CreatePsbtResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CreatePsbtResponse { value: v.to_string() })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CreatePsbtResponse { value: v.to_string() })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CreatePsbtResponse { value: v.to_string() })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CreatePsbtResponse { value: v.to_string() })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CreatePsbtResponse { value: v.to_string() })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(CreatePsbtResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for CreatePsbtResponse {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for CreatePsbtResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<String> for CreatePsbtResponse {
    fn as_ref(&self) -> &String { &self.value }
}

impl From<String> for CreatePsbtResponse {
    fn from(value: String) -> Self { Self { value } }
}

impl From<CreatePsbtResponse> for String {
    fn from(wrapper: CreatePsbtResponse) -> Self { wrapper.value }
}

/// Response for the `CreateRawTransaction` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CreateRawTransactionResponse {
    /// Wrapped primitive value
    pub value: String,
}

impl<'de> serde::Deserialize<'de> for CreateRawTransactionResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = CreateRawTransactionResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CreateRawTransactionResponse { value: v.to_string() })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CreateRawTransactionResponse { value: v.to_string() })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CreateRawTransactionResponse { value: v.to_string() })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CreateRawTransactionResponse { value: v.to_string() })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CreateRawTransactionResponse { value: v.to_string() })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(CreateRawTransactionResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for CreateRawTransactionResponse {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for CreateRawTransactionResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<String> for CreateRawTransactionResponse {
    fn as_ref(&self) -> &String { &self.value }
}

impl From<String> for CreateRawTransactionResponse {
    fn from(value: String) -> Self { Self { value } }
}

impl From<CreateRawTransactionResponse> for String {
    fn from(wrapper: CreateRawTransactionResponse) -> Self { wrapper.value }
}

/// Response for the `CreateWallet` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct CreateWalletResponse {
    /// The wallet name if created successfully. If the wallet was created using a full path, the wallet_name will be the full path.
    pub name: String,
    /// Warning messages, if any, related to creating and loading the wallet.
    pub warnings: Option<Vec<String>>,
}

/// Response for the `CreateWalletDescriptor` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct CreateWalletDescriptorResponse {
    /// The public descriptors that were added to the wallet
    pub descs: Vec<String>,
}

/// Response for the `DecodePsbt` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtResponse {
    /// The decoded network-serialized unsigned transaction.
    pub tx: DecodePsbtTx,
    pub global_xpubs: serde_json::Value,
    /// The PSBT version number. Not to be confused with the unsigned transaction version
    pub psbt_version: u64,
    /// The global proprietary map
    pub proprietary: serde_json::Value,
    /// The unknown global fields
    pub unknown: serde_json::Value,
    pub inputs: Vec<DecodePsbtInput>,
    pub outputs: Vec<DecodePsbtOutput>,
    /// The transaction fee paid if all UTXOs slots in the PSBT have been filled.
    #[serde(deserialize_with = "option_amount_from_btc_float")]
    pub fee: Option<bitcoin::Amount>,
}

/// Response for the `DecodeRawTransaction` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodeRawTransactionResponse {
    /// The transaction id
    pub txid: bitcoin::Txid,
    /// The transaction hash (differs from txid for witness transactions)
    pub hash: String,
    /// The serialized transaction size
    pub size: u64,
    /// The virtual transaction size (differs from size for witness transactions)
    #[serde(rename = "vsize")]
    pub v_size: u64,
    /// The transaction's weight (between vsize*4-3 and vsize*4)
    pub weight: u64,
    /// The version
    pub version: u32,
    /// The lock time
    #[serde(rename = "locktime")]
    pub lock_time: u64,
    pub vin: Vec<DecodedVin>,
    pub vout: Vec<DecodedVout>,
}

/// Response for the `DecodeScript` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodeScriptResponse {
    /// Disassembly of the script
    pub asm: String,
    /// Inferred descriptor for the script
    pub desc: String,
    /// The output type (e.g. nonstandard, anchor, pubkey, pubkeyhash, scripthash, multisig, nulldata, witness_v0_scripthash, witness_v0_keyhash, witness_v1_taproot, witness_unknown)
    #[serde(rename = "type")]
    pub r#type: String,
    /// The Bitcoin address (only if a well-defined address exists)
    pub address: Option<String>,
    /// address of P2SH script wrapping this redeem script (not returned for types that should not be wrapped)
    pub p2sh: Option<String>,
    /// Result of a witness output script wrapping this redeem script (not returned for types that should not be wrapped)
    pub segwit: Option<serde_json::Value>,
}

/// Response for the `DeriveAddresses` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct DeriveAddressesResponse {
    /// Wrapped array value
    pub value: Vec<String>,
}

impl<'de> serde::Deserialize<'de> for DeriveAddressesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<String>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<String>> for DeriveAddressesResponse {
    fn from(value: Vec<String>) -> Self { Self { value } }
}

impl From<DeriveAddressesResponse> for Vec<String> {
    fn from(wrapper: DeriveAddressesResponse) -> Self { wrapper.value }
}

/// Response for the `DescriptorProcessPsbt` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DescriptorProcessPsbtResponse {
    /// The base64-encoded partially signed transaction
    pub psbt: String,
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// The hex-encoded network transaction if complete
    pub hex: Option<String>,
}

/// Response for the `DisconnectNode` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct DisconnectNodeResponse {
    /// Wrapped primitive value
    pub value: (),
}

impl<'de> serde::Deserialize<'de> for DisconnectNodeResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = DisconnectNodeResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(DisconnectNodeResponse { value: () })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(DisconnectNodeResponse { value: () })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(DisconnectNodeResponse { value: () })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(DisconnectNodeResponse { value: () })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(DisconnectNodeResponse { value: () })
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(DisconnectNodeResponse { value: () })
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(DisconnectNodeResponse { value: () })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value::<()>()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(DisconnectNodeResponse { value: () })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for DisconnectNodeResponse {
    type Target = ();
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for DisconnectNodeResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<()> for DisconnectNodeResponse {
    fn as_ref(&self) -> &() { &self.value }
}

impl From<()> for DisconnectNodeResponse {
    fn from(value: ()) -> Self { Self { value } }
}

impl From<DisconnectNodeResponse> for () {
    fn from(wrapper: DisconnectNodeResponse) -> Self { wrapper.value }
}

/// Response for the `DumpTxOutSet` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DumpTxOutSetResponse {
    /// the number of coins written in the snapshot
    pub coins_written: u64,
    /// the hash of the base of the snapshot
    pub base_hash: String,
    /// the height of the base of the snapshot
    pub base_height: u64,
    /// the absolute path that the snapshot was written to
    pub path: String,
    /// the hash of the UTXO set contents
    pub txoutset_hash: String,
    /// the number of transactions in the chain up to and including the base block
    #[serde(rename = "nchaintx")]
    pub n_chain_tx: u64,
}

/// Response for the `Echo` RPC method
///
/// This method returns arbitrary JSON (e.g. string, object, array) as a single value.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct EchoResponse {
    /// Wrapped JSON value
    pub value: serde_json::Value,
}

impl<'de> serde::Deserialize<'de> for EchoResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<serde_json::Value> for EchoResponse {
    fn from(value: serde_json::Value) -> Self { Self { value } }
}

/// Response for the `Echoipc` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct EchoipcResponse {
    /// Wrapped primitive value
    pub value: String,
}

impl<'de> serde::Deserialize<'de> for EchoipcResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = EchoipcResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(EchoipcResponse { value: v.to_string() })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(EchoipcResponse { value: v.to_string() })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(EchoipcResponse { value: v.to_string() })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(EchoipcResponse { value: v.to_string() })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(EchoipcResponse { value: v.to_string() })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(EchoipcResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for EchoipcResponse {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for EchoipcResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<String> for EchoipcResponse {
    fn as_ref(&self) -> &String { &self.value }
}

impl From<String> for EchoipcResponse {
    fn from(value: String) -> Self { Self { value } }
}

impl From<EchoipcResponse> for String {
    fn from(wrapper: EchoipcResponse) -> Self { wrapper.value }
}

/// Response for the `Echojson` RPC method
///
/// This method returns arbitrary JSON (e.g. string, object, array) as a single value.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct EchojsonResponse {
    /// Wrapped JSON value
    pub value: serde_json::Value,
}

impl<'de> serde::Deserialize<'de> for EchojsonResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<serde_json::Value> for EchojsonResponse {
    fn from(value: serde_json::Value) -> Self { Self { value } }
}

/// Response for the `EncryptWallet` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct EncryptWalletResponse {
    /// Wrapped primitive value
    pub value: String,
}

impl<'de> serde::Deserialize<'de> for EncryptWalletResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = EncryptWalletResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(EncryptWalletResponse { value: v.to_string() })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(EncryptWalletResponse { value: v.to_string() })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(EncryptWalletResponse { value: v.to_string() })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(EncryptWalletResponse { value: v.to_string() })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(EncryptWalletResponse { value: v.to_string() })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(EncryptWalletResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for EncryptWalletResponse {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for EncryptWalletResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<String> for EncryptWalletResponse {
    fn as_ref(&self) -> &String { &self.value }
}

impl From<String> for EncryptWalletResponse {
    fn from(value: String) -> Self { Self { value } }
}

impl From<EncryptWalletResponse> for String {
    fn from(wrapper: EncryptWalletResponse) -> Self { wrapper.value }
}

/// Response for the `EnumerateSigners` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct EnumerateSignersResponse {
    pub signers: serde_json::Value,
}

/// Response for the `EstimateRawFee` RPC method
///
/// Results are returned for any horizon which tracks blocks up to the confirmation target
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct EstimateRawFeeResponse {
    /// estimate for short time horizon
    pub short: Option<serde_json::Value>,
    /// estimate for medium time horizon
    pub medium: Option<serde_json::Value>,
    /// estimate for long time horizon
    pub long: Option<serde_json::Value>,
}
impl<'de> serde::Deserialize<'de> for EstimateRawFeeResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct ConditionalResponseVisitor;

        #[allow(clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for ConditionalResponseVisitor {
            type Value = EstimateRawFeeResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("string or object")
            }

            fn visit_str<E>(self, _v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(EstimateRawFeeResponse { short: None, medium: None, long: None })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut short = None;
                let mut medium = None;
                let mut long = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "short" {
                        if short.is_some() {
                            return Err(de::Error::duplicate_field("short"));
                        }
                        short = Some(map.next_value::<serde_json::Value>()?);
                    }
                    if key == "medium" {
                        if medium.is_some() {
                            return Err(de::Error::duplicate_field("medium"));
                        }
                        medium = Some(map.next_value::<serde_json::Value>()?);
                    }
                    if key == "long" {
                        if long.is_some() {
                            return Err(de::Error::duplicate_field("long"));
                        }
                        long = Some(map.next_value::<serde_json::Value>()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                Ok(EstimateRawFeeResponse { short, medium, long })
            }
        }

        deserializer.deserialize_any(ConditionalResponseVisitor)
    }
}

/// Response for the `EstimateSmartFee` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct EstimateSmartFeeResponse {
    /// estimate fee rate in BTC/kvB (only present if no errors were encountered)
    #[serde(rename = "feerate")]
    pub fee_rate: Option<f64>,
    /// Errors encountered during processing (if there are any)
    pub errors: Option<Vec<String>>,
    /// block number where estimate was found
    /// The request target will be clamped between 2 and the highest target
    /// fee estimation is able to return based on how long it has been running.
    /// An error is returned if not enough transactions and blocks
    /// have been observed to make an estimate for any number of blocks.
    pub blocks: u64,
}

/// Response for the `FinalizePsbt` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct FinalizePsbtResponse {
    /// The base64-encoded partially signed transaction if not extracted
    pub psbt: Option<String>,
    /// The hex-encoded network transaction if extracted
    pub hex: Option<String>,
    /// If the transaction has a complete set of signatures
    pub complete: bool,
}

/// Response for the `FundRawTransaction` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct FundRawTransactionResponse {
    /// The resulting raw transaction (hex-encoded string)
    pub hex: String,
    /// Fee in BTC the resulting transaction pays
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub fee: bitcoin::Amount,
    /// The position of the added change output, or -1
    pub changepos: i64,
}

/// Response for the `Generate` RPC method
///
/// This method returns no meaningful data.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GenerateResponse;

/// Response for the `GenerateBlock` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GenerateBlockResponse {
    /// hash of generated block
    pub hash: String,
    /// hex of generated block, only present when submit=false
    pub hex: Option<String>,
}

/// Response for the `GenerateToAddress` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GenerateToAddressResponse {
    /// Wrapped array value
    pub value: Vec<serde_json::Value>,
}

impl<'de> serde::Deserialize<'de> for GenerateToAddressResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<serde_json::Value>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<serde_json::Value>> for GenerateToAddressResponse {
    fn from(value: Vec<serde_json::Value>) -> Self { Self { value } }
}

impl From<GenerateToAddressResponse> for Vec<serde_json::Value> {
    fn from(wrapper: GenerateToAddressResponse) -> Self { wrapper.value }
}

/// Response for the `GenerateToDescriptor` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GenerateToDescriptorResponse {
    /// Wrapped array value
    pub value: Vec<serde_json::Value>,
}

impl<'de> serde::Deserialize<'de> for GenerateToDescriptorResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<serde_json::Value>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<serde_json::Value>> for GenerateToDescriptorResponse {
    fn from(value: Vec<serde_json::Value>) -> Self { Self { value } }
}

impl From<GenerateToDescriptorResponse> for Vec<serde_json::Value> {
    fn from(wrapper: GenerateToDescriptorResponse) -> Self { wrapper.value }
}

/// Response for the `GetAddedNodeInfo` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetAddedNodeInfoResponse {
    /// Wrapped array value
    pub value: Vec<GetAddedNodeInfoElement>,
}

impl<'de> serde::Deserialize<'de> for GetAddedNodeInfoResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<GetAddedNodeInfoElement>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<GetAddedNodeInfoElement>> for GetAddedNodeInfoResponse {
    fn from(value: Vec<GetAddedNodeInfoElement>) -> Self { Self { value } }
}

impl From<GetAddedNodeInfoResponse> for Vec<GetAddedNodeInfoElement> {
    fn from(wrapper: GetAddedNodeInfoResponse) -> Self { wrapper.value }
}

/// Response for the `GetAddressesByLabel` RPC method
///
/// json object with addresses as keys
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetAddressesByLabelResponse {
    /// json object with information about address
    pub address: serde_json::Value,
}

/// Response for the `GetAddressInfo` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetAddressInfoResponse {
    /// The bitcoin address validated.
    pub address: String,
    /// The hex-encoded output script generated by the address.
    #[serde(rename = "scriptPubKey")]
    pub script_pubkey: bitcoin::ScriptBuf,
    /// If the address is yours.
    #[serde(rename = "ismine")]
    pub is_mine: bool,
    /// (DEPRECATED) Always false.
    #[serde(rename = "iswatchonly")]
    pub is_watch_only: bool,
    /// If we know how to spend coins sent to this address, ignoring the possible lack of private keys.
    pub solvable: bool,
    /// A descriptor for spending coins sent to this address (only when solvable).
    pub desc: Option<String>,
    /// The descriptor used to derive this address if this is a descriptor wallet
    pub parent_desc: Option<String>,
    /// If the key is a script.
    #[serde(rename = "isscript")]
    pub is_script: Option<bool>,
    /// If the address was used for change output.
    pub ischange: bool,
    /// If the address is a witness address.
    #[serde(rename = "iswitness")]
    pub is_witness: bool,
    /// The version number of the witness program.
    pub witness_version: Option<u64>,
    /// The hex value of the witness program.
    pub witness_program: Option<String>,
    /// The output script type. Only if isscript is true and the redeemscript is known. Possible
    /// types: nonstandard, pubkey, pubkeyhash, scripthash, multisig, nulldata, witness_v0_keyhash,
    /// witness_v0_scripthash, witness_unknown.
    pub script: Option<bitcoin::ScriptBuf>,
    /// The redeemscript for the p2sh address.
    pub hex: Option<String>,
    /// Array of pubkeys associated with the known redeemscript (only if script is multisig).
    #[serde(rename = "pubkeys")]
    pub pub_keys: Option<Vec<String>>,
    /// The number of signatures required to spend multisig output (only if script is multisig).
    #[serde(rename = "sigsrequired")]
    pub sigs_required: Option<u64>,
    /// The hex value of the raw public key for single-key addresses (possibly embedded in P2SH or P2WSH).
    pub pubkey: Option<String>,
    /// Information about the address embedded in P2SH or P2WSH, if relevant and known.
    pub embedded: Option<serde_json::Value>,
    /// If the pubkey is compressed.
    #[serde(rename = "iscompressed")]
    pub is_compressed: Option<bool>,
    /// The creation time of the key, if available, expressed in UNIX epoch time.
    pub timestamp: Option<u64>,
    /// The HD keypath, if the key is HD and available.
    #[serde(rename = "hdkeypath")]
    pub hd_key_path: Option<String>,
    /// The Hash160 of the HD seed.
    #[serde(rename = "hdseedid")]
    pub hd_seed_id: Option<String>,
    /// The fingerprint of the master key.
    #[serde(rename = "hdmasterfingerprint")]
    pub hd_master_fingerprint: Option<String>,
    /// Array of labels associated with the address. Currently limited to one label but returned
    /// as an array to keep the API stable if multiple labels are enabled in the future.
    pub labels: Vec<String>,
}

/// Response for the `GetAddrManInfo` RPC method
///
/// json object with network type as keys
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetAddrManInfoResponse {
    /// the network (ipv4, ipv6, onion, i2p, cjdns, all_networks)
    #[serde(default)]
    pub network: Option<serde_json::Value>,
}

/// Response for the `GetBalance` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetBalanceResponse {
    /// Wrapped primitive value
    pub value: bitcoin::Amount,
}

impl<'de> serde::Deserialize<'de> for GetBalanceResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = GetBalanceResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetBalanceResponse { value: bitcoin::Amount::from_sat(v) })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if v < 0 {
                    return Err(de::Error::custom(format!("Amount cannot be negative: {}", v)));
                }
                Ok(GetBalanceResponse { value: bitcoin::Amount::from_sat(v as u64) })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let amount = bitcoin::Amount::from_btc(v)
                    .map_err(|e| de::Error::custom(format!("Invalid BTC amount: {}", e)))?;
                Ok(GetBalanceResponse { value: amount })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let value = v.parse::<bitcoin::Amount>().map_err(de::Error::custom)?;
                Ok(GetBalanceResponse { value })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Err(de::Error::custom("cannot convert bool to bitcoin::Amount"))
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(GetBalanceResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for GetBalanceResponse {
    type Target = bitcoin::Amount;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for GetBalanceResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<bitcoin::Amount> for GetBalanceResponse {
    fn as_ref(&self) -> &bitcoin::Amount { &self.value }
}

impl From<bitcoin::Amount> for GetBalanceResponse {
    fn from(value: bitcoin::Amount) -> Self { Self { value } }
}

impl From<GetBalanceResponse> for bitcoin::Amount {
    fn from(wrapper: GetBalanceResponse) -> Self { wrapper.value }
}

/// Response for the `GetBalances` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBalancesResponse {
    /// balances from outputs that the wallet can sign
    pub mine: serde_json::Value,
    /// hash and height of the block this information was generated on
    #[serde(rename = "lastprocessedblock")]
    pub last_processed_block: serde_json::Value,
}

/// Response for the `GetBestBlockHash` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetBestBlockHashResponse {
    /// Wrapped primitive value
    pub value: String,
}

impl<'de> serde::Deserialize<'de> for GetBestBlockHashResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = GetBestBlockHashResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetBestBlockHashResponse { value: v.to_string() })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetBestBlockHashResponse { value: v.to_string() })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetBestBlockHashResponse { value: v.to_string() })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetBestBlockHashResponse { value: v.to_string() })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetBestBlockHashResponse { value: v.to_string() })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(GetBestBlockHashResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for GetBestBlockHashResponse {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for GetBestBlockHashResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<String> for GetBestBlockHashResponse {
    fn as_ref(&self) -> &String { &self.value }
}

impl From<String> for GetBestBlockHashResponse {
    fn from(value: String) -> Self { Self { value } }
}

impl From<GetBestBlockHashResponse> for String {
    fn from(wrapper: GetBestBlockHashResponse) -> Self { wrapper.value }
}

/// Response for the `GetBlock` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockResponse {
    /// the block hash (same as provided)
    pub hash: String,
    /// The number of confirmations, or -1 if the block is not on the main chain
    pub confirmations: i64,
    /// The block size
    pub size: u64,
    /// The block size excluding witness data
    #[serde(rename = "strippedsize")]
    pub stripped_size: u64,
    /// The block weight as defined in BIP 141
    pub weight: u64,
    /// Coinbase transaction metadata
    pub coinbase_tx: serde_json::Value,
    /// The block height or index
    pub height: u64,
    /// The block version
    pub version: u32,
    /// The block version formatted in hexadecimal
    #[serde(rename = "versionHex")]
    pub version_hex: String,
    /// The merkle root
    #[serde(rename = "merkleroot")]
    pub merkle_root: String,
    /// The transaction ids
    pub tx: Vec<bitcoin::Txid>,
    /// The block time expressed in UNIX epoch time
    pub time: u64,
    /// The median block time expressed in UNIX epoch time
    #[serde(rename = "mediantime")]
    pub median_time: u64,
    /// The nonce
    pub nonce: u64,
    /// nBits: compact representation of the block difficulty target
    pub bits: String,
    /// The difficulty target
    pub target: String,
    /// The difficulty
    pub difficulty: f64,
    /// Expected number of hashes required to produce the chain up to this block (in hex)
    #[serde(rename = "chainwork")]
    pub chain_work: String,
    /// The number of transactions in the block
    pub nTx: u64,
    /// The hash of the previous block (if available)
    #[serde(rename = "previousblockhash")]
    pub previous_block_hash: Option<String>,
    /// The hash of the next block (if available)
    #[serde(rename = "nextblockhash")]
    pub next_block_hash: Option<String>,
}

/// Response for the `GetBlockchainInfo` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockchainInfoResponse {
    /// current network name (main, test, testnet4, signet, regtest)
    pub chain: String,
    /// the height of the most-work fully-validated chain. The genesis block has height 0
    pub blocks: u64,
    /// the current number of headers we have validated
    pub headers: u64,
    /// the hash of the currently best block
    #[serde(rename = "bestblockhash")]
    pub best_block_hash: String,
    /// nBits: compact representation of the block difficulty target
    pub bits: String,
    /// The difficulty target
    pub target: String,
    /// the current difficulty
    pub difficulty: f64,
    /// The block time expressed in UNIX epoch time
    pub time: u64,
    /// The median block time expressed in UNIX epoch time
    #[serde(rename = "mediantime")]
    pub median_time: u64,
    /// estimate of verification progress \[0..1\]
    #[serde(rename = "verificationprogress")]
    pub verification_progress: f64,
    /// (debug information) estimate of whether this node is in Initial Block Download mode
    #[serde(rename = "initialblockdownload")]
    pub initial_block_download: bool,
    /// total amount of work in active chain, in hexadecimal
    #[serde(rename = "chainwork")]
    pub chain_work: String,
    /// the estimated size of the block and undo files on disk
    pub size_on_disk: u64,
    /// if the blocks are subject to pruning
    pub pruned: bool,
    /// the first block unpruned, all previous blocks were pruned (only present if pruning is enabled)
    #[serde(rename = "pruneheight")]
    pub prune_height: Option<u64>,
    /// whether automatic pruning is enabled (only present if pruning is enabled)
    pub automatic_pruning: Option<bool>,
    /// the target size used by pruning (only present if automatic pruning is enabled)
    pub prune_target_size: Option<u64>,
    /// the block challenge (aka. block script), in hexadecimal (only present if the current network is a signet)
    pub signet_challenge: Option<String>,
    /// any network and blockchain warnings (run with `-deprecatedrpc=warnings` to return the latest warning as a single string)
    pub warnings: Vec<String>,
}

/// Response for the `GetBlockCount` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetBlockCountResponse {
    /// Wrapped primitive value
    pub value: u64,
}

impl<'de> serde::Deserialize<'de> for GetBlockCountResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = GetBlockCountResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetBlockCountResponse { value: v })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetBlockCountResponse { value: v as u64 })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetBlockCountResponse { value: v as u64 })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let value = v.parse::<u64>().map_err(de::Error::custom)?;
                Ok(GetBlockCountResponse { value })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetBlockCountResponse { value: v as u64 })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(GetBlockCountResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for GetBlockCountResponse {
    type Target = u64;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for GetBlockCountResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<u64> for GetBlockCountResponse {
    fn as_ref(&self) -> &u64 { &self.value }
}

impl From<u64> for GetBlockCountResponse {
    fn from(value: u64) -> Self { Self { value } }
}

impl From<GetBlockCountResponse> for u64 {
    fn from(wrapper: GetBlockCountResponse) -> Self { wrapper.value }
}

/// Response for the `GetBlockFilter` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockFilterResponse {
    /// the hex-encoded filter data
    pub filter: String,
    /// the hex-encoded filter header
    pub header: String,
}

/// Response for the `GetBlockFromPeer` RPC method
///
/// This method returns no meaningful data.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetBlockFromPeerResponse;

/// Response for the `GetBlockHash` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetBlockHashResponse {
    /// Wrapped primitive value
    pub value: String,
}

impl<'de> serde::Deserialize<'de> for GetBlockHashResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = GetBlockHashResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetBlockHashResponse { value: v.to_string() })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetBlockHashResponse { value: v.to_string() })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetBlockHashResponse { value: v.to_string() })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetBlockHashResponse { value: v.to_string() })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetBlockHashResponse { value: v.to_string() })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(GetBlockHashResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for GetBlockHashResponse {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for GetBlockHashResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<String> for GetBlockHashResponse {
    fn as_ref(&self) -> &String { &self.value }
}

impl From<String> for GetBlockHashResponse {
    fn from(value: String) -> Self { Self { value } }
}

impl From<GetBlockHashResponse> for String {
    fn from(wrapper: GetBlockHashResponse) -> Self { wrapper.value }
}

/// Response for the `GetBlockHeader` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockHeaderResponse {
    /// the block hash (same as provided)
    pub hash: String,
    /// The number of confirmations, or -1 if the block is not on the main chain
    pub confirmations: i64,
    /// The block height or index
    pub height: u64,
    /// The block version
    pub version: u32,
    /// The block version formatted in hexadecimal
    #[serde(rename = "versionHex")]
    pub version_hex: String,
    /// The merkle root
    #[serde(rename = "merkleroot")]
    pub merkle_root: String,
    /// The block time expressed in UNIX epoch time
    pub time: u64,
    /// The median block time expressed in UNIX epoch time
    #[serde(rename = "mediantime")]
    pub median_time: u64,
    /// The nonce
    pub nonce: u64,
    /// nBits: compact representation of the block difficulty target
    pub bits: String,
    /// The difficulty target
    pub target: String,
    /// The difficulty
    pub difficulty: f64,
    /// Expected number of hashes required to produce the current chain
    #[serde(rename = "chainwork")]
    pub chain_work: String,
    /// The number of transactions in the block
    pub nTx: u64,
    /// The hash of the previous block (if available)
    #[serde(rename = "previousblockhash")]
    pub previous_block_hash: Option<String>,
    /// The hash of the next block (if available)
    #[serde(rename = "nextblockhash")]
    pub next_block_hash: Option<String>,
}

/// Response for the `GetBlockStats` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockStatsResponse {
    /// Average fee in the block
    pub avgfee: Option<u64>,
    /// Average feerate (in satoshis per virtual byte)
    #[serde(rename = "avgfeerate")]
    pub avg_fee_rate: Option<u64>,
    /// Average transaction size
    #[serde(rename = "avgtxsize")]
    pub avg_tx_size: Option<u64>,
    /// The block hash (to check for potential reorgs)
    #[serde(rename = "blockhash")]
    pub block_hash: Option<bitcoin::BlockHash>,
    /// Feerates at the 10th, 25th, 50th, 75th, and 90th percentile weight unit (in satoshis per virtual byte)
    pub feerate_percentiles: Option<serde_json::Value>,
    /// The height of the block
    pub height: Option<u64>,
    /// The number of inputs (excluding coinbase)
    pub ins: Option<u64>,
    /// Maximum fee in the block
    #[serde(rename = "maxfee")]
    pub max_fee: Option<u64>,
    /// Maximum feerate (in satoshis per virtual byte)
    #[serde(rename = "maxfeerate")]
    pub max_fee_rate: Option<f64>,
    /// Maximum transaction size
    #[serde(rename = "maxtxsize")]
    pub max_tx_size: Option<u64>,
    /// Truncated median fee in the block
    pub medianfee: Option<u64>,
    /// The block median time past
    #[serde(rename = "mediantime")]
    pub median_time: Option<u64>,
    /// Truncated median transaction size
    #[serde(rename = "mediantxsize")]
    pub median_tx_size: Option<u64>,
    /// Minimum fee in the block
    #[serde(rename = "minfee")]
    pub min_fee: Option<u64>,
    /// Minimum feerate (in satoshis per virtual byte)
    #[serde(rename = "minfeerate")]
    pub min_fee_rate: Option<u64>,
    /// Minimum transaction size
    #[serde(rename = "mintxsize")]
    pub min_tx_size: Option<u64>,
    /// The number of outputs
    pub outs: Option<u64>,
    /// The block subsidy
    pub subsidy: Option<u64>,
    /// Total size of all segwit transactions
    #[serde(rename = "swtotal_size")]
    pub sw_total_size: Option<u64>,
    /// Total weight of all segwit transactions
    #[serde(rename = "swtotal_weight")]
    pub sw_total_weight: Option<u64>,
    /// The number of segwit transactions
    #[serde(rename = "swtxs")]
    pub sw_txs: Option<u64>,
    /// The block time
    pub time: Option<u64>,
    /// Total amount in all outputs (excluding coinbase and thus reward \[ie subsidy + totalfee\])
    pub total_out: Option<u64>,
    /// Total size of all non-coinbase transactions
    pub total_size: Option<u64>,
    /// Total weight of all non-coinbase transactions
    pub total_weight: Option<u64>,
    /// The fee total
    #[serde(rename = "totalfee")]
    pub total_fee: Option<u64>,
    /// The number of transactions (including coinbase)
    pub txs: Option<u64>,
    /// The increase/decrease in the number of unspent outputs (not discounting op_return and similar)
    pub utxo_increase: Option<u64>,
    /// The increase/decrease in size for the utxo index (not discounting op_return and similar)
    pub utxo_size_inc: Option<u64>,
    /// The increase/decrease in the number of unspent outputs, not counting unspendables
    pub utxo_increase_actual: Option<u64>,
    /// The increase/decrease in size for the utxo index, not counting unspendables
    pub utxo_size_inc_actual: Option<u64>,
}

/// Response for the `GetBlockTemplate` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockTemplateResponse {
    pub field_0_1: (),
    /// The preferred block version
    pub version: u32,
    /// specific block rules that are to be enforced
    pub rules: Vec<String>,
    /// set of pending, supported versionbit (BIP 9) softfork deployments
    #[serde(rename = "vbavailable")]
    pub vb_available: HashMap<String, u32>,
    pub capabilities: Vec<String>,
    /// bit mask of versionbits the server requires set in submissions
    #[serde(rename = "vbrequired")]
    pub vb_required: u64,
    /// The hash of current highest block
    #[serde(rename = "previousblockhash")]
    pub previous_block_hash: String,
    /// contents of non-coinbase transactions that should be included in the next block
    pub transactions: Vec<GetBlockTemplateTransaction>,
    /// data that should be included in the coinbase's scriptSig content
    #[serde(rename = "coinbaseaux")]
    pub coinbase_aux: HashMap<String, String>,
    /// maximum allowable input to coinbase transaction, including the generation award and transaction fees (in satoshis)
    #[serde(rename = "coinbasevalue")]
    pub coinbase_value: u64,
    /// an id to include with a request to longpoll on an update to this template
    #[serde(rename = "longpollid")]
    pub longpoll_id: String,
    /// The hash target
    pub target: String,
    /// The minimum timestamp appropriate for the next block time, expressed in UNIX epoch time. Adjusted for the proposed BIP94 timewarp rule.
    pub mintime: u64,
    /// list of ways the block template may be changed
    pub mutable: Vec<String>,
    /// A range of valid nonces
    #[serde(rename = "noncerange")]
    pub nonce_range: String,
    /// limit of sigops in blocks
    #[serde(rename = "sigoplimit")]
    pub sigop_limit: u64,
    /// limit of block size
    #[serde(rename = "sizelimit")]
    pub size_limit: u64,
    /// limit of block weight
    #[serde(rename = "weightlimit")]
    pub weight_limit: Option<u64>,
    /// current timestamp in UNIX epoch time. Adjusted for the proposed BIP94 timewarp rule.
    #[serde(rename = "curtime")]
    pub cur_time: u64,
    /// compressed target of next block
    pub bits: String,
    /// The height of the next block
    pub height: u64,
    /// Only on signet
    pub signet_challenge: Option<String>,
    /// a valid witness commitment for the unmodified block template
    pub default_witness_commitment: Option<String>,
}

/// Response for the `GetChainStates` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetChainStatesResponse {
    /// the number of headers seen so far
    pub headers: u64,
    /// list of the chainstates ordered by work, with the most-work (active) chainstate last
    pub chainstates: serde_json::Value,
}

/// Response for the `GetChainTips` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetChainTipsResponse {
    pub field: serde_json::Value,
}

/// Response for the `GetChainTxStats` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetChainTxStatsResponse {
    /// The timestamp for the final block in the window, expressed in UNIX epoch time
    pub time: u64,
    /// The total number of transactions in the chain up to that point, if known. It may be unknown when using assumeutxo.
    #[serde(rename = "txcount")]
    pub tx_count: Option<u64>,
    /// The hash of the final block in the window
    pub window_final_block_hash: String,
    /// The height of the final block in the window.
    pub window_final_block_height: u64,
    /// Size of the window in number of blocks
    pub window_block_count: u64,
    /// The elapsed time in the window in seconds. Only returned if "window_block_count" is &gt; 0
    pub window_interval: Option<u64>,
    /// The number of transactions in the window. Only returned if "window_block_count" is &gt; 0 and if txcount exists for the start and end of the window.
    pub window_tx_count: Option<u64>,
    /// The average rate of transactions per second in the window. Only returned if "window_interval" is &gt; 0 and if window_tx_count exists.
    #[serde(rename = "txrate")]
    pub tx_rate: Option<u64>,
}

/// Response for the `GetConnectionCount` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetConnectionCountResponse {
    /// Wrapped primitive value
    pub value: u64,
}

impl<'de> serde::Deserialize<'de> for GetConnectionCountResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = GetConnectionCountResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetConnectionCountResponse { value: v })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetConnectionCountResponse { value: v as u64 })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetConnectionCountResponse { value: v as u64 })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let value = v.parse::<u64>().map_err(de::Error::custom)?;
                Ok(GetConnectionCountResponse { value })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetConnectionCountResponse { value: v as u64 })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(GetConnectionCountResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for GetConnectionCountResponse {
    type Target = u64;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for GetConnectionCountResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<u64> for GetConnectionCountResponse {
    fn as_ref(&self) -> &u64 { &self.value }
}

impl From<u64> for GetConnectionCountResponse {
    fn from(value: u64) -> Self { Self { value } }
}

impl From<GetConnectionCountResponse> for u64 {
    fn from(wrapper: GetConnectionCountResponse) -> Self { wrapper.value }
}

/// Response for the `GetDeploymentInfo` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetDeploymentInfoResponse {
    /// requested block hash (or tip)
    pub hash: String,
    /// requested block height (or tip)
    pub height: u64,
    /// script verify flags for the block
    pub script_flags: Vec<String>,
    pub deployments: serde_json::Value,
}

/// Response for the `GetDescriptorActivity` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetDescriptorActivityResponse {
    /// events
    pub activity: serde_json::Value,
}

/// Response for the `GetDescriptorInfo` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetDescriptorInfoResponse {
    /// The descriptor in canonical form, without private keys. For a multipath descriptor, only the first will be returned.
    pub descriptor: String,
    /// All descriptors produced by expanding multipath derivation elements. Only if the provided descriptor specifies multipath derivation elements.
    pub multipath_expansion: Option<Vec<String>>,
    /// The checksum for the input descriptor
    pub checksum: String,
    /// Whether the descriptor is ranged
    #[serde(rename = "isrange")]
    pub is_range: bool,
    /// Whether the descriptor is solvable
    #[serde(rename = "issolvable")]
    pub is_solvable: bool,
    /// Whether the input descriptor contained at least one private key
    #[serde(rename = "hasprivatekeys")]
    pub has_private_keys: bool,
}

/// Response for the `GetDifficulty` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetDifficultyResponse {
    /// Wrapped primitive value
    pub value: u64,
}

impl<'de> serde::Deserialize<'de> for GetDifficultyResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = GetDifficultyResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetDifficultyResponse { value: v })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetDifficultyResponse { value: v as u64 })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetDifficultyResponse { value: v as u64 })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let value = v.parse::<u64>().map_err(de::Error::custom)?;
                Ok(GetDifficultyResponse { value })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetDifficultyResponse { value: v as u64 })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(GetDifficultyResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for GetDifficultyResponse {
    type Target = u64;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for GetDifficultyResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<u64> for GetDifficultyResponse {
    fn as_ref(&self) -> &u64 { &self.value }
}

impl From<u64> for GetDifficultyResponse {
    fn from(value: u64) -> Self { Self { value } }
}

impl From<GetDifficultyResponse> for u64 {
    fn from(wrapper: GetDifficultyResponse) -> Self { wrapper.value }
}

/// Response for the `GetHdKeys` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetHdKeysResponse {
    pub field: serde_json::Value,
}

/// Response for the `GetIndexInfo` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetIndexInfoResponse {
    /// The name of the index
    #[serde(default)]
    pub name: Option<serde_json::Value>,
}

/// Response for the `GetMemoryInfo` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMemoryInfoResponse {
    /// Information about locked memory manager
    pub locked: serde_json::Value,
}

/// Response for the `GetMempoolAncestors` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolAncestorsResponse {
    pub field_0: Vec<String>,
    #[serde(rename = "transactionid")]
    pub transaction_id: bitcoin::Txid,
}

/// Response for the `GetMempoolDescendants` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolDescendantsResponse {
    pub field_0: Vec<String>,
    #[serde(rename = "transactionid")]
    pub transaction_id: bitcoin::Txid,
}

/// Response for the `GetMempoolEntry` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolEntryResponse {
    /// virtual transaction size as defined in BIP 141. This is different from actual serialized size for witness transactions as witness data is discounted.
    #[serde(rename = "vsize")]
    pub v_size: u64,
    /// transaction weight as defined in BIP 141.
    pub weight: u64,
    /// local time transaction entered pool in seconds since 1 Jan 1970 GMT
    pub time: u64,
    /// block height when transaction entered pool
    pub height: u64,
    /// number of in-mempool descendant transactions (including this one)
    #[serde(rename = "descendantcount")]
    pub descendant_count: u64,
    /// virtual transaction size of in-mempool descendants (including this one)
    #[serde(rename = "descendantsize")]
    pub descendant_size: u64,
    /// number of in-mempool ancestor transactions (including this one)
    #[serde(rename = "ancestorcount")]
    pub ancestor_count: u64,
    /// virtual transaction size of in-mempool ancestors (including this one)
    #[serde(rename = "ancestorsize")]
    pub ancestor_size: u64,
    /// sigops-adjusted weight (as defined in BIP 141 and modified by '-bytespersigop') of this transaction's chunk
    #[serde(rename = "chunkweight")]
    pub chunk_weight: u64,
    /// hash of serialized transaction, including witness data
    #[serde(rename = "wtxid")]
    pub w_txid: String,
    pub fees: serde_json::Value,
    /// unconfirmed transactions used as inputs for this transaction
    pub depends: Vec<String>,
    /// unconfirmed transactions spending outputs from this transaction
    #[serde(rename = "spentby")]
    pub spent_by: Vec<String>,
    /// Whether this transaction signals BIP125 replaceability or has an unconfirmed ancestor signaling BIP125 replaceability. (DEPRECATED)
    #[serde(rename = "bip125-replaceable")]
    pub bip125_replaceable: bool,
    /// Whether this transaction is currently unbroadcast (initial broadcast not yet acknowledged by any peers)
    pub unbroadcast: bool,
}

/// Response for the `GetMempoolInfo` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolInfoResponse {
    /// True if the initial load attempt of the persisted mempool finished
    pub loaded: bool,
    /// Current tx count
    pub size: u64,
    /// Sum of all virtual transaction sizes as defined in BIP 141. Differs from actual serialized size because witness data is discounted
    pub bytes: u64,
    /// Total memory usage for the mempool
    pub usage: u64,
    /// Total fees for the mempool in BTC, ignoring modified fees through prioritisetransaction
    pub total_fee: f64,
    /// Maximum memory usage for the mempool
    #[serde(rename = "maxmempool")]
    pub max_mempool: u64,
    /// Minimum fee rate in BTC/kvB for tx to be accepted. Is the maximum of minrelaytxfee and minimum mempool fee
    #[serde(rename = "mempoolminfee")]
    pub mempool_min_fee: f64,
    /// Current minimum relay fee for transactions
    #[serde(rename = "minrelaytxfee")]
    pub min_relay_tx_fee: f64,
    /// minimum fee rate increment for mempool limiting or replacement in BTC/kvB
    #[serde(rename = "incrementalrelayfee")]
    pub incremental_relay_fee: f64,
    /// Current number of transactions that haven't passed initial broadcast yet
    #[serde(rename = "unbroadcastcount")]
    pub unbroadcast_count: u64,
    /// True if the mempool accepts RBF without replaceability signaling inspection (DEPRECATED)
    #[serde(rename = "fullrbf")]
    pub full_rbf: bool,
    /// True if the mempool accepts transactions with bare multisig outputs
    #[serde(rename = "permitbaremultisig")]
    pub permit_bare_multisig: Option<bool>,
    /// Maximum number of bytes that can be used by OP_RETURN outputs in the mempool
    #[serde(rename = "maxdatacarriersize")]
    pub max_data_carrier_size: Option<u64>,
    /// Maximum number of transactions that can be in a cluster (configured by -limitclustercount)
    #[serde(rename = "limitclustercount")]
    pub limit_cluster_count: Option<u64>,
    /// Maximum size of a cluster in virtual bytes (configured by -limitclustersize)
    #[serde(rename = "limitclustersize")]
    pub limit_cluster_size: Option<u64>,
    /// If the mempool is in a known-optimal transaction ordering
    pub optimal: bool,
}

/// Response for the `GetMiningInfo` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMiningInfoResponse {
    /// The current block
    pub blocks: u64,
    /// The block weight (including reserved weight for block header, txs count and coinbase tx) of the last assembled block (only present if a block was ever assembled)
    #[serde(rename = "currentblockweight")]
    pub current_block_weight: Option<u64>,
    /// The number of block transactions (excluding coinbase) of the last assembled block (only present if a block was ever assembled)
    #[serde(rename = "currentblocktx")]
    pub current_block_tx: Option<u64>,
    /// The current nBits, compact representation of the block difficulty target
    pub bits: String,
    /// The current difficulty
    pub difficulty: f64,
    /// The current target
    pub target: String,
    /// The network hashes per second
    #[serde(rename = "networkhashps")]
    pub network_hashps: f64,
    /// The size of the mempool
    #[serde(rename = "pooledtx")]
    pub pooled_tx: u64,
    /// Minimum feerate of packages selected for block inclusion in BTC/kvB
    #[serde(rename = "blockmintxfee")]
    pub block_min_tx_fee: Option<f64>,
    /// current network name (main, test, testnet4, signet, regtest)
    pub chain: String,
    /// The block challenge (aka. block script), in hexadecimal (only present if the current network is a signet)
    pub signet_challenge: Option<String>,
    /// The next block
    pub next: serde_json::Value,
    /// any network and blockchain warnings (run with `-deprecatedrpc=warnings` to return the latest warning as a single string)
    pub warnings: Vec<String>,
}

/// Response for the `GetNetTotals` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetNetTotalsResponse {
    /// Total bytes received
    #[serde(rename = "totalbytesrecv")]
    pub total_bytes_recv: u64,
    /// Total bytes sent
    #[serde(rename = "totalbytessent")]
    pub total_bytes_sent: u64,
    /// Current system UNIX epoch time in milliseconds
    #[serde(rename = "timemillis")]
    pub time_millis: u64,
    #[serde(rename = "uploadtarget")]
    pub upload_target: serde_json::Value,
}

/// Response for the `GetNetworkHashPs` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetNetworkHashPsResponse {
    /// Wrapped primitive value
    pub value: u64,
}

impl<'de> serde::Deserialize<'de> for GetNetworkHashPsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = GetNetworkHashPsResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetNetworkHashPsResponse { value: v })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetNetworkHashPsResponse { value: v as u64 })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetNetworkHashPsResponse { value: v as u64 })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let value = v.parse::<u64>().map_err(de::Error::custom)?;
                Ok(GetNetworkHashPsResponse { value })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetNetworkHashPsResponse { value: v as u64 })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(GetNetworkHashPsResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for GetNetworkHashPsResponse {
    type Target = u64;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for GetNetworkHashPsResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<u64> for GetNetworkHashPsResponse {
    fn as_ref(&self) -> &u64 { &self.value }
}

impl From<u64> for GetNetworkHashPsResponse {
    fn from(value: u64) -> Self { Self { value } }
}

impl From<GetNetworkHashPsResponse> for u64 {
    fn from(wrapper: GetNetworkHashPsResponse) -> Self { wrapper.value }
}

/// Response for the `GetNetworkInfo` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetNetworkInfoResponse {
    /// the server version
    pub version: u32,
    /// the server subversion string
    pub subversion: String,
    /// the protocol version
    #[serde(rename = "protocolversion")]
    pub protocol_version: u64,
    /// the services we offer to the network
    #[serde(rename = "localservices")]
    pub local_services: String,
    /// the services we offer to the network, in human-readable form
    #[serde(rename = "localservicesnames")]
    pub local_services_names: Vec<String>,
    /// true if transaction relay is requested from peers
    #[serde(rename = "localrelay")]
    pub local_relay: bool,
    /// the time offset
    #[serde(rename = "timeoffset")]
    pub time_offset: u64,
    /// the total number of connections
    pub connections: u64,
    /// the number of inbound connections
    pub connections_in: u64,
    /// the number of outbound connections
    pub connections_out: u64,
    /// whether p2p networking is enabled
    #[serde(rename = "networkactive")]
    pub network_active: bool,
    /// information per network
    pub networks: serde_json::Value,
    /// minimum relay fee rate for transactions in BTC/kvB
    #[serde(rename = "relayfee")]
    pub relay_fee: f64,
    /// minimum fee rate increment for mempool limiting or replacement in BTC/kvB
    #[serde(rename = "incrementalfee")]
    pub incremental_fee: f64,
    /// list of local addresses
    #[serde(rename = "localaddresses")]
    pub local_addresses: serde_json::Value,
    /// any network and blockchain warnings (run with `-deprecatedrpc=warnings` to return the latest warning as a single string)
    pub warnings: Vec<String>,
}

/// Response for the `GetNewAddress` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetNewAddressResponse {
    /// Wrapped primitive value
    pub value: String,
}

impl<'de> serde::Deserialize<'de> for GetNewAddressResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = GetNewAddressResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetNewAddressResponse { value: v.to_string() })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetNewAddressResponse { value: v.to_string() })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetNewAddressResponse { value: v.to_string() })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetNewAddressResponse { value: v.to_string() })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetNewAddressResponse { value: v.to_string() })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(GetNewAddressResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for GetNewAddressResponse {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for GetNewAddressResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<String> for GetNewAddressResponse {
    fn as_ref(&self) -> &String { &self.value }
}

impl From<String> for GetNewAddressResponse {
    fn from(value: String) -> Self { Self { value } }
}

impl From<GetNewAddressResponse> for String {
    fn from(wrapper: GetNewAddressResponse) -> Self { wrapper.value }
}

/// Response for the `GetNodeAddresses` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetNodeAddressesResponse {
    /// Wrapped array value
    pub value: Vec<GetNodeAddressesElement>,
}

impl<'de> serde::Deserialize<'de> for GetNodeAddressesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<GetNodeAddressesElement>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<GetNodeAddressesElement>> for GetNodeAddressesResponse {
    fn from(value: Vec<GetNodeAddressesElement>) -> Self { Self { value } }
}

impl From<GetNodeAddressesResponse> for Vec<GetNodeAddressesElement> {
    fn from(wrapper: GetNodeAddressesResponse) -> Self { wrapper.value }
}

/// Response for the `GetOrphanTxs` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetOrphanTxsResponse {
    /// Wrapped array value
    pub value: Vec<String>,
}

impl<'de> serde::Deserialize<'de> for GetOrphanTxsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<String>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<String>> for GetOrphanTxsResponse {
    fn from(value: Vec<String>) -> Self { Self { value } }
}

impl From<GetOrphanTxsResponse> for Vec<String> {
    fn from(wrapper: GetOrphanTxsResponse) -> Self { wrapper.value }
}

/// Response for the `GetPeerInfo` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetPeerInfoResponse {
    /// Wrapped array value
    pub value: Vec<GetPeerInfoElement>,
}

impl<'de> serde::Deserialize<'de> for GetPeerInfoResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<GetPeerInfoElement>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<GetPeerInfoElement>> for GetPeerInfoResponse {
    fn from(value: Vec<GetPeerInfoElement>) -> Self { Self { value } }
}

impl From<GetPeerInfoResponse> for Vec<GetPeerInfoElement> {
    fn from(wrapper: GetPeerInfoResponse) -> Self { wrapper.value }
}

/// Response for the `GetPrioritisedTransactions` RPC method
///
/// prioritisation keyed by txid
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetPrioritisedTransactionsResponse {
    #[serde(rename = "<transactionid>")]
    pub transactionid: serde_json::Value,
}

/// Response for the `GetRawAddrMan` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawAddrManResponse {
    /// buckets with addresses in the address manager table ( new, tried )
    #[serde(default)]
    pub table: Option<serde_json::Value>,
}

/// Response for the `GetRawChangeAddress` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetRawChangeAddressResponse {
    /// Wrapped primitive value
    pub value: String,
}

impl<'de> serde::Deserialize<'de> for GetRawChangeAddressResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = GetRawChangeAddressResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetRawChangeAddressResponse { value: v.to_string() })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetRawChangeAddressResponse { value: v.to_string() })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetRawChangeAddressResponse { value: v.to_string() })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetRawChangeAddressResponse { value: v.to_string() })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetRawChangeAddressResponse { value: v.to_string() })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(GetRawChangeAddressResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for GetRawChangeAddressResponse {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for GetRawChangeAddressResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<String> for GetRawChangeAddressResponse {
    fn as_ref(&self) -> &String { &self.value }
}

impl From<String> for GetRawChangeAddressResponse {
    fn from(value: String) -> Self { Self { value } }
}

impl From<GetRawChangeAddressResponse> for String {
    fn from(wrapper: GetRawChangeAddressResponse) -> Self { wrapper.value }
}

/// Response for the `GetRawMempool` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetRawMempoolResponse {
    /// Wrapped array value
    pub value: Vec<String>,
}

impl<'de> serde::Deserialize<'de> for GetRawMempoolResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<String>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<String>> for GetRawMempoolResponse {
    fn from(value: Vec<String>) -> Self { Self { value } }
}

impl From<GetRawMempoolResponse> for Vec<String> {
    fn from(wrapper: GetRawMempoolResponse) -> Self { wrapper.value }
}

/// Response for the `GetRawTransaction` RPC method
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetRawTransactionResponse {
    /// The serialized transaction as a hex-encoded string for 'txid'
    pub data: Option<String>,
    /// Whether specified block is in the active chain or not (only present with explicit "blockhash" argument)
    pub in_active_chain: Option<bool>,
    /// the block hash
    #[serde(rename = "blockhash")]
    pub block_hash: Option<bitcoin::BlockHash>,
    /// The confirmations
    pub confirmations: Option<i64>,
    /// The block time expressed in UNIX epoch time
    #[serde(rename = "blocktime")]
    pub block_time: Option<u64>,
    /// Same as "blocktime"
    pub time: Option<u64>,
    /// The serialized, hex-encoded data for 'txid'
    pub hex: Option<String>,
    /// The transaction id (same as provided)
    pub txid: Option<bitcoin::Txid>,
    /// The transaction hash (differs from txid for witness transactions)
    pub hash: Option<String>,
    /// The serialized transaction size
    pub size: Option<u64>,
    /// The virtual transaction size (differs from size for witness transactions)
    #[serde(rename = "vsize")]
    pub v_size: Option<u64>,
    /// The transaction's weight (between vsize*4-3 and vsize*4)
    pub weight: Option<u64>,
    /// The version
    pub version: Option<u32>,
    /// The lock time
    #[serde(rename = "locktime")]
    pub lock_time: Option<u64>,
    pub vin: Option<Vec<DecodedVin>>,
    pub vout: Option<Vec<DecodedVout>>,
    /// transaction fee in BTC, omitted if block undo data is not available
    pub fee: Option<f64>,
    pub vin_1: Option<serde_json::Value>,
}
impl<'de> serde::Deserialize<'de> for GetRawTransactionResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct ConditionalResponseVisitor;

        #[allow(clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for ConditionalResponseVisitor {
            type Value = GetRawTransactionResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("string or object")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let data = v.to_string();
                Ok(GetRawTransactionResponse {
                    data: Some(data),
                    in_active_chain: None,
                    block_hash: None,
                    confirmations: None,
                    block_time: None,
                    time: None,
                    hex: None,
                    txid: None,
                    hash: None,
                    size: None,
                    v_size: None,
                    weight: None,
                    version: None,
                    lock_time: None,
                    vin: None,
                    vout: None,
                    fee: None,
                    vin_1: None,
                })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut data = None;
                let mut in_active_chain = None;
                let mut block_hash = None;
                let mut confirmations = None;
                let mut block_time = None;
                let mut time = None;
                let mut hex = None;
                let mut txid = None;
                let mut hash = None;
                let mut size = None;
                let mut v_size = None;
                let mut weight = None;
                let mut version = None;
                let mut lock_time = None;
                let mut vin = None;
                let mut vout = None;
                let mut fee = None;
                let mut vin_1 = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "data" {
                        if data.is_some() {
                            return Err(de::Error::duplicate_field("data"));
                        }
                        data = Some(map.next_value::<String>()?);
                    }
                    if key == "in_active_chain" {
                        if in_active_chain.is_some() {
                            return Err(de::Error::duplicate_field("in_active_chain"));
                        }
                        in_active_chain = Some(map.next_value::<bool>()?);
                    }
                    if key == "blockhash" {
                        if block_hash.is_some() {
                            return Err(de::Error::duplicate_field("blockhash"));
                        }
                        block_hash = Some(map.next_value::<bitcoin::BlockHash>()?);
                    }
                    if key == "confirmations" {
                        if confirmations.is_some() {
                            return Err(de::Error::duplicate_field("confirmations"));
                        }
                        confirmations = Some(map.next_value::<i64>()?);
                    }
                    if key == "blocktime" {
                        if block_time.is_some() {
                            return Err(de::Error::duplicate_field("blocktime"));
                        }
                        block_time = Some(map.next_value::<u64>()?);
                    }
                    if key == "time" {
                        if time.is_some() {
                            return Err(de::Error::duplicate_field("time"));
                        }
                        time = Some(map.next_value::<u64>()?);
                    }
                    if key == "hex" {
                        if hex.is_some() {
                            return Err(de::Error::duplicate_field("hex"));
                        }
                        hex = Some(map.next_value::<String>()?);
                    }
                    if key == "txid" {
                        if txid.is_some() {
                            return Err(de::Error::duplicate_field("txid"));
                        }
                        txid = Some(map.next_value::<bitcoin::Txid>()?);
                    }
                    if key == "hash" {
                        if hash.is_some() {
                            return Err(de::Error::duplicate_field("hash"));
                        }
                        hash = Some(map.next_value::<String>()?);
                    }
                    if key == "size" {
                        if size.is_some() {
                            return Err(de::Error::duplicate_field("size"));
                        }
                        size = Some(map.next_value::<u64>()?);
                    }
                    if key == "vsize" {
                        if v_size.is_some() {
                            return Err(de::Error::duplicate_field("vsize"));
                        }
                        v_size = Some(map.next_value::<u64>()?);
                    }
                    if key == "weight" {
                        if weight.is_some() {
                            return Err(de::Error::duplicate_field("weight"));
                        }
                        weight = Some(map.next_value::<u64>()?);
                    }
                    if key == "version" {
                        if version.is_some() {
                            return Err(de::Error::duplicate_field("version"));
                        }
                        version = Some(map.next_value::<u32>()?);
                    }
                    if key == "locktime" {
                        if lock_time.is_some() {
                            return Err(de::Error::duplicate_field("locktime"));
                        }
                        lock_time = Some(map.next_value::<u64>()?);
                    }
                    if key == "vin" {
                        if vin.is_some() {
                            return Err(de::Error::duplicate_field("vin"));
                        }
                        vin = Some(map.next_value::<Vec<DecodedVin>>()?);
                    }
                    if key == "vout" {
                        if vout.is_some() {
                            return Err(de::Error::duplicate_field("vout"));
                        }
                        vout = Some(map.next_value::<Vec<DecodedVout>>()?);
                    }
                    if key == "fee" {
                        if fee.is_some() {
                            return Err(de::Error::duplicate_field("fee"));
                        }
                        fee = Some(map.next_value::<f64>()?);
                    }
                    if key == "vin_1" {
                        if vin_1.is_some() {
                            return Err(de::Error::duplicate_field("vin_1"));
                        }
                        vin_1 = Some(map.next_value::<serde_json::Value>()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                Ok(GetRawTransactionResponse {
                    data,
                    in_active_chain,
                    block_hash,
                    confirmations,
                    block_time,
                    time,
                    hex,
                    txid,
                    hash,
                    size,
                    v_size,
                    weight,
                    version,
                    lock_time,
                    vin,
                    vout,
                    fee,
                    vin_1,
                })
            }
        }

        deserializer.deserialize_any(ConditionalResponseVisitor)
    }
}

/// Response for the `GetReceivedByAddress` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetReceivedByAddressResponse {
    /// Wrapped primitive value
    pub value: bitcoin::Amount,
}

impl<'de> serde::Deserialize<'de> for GetReceivedByAddressResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = GetReceivedByAddressResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetReceivedByAddressResponse { value: bitcoin::Amount::from_sat(v) })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if v < 0 {
                    return Err(de::Error::custom(format!("Amount cannot be negative: {}", v)));
                }
                Ok(GetReceivedByAddressResponse { value: bitcoin::Amount::from_sat(v as u64) })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let amount = bitcoin::Amount::from_btc(v)
                    .map_err(|e| de::Error::custom(format!("Invalid BTC amount: {}", e)))?;
                Ok(GetReceivedByAddressResponse { value: amount })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let value = v.parse::<bitcoin::Amount>().map_err(de::Error::custom)?;
                Ok(GetReceivedByAddressResponse { value })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Err(de::Error::custom("cannot convert bool to bitcoin::Amount"))
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(GetReceivedByAddressResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for GetReceivedByAddressResponse {
    type Target = bitcoin::Amount;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for GetReceivedByAddressResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<bitcoin::Amount> for GetReceivedByAddressResponse {
    fn as_ref(&self) -> &bitcoin::Amount { &self.value }
}

impl From<bitcoin::Amount> for GetReceivedByAddressResponse {
    fn from(value: bitcoin::Amount) -> Self { Self { value } }
}

impl From<GetReceivedByAddressResponse> for bitcoin::Amount {
    fn from(wrapper: GetReceivedByAddressResponse) -> Self { wrapper.value }
}

/// Response for the `GetReceivedByLabel` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetReceivedByLabelResponse {
    /// Wrapped primitive value
    pub value: bitcoin::Amount,
}

impl<'de> serde::Deserialize<'de> for GetReceivedByLabelResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = GetReceivedByLabelResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetReceivedByLabelResponse { value: bitcoin::Amount::from_sat(v) })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if v < 0 {
                    return Err(de::Error::custom(format!("Amount cannot be negative: {}", v)));
                }
                Ok(GetReceivedByLabelResponse { value: bitcoin::Amount::from_sat(v as u64) })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let amount = bitcoin::Amount::from_btc(v)
                    .map_err(|e| de::Error::custom(format!("Invalid BTC amount: {}", e)))?;
                Ok(GetReceivedByLabelResponse { value: amount })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let value = v.parse::<bitcoin::Amount>().map_err(de::Error::custom)?;
                Ok(GetReceivedByLabelResponse { value })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Err(de::Error::custom("cannot convert bool to bitcoin::Amount"))
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(GetReceivedByLabelResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for GetReceivedByLabelResponse {
    type Target = bitcoin::Amount;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for GetReceivedByLabelResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<bitcoin::Amount> for GetReceivedByLabelResponse {
    fn as_ref(&self) -> &bitcoin::Amount { &self.value }
}

impl From<bitcoin::Amount> for GetReceivedByLabelResponse {
    fn from(value: bitcoin::Amount) -> Self { Self { value } }
}

impl From<GetReceivedByLabelResponse> for bitcoin::Amount {
    fn from(wrapper: GetReceivedByLabelResponse) -> Self { wrapper.value }
}

/// Response for the `GetRpcInfo` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRpcInfoResponse {
    /// All active commands
    pub active_commands: serde_json::Value,
    /// The complete file path to the debug log
    pub logpath: String,
}

/// Response for the `GetTransaction` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTransactionResponse {
    /// The amount in BTC
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub amount: bitcoin::Amount,
    /// The amount of the fee in BTC. This is negative and only available for the
    /// 'send' category of transactions.
    #[serde(deserialize_with = "option_amount_from_btc_float")]
    pub fee: Option<bitcoin::Amount>,
    /// The number of confirmations for the transaction. Negative confirmations means the
    /// transaction conflicted that many blocks ago.
    pub confirmations: i64,
    /// Only present if the transaction's only input is a coinbase one.
    pub generated: Option<bool>,
    /// Whether we consider the transaction to be trusted and safe to spend from.
    /// Only present when the transaction has 0 confirmations (or negative confirmations, if conflicted).
    pub trusted: Option<bool>,
    /// The block hash containing the transaction.
    #[serde(rename = "blockhash")]
    pub block_hash: Option<bitcoin::BlockHash>,
    /// The block height containing the transaction.
    #[serde(rename = "blockheight")]
    pub block_height: Option<u64>,
    /// The index of the transaction in the block that includes it.
    #[serde(rename = "blockindex")]
    pub block_index: Option<u64>,
    /// The block time expressed in UNIX epoch time.
    #[serde(rename = "blocktime")]
    pub block_time: Option<u64>,
    /// The transaction id.
    pub txid: bitcoin::Txid,
    /// The hash of serialized transaction, including witness data.
    #[serde(rename = "wtxid")]
    pub w_txid: String,
    /// Confirmed transactions that have been detected by the wallet to conflict with this transaction.
    #[serde(rename = "walletconflicts")]
    pub wallet_conflicts: Vec<String>,
    /// Only if 'category' is 'send'. The txid if this tx was replaced.
    pub replaced_by_txid: Option<String>,
    /// Only if 'category' is 'send'. The txid if this tx replaces another.
    pub replaces_txid: Option<String>,
    /// Transactions in the mempool that directly conflict with either this transaction or an ancestor transaction
    #[serde(rename = "mempoolconflicts")]
    pub mempool_conflicts: Vec<String>,
    /// If a comment to is associated with the transaction.
    pub to: Option<String>,
    /// The transaction time expressed in UNIX epoch time.
    pub time: u64,
    /// The time received expressed in UNIX epoch time.
    #[serde(rename = "timereceived")]
    pub time_received: u64,
    /// If a comment is associated with the transaction, only present if not empty.
    pub comment: Option<String>,
    /// ("yes|no|unknown") Whether this transaction signals BIP125 replaceability or has an unconfirmed ancestor signaling BIP125 replaceability.
    /// May be unknown for unconfirmed transactions not in the mempool because their unconfirmed ancestors are unknown.
    #[serde(rename = "bip125-replaceable")]
    pub bip125_replaceable: String,
    /// Only if 'category' is 'received'. List of parent descriptors for the output script of this coin.
    pub parent_descs: Option<Vec<String>>,
    pub details: serde_json::Value,
    /// Raw data for transaction
    pub hex: String,
    /// The decoded transaction (only present when `verbose` is passed)
    pub decoded: Option<serde_json::Value>,
    /// hash and height of the block this information was generated on
    #[serde(rename = "lastprocessedblock")]
    pub last_processed_block: serde_json::Value,
}

/// Response for the `GetTxOut` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTxOutResponse {
    pub field_0_1: (),
    /// The hash of the block at the tip of the chain
    #[serde(rename = "bestblock")]
    pub best_block: String,
    /// The number of confirmations
    pub confirmations: i64,
    /// The transaction value in BTC
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub value: bitcoin::Amount,
    #[serde(rename = "scriptPubKey")]
    pub script_pubkey: serde_json::Value,
    /// Coinbase or not
    pub coinbase: bool,
}

/// Response for the `GetTxOutProof` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetTxOutProofResponse {
    /// Wrapped primitive value
    pub value: String,
}

impl<'de> serde::Deserialize<'de> for GetTxOutProofResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = GetTxOutProofResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetTxOutProofResponse { value: v.to_string() })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetTxOutProofResponse { value: v.to_string() })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetTxOutProofResponse { value: v.to_string() })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetTxOutProofResponse { value: v.to_string() })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetTxOutProofResponse { value: v.to_string() })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(GetTxOutProofResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for GetTxOutProofResponse {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for GetTxOutProofResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<String> for GetTxOutProofResponse {
    fn as_ref(&self) -> &String { &self.value }
}

impl From<String> for GetTxOutProofResponse {
    fn from(value: String) -> Self { Self { value } }
}

impl From<GetTxOutProofResponse> for String {
    fn from(wrapper: GetTxOutProofResponse) -> Self { wrapper.value }
}

/// Response for the `GetTxOutSetInfo` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTxOutSetInfoResponse {
    /// The block height (index) of the returned statistics
    pub height: u64,
    /// The hash of the block at which these statistics are calculated
    #[serde(rename = "bestblock")]
    pub best_block: String,
    /// The number of unspent transaction outputs
    pub txouts: u64,
    /// Database-independent, meaningless metric indicating the UTXO set size
    #[serde(rename = "bogosize")]
    pub bogo_size: u64,
    /// The serialized hash (only present if 'hash_serialized_3' hash_type is chosen)
    pub hash_serialized_3: Option<String>,
    /// The serialized hash (only present if 'muhash' hash_type is chosen)
    pub muhash: Option<String>,
    /// The number of transactions with unspent outputs (not available when coinstatsindex is used)
    pub transactions: Option<u64>,
    /// The estimated size of the chainstate on disk (not available when coinstatsindex is used)
    pub disk_size: Option<u64>,
    /// The total amount of coins in the UTXO set
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub total_amount: bitcoin::Amount,
    /// The total amount of coins permanently excluded from the UTXO set (only available if coinstatsindex is used)
    #[serde(default)]
    #[serde(deserialize_with = "option_amount_from_btc_float")]
    pub total_unspendable_amount: Option<bitcoin::Amount>,
    /// Info on amounts in the block at this block height (only available if coinstatsindex is used)
    pub block_info: Option<serde_json::Value>,
}

/// Response for the `GetTxSpendingPrevOut` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTxSpendingPrevOutResponse {
    pub field: serde_json::Value,
}

/// Response for the `GetWalletInfo` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetWalletInfoResponse {
    /// the wallet name
    #[serde(rename = "walletname")]
    pub wallet_name: String,
    /// (DEPRECATED) only related to unsupported legacy wallet, returns the latest version 169900 for backwards compatibility
    #[serde(rename = "walletversion")]
    pub wallet_version: u64,
    /// the database format (only sqlite)
    pub format: String,
    /// the total number of transactions in the wallet
    #[serde(rename = "txcount")]
    pub tx_count: u64,
    /// how many new keys are pre-generated (only counts external keys)
    #[serde(rename = "keypoolsize")]
    pub key_pool_size: u64,
    /// how many new keys are pre-generated for internal use (used for change outputs, only appears if the wallet is using this feature, otherwise external keys are used)
    pub keypoolsize_hd_internal: Option<u64>,
    /// the UNIX epoch time until which the wallet is unlocked for transfers, or 0 if the wallet is locked (only present for passphrase-encrypted wallets)
    pub unlocked_until: Option<u64>,
    /// false if privatekeys are disabled for this wallet (enforced watch-only wallet)
    pub private_keys_enabled: bool,
    /// whether this wallet tracks clean/dirty coins in terms of reuse
    pub avoid_reuse: bool,
    /// current scanning details, or false if no scan is in progress
    pub scanning: serde_json::Value,
    /// whether this wallet uses descriptors for output script management
    pub descriptors: bool,
    /// whether this wallet is configured to use an external signer such as a hardware wallet
    pub external_signer: bool,
    /// Whether this wallet intentionally does not contain any keys, scripts, or descriptors
    pub blank: bool,
    /// The start time for blocks scanning. It could be modified by (re)importing any descriptor with an earlier timestamp.
    #[serde(rename = "birthtime")]
    pub birth_time: Option<u64>,
    /// The flags currently set on the wallet
    pub flags: Vec<String>,
    /// hash and height of the block this information was generated on
    #[serde(rename = "lastprocessedblock")]
    pub last_processed_block: serde_json::Value,
}

/// Response for the `Help` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct HelpResponse {
    /// Wrapped primitive value
    pub value: String,
}

impl<'de> serde::Deserialize<'de> for HelpResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = HelpResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(HelpResponse { value: v.to_string() })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(HelpResponse { value: v.to_string() })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(HelpResponse { value: v.to_string() })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(HelpResponse { value: v.to_string() })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(HelpResponse { value: v.to_string() })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(HelpResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for HelpResponse {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for HelpResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<String> for HelpResponse {
    fn as_ref(&self) -> &String { &self.value }
}

impl From<String> for HelpResponse {
    fn from(value: String) -> Self { Self { value } }
}

impl From<HelpResponse> for String {
    fn from(wrapper: HelpResponse) -> Self { wrapper.value }
}

/// Response for the `ImportDescriptors` RPC method
///
/// Response is an array with the same size as the input that has the execution result
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ImportDescriptorsResponse {
    pub field: serde_json::Value,
}

/// Response for the `ImportMempool` RPC method
///
/// This method returns no meaningful data.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ImportMempoolResponse;

/// Response for the `ImportPrunedFunds` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ImportPrunedFundsResponse {
    /// Wrapped primitive value
    pub value: (),
}

impl<'de> serde::Deserialize<'de> for ImportPrunedFundsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = ImportPrunedFundsResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ImportPrunedFundsResponse { value: () })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ImportPrunedFundsResponse { value: () })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ImportPrunedFundsResponse { value: () })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ImportPrunedFundsResponse { value: () })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ImportPrunedFundsResponse { value: () })
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ImportPrunedFundsResponse { value: () })
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ImportPrunedFundsResponse { value: () })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value::<()>()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(ImportPrunedFundsResponse { value: () })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for ImportPrunedFundsResponse {
    type Target = ();
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for ImportPrunedFundsResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<()> for ImportPrunedFundsResponse {
    fn as_ref(&self) -> &() { &self.value }
}

impl From<()> for ImportPrunedFundsResponse {
    fn from(value: ()) -> Self { Self { value } }
}

impl From<ImportPrunedFundsResponse> for () {
    fn from(wrapper: ImportPrunedFundsResponse) -> Self { wrapper.value }
}

/// Response for the `InvalidateBlock` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct InvalidateBlockResponse {
    /// Wrapped primitive value
    pub value: (),
}

impl<'de> serde::Deserialize<'de> for InvalidateBlockResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = InvalidateBlockResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(InvalidateBlockResponse { value: () })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(InvalidateBlockResponse { value: () })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(InvalidateBlockResponse { value: () })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(InvalidateBlockResponse { value: () })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(InvalidateBlockResponse { value: () })
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(InvalidateBlockResponse { value: () })
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(InvalidateBlockResponse { value: () })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value::<()>()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(InvalidateBlockResponse { value: () })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for InvalidateBlockResponse {
    type Target = ();
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for InvalidateBlockResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<()> for InvalidateBlockResponse {
    fn as_ref(&self) -> &() { &self.value }
}

impl From<()> for InvalidateBlockResponse {
    fn from(value: ()) -> Self { Self { value } }
}

impl From<InvalidateBlockResponse> for () {
    fn from(wrapper: InvalidateBlockResponse) -> Self { wrapper.value }
}

/// Response for the `JoinPsbts` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct JoinPsbtsResponse {
    /// Wrapped primitive value
    pub value: String,
}

impl<'de> serde::Deserialize<'de> for JoinPsbtsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = JoinPsbtsResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(JoinPsbtsResponse { value: v.to_string() })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(JoinPsbtsResponse { value: v.to_string() })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(JoinPsbtsResponse { value: v.to_string() })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(JoinPsbtsResponse { value: v.to_string() })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(JoinPsbtsResponse { value: v.to_string() })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(JoinPsbtsResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for JoinPsbtsResponse {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for JoinPsbtsResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<String> for JoinPsbtsResponse {
    fn as_ref(&self) -> &String { &self.value }
}

impl From<String> for JoinPsbtsResponse {
    fn from(value: String) -> Self { Self { value } }
}

impl From<JoinPsbtsResponse> for String {
    fn from(wrapper: JoinPsbtsResponse) -> Self { wrapper.value }
}

/// Response for the `KeypoolRefill` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct KeypoolRefillResponse {
    /// Wrapped primitive value
    pub value: (),
}

impl<'de> serde::Deserialize<'de> for KeypoolRefillResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = KeypoolRefillResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(KeypoolRefillResponse { value: () })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(KeypoolRefillResponse { value: () })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(KeypoolRefillResponse { value: () })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(KeypoolRefillResponse { value: () })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(KeypoolRefillResponse { value: () })
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(KeypoolRefillResponse { value: () })
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(KeypoolRefillResponse { value: () })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value::<()>()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(KeypoolRefillResponse { value: () })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for KeypoolRefillResponse {
    type Target = ();
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for KeypoolRefillResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<()> for KeypoolRefillResponse {
    fn as_ref(&self) -> &() { &self.value }
}

impl From<()> for KeypoolRefillResponse {
    fn from(value: ()) -> Self { Self { value } }
}

impl From<KeypoolRefillResponse> for () {
    fn from(wrapper: KeypoolRefillResponse) -> Self { wrapper.value }
}

/// Response for the `ListAddressGroupings` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ListAddressGroupingsResponse {
    /// Wrapped array value
    pub value: Vec<serde_json::Value>,
}

impl<'de> serde::Deserialize<'de> for ListAddressGroupingsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<serde_json::Value>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<serde_json::Value>> for ListAddressGroupingsResponse {
    fn from(value: Vec<serde_json::Value>) -> Self { Self { value } }
}

impl From<ListAddressGroupingsResponse> for Vec<serde_json::Value> {
    fn from(wrapper: ListAddressGroupingsResponse) -> Self { wrapper.value }
}

/// Response for the `ListBanned` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ListBannedResponse {
    /// Wrapped array value
    pub value: Vec<ListBannedElement>,
}

impl<'de> serde::Deserialize<'de> for ListBannedResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<ListBannedElement>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<ListBannedElement>> for ListBannedResponse {
    fn from(value: Vec<ListBannedElement>) -> Self { Self { value } }
}

impl From<ListBannedResponse> for Vec<ListBannedElement> {
    fn from(wrapper: ListBannedResponse) -> Self { wrapper.value }
}

/// Response for the `ListDescriptors` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ListDescriptorsResponse {
    /// Name of wallet this operation was performed on
    pub wallet_name: String,
    /// Array of descriptor objects (sorted by descriptor string representation)
    pub descriptors: serde_json::Value,
}

/// Response for the `ListLabels` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ListLabelsResponse {
    /// Wrapped array value
    pub value: Vec<serde_json::Value>,
}

impl<'de> serde::Deserialize<'de> for ListLabelsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<serde_json::Value>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<serde_json::Value>> for ListLabelsResponse {
    fn from(value: Vec<serde_json::Value>) -> Self { Self { value } }
}

impl From<ListLabelsResponse> for Vec<serde_json::Value> {
    fn from(wrapper: ListLabelsResponse) -> Self { wrapper.value }
}

/// Response for the `ListLockUnspent` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ListLockUnspentResponse {
    /// Wrapped array value
    pub value: Vec<ListLockUnspentElement>,
}

impl<'de> serde::Deserialize<'de> for ListLockUnspentResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<ListLockUnspentElement>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<ListLockUnspentElement>> for ListLockUnspentResponse {
    fn from(value: Vec<ListLockUnspentElement>) -> Self { Self { value } }
}

impl From<ListLockUnspentResponse> for Vec<ListLockUnspentElement> {
    fn from(wrapper: ListLockUnspentResponse) -> Self { wrapper.value }
}

/// Response for the `ListReceivedByAddress` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ListReceivedByAddressResponse {
    /// Wrapped array value
    pub value: Vec<ListReceivedByAddressElement>,
}

impl<'de> serde::Deserialize<'de> for ListReceivedByAddressResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<ListReceivedByAddressElement>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<ListReceivedByAddressElement>> for ListReceivedByAddressResponse {
    fn from(value: Vec<ListReceivedByAddressElement>) -> Self { Self { value } }
}

impl From<ListReceivedByAddressResponse> for Vec<ListReceivedByAddressElement> {
    fn from(wrapper: ListReceivedByAddressResponse) -> Self { wrapper.value }
}

/// Response for the `ListReceivedByLabel` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ListReceivedByLabelResponse {
    /// Wrapped array value
    pub value: Vec<ListReceivedByLabelElement>,
}

impl<'de> serde::Deserialize<'de> for ListReceivedByLabelResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<ListReceivedByLabelElement>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<ListReceivedByLabelElement>> for ListReceivedByLabelResponse {
    fn from(value: Vec<ListReceivedByLabelElement>) -> Self { Self { value } }
}

impl From<ListReceivedByLabelResponse> for Vec<ListReceivedByLabelElement> {
    fn from(wrapper: ListReceivedByLabelResponse) -> Self { wrapper.value }
}

/// Response for the `ListSinceBlock` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ListSinceBlockResponse {
    pub transactions: serde_json::Value,
    /// &lt;structure is the same as "transactions" above, only present if include_removed=true&gt;
    /// Note: transactions that were re-added in the active chain will appear as-is in this array, and may thus have a positive confirmation count.
    pub removed: Option<Vec<String>>,
    /// The hash of the block (target_confirmations-1) from the best block on the main chain, or the genesis hash if the referenced block does not exist yet. This is typically used to feed back into listsinceblock the next time you call it. So you would generally use a target_confirmations of say 6, so you will be continually re-notified of transactions until they've reached 6 confirmations plus any new ones
    #[serde(rename = "lastblock")]
    pub last_block: String,
}

/// Response for the `ListTransactions` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ListTransactionsResponse {
    /// Wrapped array value
    pub value: Vec<ListTransactionsElement>,
}

impl<'de> serde::Deserialize<'de> for ListTransactionsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<ListTransactionsElement>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<ListTransactionsElement>> for ListTransactionsResponse {
    fn from(value: Vec<ListTransactionsElement>) -> Self { Self { value } }
}

impl From<ListTransactionsResponse> for Vec<ListTransactionsElement> {
    fn from(wrapper: ListTransactionsResponse) -> Self { wrapper.value }
}

/// Response for the `ListUnspent` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ListUnspentResponse {
    /// Wrapped array value
    pub value: Vec<ListUnspentElement>,
}

impl<'de> serde::Deserialize<'de> for ListUnspentResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<ListUnspentElement>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<ListUnspentElement>> for ListUnspentResponse {
    fn from(value: Vec<ListUnspentElement>) -> Self { Self { value } }
}

impl From<ListUnspentResponse> for Vec<ListUnspentElement> {
    fn from(wrapper: ListUnspentResponse) -> Self { wrapper.value }
}

/// Response for the `ListWalletDir` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ListWalletDirResponse {
    pub wallets: serde_json::Value,
}

/// Response for the `ListWallets` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ListWalletsResponse {
    /// Wrapped array value
    pub value: Vec<serde_json::Value>,
}

impl<'de> serde::Deserialize<'de> for ListWalletsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<serde_json::Value>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<serde_json::Value>> for ListWalletsResponse {
    fn from(value: Vec<serde_json::Value>) -> Self { Self { value } }
}

impl From<ListWalletsResponse> for Vec<serde_json::Value> {
    fn from(wrapper: ListWalletsResponse) -> Self { wrapper.value }
}

/// Response for the `LoadTxOutSet` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct LoadTxOutSetResponse {
    /// the number of coins loaded from the snapshot
    pub coins_loaded: u64,
    /// the hash of the base of the snapshot
    pub tip_hash: String,
    /// the height of the base of the snapshot
    pub base_height: u64,
    /// the absolute path that the snapshot was loaded from
    pub path: String,
}

/// Response for the `LoadWallet` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct LoadWalletResponse {
    /// The wallet name if loaded successfully.
    pub name: String,
    /// Warning messages, if any, related to loading the wallet.
    pub warnings: Option<Vec<String>>,
}

/// Response for the `LockUnspent` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct LockUnspentResponse {
    /// Wrapped primitive value
    pub value: bool,
}

impl<'de> serde::Deserialize<'de> for LockUnspentResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = LockUnspentResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(LockUnspentResponse { value: v != 0 })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(LockUnspentResponse { value: v != 0 })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(LockUnspentResponse { value: v != 0.0 })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let value = v.parse::<bool>().map_err(de::Error::custom)?;
                Ok(LockUnspentResponse { value })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(LockUnspentResponse { value: v })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(LockUnspentResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for LockUnspentResponse {
    type Target = bool;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for LockUnspentResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<bool> for LockUnspentResponse {
    fn as_ref(&self) -> &bool { &self.value }
}

impl From<bool> for LockUnspentResponse {
    fn from(value: bool) -> Self { Self { value } }
}

impl From<LockUnspentResponse> for bool {
    fn from(wrapper: LockUnspentResponse) -> Self { wrapper.value }
}

/// Response for the `Logging` RPC method
///
/// keys are the logging categories, and values indicates its status
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct LoggingResponse {
    /// if being debug logged or not. false:inactive, true:active
    #[serde(default)]
    pub category: Option<bool>,
}

/// Response for the `MigrateWallet` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct MigrateWalletResponse {
    /// The name of the primary migrated wallet
    pub wallet_name: String,
    /// The name of the migrated wallet containing the watchonly scripts
    pub watchonly_name: Option<String>,
    /// The name of the migrated wallet containing solvable but not watched scripts
    pub solvables_name: Option<String>,
    /// The location of the backup of the original wallet
    pub backup_path: String,
}

/// Response for the `MockScheduler` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct MockSchedulerResponse {
    /// Wrapped primitive value
    pub value: (),
}

impl<'de> serde::Deserialize<'de> for MockSchedulerResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = MockSchedulerResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(MockSchedulerResponse { value: () })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(MockSchedulerResponse { value: () })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(MockSchedulerResponse { value: () })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(MockSchedulerResponse { value: () })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(MockSchedulerResponse { value: () })
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(MockSchedulerResponse { value: () })
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(MockSchedulerResponse { value: () })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value::<()>()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(MockSchedulerResponse { value: () })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for MockSchedulerResponse {
    type Target = ();
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for MockSchedulerResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<()> for MockSchedulerResponse {
    fn as_ref(&self) -> &() { &self.value }
}

impl From<()> for MockSchedulerResponse {
    fn from(value: ()) -> Self { Self { value } }
}

impl From<MockSchedulerResponse> for () {
    fn from(wrapper: MockSchedulerResponse) -> Self { wrapper.value }
}

/// Response for the `Ping` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PingResponse {
    /// Wrapped primitive value
    pub value: (),
}

impl<'de> serde::Deserialize<'de> for PingResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = PingResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(PingResponse { value: () })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(PingResponse { value: () })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(PingResponse { value: () })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(PingResponse { value: () })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(PingResponse { value: () })
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(PingResponse { value: () })
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(PingResponse { value: () })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value::<()>()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(PingResponse { value: () })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for PingResponse {
    type Target = ();
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for PingResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<()> for PingResponse {
    fn as_ref(&self) -> &() { &self.value }
}

impl From<()> for PingResponse {
    fn from(value: ()) -> Self { Self { value } }
}

impl From<PingResponse> for () {
    fn from(wrapper: PingResponse) -> Self { wrapper.value }
}

/// Response for the `PreciousBlock` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PreciousBlockResponse {
    /// Wrapped primitive value
    pub value: (),
}

impl<'de> serde::Deserialize<'de> for PreciousBlockResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = PreciousBlockResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(PreciousBlockResponse { value: () })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(PreciousBlockResponse { value: () })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(PreciousBlockResponse { value: () })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(PreciousBlockResponse { value: () })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(PreciousBlockResponse { value: () })
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(PreciousBlockResponse { value: () })
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(PreciousBlockResponse { value: () })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value::<()>()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(PreciousBlockResponse { value: () })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for PreciousBlockResponse {
    type Target = ();
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for PreciousBlockResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<()> for PreciousBlockResponse {
    fn as_ref(&self) -> &() { &self.value }
}

impl From<()> for PreciousBlockResponse {
    fn from(value: ()) -> Self { Self { value } }
}

impl From<PreciousBlockResponse> for () {
    fn from(wrapper: PreciousBlockResponse) -> Self { wrapper.value }
}

/// Response for the `PrioritiseTransaction` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PrioritiseTransactionResponse {
    /// Wrapped primitive value
    pub value: bool,
}

impl<'de> serde::Deserialize<'de> for PrioritiseTransactionResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = PrioritiseTransactionResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(PrioritiseTransactionResponse { value: v != 0 })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(PrioritiseTransactionResponse { value: v != 0 })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(PrioritiseTransactionResponse { value: v != 0.0 })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let value = v.parse::<bool>().map_err(de::Error::custom)?;
                Ok(PrioritiseTransactionResponse { value })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(PrioritiseTransactionResponse { value: v })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(PrioritiseTransactionResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for PrioritiseTransactionResponse {
    type Target = bool;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for PrioritiseTransactionResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<bool> for PrioritiseTransactionResponse {
    fn as_ref(&self) -> &bool { &self.value }
}

impl From<bool> for PrioritiseTransactionResponse {
    fn from(value: bool) -> Self { Self { value } }
}

impl From<PrioritiseTransactionResponse> for bool {
    fn from(wrapper: PrioritiseTransactionResponse) -> Self { wrapper.value }
}

/// Response for the `PruneBlockchain` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PruneBlockchainResponse {
    /// Wrapped primitive value
    pub value: u64,
}

impl<'de> serde::Deserialize<'de> for PruneBlockchainResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = PruneBlockchainResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(PruneBlockchainResponse { value: v })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(PruneBlockchainResponse { value: v as u64 })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(PruneBlockchainResponse { value: v as u64 })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let value = v.parse::<u64>().map_err(de::Error::custom)?;
                Ok(PruneBlockchainResponse { value })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(PruneBlockchainResponse { value: v as u64 })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(PruneBlockchainResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for PruneBlockchainResponse {
    type Target = u64;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for PruneBlockchainResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<u64> for PruneBlockchainResponse {
    fn as_ref(&self) -> &u64 { &self.value }
}

impl From<u64> for PruneBlockchainResponse {
    fn from(value: u64) -> Self { Self { value } }
}

impl From<PruneBlockchainResponse> for u64 {
    fn from(wrapper: PruneBlockchainResponse) -> Self { wrapper.value }
}

/// Response for the `PsbtBumpFee` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct PsbtBumpFeeResponse {
    /// The base64-encoded unsigned PSBT of the new transaction.
    pub psbt: String,
    /// The fee of the replaced transaction.
    #[serde(rename = "origfee")]
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub orig_fee: bitcoin::Amount,
    /// The fee of the new transaction.
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub fee: bitcoin::Amount,
    /// Errors encountered during processing (may be empty).
    pub errors: Vec<String>,
}

/// Response for the `ReconsiderBlock` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ReconsiderBlockResponse {
    /// Wrapped primitive value
    pub value: (),
}

impl<'de> serde::Deserialize<'de> for ReconsiderBlockResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = ReconsiderBlockResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ReconsiderBlockResponse { value: () })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ReconsiderBlockResponse { value: () })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ReconsiderBlockResponse { value: () })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ReconsiderBlockResponse { value: () })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ReconsiderBlockResponse { value: () })
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ReconsiderBlockResponse { value: () })
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ReconsiderBlockResponse { value: () })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value::<()>()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(ReconsiderBlockResponse { value: () })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for ReconsiderBlockResponse {
    type Target = ();
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for ReconsiderBlockResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<()> for ReconsiderBlockResponse {
    fn as_ref(&self) -> &() { &self.value }
}

impl From<()> for ReconsiderBlockResponse {
    fn from(value: ()) -> Self { Self { value } }
}

impl From<ReconsiderBlockResponse> for () {
    fn from(wrapper: ReconsiderBlockResponse) -> Self { wrapper.value }
}

/// Response for the `RemovePrunedFunds` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct RemovePrunedFundsResponse {
    /// Wrapped primitive value
    pub value: (),
}

impl<'de> serde::Deserialize<'de> for RemovePrunedFundsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = RemovePrunedFundsResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(RemovePrunedFundsResponse { value: () })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(RemovePrunedFundsResponse { value: () })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(RemovePrunedFundsResponse { value: () })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(RemovePrunedFundsResponse { value: () })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(RemovePrunedFundsResponse { value: () })
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(RemovePrunedFundsResponse { value: () })
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(RemovePrunedFundsResponse { value: () })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value::<()>()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(RemovePrunedFundsResponse { value: () })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for RemovePrunedFundsResponse {
    type Target = ();
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for RemovePrunedFundsResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<()> for RemovePrunedFundsResponse {
    fn as_ref(&self) -> &() { &self.value }
}

impl From<()> for RemovePrunedFundsResponse {
    fn from(value: ()) -> Self { Self { value } }
}

impl From<RemovePrunedFundsResponse> for () {
    fn from(wrapper: RemovePrunedFundsResponse) -> Self { wrapper.value }
}

/// Response for the `RescanBlockchain` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct RescanBlockchainResponse {
    /// The block height where the rescan started (the requested height or 0)
    pub start_height: u64,
    /// The height of the last rescanned block. May be null in rare cases if there was a reorg and the call didn't scan any blocks because they were already scanned in the background.
    pub stop_height: u64,
}

/// Response for the `RestoreWallet` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct RestoreWalletResponse {
    /// The wallet name if restored successfully.
    pub name: String,
    /// Warning messages, if any, related to restoring and loading the wallet.
    pub warnings: Option<Vec<String>>,
}

/// Response for the `SaveMempool` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SaveMempoolResponse {
    /// the directory and file where the mempool was saved
    pub filename: String,
}

/// Response for the `ScanBlocks` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ScanBlocksResponse {
    pub field_0_1: (),
    /// The height we started the scan from
    pub from_height: u64,
    /// The height we ended the scan at
    pub to_height: u64,
    /// Blocks that may have matched a scanobject.
    pub relevant_blocks: Vec<String>,
    /// true if the scan process was not aborted
    pub completed: bool,
    /// Approximate percent complete
    pub progress: u64,
    /// Height of the block currently being scanned
    pub current_height: u64,
    /// True if scan will be aborted (not necessarily before this RPC returns), or false if there is no scan to abort
    pub success: bool,
}

/// Response for the `ScanTxOutSet` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ScanTxOutSetResponse {
    /// Whether the scan was completed
    pub success: bool,
    /// The number of unspent transaction outputs scanned
    pub txouts: u64,
    /// The block height at which the scan was done
    pub height: u64,
    /// The hash of the block at the tip of the chain
    #[serde(rename = "bestblock")]
    pub best_block: String,
    pub unspents: serde_json::Value,
    /// The total amount of all found unspent outputs in BTC
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub total_amount: bitcoin::Amount,
    /// True if scan will be aborted (not necessarily before this RPC returns), or false if there is no scan to abort
    pub success_1: bool,
    /// Approximate percent complete
    pub progress: u64,
    pub field_3_1: (),
}

/// Response for the `Send` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SendResponse {
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// The transaction id for the send. Only 1 transaction is created regardless of the number of addresses.
    pub txid: Option<bitcoin::Txid>,
    /// If add_to_wallet is false, the hex-encoded raw transaction with signature(s)
    pub hex: Option<String>,
    /// If more signatures are needed, or if add_to_wallet is false, the base64-encoded (partially) signed transaction
    pub psbt: Option<String>,
}

/// Response for the `SendAll` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SendAllResponse {
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// The transaction id for the send. Only 1 transaction is created regardless of the number of addresses.
    pub txid: Option<bitcoin::Txid>,
    /// If add_to_wallet is false, the hex-encoded raw transaction with signature(s)
    pub hex: Option<String>,
    /// If more signatures are needed, or if add_to_wallet is false, the base64-encoded (partially) signed transaction
    pub psbt: Option<String>,
}

/// Response for the `SendMany` RPC method
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SendManyResponse {
    /// The transaction id for the send. Only 1 transaction is created regardless of
    /// the number of addresses.
    pub txid: Option<bitcoin::Txid>,
    /// The transaction fee reason.
    pub fee_reason: Option<String>,
}
impl<'de> serde::Deserialize<'de> for SendManyResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct ConditionalResponseVisitor;

        #[allow(clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for ConditionalResponseVisitor {
            type Value = SendManyResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("string or object")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let txid = bitcoin::Txid::from_str(v).map_err(de::Error::custom)?;
                Ok(SendManyResponse { txid: Some(txid), fee_reason: None })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut txid = None;
                let mut fee_reason = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "txid" {
                        if txid.is_some() {
                            return Err(de::Error::duplicate_field("txid"));
                        }
                        txid = Some(map.next_value::<bitcoin::Txid>()?);
                    }
                    if key == "fee_reason" {
                        if fee_reason.is_some() {
                            return Err(de::Error::duplicate_field("fee_reason"));
                        }
                        fee_reason = Some(map.next_value::<String>()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                Ok(SendManyResponse { txid, fee_reason })
            }
        }

        deserializer.deserialize_any(ConditionalResponseVisitor)
    }
}

/// Response for the `SendMsgToPeer` RPC method
///
/// This method returns no meaningful data.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SendMsgToPeerResponse;

/// Response for the `SendRawTransaction` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SendRawTransactionResponse {
    /// Wrapped primitive value
    pub value: String,
}

impl<'de> serde::Deserialize<'de> for SendRawTransactionResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = SendRawTransactionResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SendRawTransactionResponse { value: v.to_string() })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SendRawTransactionResponse { value: v.to_string() })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SendRawTransactionResponse { value: v.to_string() })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SendRawTransactionResponse { value: v.to_string() })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SendRawTransactionResponse { value: v.to_string() })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(SendRawTransactionResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for SendRawTransactionResponse {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for SendRawTransactionResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<String> for SendRawTransactionResponse {
    fn as_ref(&self) -> &String { &self.value }
}

impl From<String> for SendRawTransactionResponse {
    fn from(value: String) -> Self { Self { value } }
}

impl From<SendRawTransactionResponse> for String {
    fn from(wrapper: SendRawTransactionResponse) -> Self { wrapper.value }
}

/// Response for the `SendToAddress` RPC method
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SendToAddressResponse {
    /// The transaction id.
    pub txid: Option<bitcoin::Txid>,
    /// The transaction fee reason.
    pub fee_reason: Option<String>,
}
impl<'de> serde::Deserialize<'de> for SendToAddressResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct ConditionalResponseVisitor;

        #[allow(clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for ConditionalResponseVisitor {
            type Value = SendToAddressResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("string or object")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let txid = bitcoin::Txid::from_str(v).map_err(de::Error::custom)?;
                Ok(SendToAddressResponse { txid: Some(txid), fee_reason: None })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut txid = None;
                let mut fee_reason = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "txid" {
                        if txid.is_some() {
                            return Err(de::Error::duplicate_field("txid"));
                        }
                        txid = Some(map.next_value::<bitcoin::Txid>()?);
                    }
                    if key == "fee_reason" {
                        if fee_reason.is_some() {
                            return Err(de::Error::duplicate_field("fee_reason"));
                        }
                        fee_reason = Some(map.next_value::<String>()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                Ok(SendToAddressResponse { txid, fee_reason })
            }
        }

        deserializer.deserialize_any(ConditionalResponseVisitor)
    }
}

/// Response for the `SetBan` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SetBanResponse {
    /// Wrapped primitive value
    pub value: (),
}

impl<'de> serde::Deserialize<'de> for SetBanResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = SetBanResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SetBanResponse { value: () })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SetBanResponse { value: () })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SetBanResponse { value: () })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SetBanResponse { value: () })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SetBanResponse { value: () })
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SetBanResponse { value: () })
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SetBanResponse { value: () })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value::<()>()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(SetBanResponse { value: () })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for SetBanResponse {
    type Target = ();
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for SetBanResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<()> for SetBanResponse {
    fn as_ref(&self) -> &() { &self.value }
}

impl From<()> for SetBanResponse {
    fn from(value: ()) -> Self { Self { value } }
}

impl From<SetBanResponse> for () {
    fn from(wrapper: SetBanResponse) -> Self { wrapper.value }
}

/// Response for the `SetLabel` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SetLabelResponse {
    /// Wrapped primitive value
    pub value: (),
}

impl<'de> serde::Deserialize<'de> for SetLabelResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = SetLabelResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SetLabelResponse { value: () })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SetLabelResponse { value: () })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SetLabelResponse { value: () })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SetLabelResponse { value: () })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SetLabelResponse { value: () })
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SetLabelResponse { value: () })
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SetLabelResponse { value: () })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value::<()>()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(SetLabelResponse { value: () })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for SetLabelResponse {
    type Target = ();
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for SetLabelResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<()> for SetLabelResponse {
    fn as_ref(&self) -> &() { &self.value }
}

impl From<()> for SetLabelResponse {
    fn from(value: ()) -> Self { Self { value } }
}

impl From<SetLabelResponse> for () {
    fn from(wrapper: SetLabelResponse) -> Self { wrapper.value }
}

/// Response for the `SetMockTime` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SetMockTimeResponse {
    /// Wrapped primitive value
    pub value: (),
}

impl<'de> serde::Deserialize<'de> for SetMockTimeResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = SetMockTimeResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SetMockTimeResponse { value: () })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SetMockTimeResponse { value: () })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SetMockTimeResponse { value: () })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SetMockTimeResponse { value: () })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SetMockTimeResponse { value: () })
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SetMockTimeResponse { value: () })
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SetMockTimeResponse { value: () })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value::<()>()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(SetMockTimeResponse { value: () })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for SetMockTimeResponse {
    type Target = ();
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for SetMockTimeResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<()> for SetMockTimeResponse {
    fn as_ref(&self) -> &() { &self.value }
}

impl From<()> for SetMockTimeResponse {
    fn from(value: ()) -> Self { Self { value } }
}

impl From<SetMockTimeResponse> for () {
    fn from(wrapper: SetMockTimeResponse) -> Self { wrapper.value }
}

/// Response for the `SetNetworkActive` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SetNetworkActiveResponse {
    /// Wrapped primitive value
    pub value: bool,
}

impl<'de> serde::Deserialize<'de> for SetNetworkActiveResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = SetNetworkActiveResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SetNetworkActiveResponse { value: v != 0 })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SetNetworkActiveResponse { value: v != 0 })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SetNetworkActiveResponse { value: v != 0.0 })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let value = v.parse::<bool>().map_err(de::Error::custom)?;
                Ok(SetNetworkActiveResponse { value })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SetNetworkActiveResponse { value: v })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(SetNetworkActiveResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for SetNetworkActiveResponse {
    type Target = bool;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for SetNetworkActiveResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<bool> for SetNetworkActiveResponse {
    fn as_ref(&self) -> &bool { &self.value }
}

impl From<bool> for SetNetworkActiveResponse {
    fn from(value: bool) -> Self { Self { value } }
}

impl From<SetNetworkActiveResponse> for bool {
    fn from(wrapper: SetNetworkActiveResponse) -> Self { wrapper.value }
}

/// Response for the `SetWalletFlag` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SetWalletFlagResponse {
    /// The name of the flag that was modified
    pub flag_name: String,
    /// The new state of the flag
    pub flag_state: bool,
    /// Any warnings associated with the change
    pub warnings: Option<String>,
}

/// Response for the `SignMessage` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SignMessageResponse {
    /// Wrapped primitive value
    pub value: String,
}

impl<'de> serde::Deserialize<'de> for SignMessageResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = SignMessageResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SignMessageResponse { value: v.to_string() })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SignMessageResponse { value: v.to_string() })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SignMessageResponse { value: v.to_string() })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SignMessageResponse { value: v.to_string() })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SignMessageResponse { value: v.to_string() })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(SignMessageResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for SignMessageResponse {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for SignMessageResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<String> for SignMessageResponse {
    fn as_ref(&self) -> &String { &self.value }
}

impl From<String> for SignMessageResponse {
    fn from(value: String) -> Self { Self { value } }
}

impl From<SignMessageResponse> for String {
    fn from(wrapper: SignMessageResponse) -> Self { wrapper.value }
}

/// Response for the `SignMessageWithPrivKey` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SignMessageWithPrivKeyResponse {
    /// Wrapped primitive value
    pub value: String,
}

impl<'de> serde::Deserialize<'de> for SignMessageWithPrivKeyResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = SignMessageWithPrivKeyResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SignMessageWithPrivKeyResponse { value: v.to_string() })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SignMessageWithPrivKeyResponse { value: v.to_string() })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SignMessageWithPrivKeyResponse { value: v.to_string() })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SignMessageWithPrivKeyResponse { value: v.to_string() })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SignMessageWithPrivKeyResponse { value: v.to_string() })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(SignMessageWithPrivKeyResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for SignMessageWithPrivKeyResponse {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for SignMessageWithPrivKeyResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<String> for SignMessageWithPrivKeyResponse {
    fn as_ref(&self) -> &String { &self.value }
}

impl From<String> for SignMessageWithPrivKeyResponse {
    fn from(value: String) -> Self { Self { value } }
}

impl From<SignMessageWithPrivKeyResponse> for String {
    fn from(wrapper: SignMessageWithPrivKeyResponse) -> Self { wrapper.value }
}

/// Response for the `SignRawTransactionWithKey` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SignRawTransactionWithKeyResponse {
    /// The hex-encoded raw transaction with signature(s)
    pub hex: String,
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// Script verification errors (if there are any)
    pub errors: Option<serde_json::Value>,
}

/// Response for the `SignRawTransactionWithWallet` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SignRawTransactionWithWalletResponse {
    /// The hex-encoded raw transaction with signature(s)
    pub hex: String,
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// Script verification errors (if there are any)
    pub errors: Option<serde_json::Value>,
}

/// Response for the `SimulateRawTransaction` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SimulateRawTransactionResponse {
    /// The wallet balance change (negative means decrease).
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub balance_change: bitcoin::Amount,
}

/// Response for the `Stop` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct StopResponse {
    /// Wrapped primitive value
    pub value: String,
}

impl<'de> serde::Deserialize<'de> for StopResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = StopResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(StopResponse { value: v.to_string() })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(StopResponse { value: v.to_string() })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(StopResponse { value: v.to_string() })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(StopResponse { value: v.to_string() })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(StopResponse { value: v.to_string() })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(StopResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for StopResponse {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for StopResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<String> for StopResponse {
    fn as_ref(&self) -> &String { &self.value }
}

impl From<String> for StopResponse {
    fn from(value: String) -> Self { Self { value } }
}

impl From<StopResponse> for String {
    fn from(wrapper: StopResponse) -> Self { wrapper.value }
}

/// Response for the `SubmitBlock` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SubmitBlockResponse {
    pub field_0_1: (),
    /// According to BIP22
    pub field_1_1: String,
}

/// Response for the `SubmitHeader` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SubmitHeaderResponse {
    /// Wrapped primitive value
    pub value: (),
}

impl<'de> serde::Deserialize<'de> for SubmitHeaderResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = SubmitHeaderResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SubmitHeaderResponse { value: () })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SubmitHeaderResponse { value: () })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SubmitHeaderResponse { value: () })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SubmitHeaderResponse { value: () })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SubmitHeaderResponse { value: () })
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SubmitHeaderResponse { value: () })
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SubmitHeaderResponse { value: () })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value::<()>()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(SubmitHeaderResponse { value: () })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for SubmitHeaderResponse {
    type Target = ();
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for SubmitHeaderResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<()> for SubmitHeaderResponse {
    fn as_ref(&self) -> &() { &self.value }
}

impl From<()> for SubmitHeaderResponse {
    fn from(value: ()) -> Self { Self { value } }
}

impl From<SubmitHeaderResponse> for () {
    fn from(wrapper: SubmitHeaderResponse) -> Self { wrapper.value }
}

/// Response for the `SubmitPackage` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SubmitPackageResponse {
    /// The transaction package result message. "success" indicates all transactions were accepted into or are already in the mempool.
    pub package_msg: String,
    /// The transaction results keyed by wtxid. An entry is returned for every submitted wtxid.
    #[serde(rename = "tx-results")]
    pub tx_results: serde_json::Value,
    /// List of txids of replaced transactions
    #[serde(rename = "replaced-transactions")]
    pub replaced_transactions: Option<Vec<String>>,
}

/// Response for the `SyncWithValidationInterfaceQueue` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SyncWithValidationInterfaceQueueResponse {
    /// Wrapped primitive value
    pub value: (),
}

impl<'de> serde::Deserialize<'de> for SyncWithValidationInterfaceQueueResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = SyncWithValidationInterfaceQueueResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SyncWithValidationInterfaceQueueResponse { value: () })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SyncWithValidationInterfaceQueueResponse { value: () })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SyncWithValidationInterfaceQueueResponse { value: () })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SyncWithValidationInterfaceQueueResponse { value: () })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SyncWithValidationInterfaceQueueResponse { value: () })
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SyncWithValidationInterfaceQueueResponse { value: () })
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SyncWithValidationInterfaceQueueResponse { value: () })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value::<()>()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(SyncWithValidationInterfaceQueueResponse { value: () })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for SyncWithValidationInterfaceQueueResponse {
    type Target = ();
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for SyncWithValidationInterfaceQueueResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<()> for SyncWithValidationInterfaceQueueResponse {
    fn as_ref(&self) -> &() { &self.value }
}

impl From<()> for SyncWithValidationInterfaceQueueResponse {
    fn from(value: ()) -> Self { Self { value } }
}

impl From<SyncWithValidationInterfaceQueueResponse> for () {
    fn from(wrapper: SyncWithValidationInterfaceQueueResponse) -> Self { wrapper.value }
}

/// Response for the `TestMempoolAccept` RPC method
///
/// The result of the mempool acceptance test for each raw transaction in the input array.
/// Returns results for each transaction in the same order they were passed in.
/// Transactions that cannot be fully validated due to failures in other transactions will not contain an 'allowed' result.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct TestMempoolAcceptResponse {
    pub field: serde_json::Value,
}

/// Response for the `UnloadWallet` RPC method
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct UnloadWalletResponse {
    /// Warning messages, if any, related to unloading the wallet.
    pub warnings: Option<Vec<String>>,
}
impl<'de> serde::Deserialize<'de> for UnloadWalletResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct ConditionalResponseVisitor;

        #[allow(clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for ConditionalResponseVisitor {
            type Value = UnloadWalletResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("string or object")
            }

            fn visit_str<E>(self, _v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(UnloadWalletResponse { warnings: None })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut warnings = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "warnings" {
                        if warnings.is_some() {
                            return Err(de::Error::duplicate_field("warnings"));
                        }
                        warnings = Some(map.next_value::<Vec<String>>()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                Ok(UnloadWalletResponse { warnings })
            }
        }

        deserializer.deserialize_any(ConditionalResponseVisitor)
    }
}

/// Response for the `Uptime` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct UptimeResponse {
    /// Wrapped primitive value
    pub value: u64,
}

impl<'de> serde::Deserialize<'de> for UptimeResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = UptimeResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(UptimeResponse { value: v })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(UptimeResponse { value: v as u64 })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(UptimeResponse { value: v as u64 })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let value = v.parse::<u64>().map_err(de::Error::custom)?;
                Ok(UptimeResponse { value })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(UptimeResponse { value: v as u64 })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(UptimeResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for UptimeResponse {
    type Target = u64;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for UptimeResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<u64> for UptimeResponse {
    fn as_ref(&self) -> &u64 { &self.value }
}

impl From<u64> for UptimeResponse {
    fn from(value: u64) -> Self { Self { value } }
}

impl From<UptimeResponse> for u64 {
    fn from(wrapper: UptimeResponse) -> Self { wrapper.value }
}

/// Response for the `UtxoUpdatePsbt` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct UtxoUpdatePsbtResponse {
    /// Wrapped primitive value
    pub value: String,
}

impl<'de> serde::Deserialize<'de> for UtxoUpdatePsbtResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = UtxoUpdatePsbtResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(UtxoUpdatePsbtResponse { value: v.to_string() })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(UtxoUpdatePsbtResponse { value: v.to_string() })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(UtxoUpdatePsbtResponse { value: v.to_string() })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(UtxoUpdatePsbtResponse { value: v.to_string() })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(UtxoUpdatePsbtResponse { value: v.to_string() })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(UtxoUpdatePsbtResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for UtxoUpdatePsbtResponse {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for UtxoUpdatePsbtResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<String> for UtxoUpdatePsbtResponse {
    fn as_ref(&self) -> &String { &self.value }
}

impl From<String> for UtxoUpdatePsbtResponse {
    fn from(value: String) -> Self { Self { value } }
}

impl From<UtxoUpdatePsbtResponse> for String {
    fn from(wrapper: UtxoUpdatePsbtResponse) -> Self { wrapper.value }
}

/// Response for the `ValidateAddress` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ValidateAddressResponse {
    /// If the address is valid or not
    pub isvalid: bool,
    /// The bitcoin address validated
    pub address: Option<String>,
    /// The hex-encoded output script generated by the address
    #[serde(rename = "scriptPubKey")]
    pub script_pubkey: Option<bitcoin::ScriptBuf>,
    /// If the key is a script
    #[serde(rename = "isscript")]
    pub is_script: Option<bool>,
    /// If the address is a witness address
    #[serde(rename = "iswitness")]
    pub is_witness: Option<bool>,
    /// The version number of the witness program
    pub witness_version: Option<u64>,
    /// The hex value of the witness program
    pub witness_program: Option<String>,
    /// Error message, if any
    pub error: Option<String>,
    /// Indices of likely error locations in address, if known (e.g. Bech32 errors)
    pub error_locations: Option<Vec<String>>,
}

/// Response for the `VerifyChain` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct VerifyChainResponse {
    /// Wrapped primitive value
    pub value: bool,
}

impl<'de> serde::Deserialize<'de> for VerifyChainResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = VerifyChainResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(VerifyChainResponse { value: v != 0 })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(VerifyChainResponse { value: v != 0 })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(VerifyChainResponse { value: v != 0.0 })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let value = v.parse::<bool>().map_err(de::Error::custom)?;
                Ok(VerifyChainResponse { value })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(VerifyChainResponse { value: v })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(VerifyChainResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for VerifyChainResponse {
    type Target = bool;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for VerifyChainResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<bool> for VerifyChainResponse {
    fn as_ref(&self) -> &bool { &self.value }
}

impl From<bool> for VerifyChainResponse {
    fn from(value: bool) -> Self { Self { value } }
}

impl From<VerifyChainResponse> for bool {
    fn from(wrapper: VerifyChainResponse) -> Self { wrapper.value }
}

/// Response for the `VerifyMessage` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct VerifyMessageResponse {
    /// Wrapped primitive value
    pub value: bool,
}

impl<'de> serde::Deserialize<'de> for VerifyMessageResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = VerifyMessageResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(VerifyMessageResponse { value: v != 0 })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(VerifyMessageResponse { value: v != 0 })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(VerifyMessageResponse { value: v != 0.0 })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let value = v.parse::<bool>().map_err(de::Error::custom)?;
                Ok(VerifyMessageResponse { value })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(VerifyMessageResponse { value: v })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(VerifyMessageResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for VerifyMessageResponse {
    type Target = bool;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for VerifyMessageResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<bool> for VerifyMessageResponse {
    fn as_ref(&self) -> &bool { &self.value }
}

impl From<bool> for VerifyMessageResponse {
    fn from(value: bool) -> Self { Self { value } }
}

impl From<VerifyMessageResponse> for bool {
    fn from(wrapper: VerifyMessageResponse) -> Self { wrapper.value }
}

/// Response for the `VerifyTxOutProof` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct VerifyTxOutProofResponse {
    /// Wrapped array value
    pub value: Vec<serde_json::Value>,
}

impl<'de> serde::Deserialize<'de> for VerifyTxOutProofResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<serde_json::Value>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<serde_json::Value>> for VerifyTxOutProofResponse {
    fn from(value: Vec<serde_json::Value>) -> Self { Self { value } }
}

impl From<VerifyTxOutProofResponse> for Vec<serde_json::Value> {
    fn from(wrapper: VerifyTxOutProofResponse) -> Self { wrapper.value }
}

/// Response for the `WaitForBlock` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct WaitForBlockResponse {
    /// The blockhash
    pub hash: String,
    /// Block height
    pub height: u64,
}

/// Response for the `WaitForBlockHeight` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct WaitForBlockHeightResponse {
    /// The blockhash
    pub hash: String,
    /// Block height
    pub height: u64,
}

/// Response for the `WaitForNewBlock` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct WaitForNewBlockResponse {
    /// The blockhash
    pub hash: String,
    /// Block height
    pub height: u64,
}

/// Response for the `WalletCreateFundedPsbt` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct WalletCreateFundedPsbtResponse {
    /// The resulting raw transaction (base64-encoded string)
    pub psbt: String,
    /// Fee in BTC the resulting transaction pays
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub fee: bitcoin::Amount,
    /// The position of the added change output, or -1
    pub changepos: i64,
}

/// Response for the `WalletDisplayAddress` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct WalletDisplayAddressResponse {
    /// The address as confirmed by the signer
    pub address: String,
}

/// Response for the `WalletLock` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct WalletLockResponse {
    /// Wrapped primitive value
    pub value: (),
}

impl<'de> serde::Deserialize<'de> for WalletLockResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = WalletLockResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(WalletLockResponse { value: () })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(WalletLockResponse { value: () })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(WalletLockResponse { value: () })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(WalletLockResponse { value: () })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(WalletLockResponse { value: () })
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(WalletLockResponse { value: () })
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(WalletLockResponse { value: () })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value::<()>()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(WalletLockResponse { value: () })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for WalletLockResponse {
    type Target = ();
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for WalletLockResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<()> for WalletLockResponse {
    fn as_ref(&self) -> &() { &self.value }
}

impl From<()> for WalletLockResponse {
    fn from(value: ()) -> Self { Self { value } }
}

impl From<WalletLockResponse> for () {
    fn from(wrapper: WalletLockResponse) -> Self { wrapper.value }
}

/// Response for the `WalletPassphrase` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct WalletPassphraseResponse {
    /// Wrapped primitive value
    pub value: (),
}

impl<'de> serde::Deserialize<'de> for WalletPassphraseResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = WalletPassphraseResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(WalletPassphraseResponse { value: () })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(WalletPassphraseResponse { value: () })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(WalletPassphraseResponse { value: () })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(WalletPassphraseResponse { value: () })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(WalletPassphraseResponse { value: () })
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(WalletPassphraseResponse { value: () })
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(WalletPassphraseResponse { value: () })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value::<()>()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(WalletPassphraseResponse { value: () })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for WalletPassphraseResponse {
    type Target = ();
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for WalletPassphraseResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<()> for WalletPassphraseResponse {
    fn as_ref(&self) -> &() { &self.value }
}

impl From<()> for WalletPassphraseResponse {
    fn from(value: ()) -> Self { Self { value } }
}

impl From<WalletPassphraseResponse> for () {
    fn from(wrapper: WalletPassphraseResponse) -> Self { wrapper.value }
}

/// Response for the `WalletPassphraseChange` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct WalletPassphraseChangeResponse {
    /// Wrapped primitive value
    pub value: (),
}

impl<'de> serde::Deserialize<'de> for WalletPassphraseChangeResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = WalletPassphraseChangeResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(WalletPassphraseChangeResponse { value: () })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(WalletPassphraseChangeResponse { value: () })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(WalletPassphraseChangeResponse { value: () })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(WalletPassphraseChangeResponse { value: () })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(WalletPassphraseChangeResponse { value: () })
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(WalletPassphraseChangeResponse { value: () })
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(WalletPassphraseChangeResponse { value: () })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value::<()>()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(WalletPassphraseChangeResponse { value: () })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for WalletPassphraseChangeResponse {
    type Target = ();
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for WalletPassphraseChangeResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<()> for WalletPassphraseChangeResponse {
    fn as_ref(&self) -> &() { &self.value }
}

impl From<()> for WalletPassphraseChangeResponse {
    fn from(value: ()) -> Self { Self { value } }
}

impl From<WalletPassphraseChangeResponse> for () {
    fn from(wrapper: WalletPassphraseChangeResponse) -> Self { wrapper.value }
}

/// Response for the `WalletProcessPsbt` RPC method
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct WalletProcessPsbtResponse {
    /// The base64-encoded partially signed transaction
    pub psbt: String,
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// The hex-encoded network transaction if complete
    pub hex: Option<String>,
}

/// Deserializer for bitcoin::Amount that handles both float (BTC) and integer (satoshis) formats
/// Bitcoin Core returns amounts as floats in BTC, but some fields may be integers in satoshis
fn amount_from_btc_float<'de, D>(deserializer: D) -> Result<bitcoin::Amount, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use std::fmt;

    use serde::de::{self, Visitor};

    struct AmountVisitor;

    impl Visitor<'_> for AmountVisitor {
        type Value = bitcoin::Amount;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a number (float BTC or integer satoshis)")
        }

        fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            bitcoin::Amount::from_btc(v)
                .map_err(|e| E::custom(format!("Invalid BTC amount: {}", e)))
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(bitcoin::Amount::from_sat(v))
        }

        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if v < 0 {
                return Err(E::custom(format!("Amount cannot be negative: {}", v)));
            }
            Ok(bitcoin::Amount::from_sat(v as u64))
        }
    }

    deserializer.deserialize_any(AmountVisitor)
}

/// Deserializer for Option<bitcoin::Amount> that handles both float (BTC) and integer (satoshis) formats
/// Bitcoin Core returns amounts as floats in BTC, but some fields may be integers in satoshis
/// This deserializer also handles null/None values
fn option_amount_from_btc_float<'de, D>(
    deserializer: D,
) -> Result<Option<bitcoin::Amount>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use std::fmt;

    use serde::de::{self, Visitor};

    struct OptionAmountVisitor;

    #[allow(clippy::needless_lifetimes)]
    impl<'de> Visitor<'de> for OptionAmountVisitor {
        type Value = Option<bitcoin::Amount>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an optional number (float BTC or integer satoshis)")
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            amount_from_btc_float(deserializer).map(Some)
        }

        fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            bitcoin::Amount::from_btc(v)
                .map_err(|e| E::custom(format!("Invalid BTC amount: {}", e)))
                .map(Some)
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(bitcoin::Amount::from_sat(v)))
        }

        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if v < 0 {
                return Err(E::custom(format!("Amount cannot be negative: {}", v)));
            }
            Ok(Some(bitcoin::Amount::from_sat(v as u64)))
        }
    }

    deserializer.deserialize_any(OptionAmountVisitor)
}
