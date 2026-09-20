// Generated version-specific RPC response types
//
// Generated for Bitcoin Core v32.0
//
// These types are version-specific and may not match other versions.
//
// JSON `number` (Core NUM) mapping in this crate (field-name rules, not OpenRPC `integer`):
//
// - Default / catchall: `u64` (height, size, time, count, blocks, …)
// - Signed domains (`confirmations`, `changepos`, `nblocks`, …): `i64`
// - Small unsigned (`version`, `verbosity`, `locktime`, `n`, min/max conf, …): `u32`
// - Port: `u16`
// - Fee / difficulty / probability / percentage / rate fields: `f64`
// - Amounts (`amount`, `balance`, …): `bitcoin::Amount` (BTC float on the wire)
//
// When Core OpenRPC gains explicit integer stamps, IR/codegen will prefer those over names.
// Do not treat field-name heuristics as a Core dump contract.
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Transactions removed from the private broadcast queue
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AbortPrivateBroadcastRemovedTransactions {
    /// The serialized, hex-encoded transaction data
    pub hex: String,
    /// The transaction hash in hex
    pub txid: bitcoin::Txid,
    /// The transaction witness hash in hex
    /// Wire JSON key: `wtxid` (Rust field `w_txid`).
    #[serde(rename = "wtxid")]
    pub w_txid: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AbortPrivateBroadcastResult {
    /// Transactions removed from the private broadcast queue
    pub removed_transactions: Vec<AbortPrivateBroadcastRemovedTransactions>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AddConnectionResult {
    /// Address of newly added connection.
    pub address: String,
    /// Type of connection opened.
    pub connection_type: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AddHdKeyResult {
    /// The xpub of the HD key that was added to the wallet
    pub xpub: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AddPeerAddressResult {
    /// error description, if the address could not be added
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// whether the peer address was successfully added to the address manager table
    pub success: bool,
}

/// Type alias for Amount
pub type Amount = bitcoin::Amount;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AnalyzePsbtInputs {
    /// Whether a UTXO is provided
    pub has_utxo: bool,
    /// Whether the input is finalized
    pub is_final: bool,
    /// Things that are missing that are required to complete this input
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing: Option<AnalyzePsbtMissing>,
    /// Role of the next person that this input needs to go to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
}

/// Things that are missing that are required to complete this input
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AnalyzePsbtMissing {
    /// Wire JSON key: `pubkeys` (Rust field `pub_keys`).
    #[serde(rename = "pubkeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pub_keys: Option<Vec<String>>,
    /// Hash160 of the redeem script that is missing
    /// Wire JSON key: `redeemscript` (Rust field `redeem_script`).
    #[serde(rename = "redeemscript")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redeem_script: Option<bitcoin::ScriptBuf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signatures: Option<Vec<String>>,
    /// SHA256 of the witness script that is missing
    /// Wire JSON key: `witnessscript` (Rust field `witness_script`).
    #[serde(rename = "witnessscript")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_script: Option<bitcoin::ScriptBuf>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AnalyzePsbtResult {
    /// Error message (if there is one)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Estimated feerate of the final signed transaction in BTC/kvB. Shown only if all UTXO slots in the PSBT have been filled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_feerate: Option<f64>,
    /// Estimated vsize of the final signed transaction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_vsize: Option<u64>,
    /// The transaction fee paid. Shown only if all UTXO slots in the PSBT have been filled
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        serialize_with = "option_amount_to_btc_float",
        deserialize_with = "option_amount_from_btc_float"
    )]
    pub fee: Option<bitcoin::Amount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<AnalyzePsbtInputs>>,
    /// Role of the next person that this psbt needs to go to
    pub next: String,
}

/// Type alias for BlockHash
pub type BlockHash = bitcoin::BlockHash;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct BumpFeeResult {
    /// Errors encountered during processing (may be empty).
    pub errors: Vec<String>,
    /// The fee of the new transaction.
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub fee: bitcoin::Amount,
    /// The fee of the replaced transaction.
    /// Wire JSON key: `origfee` (Rust field `orig_fee`).
    #[serde(rename = "origfee")]
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub orig_fee: bitcoin::Amount,
    /// The id of the new transaction.
    pub txid: bitcoin::Txid,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CreateMultisigResult {
    /// The value of the new multisig address.
    pub address: String,
    /// The descriptor for this multisig
    pub descriptor: String,
    /// The string value of the hex-encoded redemption script.
    /// Wire JSON key: `redeemScript` (Rust field `redeem_script`).
    #[serde(rename = "redeemScript")]
    pub redeem_script: bitcoin::ScriptBuf,
    /// Any warnings resulting from the creation of this multisig
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CreateWalletDescriptorResult {
    /// The public descriptors that were added to the wallet
    pub descs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CreateWalletResult {
    /// The wallet name if created successfully. If the wallet was created using a full path, the wallet_name will be the full path.
    pub name: String,
    /// Warning messages, if any, related to creating and loading the wallet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodePsbtBip32Derivs {
    /// The fingerprint of the master key
    pub master_fingerprint: String,
    /// The path
    pub path: String,
    /// The public key with the derivation path as the value.
    pub pubkey: String,
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
    /// The fingerprint of the master key
    pub master_fingerprint: String,
    /// The path
    pub path: String,
    /// The extended public key this path corresponds to
    pub xpub: String,
}

/// Type alias for DecodePsbtHash160Preimages
pub type DecodePsbtHash160Preimages = BTreeMap<String, String>;

/// Type alias for DecodePsbtHash256Preimages
pub type DecodePsbtHash256Preimages = BTreeMap<String, String>;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodePsbtInputs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bip32_derivs: Option<Vec<DecodePsbtBip32Derivs>>,
    /// Wire JSON key: `final_scriptSig` (Rust field `final_script_sig`).
    #[serde(rename = "final_scriptSig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_script_sig: Option<DecodePsbtFinalScriptSig>,
    /// Wire JSON key: `final_scriptwitness` (Rust field `final_script_witness`).
    #[serde(rename = "final_scriptwitness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_script_witness: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash160_preimages: Option<BTreeMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash256_preimages: Option<BTreeMap<String, String>>,
    /// Height-based locktime required for this input
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height_locktime: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub musig2_partial_sigs: Option<Vec<DecodePsbtMusig2PartialSigs>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub musig2_participant_pubkeys: Option<Vec<DecodePsbtMusig2ParticipantPubkeys>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub musig2_pubnonces: Option<Vec<DecodePsbtMusig2PubNonces>>,
    /// Decoded network transaction for non-witness UTXOs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_witness_utxo: Option<DecodePsbtNonWitnessUtxo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial_signatures: Option<BTreeMap<String, String>>,
    /// TXID of the transaction containing the output being spent by this input
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_txid: Option<String>,
    /// Index of the output being spent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_vout: Option<u64>,
    /// The input proprietary map
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proprietary: Option<Vec<DecodePsbtProprietary>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redeem_script: Option<DecodePsbtRedeemScript>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ripemd160_preimages: Option<BTreeMap<String, String>>,
    /// Sequence number for this input
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha256_preimages: Option<BTreeMap<String, String>>,
    /// The sighash type to be used
    /// Wire JSON key: `sighash` (Rust field `sig_hash`).
    #[serde(rename = "sighash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sig_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taproot_bip32_derivs: Option<Vec<DecodePsbtTaprootBip32Derivs>>,
    /// The hex-encoded Taproot x-only internal key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taproot_internal_key: Option<String>,
    /// hex-encoded signature for the Taproot key path spend
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taproot_key_path_sig: Option<String>,
    /// The hex-encoded Taproot merkle root
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taproot_merkle_root: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taproot_script_path_sigs: Option<Vec<DecodePsbtTaprootScriptPathSigs>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taproot_scripts: Option<Vec<DecodePsbtTaprootScripts>>,
    /// Time-based locktime required for this input
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_locktime: Option<u64>,
    /// The unknown input fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown: Option<BTreeMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_script: Option<DecodePsbtWitnessScript>,
    /// Transaction output for witness UTXOs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_utxo: Option<DecodePsbtWitnessUtxo>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodePsbtMusig2PartialSigs {
    /// The compressed aggregate public key for which this partial signature is for.
    pub aggregate_pubkey: String,
    /// The hash of the leaf script that contains the aggregate pubkey being signed for. Omitted when signing for the internal key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leaf_hash: Option<String>,
    /// The partial signature itself.
    pub partial_sig: String,
    /// The compressed public key of the participant that created this partial signature.
    pub participant_pubkey: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodePsbtMusig2ParticipantPubkeys {
    /// The compressed aggregate public key for which the participants create.
    pub aggregate_pubkey: String,
    pub participant_pubkeys: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodePsbtMusig2PubNonces {
    /// The compressed aggregate public key for which this pubnonce is for.
    pub aggregate_pubkey: String,
    /// The hash of the leaf script that contains the aggregate pubkey being signed for. Omitted when signing for the internal key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leaf_hash: Option<String>,
    /// The compressed public key of the participant that created this pubnonce.
    pub participant_pubkey: String,
    /// The public nonce itself.
    /// Wire JSON key: `pubnonce` (Rust field `pub_nonce`).
    #[serde(rename = "pubnonce")]
    pub pub_nonce: String,
}

/// Decoded network transaction for non-witness UTXOs
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodePsbtNonWitnessUtxo {
    /// The transaction hash (differs from txid for witness transactions)
    pub hash: String,
    /// The lock time
    /// Wire JSON key: `locktime` (Rust field `lock_time`).
    #[serde(rename = "locktime")]
    pub lock_time: u64,
    /// The serialized transaction size
    pub size: u64,
    /// The transaction id
    pub txid: bitcoin::Txid,
    /// The version
    pub version: u32,
    pub vin: Vec<DecodePsbtVin>,
    pub vout: Vec<DecodePsbtVout>,
    /// The virtual transaction size (differs from size for witness transactions)
    /// Wire JSON key: `vsize` (Rust field `v_size`).
    #[serde(rename = "vsize")]
    pub v_size: u64,
    /// The transaction's weight (between vsize*4-3 and vsize*4)
    pub weight: u64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodePsbtOutputs {
    /// The amount (nValue) for this output
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        serialize_with = "option_amount_to_btc_float",
        deserialize_with = "option_amount_from_btc_float"
    )]
    pub amount: Option<bitcoin::Amount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bip32_derivs: Option<Vec<DecodePsbtBip32Derivs>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub musig2_participant_pubkeys: Option<Vec<DecodePsbtMusig2ParticipantPubkeys>>,
    /// The output proprietary map
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proprietary: Option<Vec<DecodePsbtProprietary>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redeem_script: Option<DecodePsbtRedeemScript>,
    /// The output script (scriptPubKey) for this output
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<DecodePsbtScript>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taproot_bip32_derivs: Option<Vec<DecodePsbtTaprootBip32Derivs>>,
    /// The hex-encoded Taproot x-only internal key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taproot_internal_key: Option<String>,
    /// The tuples that make up the Taproot tree, in depth first search order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taproot_tree: Option<Vec<DecodePsbtTaprootTree>>,
    /// The unknown output fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown: Option<BTreeMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_script: Option<DecodePsbtWitnessScript>,
}

/// Type alias for DecodePsbtPartialSignatures
pub type DecodePsbtPartialSignatures = BTreeMap<String, String>;

/// The input proprietary map
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodePsbtProprietary {
    /// The hex string for the proprietary identifier
    pub identifier: String,
    /// The hex for the key
    pub key: String,
    /// The number for the subtype
    /// Wire JSON key: `subtype` (Rust field `sub_type`).
    #[serde(rename = "subtype")]
    pub sub_type: u64,
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
    /// Wire JSON key: `type` (Rust field `r#type`).
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodePsbtResult {
    /// The locktime to fallback to if no inputs specify a required locktime.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback_locktime: Option<u64>,
    /// The transaction fee paid if all UTXOs slots in the PSBT have been filled.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        serialize_with = "option_amount_to_btc_float",
        deserialize_with = "option_amount_from_btc_float"
    )]
    pub fee: Option<bitcoin::Amount>,
    pub global_xpubs: Vec<DecodePsbtGlobalXpubs>,
    /// Whether this PSBT has SIGHASH_SINGLE inputs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_sighash_single: Option<bool>,
    /// The number of inputs in this psbt
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_count: Option<u64>,
    pub inputs: Vec<DecodePsbtInputs>,
    /// Whether inputs can be modified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs_modifiable: Option<bool>,
    /// The number of outputs in this psbt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_count: Option<u64>,
    pub outputs: Vec<DecodePsbtOutputs>,
    /// Whether outputs can be modified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs_modifiable: Option<bool>,
    /// The global proprietary map
    pub proprietary: Vec<DecodePsbtProprietary>,
    /// The PSBT version number. Not to be confused with the unsigned transaction version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psbt_version: Option<u64>,
    /// The decoded network-serialized unsigned transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx: Option<DecodePsbtTx>,
    /// The version number of the unsigned transaction. Not to be confused with PSBT version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_version: Option<u64>,
    /// The unknown global fields
    pub unknown: BTreeMap<String, String>,
}

/// Type alias for DecodePsbtRipemd160Preimages
pub type DecodePsbtRipemd160Preimages = BTreeMap<String, String>;

/// The output script (scriptPubKey) for this output
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodePsbtScript {
    /// The Bitcoin address (only if a well-defined address exists)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Disassembly of the output script
    pub asm: String,
    /// Inferred descriptor for the output
    pub desc: String,
    /// The raw output script bytes, hex-encoded
    pub hex: String,
    /// The type (one of: nonstandard, anchor, pubkey, pubkeyhash, scripthash, multisig, nulldata, witness_v0_scripthash, witness_v0_keyhash, witness_v1_taproot, witness_unknown)
    /// Wire JSON key: `type` (Rust field `r#type`).
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodePsbtScriptPubKey {
    /// The Bitcoin address (only if a well-defined address exists)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Disassembly of the output script
    pub asm: String,
    /// Inferred descriptor for the output
    pub desc: String,
    /// The raw output script bytes, hex-encoded
    pub hex: String,
    /// The type (one of: nonstandard, anchor, pubkey, pubkeyhash, scripthash, multisig, nulldata, witness_v0_scripthash, witness_v0_keyhash, witness_v1_taproot, witness_unknown)
    /// Wire JSON key: `type` (Rust field `r#type`).
    #[serde(rename = "type")]
    pub r#type: String,
}

/// The script (if not coinbase transaction)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodePsbtScriptSig {
    /// Disassembly of the signature script
    pub asm: String,
    /// The raw signature script bytes, hex-encoded
    pub hex: String,
}

/// Type alias for DecodePsbtSha256Preimages
pub type DecodePsbtSha256Preimages = BTreeMap<String, String>;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodePsbtTaprootBip32Derivs {
    /// The hashes of the leaves this pubkey appears in
    pub leaf_hashes: Vec<String>,
    /// The fingerprint of the master key
    pub master_fingerprint: String,
    /// The path
    pub path: String,
    /// The x-only public key this path corresponds to
    pub pubkey: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodePsbtTaprootScriptPathSigs {
    /// The leaf hash for this signature
    pub leaf_hash: String,
    /// The x-only pubkey for this signature
    pub pubkey: String,
    /// The signature itself
    pub sig: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodePsbtTaprootScripts {
    /// The control blocks for this script
    pub control_blocks: Vec<String>,
    /// The version number for the leaf script
    pub leaf_ver: u64,
    /// A leaf script
    pub script: bitcoin::ScriptBuf,
}

/// The tuples that make up the Taproot tree, in depth first search order
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodePsbtTaprootTree {
    /// The depth of this element in the tree
    pub depth: u64,
    /// The version of this leaf
    pub leaf_ver: u64,
    /// The hex-encoded script itself
    pub script: bitcoin::ScriptBuf,
}

/// The decoded network-serialized unsigned transaction.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodePsbtTx {
    /// The transaction hash (differs from txid for witness transactions)
    pub hash: String,
    /// The lock time
    /// Wire JSON key: `locktime` (Rust field `lock_time`).
    #[serde(rename = "locktime")]
    pub lock_time: u64,
    /// The serialized transaction size
    pub size: u64,
    /// The transaction id
    pub txid: bitcoin::Txid,
    /// The version
    pub version: u32,
    pub vin: Vec<DecodePsbtVin>,
    pub vout: Vec<DecodePsbtVout>,
    /// The virtual transaction size (differs from size for witness transactions)
    /// Wire JSON key: `vsize` (Rust field `v_size`).
    #[serde(rename = "vsize")]
    pub v_size: u64,
    /// The transaction's weight (between vsize*4-3 and vsize*4)
    pub weight: u64,
}

/// Type alias for DecodePsbtUnknown
pub type DecodePsbtUnknown = BTreeMap<String, String>;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodePsbtVin {
    /// The coinbase value (only if coinbase transaction)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coinbase: Option<String>,
    /// The script (if not coinbase transaction)
    /// Wire JSON key: `scriptSig` (Rust field `script_sig`).
    #[serde(rename = "scriptSig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_sig: Option<DecodePsbtScriptSig>,
    /// The script sequence number
    pub sequence: u64,
    /// The transaction id (if not coinbase transaction)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txid: Option<bitcoin::Txid>,
    /// Wire JSON key: `txinwitness` (Rust field `tx_in_witness`).
    #[serde(rename = "txinwitness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_in_witness: Option<Vec<String>>,
    /// The output number (if not coinbase transaction)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vout: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodePsbtVout {
    /// index
    pub n: u32,
    /// Wire JSON key: `scriptPubKey` (Rust field `script_pubkey`).
    #[serde(rename = "scriptPubKey")]
    pub script_pubkey: DecodePsbtScriptPubKey,
    /// The value in BTC
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub value: bitcoin::Amount,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodePsbtWitnessScript {
    /// Disassembly of the witness script
    pub asm: String,
    /// The raw witness script bytes, hex-encoded
    pub hex: String,
    /// The type, eg 'pubkeyhash'
    /// Wire JSON key: `type` (Rust field `r#type`).
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Transaction output for witness UTXOs
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodePsbtWitnessUtxo {
    /// The value in BTC
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub amount: bitcoin::Amount,
    /// Wire JSON key: `scriptPubKey` (Rust field `script_pubkey`).
    #[serde(rename = "scriptPubKey")]
    pub script_pubkey: DecodePsbtScriptPubKey,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodeRawTransactionResult {
    /// The transaction hash (differs from txid for witness transactions)
    pub hash: String,
    /// The lock time
    /// Wire JSON key: `locktime` (Rust field `lock_time`).
    #[serde(rename = "locktime")]
    pub lock_time: u64,
    /// The serialized transaction size
    pub size: u64,
    /// The transaction id
    pub txid: bitcoin::Txid,
    /// The version
    pub version: u32,
    pub vin: Vec<DecodeRawTransactionVin>,
    pub vout: Vec<DecodeRawTransactionVout>,
    /// The virtual transaction size (differs from size for witness transactions)
    /// Wire JSON key: `vsize` (Rust field `v_size`).
    #[serde(rename = "vsize")]
    pub v_size: u64,
    /// The transaction's weight (between vsize*4-3 and vsize*4)
    pub weight: u64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodeRawTransactionScriptPubKey {
    /// The Bitcoin address (only if a well-defined address exists)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Disassembly of the output script
    pub asm: String,
    /// Inferred descriptor for the output
    pub desc: String,
    /// The raw output script bytes, hex-encoded
    pub hex: String,
    /// The type (one of: nonstandard, anchor, pubkey, pubkeyhash, scripthash, multisig, nulldata, witness_v0_scripthash, witness_v0_keyhash, witness_v1_taproot, witness_unknown)
    /// Wire JSON key: `type` (Rust field `r#type`).
    #[serde(rename = "type")]
    pub r#type: String,
}

/// The script (if not coinbase transaction)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodeRawTransactionScriptSig {
    /// Disassembly of the signature script
    pub asm: String,
    /// The raw signature script bytes, hex-encoded
    pub hex: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodeRawTransactionVin {
    /// The coinbase value (only if coinbase transaction)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coinbase: Option<String>,
    /// The script (if not coinbase transaction)
    /// Wire JSON key: `scriptSig` (Rust field `script_sig`).
    #[serde(rename = "scriptSig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_sig: Option<DecodeRawTransactionScriptSig>,
    /// The script sequence number
    pub sequence: u64,
    /// The transaction id (if not coinbase transaction)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txid: Option<bitcoin::Txid>,
    /// Wire JSON key: `txinwitness` (Rust field `tx_in_witness`).
    #[serde(rename = "txinwitness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_in_witness: Option<Vec<String>>,
    /// The output number (if not coinbase transaction)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vout: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodeRawTransactionVout {
    /// index
    pub n: u32,
    /// Wire JSON key: `scriptPubKey` (Rust field `script_pubkey`).
    #[serde(rename = "scriptPubKey")]
    pub script_pubkey: DecodeRawTransactionScriptPubKey,
    /// The value in BTC
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub value: bitcoin::Amount,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodeScriptResult {
    /// The Bitcoin address (only if a well-defined address exists)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Disassembly of the script
    pub asm: String,
    /// Inferred descriptor for the script
    pub desc: String,
    /// address of P2SH script wrapping this redeem script (not returned for types that should not be wrapped)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p2sh: Option<String>,
    /// Result of a witness output script wrapping this redeem script (not returned for types that should not be wrapped)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segwit: Option<DecodeScriptSegwit>,
    /// The output type (e.g. nonstandard, anchor, pubkey, pubkeyhash, scripthash, multisig, nulldata, witness_v0_scripthash, witness_v0_keyhash, witness_v1_taproot, witness_unknown)
    /// Wire JSON key: `type` (Rust field `r#type`).
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Result of a witness output script wrapping this redeem script (not returned for types that should not be wrapped)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DecodeScriptSegwit {
    /// The Bitcoin address (only if a well-defined address exists)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Disassembly of the output script
    pub asm: String,
    /// Inferred descriptor for the script
    pub desc: String,
    /// The raw output script bytes, hex-encoded
    pub hex: String,
    /// address of the P2SH script wrapping this witness redeem script
    /// Wire JSON key: `p2sh-segwit` (Rust field `p2sh_segwit`).
    #[serde(rename = "p2sh-segwit")]
    pub p2sh_segwit: String,
    /// The type of the output script (e.g. witness_v0_keyhash or witness_v0_scripthash)
    /// Wire JSON key: `type` (Rust field `r#type`).
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Type alias for DeriveAddressesArray2Array1
pub type DeriveAddressesArray2Array1 = String;

/// Type alias for DeriveAddressesArray2Array2
pub type DeriveAddressesArray2Array2 = String;

/// Type alias for DeriveAddressesArrayArray1
pub type DeriveAddressesArrayArray1 = String;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DeriveHdKeyResult {
    /// Fingerprint and path for use in descriptors
    pub origin: String,
    /// The extended private key if "private" is true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xprv: Option<String>,
    /// The extended public key
    pub xpub: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DescriptorProcessPsbtResult {
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// The hex-encoded network transaction if complete
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<String>,
    /// The base64-encoded partially signed transaction
    pub psbt: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DumpTxOutSetResult {
    /// the hash of the base of the snapshot
    pub base_hash: String,
    /// the height of the base of the snapshot
    pub base_height: u64,
    /// the number of coins written in the snapshot
    pub coins_written: u64,
    /// the number of transactions in the chain up to and including the base block
    /// Wire JSON key: `nchaintx` (Rust field `n_chain_tx`).
    #[serde(rename = "nchaintx")]
    pub n_chain_tx: u64,
    /// the absolute path that the snapshot was written to
    pub path: String,
    /// the hash of the UTXO set contents
    pub txoutset_hash: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct EnumerateSignersResult {
    pub signers: Vec<EnumerateSignersSigners>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct EnumerateSignersSigners {
    /// Master key fingerprint
    pub fingerprint: String,
    /// Device name, the model returned by the signer
    pub name: String,
}

/// information about the highest range of feerates to fail to meet the threshold
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct EstimateRawFeeFail {
    /// end of feerate range
    /// Wire JSON key: `endrange` (Rust field `end_range`).
    #[serde(rename = "endrange")]
    pub end_range: u64,
    /// current number of txs in mempool in the feerate range unconfirmed for at least target blocks
    /// Wire JSON key: `inmempool` (Rust field `in_mempool`).
    #[serde(rename = "inmempool")]
    pub in_mempool: u64,
    /// number of txs over history horizon in the feerate range that left mempool unconfirmed after target
    /// Wire JSON key: `leftmempool` (Rust field `left_mempool`).
    #[serde(rename = "leftmempool")]
    pub left_mempool: u64,
    /// start of feerate range
    /// Wire JSON key: `startrange` (Rust field `start_range`).
    #[serde(rename = "startrange")]
    pub start_range: u64,
    /// number of txs over history horizon in the feerate range that were confirmed at any point
    /// Wire JSON key: `totalconfirmed` (Rust field `total_confirmed`).
    #[serde(rename = "totalconfirmed")]
    pub total_confirmed: u64,
    /// number of txs over history horizon in the feerate range that were confirmed within target
    /// Wire JSON key: `withintarget` (Rust field `within_target`).
    #[serde(rename = "withintarget")]
    pub within_target: u64,
}

/// estimate for long time horizon
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct EstimateRawFeeLong {
    /// exponential decay (per block) for historical moving average of confirmation data
    pub decay: u64,
    /// Errors encountered during processing (if there are any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,
    /// information about the highest range of feerates to fail to meet the threshold
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail: Option<EstimateRawFeeFail>,
    /// estimate fee rate in BTC/kvB
    /// Wire JSON key: `feerate` (Rust field `fee_rate`).
    #[serde(rename = "feerate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_rate: Option<f64>,
    /// information about the lowest range of feerates to succeed in meeting the threshold
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pass: Option<EstimateRawFeePass>,
    /// The resolution of confirmation targets at this time horizon
    pub scale: u64,
}

/// estimate for medium time horizon
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct EstimateRawFeeMedium {
    /// exponential decay (per block) for historical moving average of confirmation data
    pub decay: u64,
    /// Errors encountered during processing (if there are any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,
    /// information about the highest range of feerates to fail to meet the threshold
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail: Option<EstimateRawFeeFail>,
    /// estimate fee rate in BTC/kvB
    /// Wire JSON key: `feerate` (Rust field `fee_rate`).
    #[serde(rename = "feerate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_rate: Option<f64>,
    /// information about the lowest range of feerates to succeed in meeting the threshold
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pass: Option<EstimateRawFeePass>,
    /// The resolution of confirmation targets at this time horizon
    pub scale: u64,
}

/// information about the lowest range of feerates to succeed in meeting the threshold
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct EstimateRawFeePass {
    /// end of feerate range
    /// Wire JSON key: `endrange` (Rust field `end_range`).
    #[serde(rename = "endrange")]
    pub end_range: u64,
    /// current number of txs in mempool in the feerate range unconfirmed for at least target blocks
    /// Wire JSON key: `inmempool` (Rust field `in_mempool`).
    #[serde(rename = "inmempool")]
    pub in_mempool: u64,
    /// number of txs over history horizon in the feerate range that left mempool unconfirmed after target
    /// Wire JSON key: `leftmempool` (Rust field `left_mempool`).
    #[serde(rename = "leftmempool")]
    pub left_mempool: u64,
    /// start of feerate range
    /// Wire JSON key: `startrange` (Rust field `start_range`).
    #[serde(rename = "startrange")]
    pub start_range: u64,
    /// number of txs over history horizon in the feerate range that were confirmed at any point
    /// Wire JSON key: `totalconfirmed` (Rust field `total_confirmed`).
    #[serde(rename = "totalconfirmed")]
    pub total_confirmed: u64,
    /// number of txs over history horizon in the feerate range that were confirmed within target
    /// Wire JSON key: `withintarget` (Rust field `within_target`).
    #[serde(rename = "withintarget")]
    pub within_target: u64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct EstimateRawFeeResult {
    /// estimate for long time horizon
    #[serde(skip_serializing_if = "Option::is_none")]
    pub long: Option<EstimateRawFeeLong>,
    /// estimate for medium time horizon
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium: Option<EstimateRawFeeMedium>,
    /// estimate for short time horizon
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short: Option<EstimateRawFeeShort>,
}

/// estimate for short time horizon
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct EstimateRawFeeShort {
    /// exponential decay (per block) for historical moving average of confirmation data
    pub decay: u64,
    /// Errors encountered during processing (if there are any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,
    /// information about the highest range of feerates to fail to meet the threshold
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail: Option<EstimateRawFeeFail>,
    /// estimate fee rate in BTC/kvB
    /// Wire JSON key: `feerate` (Rust field `fee_rate`).
    #[serde(rename = "feerate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_rate: Option<f64>,
    /// information about the lowest range of feerates to succeed in meeting the threshold
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pass: Option<EstimateRawFeePass>,
    /// The resolution of confirmation targets at this time horizon
    pub scale: u64,
}

/// Health statistics for the most recently mined blocks tracked by the mempool fee rate estimator (only present when verbosity &gt;= 2)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct EstimateSmartFeeMempoolHealthStatistics {
    /// Block height
    pub block_height: u64,
    /// Total weight of non-coinbase transactions in the block
    pub block_weight: u64,
    /// Total weight of transactions removed from the mempool for this block
    pub mempool_txs_weight: u64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct EstimateSmartFeeResult {
    /// the confirmation target in blocks for the returned fee rate estimate.
    /// For the block policy fee rate estimator, this is the target the estimate was found at, clamped to at
    /// least 2 and at most the estimator's maximum usable target. For the mempool fee rate
    /// estimator, it is always 2.
    pub blocks: u64,
    /// Errors encountered during processing (if there are any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,
    /// the fee estimator used to produce the result (only present for successful estimates when fee_rate_estimator is "none")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimator: Option<String>,
    /// estimate fee rate in BTC/kvB (only present if no errors were encountered)
    /// Wire JSON key: `feerate` (Rust field `fee_rate`).
    #[serde(rename = "feerate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_rate: Option<f64>,
    /// Health statistics for the most recently mined blocks tracked by the mempool fee rate estimator (only present when verbosity &gt;= 2)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mempool_health_statistics: Option<Vec<EstimateSmartFeeMempoolHealthStatistics>>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ExportAsMapResult {
    /// the number of bytes written to the file
    pub bytes_written: u64,
    /// the SHA256 hash of the exported ASMap data
    pub file_hash: String,
    /// the absolute path that the ASMap data was written to
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ExportWatchOnlyWalletResult {
    /// The full path that the file has been exported to
    pub exported_file: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct FinalizePsbtResult {
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// The hex-encoded network transaction if extracted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<String>,
    /// The base64-encoded partially signed transaction if not extracted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psbt: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct FundRawTransactionResult {
    /// The position of the added change output, or -1
    pub changepos: i64,
    /// Fee in BTC the resulting transaction pays
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub fee: bitcoin::Amount,
    /// The resulting raw transaction (hex-encoded string)
    pub hex: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GenerateBlockResult {
    /// hash of generated block
    pub hash: String,
    /// hex of generated block, only present when submit=false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<String>,
}

/// Only when connected = true
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetAddedNodeInfoAddresses {
    /// The bitcoin server IP and port we're connected to
    pub address: String,
    /// connection, inbound or outbound
    pub connected: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetAddedNodeInfoResultItem {
    /// The node IP address or name (as provided to addnode)
    /// Wire JSON key: `addednode` (Rust field `added_node`).
    #[serde(rename = "addednode")]
    pub added_node: String,
    /// Only when connected = true
    pub addresses: Vec<GetAddedNodeInfoAddresses>,
    /// If connected
    pub connected: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetAddrManInfoMapValue {
    /// number of addresses in the new table, which represent potential peers the node has discovered but hasn't yet successfully connected to.
    pub new: u64,
    /// total number of addresses in both new/tried tables
    pub total: u64,
    /// number of addresses in the tried table, which represent peers the node has successfully connected to in the past.
    pub tried: u64,
}

/// Type alias for GetAddrManInfoResultMap
pub type GetAddrManInfoResultMap = BTreeMap<String, GetAddrManInfoMapValue>;

/// Information about the address embedded in P2SH or P2WSH, if relevant and known.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetAddressInfoEmbedded {
    /// The bitcoin address of the embedded script.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Information about the address embedded in P2SH or P2WSH, if relevant and known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedded: Option<Box<GetAddressInfoEmbedded>>,
    /// The redeemscript for the p2sh address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<String>,
    /// If the pubkey is compressed.
    /// Wire JSON key: `iscompressed` (Rust field `is_compressed`).
    #[serde(rename = "iscompressed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_compressed: Option<bool>,
    /// If the key is a script.
    /// Wire JSON key: `isscript` (Rust field `is_script`).
    #[serde(rename = "isscript")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_script: Option<bool>,
    /// If the address is a witness address.
    /// Wire JSON key: `iswitness` (Rust field `is_witness`).
    #[serde(rename = "iswitness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_witness: Option<bool>,
    /// The hex value of the raw public key for single-key addresses (possibly embedded in P2SH or P2WSH).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pubkey: Option<String>,
    /// Array of pubkeys associated with the known redeemscript (only if script is multisig).
    /// Wire JSON key: `pubkeys` (Rust field `pub_keys`).
    #[serde(rename = "pubkeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pub_keys: Option<Vec<String>>,
    /// The output script type. Only if isscript is true and the redeemscript is known. Possible
    /// types: nonstandard, pubkey, pubkeyhash, scripthash, multisig, nulldata, witness_v0_keyhash,
    /// witness_v0_scripthash, witness_unknown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<bitcoin::ScriptBuf>,
    /// The hex-encoded output script generated by the address.
    /// Wire JSON key: `scriptPubKey` (Rust field `script_pubkey`).
    #[serde(rename = "scriptPubKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_pubkey: Option<bitcoin::ScriptBuf>,
    /// The number of signatures required to spend multisig output (only if script is multisig).
    /// Wire JSON key: `sigsrequired` (Rust field `sigs_required`).
    #[serde(rename = "sigsrequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sigs_required: Option<u64>,
    /// The hex value of the witness program.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_program: Option<String>,
    /// The version number of the witness program.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_version: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetAddressInfoResult {
    /// The bitcoin address validated.
    pub address: String,
    /// A descriptor for spending coins sent to this address (only when solvable).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    /// Information about the address embedded in P2SH or P2WSH, if relevant and known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedded: Option<GetAddressInfoEmbedded>,
    /// The HD keypath, if the key is HD and available.
    /// Wire JSON key: `hdkeypath` (Rust field `hd_key_path`).
    #[serde(rename = "hdkeypath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hd_key_path: Option<String>,
    /// The fingerprint of the master key.
    /// Wire JSON key: `hdmasterfingerprint` (Rust field `hd_master_fingerprint`).
    #[serde(rename = "hdmasterfingerprint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hd_master_fingerprint: Option<String>,
    /// The Hash160 of the HD seed.
    /// Wire JSON key: `hdseedid` (Rust field `hd_seed_id`).
    #[serde(rename = "hdseedid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hd_seed_id: Option<String>,
    /// The redeemscript for the p2sh address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<String>,
    /// If the address was used for change output.
    pub ischange: bool,
    /// If the pubkey is compressed.
    /// Wire JSON key: `iscompressed` (Rust field `is_compressed`).
    #[serde(rename = "iscompressed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_compressed: Option<bool>,
    /// If the address is yours.
    /// Wire JSON key: `ismine` (Rust field `is_mine`).
    #[serde(rename = "ismine")]
    pub is_mine: bool,
    /// If the key is a script.
    /// Wire JSON key: `isscript` (Rust field `is_script`).
    #[serde(rename = "isscript")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_script: Option<bool>,
    /// (DEPRECATED) Always false.
    /// Wire JSON key: `iswatchonly` (Rust field `is_watch_only`).
    #[serde(rename = "iswatchonly")]
    pub is_watch_only: bool,
    /// If the address is a witness address.
    /// Wire JSON key: `iswitness` (Rust field `is_witness`).
    #[serde(rename = "iswitness")]
    pub is_witness: bool,
    /// Array of labels associated with the address. Currently limited to one label but returned
    /// as an array to keep the API stable if multiple labels are enabled in the future.
    pub labels: Vec<String>,
    /// The descriptor used to derive this address if this is a descriptor wallet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_desc: Option<String>,
    /// The hex value of the raw public key for single-key addresses (possibly embedded in P2SH or P2WSH).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pubkey: Option<String>,
    /// Array of pubkeys associated with the known redeemscript (only if script is multisig).
    /// Wire JSON key: `pubkeys` (Rust field `pub_keys`).
    #[serde(rename = "pubkeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pub_keys: Option<Vec<String>>,
    /// The output script type. Only if isscript is true and the redeemscript is known. Possible
    /// types: nonstandard, pubkey, pubkeyhash, scripthash, multisig, nulldata, witness_v0_keyhash,
    /// witness_v0_scripthash, witness_unknown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<bitcoin::ScriptBuf>,
    /// The hex-encoded output script generated by the address.
    /// Wire JSON key: `scriptPubKey` (Rust field `script_pubkey`).
    #[serde(rename = "scriptPubKey")]
    pub script_pubkey: bitcoin::ScriptBuf,
    /// The number of signatures required to spend multisig output (only if script is multisig).
    /// Wire JSON key: `sigsrequired` (Rust field `sigs_required`).
    #[serde(rename = "sigsrequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sigs_required: Option<u64>,
    /// If we know how to spend coins sent to this address, ignoring the possible lack of private keys.
    pub solvable: bool,
    /// The creation time of the key, if available, expressed in UNIX epoch time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<u64>,
    /// The hex value of the witness program.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_program: Option<String>,
    /// The version number of the witness program.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_version: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetAddressesByLabelMapValue {
    /// Purpose of address ("send" for sending address, "receive" for receiving address)
    pub purpose: String,
}

/// Type alias for GetAddressesByLabelResultMap
pub type GetAddressesByLabelResultMap = BTreeMap<String, GetAddressesByLabelMapValue>;

/// hash and height of the block this information was generated on
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetBalancesLastProcessedBlock {
    /// hash of the block this information was generated on
    pub hash: String,
    /// height of the block this information was generated on
    pub height: u64,
}

/// balances from outputs that the wallet can sign
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetBalancesMine {
    /// balance from immature coinbase outputs
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub immature: bitcoin::Amount,
    /// sum of coins that are spent by transactions not in the mempool (usually an over-estimate due to not accounting for change or spends that conflict with each other)
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub nonmempool: bitcoin::Amount,
    /// trusted balance (outputs created by the wallet or confirmed outputs)
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub trusted: bitcoin::Amount,
    /// untrusted pending balance (outputs created by others that are in the mempool)
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub untrusted_pending: bitcoin::Amount,
    /// (only present if avoid_reuse is set) balance from coins sent to addresses that were previously spent from (potentially privacy violating)
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        serialize_with = "option_amount_to_btc_float",
        deserialize_with = "option_amount_from_btc_float"
    )]
    pub used: Option<bitcoin::Amount>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetBalancesResult {
    /// hash and height of the block this information was generated on
    /// Wire JSON key: `lastprocessedblock` (Rust field `last_processed_block`).
    #[serde(rename = "lastprocessedblock")]
    pub last_processed_block: GetBalancesLastProcessedBlock,
    /// balances from outputs that the wallet can sign
    pub mine: GetBalancesMine,
}

/// Coinbase transaction metadata
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetBlockCoinbaseTx {
    /// The coinbase input's script
    pub coinbase: String,
    /// The coinbase transaction's locktime (nLockTime)
    /// Wire JSON key: `locktime` (Rust field `lock_time`).
    #[serde(rename = "locktime")]
    pub lock_time: u32,
    /// The coinbase input's sequence number (nSequence)
    pub sequence: u64,
    /// The coinbase transaction version
    pub version: u32,
    /// The coinbase input's first (and only) witness stack element, if present
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetBlockFilterResult {
    /// the hex-encoded filter data
    pub filter: String,
    /// the hex-encoded filter header
    pub header: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetBlockFromPeerResult {}

/// Type alias for GetBlockObject2Array1
pub type GetBlockObject2Array1 = String;

/// Type alias for GetBlockObject2Array2
pub type GetBlockObject2Array2 = String;

/// Type alias for GetBlockObject2Array3
pub type GetBlockObject2Array3 = String;

/// Type alias for GetBlockObject2Array4
pub type GetBlockObject2Array4 = String;

/// Type alias for GetBlockObject3Array1
pub type GetBlockObject3Array1 = String;

/// Type alias for GetBlockObject3Array2
pub type GetBlockObject3Array2 = String;

/// Type alias for GetBlockObject3Array3
pub type GetBlockObject3Array3 = String;

/// Type alias for GetBlockObject3Array4
pub type GetBlockObject3Array4 = String;

/// Type alias for GetBlockObjectArray1
pub type GetBlockObjectArray1 = String;

/// (Only if undo information is available)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetBlockPrevout {
    /// Coinbase or not
    pub generated: bool,
    /// The height of the prevout
    pub height: u64,
    /// Wire JSON key: `scriptPubKey` (Rust field `script_pubkey`).
    #[serde(rename = "scriptPubKey")]
    pub script_pubkey: GetBlockScriptPubKey,
    /// The value in BTC
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub value: bitcoin::Amount,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetBlockScriptPubKey {
    /// The Bitcoin address (only if a well-defined address exists)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Disassembly of the output script
    pub asm: String,
    /// Inferred descriptor for the output
    pub desc: String,
    /// The raw output script bytes, hex-encoded
    pub hex: String,
    /// The type (one of: nonstandard, anchor, pubkey, pubkeyhash, scripthash, multisig, nulldata, witness_v0_scripthash, witness_v0_keyhash, witness_v1_taproot, witness_unknown)
    /// Wire JSON key: `type` (Rust field `r#type`).
    #[serde(rename = "type")]
    pub r#type: String,
}

/// The script (if not coinbase transaction)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetBlockScriptSig {
    /// Disassembly of the signature script
    pub asm: String,
    /// The raw signature script bytes, hex-encoded
    pub hex: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetBlockStatsResult {
    /// Average fee in the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avgfee: Option<u64>,
    /// Average feerate (in satoshis per virtual byte)
    /// Wire JSON key: `avgfeerate` (Rust field `avg_fee_rate`).
    #[serde(rename = "avgfeerate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_fee_rate: Option<u64>,
    /// Average transaction size
    /// Wire JSON key: `avgtxsize` (Rust field `avg_tx_size`).
    #[serde(rename = "avgtxsize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_tx_size: Option<u64>,
    /// The block hash (to check for potential reorgs)
    /// Wire JSON key: `blockhash` (Rust field `block_hash`).
    #[serde(rename = "blockhash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_hash: Option<bitcoin::BlockHash>,
    /// Feerates at the 10th, 25th, 50th, 75th, and 90th percentile weight unit (in satoshis per virtual byte)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feerate_percentiles: Option<Vec<u64>>,
    /// The height of the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u64>,
    /// The number of inputs (excluding coinbase)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ins: Option<u64>,
    /// Maximum fee in the block
    /// Wire JSON key: `maxfee` (Rust field `max_fee`).
    #[serde(rename = "maxfee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_fee: Option<u64>,
    /// Maximum feerate (in satoshis per virtual byte)
    /// Wire JSON key: `maxfeerate` (Rust field `max_fee_rate`).
    #[serde(rename = "maxfeerate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_fee_rate: Option<f64>,
    /// Maximum transaction size
    /// Wire JSON key: `maxtxsize` (Rust field `max_tx_size`).
    #[serde(rename = "maxtxsize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tx_size: Option<u64>,
    /// Truncated median fee in the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medianfee: Option<u64>,
    /// The block median time past
    /// Wire JSON key: `mediantime` (Rust field `median_time`).
    #[serde(rename = "mediantime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub median_time: Option<u64>,
    /// Truncated median transaction size
    /// Wire JSON key: `mediantxsize` (Rust field `median_tx_size`).
    #[serde(rename = "mediantxsize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub median_tx_size: Option<u64>,
    /// Minimum fee in the block
    /// Wire JSON key: `minfee` (Rust field `min_fee`).
    #[serde(rename = "minfee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_fee: Option<u64>,
    /// Minimum feerate (in satoshis per virtual byte)
    /// Wire JSON key: `minfeerate` (Rust field `min_fee_rate`).
    #[serde(rename = "minfeerate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_fee_rate: Option<u64>,
    /// Minimum transaction size
    /// Wire JSON key: `mintxsize` (Rust field `min_tx_size`).
    #[serde(rename = "mintxsize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_tx_size: Option<u64>,
    /// The number of outputs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outs: Option<u64>,
    /// The block subsidy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subsidy: Option<u64>,
    /// Total size of all segwit transactions
    /// Wire JSON key: `swtotal_size` (Rust field `sw_total_size`).
    #[serde(rename = "swtotal_size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sw_total_size: Option<u64>,
    /// Total weight of all segwit transactions
    /// Wire JSON key: `swtotal_weight` (Rust field `sw_total_weight`).
    #[serde(rename = "swtotal_weight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sw_total_weight: Option<u64>,
    /// The number of segwit transactions
    /// Wire JSON key: `swtxs` (Rust field `sw_txs`).
    #[serde(rename = "swtxs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sw_txs: Option<u64>,
    /// The block time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<u64>,
    /// Total amount in all outputs (excluding coinbase and thus reward \[ie subsidy + totalfee\])
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_out: Option<u64>,
    /// Total size of all non-coinbase transactions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_size: Option<u64>,
    /// Total weight of all non-coinbase transactions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_weight: Option<u64>,
    /// The fee total
    /// Wire JSON key: `totalfee` (Rust field `total_fee`).
    #[serde(rename = "totalfee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_fee: Option<u64>,
    /// The number of transactions (including coinbase)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txs: Option<u64>,
    /// The increase/decrease in the number of unspent outputs (not discounting op_return and similar)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utxo_increase: Option<u64>,
    /// The increase/decrease in the number of unspent outputs, not counting unspendables
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utxo_increase_actual: Option<u64>,
    /// The increase/decrease in size for the utxo index (not discounting op_return and similar)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utxo_size_inc: Option<u64>,
    /// The increase/decrease in size for the utxo index, not counting unspendables
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utxo_size_inc_actual: Option<u64>,
}

/// Type alias for GetBlockTemplateCoinbaseaux
pub type GetBlockTemplateCoinbaseaux = BTreeMap<String, String>;

/// Type alias for GetBlockTemplateObjectArray1
pub type GetBlockTemplateObjectArray1 = String;

/// Type alias for GetBlockTemplateObjectArray2
pub type GetBlockTemplateObjectArray2 = String;

/// Type alias for GetBlockTemplateObjectArray3
pub type GetBlockTemplateObjectArray3 = String;

/// Type alias for GetBlockTemplateObjectArray4
pub type GetBlockTemplateObjectArray4 = String;

/// Type alias for GetBlockTemplateObjectArray5
pub type GetBlockTemplateObjectArray5 = String;

/// contents of non-coinbase transactions that should be included in the next block
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetBlockTemplateTransactions {
    /// transaction data encoded in hexadecimal (byte-for-byte)
    pub data: String,
    /// array of numbers
    pub depends: Vec<u64>,
    /// difference in value between transaction inputs and outputs (in satoshis); for coinbase transactions, this is a negative Number of the total collected block fees (ie, not including the block subsidy); if key is not present, fee is unknown and clients MUST NOT assume there isn't one
    pub fee: f64,
    /// transaction hash including witness data, shown in byte-reversed hex
    pub hash: String,
    /// total SigOps cost, as counted for purposes of block limits; if key is not present, sigop cost is unknown and clients MUST NOT assume it is zero
    /// Wire JSON key: `sigops` (Rust field `sig_ops`).
    #[serde(rename = "sigops")]
    pub sig_ops: u64,
    /// transaction hash excluding witness data, shown in byte-reversed hex
    pub txid: bitcoin::Txid,
    /// total transaction weight, as counted for purposes of block limits
    pub weight: u64,
}

/// Type alias for GetBlockTemplateVbavailable
pub type GetBlockTemplateVbavailable = BTreeMap<String, u64>;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetBlockTx {
    /// The transaction fee in BTC, omitted if block undo data is not available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<f64>,
    /// The transaction hash (differs from txid for witness transactions)
    pub hash: String,
    /// The hex-encoded transaction data
    pub hex: String,
    /// The lock time
    /// Wire JSON key: `locktime` (Rust field `lock_time`).
    #[serde(rename = "locktime")]
    pub lock_time: u64,
    /// The serialized transaction size
    pub size: u64,
    /// The transaction id
    pub txid: bitcoin::Txid,
    /// The version
    pub version: u32,
    pub vin: Vec<GetBlockVin>,
    pub vout: Vec<GetBlockVout>,
    /// The virtual transaction size (differs from size for witness transactions)
    /// Wire JSON key: `vsize` (Rust field `v_size`).
    #[serde(rename = "vsize")]
    pub v_size: u64,
    /// The transaction's weight (between vsize*4-3 and vsize*4)
    pub weight: u64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetBlockVin {
    /// The coinbase value (only if coinbase transaction)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coinbase: Option<String>,
    /// The script (if not coinbase transaction)
    /// Wire JSON key: `scriptSig` (Rust field `script_sig`).
    #[serde(rename = "scriptSig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_sig: Option<GetBlockScriptSig>,
    /// The script sequence number
    pub sequence: u64,
    /// The transaction id (if not coinbase transaction)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txid: Option<bitcoin::Txid>,
    /// Wire JSON key: `txinwitness` (Rust field `tx_in_witness`).
    #[serde(rename = "txinwitness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_in_witness: Option<Vec<String>>,
    /// The output number (if not coinbase transaction)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vout: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetBlockVout {
    /// index
    pub n: u32,
    /// Wire JSON key: `scriptPubKey` (Rust field `script_pubkey`).
    #[serde(rename = "scriptPubKey")]
    pub script_pubkey: GetBlockScriptPubKey,
    /// The value in BTC
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub value: bitcoin::Amount,
}

/// state info regarding background validation process
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetBlockchainInfoBackgroundvalidation {
    /// the hash of the currently best block validated in the background
    /// Wire JSON key: `bestblockhash` (Rust field `best_block_hash`).
    #[serde(rename = "bestblockhash")]
    pub best_block_hash: String,
    /// the height of the most-work background fully-validated chain. The genesis block has height 0
    pub blocks: u64,
    /// total amount of work in background validated chain, in hexadecimal
    /// Wire JSON key: `chainwork` (Rust field `chain_work`).
    #[serde(rename = "chainwork")]
    pub chain_work: String,
    /// the median block time expressed in UNIX epoch time
    /// Wire JSON key: `mediantime` (Rust field `median_time`).
    #[serde(rename = "mediantime")]
    pub median_time: u64,
    /// the height of the snapshot block. Background validation verifies the chain from genesis up to this height
    /// Wire JSON key: `snapshotheight` (Rust field `snapshot_height`).
    #[serde(rename = "snapshotheight")]
    pub snapshot_height: u64,
    /// estimate of background verification progress \[0..1\]
    /// Wire JSON key: `verificationprogress` (Rust field `verification_progress`).
    #[serde(rename = "verificationprogress")]
    pub verification_progress: f64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetBlockchainInfoResult {
    /// whether automatic pruning is enabled (only present if pruning is enabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_pruning: Option<bool>,
    /// state info regarding background validation process
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backgroundvalidation: Option<GetBlockchainInfoBackgroundvalidation>,
    /// the hash of the currently best block
    /// Wire JSON key: `bestblockhash` (Rust field `best_block_hash`).
    #[serde(rename = "bestblockhash")]
    pub best_block_hash: String,
    /// nBits: compact representation of the block difficulty target
    pub bits: String,
    /// the height of the most-work fully-validated chain. The genesis block has height 0
    pub blocks: u64,
    /// current network name (main, test, testnet4, signet, regtest)
    pub chain: String,
    /// total amount of work in active chain, in hexadecimal
    /// Wire JSON key: `chainwork` (Rust field `chain_work`).
    #[serde(rename = "chainwork")]
    pub chain_work: String,
    /// the current difficulty
    pub difficulty: f64,
    /// the current number of headers we have validated
    pub headers: u64,
    /// (debug information) estimate of whether this node is in Initial Block Download mode
    /// Wire JSON key: `initialblockdownload` (Rust field `initial_block_download`).
    #[serde(rename = "initialblockdownload")]
    pub initial_block_download: bool,
    /// the median block time expressed in UNIX epoch time
    /// Wire JSON key: `mediantime` (Rust field `median_time`).
    #[serde(rename = "mediantime")]
    pub median_time: u64,
    /// the target size used by pruning (only present if automatic pruning is enabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prune_target_size: Option<u64>,
    /// if the blocks are subject to pruning
    pub pruned: bool,
    /// the first block unpruned, all previous blocks were pruned (only present if pruning is enabled)
    /// Wire JSON key: `pruneheight` (Rust field `prune_height`).
    #[serde(rename = "pruneheight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prune_height: Option<u64>,
    /// the block challenge (aka. block script), in hexadecimal (only present if the current network is a signet)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signet_challenge: Option<String>,
    /// the estimated size of the block and undo files on disk
    pub size_on_disk: u64,
    /// the difficulty target
    pub target: String,
    /// the block time expressed in UNIX epoch time
    pub time: u64,
    /// estimate of verification progress \[0..1\]
    /// Wire JSON key: `verificationprogress` (Rust field `verification_progress`).
    #[serde(rename = "verificationprogress")]
    pub verification_progress: f64,
    /// any network and blockchain warnings (run with `-deprecatedrpc=warnings` to return the latest warning as a single string)
    pub warnings: Vec<String>,
}

/// list of the chainstates ordered by work, with the most-work (active) chainstate last
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetChainStatesChainstates {
    /// blockhash of the tip
    /// Wire JSON key: `bestblockhash` (Rust field `best_block_hash`).
    #[serde(rename = "bestblockhash")]
    pub best_block_hash: String,
    /// nBits: compact representation of the block difficulty target
    pub bits: String,
    /// number of blocks in this chainstate
    pub blocks: u64,
    /// size of the coinsdb cache
    pub coins_db_cache_bytes: u64,
    /// size of the coinstip cache
    pub coins_tip_cache_bytes: u64,
    /// difficulty of the tip
    pub difficulty: f64,
    /// the base block of the snapshot this chainstate is based on, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_blockhash: Option<String>,
    /// The difficulty target
    pub target: String,
    /// whether the chainstate is fully validated. True if all blocks in the chainstate were validated, false if the chain is based on a snapshot and the snapshot has not yet been validated.
    pub validated: bool,
    /// progress towards the network tip
    /// Wire JSON key: `verificationprogress` (Rust field `verification_progress`).
    #[serde(rename = "verificationprogress")]
    pub verification_progress: f64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetChainStatesResult {
    /// list of the chainstates ordered by work, with the most-work (active) chainstate last
    pub chainstates: Vec<GetChainStatesChainstates>,
    /// the number of headers seen so far
    pub headers: u64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetChainTipsResultItem {
    /// zero for main chain, otherwise length of branch connecting the tip to the main chain
    /// Wire JSON key: `branchlen` (Rust field `branch_len`).
    #[serde(rename = "branchlen")]
    pub branch_len: u64,
    /// block hash of the tip
    pub hash: String,
    /// height of the chain tip
    pub height: u64,
    /// status of the chain, "active" for the main chain
    /// Possible values for status:
    /// 1.  "invalid"               This branch contains at least one invalid block
    /// 2.  "headers-only"          Not all blocks for this branch are available, but the headers are valid
    /// 3.  "valid-headers"         All blocks are available for this branch, but they were never fully validated
    /// 4.  "valid-fork"            This branch is not part of the active chain, but is fully validated
    /// 5.  "active"                This is the tip of the active main chain, which is certainly valid
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetChainTxStatsResult {
    /// The timestamp for the final block in the window, expressed in UNIX epoch time
    pub time: u64,
    /// The total number of transactions in the chain up to that point, if known. It may be unknown when using assumeutxo.
    /// Wire JSON key: `txcount` (Rust field `tx_count`).
    #[serde(rename = "txcount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_count: Option<u64>,
    /// The average rate of transactions per second in the window. Only returned if "window_interval" is &gt; 0 and if window_tx_count exists.
    /// Wire JSON key: `txrate` (Rust field `tx_rate`).
    #[serde(rename = "txrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_rate: Option<u64>,
    /// Size of the window in number of blocks
    pub window_block_count: u64,
    /// The hash of the final block in the window
    pub window_final_block_hash: String,
    /// The height of the final block in the window.
    pub window_final_block_height: u64,
    /// The elapsed time in the window in seconds. Only returned if "window_block_count" is &gt; 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_interval: Option<u64>,
    /// The number of transactions in the window. Only returned if "window_block_count" is &gt; 0 and if txcount exists for the start and end of the window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_tx_count: Option<u64>,
}

/// status of bip9 softforks (only for "bip9" type)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetDeploymentInfoBip9 {
    /// the bit (0-28) in the block version field used to signal this softfork (only for "started" and "locked_in" status)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bit: Option<u64>,
    /// minimum height of blocks for which the rules may be enforced
    pub min_activation_height: u64,
    /// indicates blocks that signalled with a # and blocks that did not with a -
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signalling: Option<String>,
    /// height of the first block to which the status applies
    pub since: u64,
    /// the minimum median time past of a block at which the bit gains its meaning
    pub start_time: u64,
    /// numeric statistics about signalling for a softfork (only for "started" and "locked_in" status)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<GetDeploymentInfoStatistics>,
    /// status of deployment at specified block (one of "defined", "started", "locked_in", "active", "failed")
    pub status: String,
    /// status of deployment at the next block
    pub status_next: String,
    /// the median time past of a block at which the deployment is considered failed if not yet locked in
    pub timeout: u64,
}

/// Type alias for GetDeploymentInfoDeployments
pub type GetDeploymentInfoDeployments = BTreeMap<String, GetDeploymentInfoMapValue>;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetDeploymentInfoMapValue {
    /// true if the rules are enforced for the mempool and the next block
    pub active: bool,
    /// status of bip9 softforks (only for "bip9" type)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bip9: Option<GetDeploymentInfoBip9>,
    /// height of the first block which the rules are or will be enforced (only for "buried" type, or "bip9" type with "active" status)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u64>,
    /// one of "buried", "bip9"
    /// Wire JSON key: `type` (Rust field `r#type`).
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetDeploymentInfoResult {
    pub deployments: BTreeMap<String, GetDeploymentInfoMapValue>,
    /// requested block hash (or tip)
    pub hash: String,
    /// requested block height (or tip)
    pub height: u64,
    /// script verify flags for the block
    pub script_flags: Vec<String>,
}

/// numeric statistics about signalling for a softfork (only for "started" and "locked_in" status)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetDeploymentInfoStatistics {
    /// the number of blocks with the version bit set in the current period
    pub count: u64,
    /// the number of blocks elapsed since the beginning of the current period
    pub elapsed: u64,
    /// the length in blocks of the signalling period
    pub period: u64,
    /// returns false if there are not enough blocks left in this period to pass activation threshold (only for "started" status)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub possible: Option<bool>,
    /// the number of blocks with the version bit set required to activate the feature (only for "started" status)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetDescriptorActivityOutputSpk {
    /// The Bitcoin address (only if a well-defined address exists)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Disassembly of the output script
    pub asm: String,
    /// Inferred descriptor for the output
    pub desc: String,
    /// The raw output script bytes, hex-encoded
    pub hex: String,
    /// The type (one of: nonstandard, anchor, pubkey, pubkeyhash, scripthash, multisig, nulldata, witness_v0_scripthash, witness_v0_keyhash, witness_v1_taproot, witness_unknown)
    /// Wire JSON key: `type` (Rust field `r#type`).
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetDescriptorActivityPrevoutSpk {
    /// The Bitcoin address (only if a well-defined address exists)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Disassembly of the output script
    pub asm: String,
    /// Inferred descriptor for the output
    pub desc: String,
    /// The raw output script bytes, hex-encoded
    pub hex: String,
    /// The type (one of: nonstandard, anchor, pubkey, pubkeyhash, scripthash, multisig, nulldata, witness_v0_scripthash, witness_v0_keyhash, witness_v1_taproot, witness_unknown)
    /// Wire JSON key: `type` (Rust field `r#type`).
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetDescriptorActivityResult {
    /// events
    pub activity: Vec<GetDescriptorActivityResultActivityItem>,
}

/// Object shape from an OpenRPC union branch (nested field)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetDescriptorActivityResultActivityItemGetDescriptorActivityObject {
    /// The total amount in BTC of the spent output
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub amount: bitcoin::Amount,
    /// The blockhash this spend appears in (omitted if unconfirmed)
    /// Wire JSON key: `blockhash` (Rust field `block_hash`).
    #[serde(rename = "blockhash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_hash: Option<bitcoin::BlockHash>,
    /// Height of the spend (omitted if unconfirmed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u64>,
    pub prevout_spk: GetDescriptorActivityPrevoutSpk,
    /// The txid of the prevout
    pub prevout_txid: String,
    /// The vout of the prevout
    pub prevout_vout: u64,
    /// The txid of the spending transaction
    pub spend_txid: String,
    /// The input index of the spend
    pub spend_vin: u64,
    /// always 'spend'
    /// Wire JSON key: `type` (Rust field `r#type`).
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Object shape from an OpenRPC union branch (nested field)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetDescriptorActivityResultActivityItemGetDescriptorActivityObject2 {
    /// The total amount in BTC of the new output
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub amount: bitcoin::Amount,
    /// The block that this receive is in (omitted if unconfirmed)
    /// Wire JSON key: `blockhash` (Rust field `block_hash`).
    #[serde(rename = "blockhash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_hash: Option<bitcoin::BlockHash>,
    /// The height of the receive (omitted if unconfirmed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u64>,
    pub output_spk: GetDescriptorActivityOutputSpk,
    /// The txid of the receiving transaction
    pub txid: bitcoin::Txid,
    /// always 'receive'
    /// Wire JSON key: `type` (Rust field `r#type`).
    #[serde(rename = "type")]
    pub r#type: String,
    /// The vout of the receiving output
    pub vout: u64,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDescriptorActivityResultActivityItem {
    Object(GetDescriptorActivityResultActivityItemGetDescriptorActivityObject),
    Object2(GetDescriptorActivityResultActivityItemGetDescriptorActivityObject2),
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetDescriptorInfoResult {
    /// The checksum for the input descriptor
    pub checksum: String,
    /// The descriptor, without private keys. For a multipath descriptor, only the first will be returned.
    pub descriptor: String,
    /// Whether the input descriptor contained at least one private key
    /// Wire JSON key: `hasprivatekeys` (Rust field `has_private_keys`).
    #[serde(rename = "hasprivatekeys")]
    pub has_private_keys: bool,
    /// Whether the descriptor is ranged
    /// Wire JSON key: `isrange` (Rust field `is_range`).
    #[serde(rename = "isrange")]
    pub is_range: bool,
    /// Whether the descriptor is solvable
    /// Wire JSON key: `issolvable` (Rust field `is_solvable`).
    #[serde(rename = "issolvable")]
    pub is_solvable: bool,
    /// All descriptors produced by expanding multipath derivation elements. Only if the provided descriptor specifies multipath derivation elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multipath_expansion: Option<Vec<String>>,
}

/// Array of descriptor objects that use this HD key
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetHdKeysDescriptors {
    /// Whether this descriptor is currently used to generate new addresses
    pub active: bool,
    /// Descriptor string public representation
    pub desc: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetHdKeysResultItem {
    /// Array of descriptor objects that use this HD key
    pub descriptors: Vec<GetHdKeysDescriptors>,
    /// Whether the wallet has the private key for this xpub
    pub has_private: bool,
    /// The extended private key if "private" is true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xprv: Option<String>,
    /// The extended public key
    pub xpub: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetIndexInfoMapValue {
    /// The block height to which the index is synced
    pub best_block_height: u64,
    /// Whether the index is synced or not
    pub synced: bool,
}

/// Type alias for GetIndexInfoResultMap
pub type GetIndexInfoResultMap = BTreeMap<String, GetIndexInfoMapValue>;

/// Information about locked memory manager
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetMemoryInfoLocked {
    /// Number unused chunks
    pub chunks_free: u64,
    /// Number allocated chunks
    pub chunks_used: u64,
    /// Number of bytes available in current arenas
    pub free: u64,
    /// Amount of bytes that succeeded locking. If this number is smaller than total, locking pages failed at some point and key data could be swapped to disk.
    pub locked: u64,
    /// Total number of bytes managed
    pub total: u64,
    /// Number of bytes used
    pub used: u64,
}

/// Type alias for GetMempoolAncestorsArrayArray1
pub type GetMempoolAncestorsArrayArray1 = String;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetMempoolAncestorsFees {
    /// transaction fees of in-mempool ancestors (including this one) with fee deltas used for mining priority, denominated in BTC
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub ancestor: bitcoin::Amount,
    /// transaction fee, denominated in BTC
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub base: bitcoin::Amount,
    /// transaction fees of chunk, denominated in BTC
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub chunk: bitcoin::Amount,
    /// transaction fees of in-mempool descendants (including this one) with fee deltas used for mining priority, denominated in BTC
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub descendant: bitcoin::Amount,
    /// transaction fee with fee deltas used for mining priority, denominated in BTC
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub modified: bitcoin::Amount,
}

/// for verbose = true
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetMempoolAncestorsMapValue {
    /// number of in-mempool ancestor transactions (including this one)
    /// Wire JSON key: `ancestorcount` (Rust field `ancestor_count`).
    #[serde(rename = "ancestorcount")]
    pub ancestor_count: u64,
    /// virtual transaction size of in-mempool ancestors (including this one)
    /// Wire JSON key: `ancestorsize` (Rust field `ancestor_size`).
    #[serde(rename = "ancestorsize")]
    pub ancestor_size: u64,
    /// sigops-adjusted weight (as defined in BIP 141 and modified by '-bytespersigop') of this transaction's chunk
    /// Wire JSON key: `chunkweight` (Rust field `chunk_weight`).
    #[serde(rename = "chunkweight")]
    pub chunk_weight: u64,
    /// unconfirmed transactions used as inputs for this transaction
    pub depends: Vec<String>,
    /// number of in-mempool descendant transactions (including this one)
    /// Wire JSON key: `descendantcount` (Rust field `descendant_count`).
    #[serde(rename = "descendantcount")]
    pub descendant_count: u64,
    /// virtual transaction size of in-mempool descendants (including this one)
    /// Wire JSON key: `descendantsize` (Rust field `descendant_size`).
    #[serde(rename = "descendantsize")]
    pub descendant_size: u64,
    pub fees: GetMempoolAncestorsFees,
    /// block height when transaction entered pool
    pub height: u64,
    /// unconfirmed transactions spending outputs from this transaction
    /// Wire JSON key: `spentby` (Rust field `spent_by`).
    #[serde(rename = "spentby")]
    pub spent_by: Vec<String>,
    /// local time transaction entered pool in seconds since 1 Jan 1970 GMT
    pub time: u64,
    /// Whether this transaction is currently unbroadcast (initial broadcast not yet acknowledged by any peers)
    pub unbroadcast: bool,
    /// (DEPRECATED) Was previously erroneously described as the BIP 141 vsize, but is actually sigops-adjusted vsize.
    /// Use vsize_bip141 to actually get that behavior or switch to the explicit vsize_adjusted for retained behavior.
    /// Wire JSON key: `vsize` (Rust field `v_size`).
    #[serde(rename = "vsize")]
    pub v_size: u64,
    /// Maximum of sigop-adjusted size (-bytespersigop) and virtual transaction size as defined in BIP 141.
    pub vsize_adjusted: u64,
    /// Virtual transaction size as defined in BIP 141.
    /// This is different from actual serialized size for witness transactions as witness data is discounted.
    pub vsize_bip141: u64,
    /// transaction weight as defined in BIP 141.
    pub weight: u64,
    /// hash of serialized transaction, including witness data
    /// Wire JSON key: `wtxid` (Rust field `w_txid`).
    #[serde(rename = "wtxid")]
    pub w_txid: String,
}

/// Type alias for GetMempoolAncestorsObject
pub type GetMempoolAncestorsObject = BTreeMap<String, GetMempoolAncestorsMapValue>;

/// Type alias for GetMempoolAncestorsObjectArray1
pub type GetMempoolAncestorsObjectArray1 = String;

/// Type alias for GetMempoolAncestorsObjectArray2
pub type GetMempoolAncestorsObjectArray2 = String;

/// chunks in this cluster (in mining order)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetMempoolClusterChunks {
    /// fees of the transactions in this chunk
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub chunkfee: bitcoin::Amount,
    /// sigops-adjusted weight of all transactions in this chunk
    /// Wire JSON key: `chunkweight` (Rust field `chunk_weight`).
    #[serde(rename = "chunkweight")]
    pub chunk_weight: u64,
    /// transactions in this chunk in mining order
    pub txs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetMempoolClusterResult {
    /// chunks in this cluster (in mining order)
    pub chunks: Vec<GetMempoolClusterChunks>,
    /// total sigops-adjusted weight (as defined in BIP 141 and modified by '-bytespersigop')
    pub clusterweight: u64,
    /// number of transactions
    /// Wire JSON key: `txcount` (Rust field `tx_count`).
    #[serde(rename = "txcount")]
    pub tx_count: u64,
}

/// Type alias for GetMempoolDescendantsArrayArray1
pub type GetMempoolDescendantsArrayArray1 = String;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetMempoolDescendantsFees {
    /// transaction fees of in-mempool ancestors (including this one) with fee deltas used for mining priority, denominated in BTC
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub ancestor: bitcoin::Amount,
    /// transaction fee, denominated in BTC
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub base: bitcoin::Amount,
    /// transaction fees of chunk, denominated in BTC
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub chunk: bitcoin::Amount,
    /// transaction fees of in-mempool descendants (including this one) with fee deltas used for mining priority, denominated in BTC
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub descendant: bitcoin::Amount,
    /// transaction fee with fee deltas used for mining priority, denominated in BTC
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub modified: bitcoin::Amount,
}

/// for verbose = true
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetMempoolDescendantsMapValue {
    /// number of in-mempool ancestor transactions (including this one)
    /// Wire JSON key: `ancestorcount` (Rust field `ancestor_count`).
    #[serde(rename = "ancestorcount")]
    pub ancestor_count: u64,
    /// virtual transaction size of in-mempool ancestors (including this one)
    /// Wire JSON key: `ancestorsize` (Rust field `ancestor_size`).
    #[serde(rename = "ancestorsize")]
    pub ancestor_size: u64,
    /// sigops-adjusted weight (as defined in BIP 141 and modified by '-bytespersigop') of this transaction's chunk
    /// Wire JSON key: `chunkweight` (Rust field `chunk_weight`).
    #[serde(rename = "chunkweight")]
    pub chunk_weight: u64,
    /// unconfirmed transactions used as inputs for this transaction
    pub depends: Vec<String>,
    /// number of in-mempool descendant transactions (including this one)
    /// Wire JSON key: `descendantcount` (Rust field `descendant_count`).
    #[serde(rename = "descendantcount")]
    pub descendant_count: u64,
    /// virtual transaction size of in-mempool descendants (including this one)
    /// Wire JSON key: `descendantsize` (Rust field `descendant_size`).
    #[serde(rename = "descendantsize")]
    pub descendant_size: u64,
    pub fees: GetMempoolDescendantsFees,
    /// block height when transaction entered pool
    pub height: u64,
    /// unconfirmed transactions spending outputs from this transaction
    /// Wire JSON key: `spentby` (Rust field `spent_by`).
    #[serde(rename = "spentby")]
    pub spent_by: Vec<String>,
    /// local time transaction entered pool in seconds since 1 Jan 1970 GMT
    pub time: u64,
    /// Whether this transaction is currently unbroadcast (initial broadcast not yet acknowledged by any peers)
    pub unbroadcast: bool,
    /// (DEPRECATED) Was previously erroneously described as the BIP 141 vsize, but is actually sigops-adjusted vsize.
    /// Use vsize_bip141 to actually get that behavior or switch to the explicit vsize_adjusted for retained behavior.
    /// Wire JSON key: `vsize` (Rust field `v_size`).
    #[serde(rename = "vsize")]
    pub v_size: u64,
    /// Maximum of sigop-adjusted size (-bytespersigop) and virtual transaction size as defined in BIP 141.
    pub vsize_adjusted: u64,
    /// Virtual transaction size as defined in BIP 141.
    /// This is different from actual serialized size for witness transactions as witness data is discounted.
    pub vsize_bip141: u64,
    /// transaction weight as defined in BIP 141.
    pub weight: u64,
    /// hash of serialized transaction, including witness data
    /// Wire JSON key: `wtxid` (Rust field `w_txid`).
    #[serde(rename = "wtxid")]
    pub w_txid: String,
}

/// Type alias for GetMempoolDescendantsObject
pub type GetMempoolDescendantsObject = BTreeMap<String, GetMempoolDescendantsMapValue>;

/// Type alias for GetMempoolDescendantsObjectArray1
pub type GetMempoolDescendantsObjectArray1 = String;

/// Type alias for GetMempoolDescendantsObjectArray2
pub type GetMempoolDescendantsObjectArray2 = String;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetMempoolEntryFees {
    /// transaction fees of in-mempool ancestors (including this one) with fee deltas used for mining priority, denominated in BTC
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub ancestor: bitcoin::Amount,
    /// transaction fee, denominated in BTC
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub base: bitcoin::Amount,
    /// transaction fees of chunk, denominated in BTC
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub chunk: bitcoin::Amount,
    /// transaction fees of in-mempool descendants (including this one) with fee deltas used for mining priority, denominated in BTC
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub descendant: bitcoin::Amount,
    /// transaction fee with fee deltas used for mining priority, denominated in BTC
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub modified: bitcoin::Amount,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetMempoolEntryResult {
    /// number of in-mempool ancestor transactions (including this one)
    /// Wire JSON key: `ancestorcount` (Rust field `ancestor_count`).
    #[serde(rename = "ancestorcount")]
    pub ancestor_count: u64,
    /// virtual transaction size of in-mempool ancestors (including this one)
    /// Wire JSON key: `ancestorsize` (Rust field `ancestor_size`).
    #[serde(rename = "ancestorsize")]
    pub ancestor_size: u64,
    /// sigops-adjusted weight (as defined in BIP 141 and modified by '-bytespersigop') of this transaction's chunk
    /// Wire JSON key: `chunkweight` (Rust field `chunk_weight`).
    #[serde(rename = "chunkweight")]
    pub chunk_weight: u64,
    /// unconfirmed transactions used as inputs for this transaction
    pub depends: Vec<String>,
    /// number of in-mempool descendant transactions (including this one)
    /// Wire JSON key: `descendantcount` (Rust field `descendant_count`).
    #[serde(rename = "descendantcount")]
    pub descendant_count: u64,
    /// virtual transaction size of in-mempool descendants (including this one)
    /// Wire JSON key: `descendantsize` (Rust field `descendant_size`).
    #[serde(rename = "descendantsize")]
    pub descendant_size: u64,
    pub fees: GetMempoolEntryFees,
    /// block height when transaction entered pool
    pub height: u64,
    /// unconfirmed transactions spending outputs from this transaction
    /// Wire JSON key: `spentby` (Rust field `spent_by`).
    #[serde(rename = "spentby")]
    pub spent_by: Vec<String>,
    /// local time transaction entered pool in seconds since 1 Jan 1970 GMT
    pub time: u64,
    /// Whether this transaction is currently unbroadcast (initial broadcast not yet acknowledged by any peers)
    pub unbroadcast: bool,
    /// (DEPRECATED) Was previously erroneously described as the BIP 141 vsize, but is actually sigops-adjusted vsize.
    /// Use vsize_bip141 to actually get that behavior or switch to the explicit vsize_adjusted for retained behavior.
    /// Wire JSON key: `vsize` (Rust field `v_size`).
    #[serde(rename = "vsize")]
    pub v_size: u64,
    /// Maximum of sigop-adjusted size (-bytespersigop) and virtual transaction size as defined in BIP 141.
    pub vsize_adjusted: u64,
    /// Virtual transaction size as defined in BIP 141.
    /// This is different from actual serialized size for witness transactions as witness data is discounted.
    pub vsize_bip141: u64,
    /// transaction weight as defined in BIP 141.
    pub weight: u64,
    /// hash of serialized transaction, including witness data
    /// Wire JSON key: `wtxid` (Rust field `w_txid`).
    #[serde(rename = "wtxid")]
    pub w_txid: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetMempoolFeeRateDiagramResultItem {
    /// cumulative fee
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub fee: bitcoin::Amount,
    /// cumulative sigops-adjusted weight
    pub weight: u64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetMempoolInfoResult {
    /// Sum of all virtual transaction sizes as defined in BIP 141. Differs from actual serialized size because witness data is discounted
    pub bytes: u64,
    /// minimum fee rate increment for mempool limiting or replacement in BTC/kvB
    /// Wire JSON key: `incrementalrelayfee` (Rust field `incremental_relay_fee`).
    #[serde(rename = "incrementalrelayfee")]
    pub incremental_relay_fee: f64,
    /// Maximum number of transactions that can be in a cluster (configured by -limitclustercount)
    /// Wire JSON key: `limitclustercount` (Rust field `limit_cluster_count`).
    #[serde(rename = "limitclustercount")]
    pub limit_cluster_count: u64,
    /// Maximum size of a cluster in virtual bytes (configured by -limitclustersize)
    /// Wire JSON key: `limitclustersize` (Rust field `limit_cluster_size`).
    #[serde(rename = "limitclustersize")]
    pub limit_cluster_size: u64,
    /// True if the initial load attempt of the persisted mempool finished
    pub loaded: bool,
    /// Maximum number of bytes that can be used by OP_RETURN outputs in the mempool
    /// Wire JSON key: `maxdatacarriersize` (Rust field `max_data_carrier_size`).
    #[serde(rename = "maxdatacarriersize")]
    pub max_data_carrier_size: u64,
    /// Maximum memory usage for the mempool
    /// Wire JSON key: `maxmempool` (Rust field `max_mempool`).
    #[serde(rename = "maxmempool")]
    pub max_mempool: u64,
    /// Minimum fee rate in BTC/kvB for tx to be accepted. Is the maximum of minrelaytxfee and minimum mempool fee
    /// Wire JSON key: `mempoolminfee` (Rust field `mempool_min_fee`).
    #[serde(rename = "mempoolminfee")]
    pub mempool_min_fee: f64,
    /// Current minimum relay fee for transactions
    /// Wire JSON key: `minrelaytxfee` (Rust field `min_relay_tx_fee`).
    #[serde(rename = "minrelaytxfee")]
    pub min_relay_tx_fee: f64,
    /// If the mempool is in a known-optimal transaction ordering
    pub optimal: bool,
    /// True if the mempool accepts transactions with bare multisig outputs
    /// Wire JSON key: `permitbaremultisig` (Rust field `permit_bare_multisig`).
    #[serde(rename = "permitbaremultisig")]
    pub permit_bare_multisig: bool,
    /// Current tx count
    pub size: u64,
    /// Total fees for the mempool in BTC, ignoring modified fees through prioritisetransaction
    pub total_fee: f64,
    /// Current number of transactions that haven't passed initial broadcast yet
    /// Wire JSON key: `unbroadcastcount` (Rust field `unbroadcast_count`).
    #[serde(rename = "unbroadcastcount")]
    pub unbroadcast_count: u64,
    /// Total memory usage for the mempool
    pub usage: u64,
}

/// The next block
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetMiningInfoNext {
    /// The next target nBits
    pub bits: String,
    /// The next difficulty
    pub difficulty: f64,
    /// The next height
    pub height: u64,
    /// The next target
    pub target: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetMiningInfoResult {
    /// The hash of the current best block
    /// Wire JSON key: `bestblockhash` (Rust field `best_block_hash`).
    #[serde(rename = "bestblockhash")]
    pub best_block_hash: String,
    /// The current nBits, compact representation of the block difficulty target
    pub bits: String,
    /// Minimum feerate of packages selected for block inclusion in BTC/kvB
    /// Wire JSON key: `blockmintxfee` (Rust field `block_min_tx_fee`).
    #[serde(rename = "blockmintxfee")]
    pub block_min_tx_fee: f64,
    /// The current block
    pub blocks: u64,
    /// current network name (main, test, testnet4, signet, regtest)
    pub chain: String,
    /// The number of block transactions (excluding coinbase) of the last assembled block (only present if a block was ever assembled)
    /// Wire JSON key: `currentblocktx` (Rust field `current_block_tx`).
    #[serde(rename = "currentblocktx")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_block_tx: Option<u64>,
    /// The block weight (including reserved weight for block header, txs count and coinbase tx) of the last assembled block (only present if a block was ever assembled)
    /// Wire JSON key: `currentblockweight` (Rust field `current_block_weight`).
    #[serde(rename = "currentblockweight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_block_weight: Option<u64>,
    /// The current difficulty
    pub difficulty: f64,
    /// The network hashes per second
    /// Wire JSON key: `networkhashps` (Rust field `network_hashps`).
    #[serde(rename = "networkhashps")]
    pub network_hashps: f64,
    /// The next block
    pub next: GetMiningInfoNext,
    /// The size of the mempool
    /// Wire JSON key: `pooledtx` (Rust field `pooled_tx`).
    #[serde(rename = "pooledtx")]
    pub pooled_tx: u64,
    /// The block challenge (aka. block script), in hexadecimal (only present if the current network is a signet)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signet_challenge: Option<String>,
    /// The current target
    pub target: String,
    /// any network and blockchain warnings (run with `-deprecatedrpc=warnings` to return the latest warning as a single string)
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetNetTotalsResult {
    /// Current system UNIX epoch time in milliseconds
    /// Wire JSON key: `timemillis` (Rust field `time_millis`).
    #[serde(rename = "timemillis")]
    pub time_millis: u64,
    /// Total bytes received
    /// Wire JSON key: `totalbytesrecv` (Rust field `total_bytes_recv`).
    #[serde(rename = "totalbytesrecv")]
    pub total_bytes_recv: u64,
    /// Total bytes sent
    /// Wire JSON key: `totalbytessent` (Rust field `total_bytes_sent`).
    #[serde(rename = "totalbytessent")]
    pub total_bytes_sent: u64,
    /// Wire JSON key: `uploadtarget` (Rust field `upload_target`).
    #[serde(rename = "uploadtarget")]
    pub upload_target: GetNetTotalsUploadtarget,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetNetTotalsUploadtarget {
    /// Bytes left in current time cycle
    pub bytes_left_in_cycle: u64,
    /// True if serving historical blocks
    pub serve_historical_blocks: bool,
    /// Target in bytes
    pub target: u64,
    /// True if target is reached
    pub target_reached: bool,
    /// Seconds left in current time cycle
    pub time_left_in_cycle: u64,
    /// Length of the measuring timeframe in seconds
    pub timeframe: u64,
}

/// Type alias for GetNetworkInfoInvBuckets
pub type GetNetworkInfoInvBuckets = BTreeMap<String, GetNetworkInfoMapValue>;

/// list of local addresses
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetNetworkInfoLocaladdresses {
    /// network address
    pub address: String,
    /// network port
    pub port: u16,
    /// relative score
    pub score: u64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetNetworkInfoMapValue {
    /// number of queued txs to announce
    pub backlog: u64,
    /// tokens available to be consumed per-transaction
    pub count_tok: f64,
    /// tokens available to be consumed per-byte
    pub size_tok: f64,
}

/// information per network
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetNetworkInfoNetworks {
    /// is the network limited using -onlynet?
    pub limited: bool,
    /// network (ipv4, ipv6, onion, i2p, cjdns)
    pub name: String,
    /// ("host:port") the proxy that is used for this network, or empty if none
    pub proxy: String,
    /// Whether randomized credentials are used
    pub proxy_randomize_credentials: bool,
    /// is the network reachable?
    pub reachable: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetNetworkInfoResult {
    /// the SHA256 hash of the asmap data used for IP bucketing (only displayed if the -asmap config option is set)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asmap_version: Option<String>,
    /// the total number of connections
    pub connections: u64,
    /// the number of inbound connections
    pub connections_in: u64,
    /// the number of outbound connections
    pub connections_out: u64,
    /// minimum fee rate increment for mempool limiting or replacement in BTC/kvB
    /// Wire JSON key: `incrementalfee` (Rust field `incremental_fee`).
    #[serde(rename = "incrementalfee")]
    pub incremental_fee: f64,
    pub inv_buckets: BTreeMap<String, GetNetworkInfoMapValue>,
    /// list of local addresses
    /// Wire JSON key: `localaddresses` (Rust field `local_addresses`).
    #[serde(rename = "localaddresses")]
    pub local_addresses: Vec<GetNetworkInfoLocaladdresses>,
    /// true if transaction relay is requested from peers
    /// Wire JSON key: `localrelay` (Rust field `local_relay`).
    #[serde(rename = "localrelay")]
    pub local_relay: bool,
    /// the services we offer to the network
    /// Wire JSON key: `localservices` (Rust field `local_services`).
    #[serde(rename = "localservices")]
    pub local_services: String,
    /// the services we offer to the network, in human-readable form
    /// Wire JSON key: `localservicesnames` (Rust field `local_services_names`).
    #[serde(rename = "localservicesnames")]
    pub local_services_names: Vec<String>,
    /// whether p2p networking is enabled
    /// Wire JSON key: `networkactive` (Rust field `network_active`).
    #[serde(rename = "networkactive")]
    pub network_active: bool,
    /// information per network
    pub networks: Vec<GetNetworkInfoNetworks>,
    /// the protocol version
    /// Wire JSON key: `protocolversion` (Rust field `protocol_version`).
    #[serde(rename = "protocolversion")]
    pub protocol_version: u64,
    /// minimum relay fee rate for transactions in BTC/kvB
    /// Wire JSON key: `relayfee` (Rust field `relay_fee`).
    #[serde(rename = "relayfee")]
    pub relay_fee: f64,
    /// the server subversion string
    pub subversion: String,
    /// the time offset
    /// Wire JSON key: `timeoffset` (Rust field `time_offset`).
    #[serde(rename = "timeoffset")]
    pub time_offset: u64,
    /// configured target for maximum number of transactions per second to send to inbound peers
    pub tx_send_rate: u64,
    /// the server version
    pub version: u32,
    /// any network and blockchain warnings (run with `-deprecatedrpc=warnings` to return the latest warning as a single string)
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetNodeAddressesResultItem {
    /// The address of the node
    pub address: String,
    /// The network (ipv4, ipv6, onion, i2p, cjdns) the node connected through
    pub network: String,
    /// The port number of the node
    pub port: u16,
    /// The services offered by the node
    pub services: u64,
    /// The UNIX epoch time when the node was last seen
    pub time: u64,
}

/// Metadata about this JSON-RPC interface.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetOpenRpcInfoInfo {
    /// API description.
    pub description: String,
    /// API title.
    pub title: String,
    /// Bitcoin Core version string.
    pub version: String,
}

/// Documented RPC methods.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetOpenRpcInfoMethods {
    /// Method description.
    pub description: String,
    /// Method name.
    pub name: String,
    /// Method parameters.
    pub params: Vec<GetOpenRpcInfoParams>,
    /// Method result.
    pub result: GetOpenRpcInfoMethodsResult,
    /// RPC category.
    /// Wire JSON key: `x-bitcoin-category` (Rust field `x_bitcoin_category`).
    #[serde(rename = "x-bitcoin-category")]
    pub x_bitcoin_category: String,
}

/// Method result.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetOpenRpcInfoMethodsResult {
    /// Result name.
    pub name: String,
    /// JSON Schema for the result. Numeric schemas may include "x-bitcoin-unit" property: "amount" which denotes a Bitcoin amount in BTC.
    pub schema: serde_json::Value,
}

/// Method parameters.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetOpenRpcInfoParams {
    /// Parameter description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Parameter name.
    pub name: String,
    /// Whether the parameter is required.
    pub required: bool,
    /// JSON Schema for the parameter.
    pub schema: serde_json::Value,
    /// Alternative parameter names.
    /// Wire JSON key: `x-bitcoin-aliases` (Rust field `x_bitcoin_aliases`).
    #[serde(rename = "x-bitcoin-aliases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_bitcoin_aliases: Option<Vec<String>>,
    /// Whether the parameter can also be passed positionally.
    /// Wire JSON key: `x-bitcoin-also-positional` (Rust field `x_bitcoin_also_positional`).
    #[serde(rename = "x-bitcoin-also-positional")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_bitcoin_also_positional: Option<bool>,
    /// Whether the parameter is retained only for compatibility.
    /// Wire JSON key: `x-bitcoin-placeholder` (Rust field `x_bitcoin_placeholder`).
    #[serde(rename = "x-bitcoin-placeholder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_bitcoin_placeholder: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetOpenRpcInfoResult {
    /// Metadata about this JSON-RPC interface.
    pub info: GetOpenRpcInfoInfo,
    /// Documented RPC methods.
    pub methods: Vec<GetOpenRpcInfoMethods>,
    /// OpenRPC specification version.
    pub openrpc: String,
}

/// Type alias for GetOrphanTxsArray2Array1
pub type GetOrphanTxsArray2Array1 = String;

/// Type alias for GetOrphanTxsArray2Array2
pub type GetOrphanTxsArray2Array2 = String;

/// Type alias for GetOrphanTxsArray3Array1
pub type GetOrphanTxsArray3Array1 = String;

/// Type alias for GetOrphanTxsArray3Array2
pub type GetOrphanTxsArray3Array2 = String;

/// Type alias for GetOrphanTxsArrayArray1
pub type GetOrphanTxsArrayArray1 = String;

/// Type alias for GetPeerInfoBytesrecvPerMsg
pub type GetPeerInfoBytesrecvPerMsg = BTreeMap<String, u64>;

/// Type alias for GetPeerInfoBytessentPerMsg
pub type GetPeerInfoBytessentPerMsg = BTreeMap<String, u64>;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetPeerInfoResultItem {
    /// (host:port) The IP address/hostname optionally followed by :port of the peer
    pub addr: String,
    /// The total number of addresses processed, excluding those dropped due to rate limiting
    pub addr_processed: u64,
    /// The total number of addresses dropped due to rate limiting
    pub addr_rate_limited: u64,
    /// Whether we participate in address relay with this peer
    pub addr_relay_enabled: bool,
    /// (ip:port) Bind address of the connection to the peer
    /// Wire JSON key: `addrbind` (Rust field `addr_bind`).
    #[serde(rename = "addrbind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addr_bind: Option<String>,
    /// (ip:port) Local address as reported by the peer
    /// Wire JSON key: `addrlocal` (Rust field `addr_local`).
    #[serde(rename = "addrlocal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addr_local: Option<String>,
    /// Whether peer selected us as (compact blocks) high-bandwidth peer
    pub bip152_hb_from: bool,
    /// Whether we selected peer as (compact blocks) high-bandwidth peer
    pub bip152_hb_to: bool,
    /// The total bytes received
    /// Wire JSON key: `bytesrecv` (Rust field `bytes_recv`).
    #[serde(rename = "bytesrecv")]
    pub bytes_recv: u64,
    /// Wire JSON key: `bytesrecv_per_msg` (Rust field `bytes_recv_per_msg`).
    #[serde(rename = "bytesrecv_per_msg")]
    pub bytes_recv_per_msg: BTreeMap<String, u64>,
    /// The total bytes sent
    /// Wire JSON key: `bytessent` (Rust field `bytes_sent`).
    #[serde(rename = "bytessent")]
    pub bytes_sent: u64,
    /// Wire JSON key: `bytessent_per_msg` (Rust field `bytes_sent_per_msg`).
    #[serde(rename = "bytessent_per_msg")]
    pub bytes_sent_per_msg: BTreeMap<String, u64>,
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
    /// The UNIX epoch time of the connection
    /// Wire JSON key: `conntime` (Rust field `conn_time`).
    #[serde(rename = "conntime")]
    pub conn_time: u64,
    /// Peer index
    pub id: u64,
    /// Inbound (true) or Outbound (false)
    pub inbound: bool,
    /// Wire JSON key: `inflight` (Rust field `in_flight`).
    #[serde(rename = "inflight")]
    pub in_flight: Vec<u64>,
    /// How many txs we have queued to announce to this peer
    pub inv_to_send: u64,
    /// The UNIX epoch time of the last block received from this peer
    pub last_block: u64,
    /// Mempool sequence number of this peer's last INV
    pub last_inv_sequence: u64,
    /// The UNIX epoch time of the last valid transaction received from this peer
    pub last_transaction: u64,
    /// The UNIX epoch time of the last receive
    /// Wire JSON key: `lastrecv` (Rust field `last_recv`).
    #[serde(rename = "lastrecv")]
    pub last_recv: u64,
    /// The UNIX epoch time of the last send
    /// Wire JSON key: `lastsend` (Rust field `last_send`).
    #[serde(rename = "lastsend")]
    pub last_send: u64,
    /// Mapped AS (Autonomous System) number at the end of the BGP route to the peer, used for diversifying
    /// peer selection (only displayed if the -asmap config option is set)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mapped_as: Option<u64>,
    /// The minimum fee rate for transactions this peer accepts
    /// Wire JSON key: `minfeefilter` (Rust field `min_fee_filter`).
    #[serde(rename = "minfeefilter")]
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub min_fee_filter: bitcoin::Amount,
    /// The minimum observed ping time in seconds, if any
    /// Wire JSON key: `minping` (Rust field `min_ping`).
    #[serde(rename = "minping")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ping: Option<u64>,
    /// Network (ipv4, ipv6, onion, i2p, cjdns, not_publicly_routable)
    pub network: String,
    /// Any special permissions that have been granted to this peer
    pub permissions: Vec<String>,
    /// The last ping time in seconds, if any
    /// Wire JSON key: `pingtime` (Rust field `ping_time`).
    #[serde(rename = "pingtime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ping_time: Option<u64>,
    /// The duration in seconds of an outstanding ping (if non-zero)
    /// Wire JSON key: `pingwait` (Rust field `ping_wait`).
    #[serde(rename = "pingwait")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ping_wait: Option<u64>,
    /// The current height of header pre-synchronization with this peer, or -1 if no low-work sync is in progress
    pub presynced_headers: u64,
    /// Whether we relay transactions to this peer
    /// Wire JSON key: `relaytxes` (Rust field `relay_txes`).
    #[serde(rename = "relaytxes")]
    pub relay_txes: bool,
    /// The services offered
    pub services: String,
    /// the services offered, in human-readable form
    /// Wire JSON key: `servicesnames` (Rust field `services_names`).
    #[serde(rename = "servicesnames")]
    pub services_names: Vec<String>,
    /// The session ID for this connection, or "" if there is none ("v2" transport protocol only).
    pub session_id: String,
    /// The string version
    pub subver: String,
    /// The last block we have in common with this peer
    pub synced_blocks: u64,
    /// The last header we have in common with this peer
    pub synced_headers: u64,
    /// The time offset in seconds
    /// Wire JSON key: `timeoffset` (Rust field `time_offset`).
    #[serde(rename = "timeoffset")]
    pub time_offset: u64,
    /// Type of transport protocol:
    /// detecting (peer could be v1 or v2),
    /// v1 (plaintext transport protocol),
    /// v2 (BIP324 encrypted transport protocol).
    pub transport_protocol_type: String,
    /// The peer version, such as 70001
    pub version: u32,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetPrioritisedTransactionsMapValue {
    /// transaction fee delta in satoshis
    pub fee_delta: u64,
    /// whether this transaction is currently in mempool
    pub in_mempool: bool,
    /// modified fee in satoshis. Only returned if in_mempool=true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_fee: Option<u64>,
}

/// Type alias for GetPrioritisedTransactionsResultMap
pub type GetPrioritisedTransactionsResultMap = BTreeMap<String, GetPrioritisedTransactionsMapValue>;

/// Per-peer send and acknowledgment information for this transaction
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetPrivateBroadcastInfoPeers {
    /// The address of the peer to which the transaction was sent
    pub address: String,
    /// The time this peer acknowledged reception of the transaction (seconds since epoch)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received: Option<u64>,
    /// The time this transaction was picked for sending to this peer via private broadcast (seconds since epoch)
    pub sent: u64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetPrivateBroadcastInfoResult {
    pub transactions: Vec<GetPrivateBroadcastInfoTransactions>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetPrivateBroadcastInfoTransactions {
    /// The number of additional private broadcast send attempts allowed for this transaction
    pub attempts_remaining: u64,
    /// The serialized, hex-encoded transaction data
    pub hex: String,
    /// Per-peer send and acknowledgment information for this transaction
    pub peers: Vec<GetPrivateBroadcastInfoPeers>,
    /// The time this transaction was added to the private broadcast queue (seconds since epoch)
    pub time_added: u64,
    /// The transaction hash in hex
    pub txid: bitcoin::Txid,
    /// The transaction witness hash in hex
    /// Wire JSON key: `wtxid` (Rust field `w_txid`).
    #[serde(rename = "wtxid")]
    pub w_txid: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetRawAddrManMapValue {
    /// The address of the node
    pub address: String,
    /// Mapped AS (Autonomous System) number at the end of the BGP route to the peer, used for diversifying peer selection (only displayed if the -asmap config option is set)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mapped_as: Option<u64>,
    /// The network (ipv4, ipv6, onion, i2p, cjdns) of the address
    pub network: String,
    /// The port number of the node
    pub port: u16,
    /// The services offered by the node
    pub services: u64,
    /// The address that relayed the address to us
    pub source: String,
    /// Mapped AS (Autonomous System) number at the end of the BGP route to the source, used for diversifying peer selection (only displayed if the -asmap config option is set)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_mapped_as: Option<u64>,
    /// The network (ipv4, ipv6, onion, i2p, cjdns) of the source address
    pub source_network: String,
    /// The UNIX epoch time when the node was last seen
    pub time: u64,
}

/// Type alias for GetRawAddrManResultMap
pub type GetRawAddrManResultMap = BTreeMap<String, BTreeMap<String, GetRawAddrManMapValue>>;

/// Type alias for GetRawAddrManResultValueMap
pub type GetRawAddrManResultValueMap = BTreeMap<String, GetRawAddrManMapValue>;

/// Type alias for GetRawMempoolArrayArray1
pub type GetRawMempoolArrayArray1 = String;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetRawMempoolFees {
    /// transaction fees of in-mempool ancestors (including this one) with fee deltas used for mining priority, denominated in BTC
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub ancestor: bitcoin::Amount,
    /// transaction fee, denominated in BTC
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub base: bitcoin::Amount,
    /// transaction fees of chunk, denominated in BTC
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub chunk: bitcoin::Amount,
    /// transaction fees of in-mempool descendants (including this one) with fee deltas used for mining priority, denominated in BTC
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub descendant: bitcoin::Amount,
    /// transaction fee with fee deltas used for mining priority, denominated in BTC
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub modified: bitcoin::Amount,
}

/// for verbose = true
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetRawMempoolMapValue {
    /// number of in-mempool ancestor transactions (including this one)
    /// Wire JSON key: `ancestorcount` (Rust field `ancestor_count`).
    #[serde(rename = "ancestorcount")]
    pub ancestor_count: u64,
    /// virtual transaction size of in-mempool ancestors (including this one)
    /// Wire JSON key: `ancestorsize` (Rust field `ancestor_size`).
    #[serde(rename = "ancestorsize")]
    pub ancestor_size: u64,
    /// sigops-adjusted weight (as defined in BIP 141 and modified by '-bytespersigop') of this transaction's chunk
    /// Wire JSON key: `chunkweight` (Rust field `chunk_weight`).
    #[serde(rename = "chunkweight")]
    pub chunk_weight: u64,
    /// unconfirmed transactions used as inputs for this transaction
    pub depends: Vec<String>,
    /// number of in-mempool descendant transactions (including this one)
    /// Wire JSON key: `descendantcount` (Rust field `descendant_count`).
    #[serde(rename = "descendantcount")]
    pub descendant_count: u64,
    /// virtual transaction size of in-mempool descendants (including this one)
    /// Wire JSON key: `descendantsize` (Rust field `descendant_size`).
    #[serde(rename = "descendantsize")]
    pub descendant_size: u64,
    pub fees: GetRawMempoolFees,
    /// block height when transaction entered pool
    pub height: u64,
    /// unconfirmed transactions spending outputs from this transaction
    /// Wire JSON key: `spentby` (Rust field `spent_by`).
    #[serde(rename = "spentby")]
    pub spent_by: Vec<String>,
    /// local time transaction entered pool in seconds since 1 Jan 1970 GMT
    pub time: u64,
    /// Whether this transaction is currently unbroadcast (initial broadcast not yet acknowledged by any peers)
    pub unbroadcast: bool,
    /// (DEPRECATED) Was previously erroneously described as the BIP 141 vsize, but is actually sigops-adjusted vsize.
    /// Use vsize_bip141 to actually get that behavior or switch to the explicit vsize_adjusted for retained behavior.
    /// Wire JSON key: `vsize` (Rust field `v_size`).
    #[serde(rename = "vsize")]
    pub v_size: u64,
    /// Maximum of sigop-adjusted size (-bytespersigop) and virtual transaction size as defined in BIP 141.
    pub vsize_adjusted: u64,
    /// Virtual transaction size as defined in BIP 141.
    /// This is different from actual serialized size for witness transactions as witness data is discounted.
    pub vsize_bip141: u64,
    /// transaction weight as defined in BIP 141.
    pub weight: u64,
    /// hash of serialized transaction, including witness data
    /// Wire JSON key: `wtxid` (Rust field `w_txid`).
    #[serde(rename = "wtxid")]
    pub w_txid: String,
}

/// Type alias for GetRawMempoolObject
pub type GetRawMempoolObject = BTreeMap<String, GetRawMempoolMapValue>;

/// Type alias for GetRawMempoolObject2Array1
pub type GetRawMempoolObject2Array1 = String;

/// Type alias for GetRawMempoolObjectArray1
pub type GetRawMempoolObjectArray1 = String;

/// Type alias for GetRawMempoolObjectArray2
pub type GetRawMempoolObjectArray2 = String;

/// Type alias for GetRawTransactionObject2Array1
pub type GetRawTransactionObject2Array1 = String;

/// Type alias for GetRawTransactionObject2Array2
pub type GetRawTransactionObject2Array2 = String;

/// Type alias for GetRawTransactionObject2Array3
pub type GetRawTransactionObject2Array3 = String;

/// Type alias for GetRawTransactionObjectArray1
pub type GetRawTransactionObjectArray1 = String;

/// Type alias for GetRawTransactionObjectArray2
pub type GetRawTransactionObjectArray2 = String;

/// Type alias for GetRawTransactionObjectArray3
pub type GetRawTransactionObjectArray3 = String;

/// The previous output, omitted if block undo data is not available
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetRawTransactionPrevout {
    /// Coinbase or not
    pub generated: bool,
    /// The height of the prevout
    pub height: u64,
    /// Wire JSON key: `scriptPubKey` (Rust field `script_pubkey`).
    #[serde(rename = "scriptPubKey")]
    pub script_pubkey: GetRawTransactionScriptPubKey,
    /// The value in BTC
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub value: bitcoin::Amount,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetRawTransactionScriptPubKey {
    /// The Bitcoin address (only if a well-defined address exists)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Disassembly of the output script
    pub asm: String,
    /// Inferred descriptor for the output
    pub desc: String,
    /// The raw output script bytes, hex-encoded
    pub hex: String,
    /// The type (one of: nonstandard, anchor, pubkey, pubkeyhash, scripthash, multisig, nulldata, witness_v0_scripthash, witness_v0_keyhash, witness_v1_taproot, witness_unknown)
    /// Wire JSON key: `type` (Rust field `r#type`).
    #[serde(rename = "type")]
    pub r#type: String,
}

/// The script (if not coinbase transaction)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetRawTransactionScriptSig {
    /// Disassembly of the signature script
    pub asm: String,
    /// The raw signature script bytes, hex-encoded
    pub hex: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetRawTransactionVin {
    /// The coinbase value (only if coinbase transaction)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coinbase: Option<String>,
    /// The script (if not coinbase transaction)
    /// Wire JSON key: `scriptSig` (Rust field `script_sig`).
    #[serde(rename = "scriptSig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_sig: Option<GetRawTransactionScriptSig>,
    /// The script sequence number
    pub sequence: u64,
    /// The transaction id (if not coinbase transaction)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txid: Option<bitcoin::Txid>,
    /// Wire JSON key: `txinwitness` (Rust field `tx_in_witness`).
    #[serde(rename = "txinwitness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_in_witness: Option<Vec<String>>,
    /// The output number (if not coinbase transaction)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vout: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetRawTransactionVout {
    /// index
    pub n: u32,
    /// Wire JSON key: `scriptPubKey` (Rust field `script_pubkey`).
    #[serde(rename = "scriptPubKey")]
    pub script_pubkey: GetRawTransactionScriptPubKey,
    /// The value in BTC
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub value: bitcoin::Amount,
}

/// All active commands
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetRpcInfoActiveCommands {
    /// The running time in microseconds
    pub duration: u64,
    /// The name of the RPC command
    pub method: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetRpcInfoResult {
    /// All active commands
    pub active_commands: Vec<GetRpcInfoActiveCommands>,
    /// The complete file path to the debug log
    pub logpath: String,
}

/// The decoded transaction (only present when `verbose` is passed)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetTransactionDecoded {
    /// The transaction hash (differs from txid for witness transactions)
    pub hash: String,
    /// The lock time
    /// Wire JSON key: `locktime` (Rust field `lock_time`).
    #[serde(rename = "locktime")]
    pub lock_time: u64,
    /// The serialized transaction size
    pub size: u64,
    /// The transaction id
    pub txid: bitcoin::Txid,
    /// The version
    pub version: u32,
    pub vin: Vec<GetTransactionVin>,
    pub vout: Vec<GetTransactionVout>,
    /// The virtual transaction size (differs from size for witness transactions)
    /// Wire JSON key: `vsize` (Rust field `v_size`).
    #[serde(rename = "vsize")]
    pub v_size: u64,
    /// The transaction's weight (between vsize*4-3 and vsize*4)
    pub weight: u64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetTransactionDetails {
    /// 'true' if the transaction has been abandoned (inputs are respendable).
    pub abandoned: bool,
    /// The bitcoin address involved in the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// The amount in BTC
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub amount: bitcoin::Amount,
    /// The transaction category.
    /// "send"                  Transactions sent.
    /// "receive"               Non-coinbase transactions received.
    /// "generate"              Coinbase transactions received with more than 100 confirmations.
    /// "immature"              Coinbase transactions received with 100 or fewer confirmations.
    /// "orphan"                Orphaned coinbase transactions received.
    pub category: String,
    /// The amount of the fee in BTC. This is negative and only available for the
    /// 'send' category of transactions.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        serialize_with = "option_amount_to_btc_float",
        deserialize_with = "option_amount_from_btc_float"
    )]
    pub fee: Option<bitcoin::Amount>,
    /// A comment for the address/transaction, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Only if 'category' is 'receive'. List of parent descriptors for the output script of this coin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_descs: Option<Vec<String>>,
    /// the vout value
    pub vout: u64,
}

/// hash and height of the block this information was generated on
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetTransactionLastProcessedBlock {
    /// hash of the block this information was generated on
    pub hash: String,
    /// height of the block this information was generated on
    pub height: u64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetTransactionResult {
    /// The wtxids of transactions with different witness data but the same txid.
    pub alternate_wtxids: Vec<String>,
    /// The amount in BTC
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub amount: bitcoin::Amount,
    /// ("yes|no|unknown") (DEPRECATED) Whether this transaction signals BIP125 replaceability or has an unconfirmed ancestor signaling BIP125 replaceability.
    /// May be unknown for unconfirmed transactions not in the mempool because their unconfirmed ancestors are unknown.
    /// Wire JSON key: `bip125-replaceable` (Rust field `bip125_replaceable`).
    #[serde(rename = "bip125-replaceable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bip125_replaceable: Option<String>,
    /// The block hash containing the transaction.
    /// Wire JSON key: `blockhash` (Rust field `block_hash`).
    #[serde(rename = "blockhash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_hash: Option<bitcoin::BlockHash>,
    /// The block height containing the transaction.
    /// Wire JSON key: `blockheight` (Rust field `block_height`).
    #[serde(rename = "blockheight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_height: Option<u64>,
    /// The index of the transaction in the block that includes it.
    /// Wire JSON key: `blockindex` (Rust field `block_index`).
    #[serde(rename = "blockindex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_index: Option<u64>,
    /// The block time expressed in UNIX epoch time.
    /// Wire JSON key: `blocktime` (Rust field `block_time`).
    #[serde(rename = "blocktime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_time: Option<u64>,
    /// If a comment is associated with the transaction, only present if not empty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// The number of confirmations for the transaction. Negative confirmations means the
    /// transaction conflicted that many blocks ago.
    pub confirmations: i64,
    /// The decoded transaction (only present when `verbose` is passed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decoded: Option<GetTransactionDecoded>,
    pub details: Vec<GetTransactionDetails>,
    /// The amount of the fee in BTC. This is negative and only available for the
    /// 'send' category of transactions.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        serialize_with = "option_amount_to_btc_float",
        deserialize_with = "option_amount_from_btc_float"
    )]
    pub fee: Option<bitcoin::Amount>,
    /// Only present if the transaction's only input is a coinbase one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated: Option<bool>,
    /// Raw data for transaction
    pub hex: String,
    /// hash and height of the block this information was generated on
    /// Wire JSON key: `lastprocessedblock` (Rust field `last_processed_block`).
    #[serde(rename = "lastprocessedblock")]
    pub last_processed_block: GetTransactionLastProcessedBlock,
    /// Transactions in the mempool that directly conflict with either this transaction or an ancestor transaction
    /// Wire JSON key: `mempoolconflicts` (Rust field `mempool_conflicts`).
    #[serde(rename = "mempoolconflicts")]
    pub mempool_conflicts: Vec<String>,
    /// Only if 'category' is 'receive'. List of parent descriptors for the output script of this coin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_descs: Option<Vec<String>>,
    /// Only if 'category' is 'send'. The txid if this tx was replaced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replaced_by_txid: Option<String>,
    /// Only if 'category' is 'send'. The txid if this tx replaces another.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replaces_txid: Option<String>,
    /// The transaction time expressed in UNIX epoch time.
    pub time: u64,
    /// The time received expressed in UNIX epoch time.
    /// Wire JSON key: `timereceived` (Rust field `time_received`).
    #[serde(rename = "timereceived")]
    pub time_received: u64,
    /// If a comment to is associated with the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    /// Whether we consider the transaction to be trusted and safe to spend from.
    /// Only present when the transaction has 0 confirmations (or negative confirmations, if conflicted).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted: Option<bool>,
    /// The transaction id.
    pub txid: bitcoin::Txid,
    /// Confirmed transactions that have been detected by the wallet to conflict with this transaction.
    /// Wire JSON key: `walletconflicts` (Rust field `wallet_conflicts`).
    #[serde(rename = "walletconflicts")]
    pub wallet_conflicts: Vec<String>,
    /// The hash of serialized transaction, including witness data.
    /// Wire JSON key: `wtxid` (Rust field `w_txid`).
    #[serde(rename = "wtxid")]
    pub w_txid: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetTransactionScriptPubKey {
    /// The Bitcoin address (only if a well-defined address exists)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Disassembly of the output script
    pub asm: String,
    /// Inferred descriptor for the output
    pub desc: String,
    /// The raw output script bytes, hex-encoded
    pub hex: String,
    /// The type (one of: nonstandard, anchor, pubkey, pubkeyhash, scripthash, multisig, nulldata, witness_v0_scripthash, witness_v0_keyhash, witness_v1_taproot, witness_unknown)
    /// Wire JSON key: `type` (Rust field `r#type`).
    #[serde(rename = "type")]
    pub r#type: String,
}

/// The script (if not coinbase transaction)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetTransactionScriptSig {
    /// Disassembly of the signature script
    pub asm: String,
    /// The raw signature script bytes, hex-encoded
    pub hex: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetTransactionVin {
    /// The coinbase value (only if coinbase transaction)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coinbase: Option<String>,
    /// The script (if not coinbase transaction)
    /// Wire JSON key: `scriptSig` (Rust field `script_sig`).
    #[serde(rename = "scriptSig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_sig: Option<GetTransactionScriptSig>,
    /// The script sequence number
    pub sequence: u64,
    /// The transaction id (if not coinbase transaction)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txid: Option<bitcoin::Txid>,
    /// Wire JSON key: `txinwitness` (Rust field `tx_in_witness`).
    #[serde(rename = "txinwitness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_in_witness: Option<Vec<String>>,
    /// The output number (if not coinbase transaction)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vout: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetTransactionVout {
    /// Output script is change (only present if true)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ischange: Option<bool>,
    /// index
    pub n: u32,
    /// Wire JSON key: `scriptPubKey` (Rust field `script_pubkey`).
    #[serde(rename = "scriptPubKey")]
    pub script_pubkey: GetTransactionScriptPubKey,
    /// The value in BTC
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub value: bitcoin::Amount,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetTxOutScriptPubKey {
    /// The Bitcoin address (only if a well-defined address exists)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Disassembly of the output script
    pub asm: String,
    /// Inferred descriptor for the output
    pub desc: String,
    /// The raw output script bytes, hex-encoded
    pub hex: String,
    /// The type, eg pubkeyhash
    /// Wire JSON key: `type` (Rust field `r#type`).
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Info on amounts in the block at this block height (only available if coinstatsindex is used)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetTxOutSetInfoBlockInfo {
    /// Coinbase subsidy amount of this block
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub coinbase: bitcoin::Amount,
    /// Total amount of new outputs created by this block
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub new_outputs_ex_coinbase: bitcoin::Amount,
    /// Total amount of all prevouts spent in this block
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub prevout_spent: bitcoin::Amount,
    /// Total amount of unspendable outputs created in this block
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub unspendable: bitcoin::Amount,
    /// Detailed view of the unspendable categories
    pub unspendables: GetTxOutSetInfoUnspendables,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetTxOutSetInfoResult {
    /// The hash of the block at which these statistics are calculated
    /// Wire JSON key: `bestblock` (Rust field `best_block`).
    #[serde(rename = "bestblock")]
    pub best_block: String,
    /// Info on amounts in the block at this block height (only available if coinstatsindex is used)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_info: Option<GetTxOutSetInfoBlockInfo>,
    /// Database-independent, meaningless metric indicating the UTXO set size
    /// Wire JSON key: `bogosize` (Rust field `bogo_size`).
    #[serde(rename = "bogosize")]
    pub bogo_size: u64,
    /// The estimated size of the chainstate on disk (not available when coinstatsindex is used)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_size: Option<u64>,
    /// The serialized hash (only present if 'hash_serialized_3' hash_type is chosen)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash_serialized_3: Option<String>,
    /// The block height (index) of the returned statistics
    pub height: u64,
    /// The serialized hash (only present if 'muhash' hash_type is chosen)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub muhash: Option<String>,
    /// The total amount of coins in the UTXO set
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub total_amount: bitcoin::Amount,
    /// The total amount of coins permanently excluded from the UTXO set (only available if coinstatsindex is used)
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        serialize_with = "option_amount_to_btc_float",
        deserialize_with = "option_amount_from_btc_float"
    )]
    pub total_unspendable_amount: Option<bitcoin::Amount>,
    /// The number of transactions with unspent outputs (not available when coinstatsindex is used)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactions: Option<u64>,
    /// The number of unspent transaction outputs
    pub txouts: u64,
}

/// Detailed view of the unspendable categories
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetTxOutSetInfoUnspendables {
    /// Transactions overridden by duplicates (no longer possible with BIP30)
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub bip30: bitcoin::Amount,
    /// The unspendable amount of the Genesis block subsidy
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub genesis_block: bitcoin::Amount,
    /// Amounts sent to scripts that are unspendable (for example OP_RETURN outputs)
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub scripts: bitcoin::Amount,
    /// Fee rewards that miners did not claim in their coinbase transaction
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub unclaimed_rewards: bitcoin::Amount,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetTxSpendingPrevOutResultItem {
    /// the hash of the spending block (omitted if unspent or the spending tx is not confirmed)
    /// Wire JSON key: `blockhash` (Rust field `block_hash`).
    #[serde(rename = "blockhash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_hash: Option<bitcoin::BlockHash>,
    /// the transaction spending this output (only if return_spending_tx is set, omitted if unspent)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spendingtx: Option<String>,
    /// the transaction id of the mempool transaction spending this output (omitted if unspent)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spendingtxid: Option<String>,
    /// the transaction id of the checked output
    pub txid: bitcoin::Txid,
    /// the vout value of the checked output
    pub vout: u64,
}

/// hash and height of the block this information was generated on
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetWalletInfoLastProcessedBlock {
    /// hash of the block this information was generated on
    pub hash: String,
    /// height of the block this information was generated on
    pub height: u64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetWalletInfoResult {
    /// whether this wallet tracks clean/dirty coins in terms of reuse
    pub avoid_reuse: bool,
    /// The start time for blocks scanning. It could be modified by (re)importing any descriptor with an earlier timestamp.
    /// Wire JSON key: `birthtime` (Rust field `birth_time`).
    #[serde(rename = "birthtime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_time: Option<u64>,
    /// Whether this wallet intentionally does not contain any keys, scripts, or descriptors
    pub blank: bool,
    /// whether this wallet uses descriptors for output script management
    pub descriptors: bool,
    /// whether this wallet is configured to use an external signer such as a hardware wallet
    pub external_signer: bool,
    /// The flags currently set on the wallet
    pub flags: Vec<String>,
    /// the database format (only sqlite)
    pub format: String,
    /// how many new keys are pre-generated (only counts external keys)
    /// Wire JSON key: `keypoolsize` (Rust field `key_pool_size`).
    #[serde(rename = "keypoolsize")]
    pub key_pool_size: u64,
    /// how many new keys are pre-generated for internal use (used for change outputs; 0 if external keys are used for change)
    pub keypoolsize_hd_internal: u64,
    /// hash and height of the block this information was generated on
    /// Wire JSON key: `lastprocessedblock` (Rust field `last_processed_block`).
    #[serde(rename = "lastprocessedblock")]
    pub last_processed_block: GetWalletInfoLastProcessedBlock,
    /// false if privatekeys are disabled for this wallet (enforced watch-only wallet)
    pub private_keys_enabled: bool,
    /// current scanning details, or false if no scan is in progress
    pub scanning: GetWalletInfoResultScanning,
    /// the total number of transactions in the wallet
    /// Wire JSON key: `txcount` (Rust field `tx_count`).
    #[serde(rename = "txcount")]
    pub tx_count: u64,
    /// the UNIX epoch time until which the wallet is unlocked for transfers, or 0 if the wallet is locked (only present for passphrase-encrypted wallets)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlocked_until: Option<u64>,
    /// the wallet name
    /// Wire JSON key: `walletname` (Rust field `wallet_name`).
    #[serde(rename = "walletname")]
    pub wallet_name: String,
    /// (DEPRECATED) only related to unsupported legacy wallet, returns the latest version 169900 for backwards compatibility
    /// Wire JSON key: `walletversion` (Rust field `wallet_version`).
    #[serde(rename = "walletversion")]
    pub wallet_version: u64,
}

/// Object shape from an OpenRPC union branch (nested field)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetWalletInfoResultScanningGetWalletInfoObject {
    /// elapsed seconds since scan start
    pub duration: u64,
    /// scanning progress percentage \[0.0, 1.0\]
    pub progress: u64,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWalletInfoResultScanning {
    Branch2(serde_json::Value),
    Object(GetWalletInfoResultScanningGetWalletInfoObject),
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetZmqNotificationsResultItem {
    /// Address of the publisher
    pub address: String,
    /// Outbound message high water mark
    pub hwm: u64,
    /// Type of notification
    /// Wire JSON key: `type` (Rust field `r#type`).
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ImportDescriptorsError {
    /// JSONRPC error code
    pub code: u64,
    /// JSONRPC error message
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ImportDescriptorsResultItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ImportDescriptorsError>,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ImportMempoolResult {}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ListBannedResultItem {
    /// The IP/Subnet of the banned node
    pub address: String,
    /// The UNIX epoch time the ban was created
    pub ban_created: u64,
    /// The ban duration, in seconds
    pub ban_duration: u64,
    /// The UNIX epoch time the ban expires
    pub banned_until: u64,
    /// The time remaining until the ban expires, in seconds
    pub time_remaining: u64,
}

/// Array of descriptor objects (sorted by descriptor string representation)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ListDescriptorsDescriptors {
    /// Whether this descriptor is currently used to generate new addresses
    pub active: bool,
    /// Descriptor string representation
    pub desc: String,
    /// True if this descriptor is used to generate change addresses. False if this descriptor is used to generate receiving addresses; defined only for active descriptors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal: Option<bool>,
    /// Same as next_index field. Kept for compatibility reason.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<u64>,
    /// The next index to generate addresses from; defined only for ranged descriptors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_index: Option<u64>,
    /// Defined only for ranged descriptors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<Vec<u64>>,
    /// The creation time of the descriptor
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ListDescriptorsResult {
    /// Array of descriptor objects (sorted by descriptor string representation)
    pub descriptors: Vec<ListDescriptorsDescriptors>,
    /// Name of wallet this operation was performed on
    pub wallet_name: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ListLockUnspentResultItem {
    /// The transaction id locked
    pub txid: bitcoin::Txid,
    /// The vout value
    pub vout: u64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ListReceivedByAddressResultItem {
    /// The receiving address
    pub address: String,
    /// The total amount in BTC received by the address
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub amount: bitcoin::Amount,
    /// The number of confirmations of the most recent transaction included
    pub confirmations: i64,
    /// The label of the receiving address. The default label is ""
    pub label: String,
    pub txids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ListReceivedByLabelResultItem {
    /// The total amount received by addresses with this label
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub amount: bitcoin::Amount,
    /// The number of confirmations of the most recent transaction included
    pub confirmations: i64,
    /// The label of the receiving address. The default label is ""
    pub label: String,
}

/// &lt;structure is the same as "transactions" above, only present if include_removed=true&gt;
/// Note: transactions that were re-added in the active chain will appear as-is in this array, and may thus have a positive confirmation count.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ListSinceBlockRemoved {
    /// 'true' if the transaction has been abandoned (inputs are respendable).
    pub abandoned: bool,
    /// The bitcoin address of the transaction (not returned if the output does not have an address, e.g. OP_RETURN null data).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// The wtxids of transactions with different witness data but the same txid.
    pub alternate_wtxids: Vec<String>,
    /// The amount in BTC. This is negative for the 'send' category, and is positive
    /// for all other categories
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub amount: bitcoin::Amount,
    /// ("yes|no|unknown") (DEPRECATED) Whether this transaction signals BIP125 replaceability or has an unconfirmed ancestor signaling BIP125 replaceability.
    /// May be unknown for unconfirmed transactions not in the mempool because their unconfirmed ancestors are unknown.
    /// Wire JSON key: `bip125-replaceable` (Rust field `bip125_replaceable`).
    #[serde(rename = "bip125-replaceable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bip125_replaceable: Option<String>,
    /// The block hash containing the transaction.
    /// Wire JSON key: `blockhash` (Rust field `block_hash`).
    #[serde(rename = "blockhash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_hash: Option<bitcoin::BlockHash>,
    /// The block height containing the transaction.
    /// Wire JSON key: `blockheight` (Rust field `block_height`).
    #[serde(rename = "blockheight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_height: Option<u64>,
    /// The index of the transaction in the block that includes it.
    /// Wire JSON key: `blockindex` (Rust field `block_index`).
    #[serde(rename = "blockindex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_index: Option<u64>,
    /// The block time expressed in UNIX epoch time.
    /// Wire JSON key: `blocktime` (Rust field `block_time`).
    #[serde(rename = "blocktime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_time: Option<u64>,
    /// The transaction category.
    /// "send"                  Transactions sent.
    /// "receive"               Non-coinbase transactions received.
    /// "generate"              Coinbase transactions received with more than 100 confirmations.
    /// "immature"              Coinbase transactions received with 100 or fewer confirmations.
    /// "orphan"                Orphaned coinbase transactions received.
    pub category: String,
    /// If a comment is associated with the transaction, only present if not empty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// The number of confirmations for the transaction. Negative confirmations means the
    /// transaction conflicted that many blocks ago.
    pub confirmations: i64,
    /// The amount of the fee in BTC. This is negative and only available for the
    /// 'send' category of transactions.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        serialize_with = "option_amount_to_btc_float",
        deserialize_with = "option_amount_from_btc_float"
    )]
    pub fee: Option<bitcoin::Amount>,
    /// Only present if the transaction's only input is a coinbase one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated: Option<bool>,
    /// A comment for the address/transaction, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Transactions in the mempool that directly conflict with either this transaction or an ancestor transaction
    /// Wire JSON key: `mempoolconflicts` (Rust field `mempool_conflicts`).
    #[serde(rename = "mempoolconflicts")]
    pub mempool_conflicts: Vec<String>,
    /// Only if 'category' is 'receive'. List of parent descriptors for the output script of this coin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_descs: Option<Vec<String>>,
    /// Only if 'category' is 'send'. The txid if this tx was replaced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replaced_by_txid: Option<String>,
    /// Only if 'category' is 'send'. The txid if this tx replaces another.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replaces_txid: Option<String>,
    /// The transaction time expressed in UNIX epoch time.
    pub time: u64,
    /// The time received expressed in UNIX epoch time.
    /// Wire JSON key: `timereceived` (Rust field `time_received`).
    #[serde(rename = "timereceived")]
    pub time_received: u64,
    /// If a comment to is associated with the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    /// Whether we consider the transaction to be trusted and safe to spend from.
    /// Only present when the transaction has 0 confirmations (or negative confirmations, if conflicted).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted: Option<bool>,
    /// The transaction id.
    pub txid: bitcoin::Txid,
    /// the vout value
    pub vout: u64,
    /// Confirmed transactions that have been detected by the wallet to conflict with this transaction.
    /// Wire JSON key: `walletconflicts` (Rust field `wallet_conflicts`).
    #[serde(rename = "walletconflicts")]
    pub wallet_conflicts: Vec<String>,
    /// The hash of serialized transaction, including witness data.
    /// Wire JSON key: `wtxid` (Rust field `w_txid`).
    #[serde(rename = "wtxid")]
    pub w_txid: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ListSinceBlockResult {
    /// The hash of the block (target_confirmations-1) from the best block on the main chain, or the genesis hash if the referenced block does not exist yet. This is typically used to feed back into listsinceblock the next time you call it. So you would generally use a target_confirmations of say 6, so you will be continually re-notified of transactions until they've reached 6 confirmations plus any new ones
    /// Wire JSON key: `lastblock` (Rust field `last_block`).
    #[serde(rename = "lastblock")]
    pub last_block: String,
    /// &lt;structure is the same as "transactions" above, only present if include_removed=true&gt;
    /// Note: transactions that were re-added in the active chain will appear as-is in this array, and may thus have a positive confirmation count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed: Option<Vec<ListSinceBlockRemoved>>,
    pub transactions: Vec<ListSinceBlockTransactions>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ListSinceBlockTransactions {
    /// 'true' if the transaction has been abandoned (inputs are respendable).
    pub abandoned: bool,
    /// The bitcoin address of the transaction (not returned if the output does not have an address, e.g. OP_RETURN null data).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// The wtxids of transactions with different witness data but the same txid.
    pub alternate_wtxids: Vec<String>,
    /// The amount in BTC. This is negative for the 'send' category, and is positive
    /// for all other categories
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub amount: bitcoin::Amount,
    /// ("yes|no|unknown") (DEPRECATED) Whether this transaction signals BIP125 replaceability or has an unconfirmed ancestor signaling BIP125 replaceability.
    /// May be unknown for unconfirmed transactions not in the mempool because their unconfirmed ancestors are unknown.
    /// Wire JSON key: `bip125-replaceable` (Rust field `bip125_replaceable`).
    #[serde(rename = "bip125-replaceable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bip125_replaceable: Option<String>,
    /// The block hash containing the transaction.
    /// Wire JSON key: `blockhash` (Rust field `block_hash`).
    #[serde(rename = "blockhash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_hash: Option<bitcoin::BlockHash>,
    /// The block height containing the transaction.
    /// Wire JSON key: `blockheight` (Rust field `block_height`).
    #[serde(rename = "blockheight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_height: Option<u64>,
    /// The index of the transaction in the block that includes it.
    /// Wire JSON key: `blockindex` (Rust field `block_index`).
    #[serde(rename = "blockindex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_index: Option<u64>,
    /// The block time expressed in UNIX epoch time.
    /// Wire JSON key: `blocktime` (Rust field `block_time`).
    #[serde(rename = "blocktime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_time: Option<u64>,
    /// The transaction category.
    /// "send"                  Transactions sent.
    /// "receive"               Non-coinbase transactions received.
    /// "generate"              Coinbase transactions received with more than 100 confirmations.
    /// "immature"              Coinbase transactions received with 100 or fewer confirmations.
    /// "orphan"                Orphaned coinbase transactions received.
    pub category: String,
    /// If a comment is associated with the transaction, only present if not empty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// The number of confirmations for the transaction. Negative confirmations means the
    /// transaction conflicted that many blocks ago.
    pub confirmations: i64,
    /// The amount of the fee in BTC. This is negative and only available for the
    /// 'send' category of transactions.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        serialize_with = "option_amount_to_btc_float",
        deserialize_with = "option_amount_from_btc_float"
    )]
    pub fee: Option<bitcoin::Amount>,
    /// Only present if the transaction's only input is a coinbase one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated: Option<bool>,
    /// A comment for the address/transaction, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Transactions in the mempool that directly conflict with either this transaction or an ancestor transaction
    /// Wire JSON key: `mempoolconflicts` (Rust field `mempool_conflicts`).
    #[serde(rename = "mempoolconflicts")]
    pub mempool_conflicts: Vec<String>,
    /// Only if 'category' is 'receive'. List of parent descriptors for the output script of this coin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_descs: Option<Vec<String>>,
    /// Only if 'category' is 'send'. The txid if this tx was replaced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replaced_by_txid: Option<String>,
    /// Only if 'category' is 'send'. The txid if this tx replaces another.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replaces_txid: Option<String>,
    /// The transaction time expressed in UNIX epoch time.
    pub time: u64,
    /// The time received expressed in UNIX epoch time.
    /// Wire JSON key: `timereceived` (Rust field `time_received`).
    #[serde(rename = "timereceived")]
    pub time_received: u64,
    /// If a comment to is associated with the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    /// Whether we consider the transaction to be trusted and safe to spend from.
    /// Only present when the transaction has 0 confirmations (or negative confirmations, if conflicted).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted: Option<bool>,
    /// The transaction id.
    pub txid: bitcoin::Txid,
    /// the vout value
    pub vout: u64,
    /// Confirmed transactions that have been detected by the wallet to conflict with this transaction.
    /// Wire JSON key: `walletconflicts` (Rust field `wallet_conflicts`).
    #[serde(rename = "walletconflicts")]
    pub wallet_conflicts: Vec<String>,
    /// The hash of serialized transaction, including witness data.
    /// Wire JSON key: `wtxid` (Rust field `w_txid`).
    #[serde(rename = "wtxid")]
    pub w_txid: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ListTransactionsResultItem {
    /// 'true' if the transaction has been abandoned (inputs are respendable).
    pub abandoned: bool,
    /// The bitcoin address of the transaction (not returned if the output does not have an address, e.g. OP_RETURN null data).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// The wtxids of transactions with different witness data but the same txid.
    pub alternate_wtxids: Vec<String>,
    /// The amount in BTC. This is negative for the 'send' category, and is positive
    /// for all other categories
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub amount: bitcoin::Amount,
    /// ("yes|no|unknown") (DEPRECATED) Whether this transaction signals BIP125 replaceability or has an unconfirmed ancestor signaling BIP125 replaceability.
    /// May be unknown for unconfirmed transactions not in the mempool because their unconfirmed ancestors are unknown.
    /// Wire JSON key: `bip125-replaceable` (Rust field `bip125_replaceable`).
    #[serde(rename = "bip125-replaceable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bip125_replaceable: Option<String>,
    /// The block hash containing the transaction.
    /// Wire JSON key: `blockhash` (Rust field `block_hash`).
    #[serde(rename = "blockhash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_hash: Option<bitcoin::BlockHash>,
    /// The block height containing the transaction.
    /// Wire JSON key: `blockheight` (Rust field `block_height`).
    #[serde(rename = "blockheight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_height: Option<u64>,
    /// The index of the transaction in the block that includes it.
    /// Wire JSON key: `blockindex` (Rust field `block_index`).
    #[serde(rename = "blockindex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_index: Option<u64>,
    /// The block time expressed in UNIX epoch time.
    /// Wire JSON key: `blocktime` (Rust field `block_time`).
    #[serde(rename = "blocktime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_time: Option<u64>,
    /// The transaction category.
    /// "send"                  Transactions sent.
    /// "receive"               Non-coinbase transactions received.
    /// "generate"              Coinbase transactions received with more than 100 confirmations.
    /// "immature"              Coinbase transactions received with 100 or fewer confirmations.
    /// "orphan"                Orphaned coinbase transactions received.
    pub category: String,
    /// If a comment is associated with the transaction, only present if not empty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// The number of confirmations for the transaction. Negative confirmations means the
    /// transaction conflicted that many blocks ago.
    pub confirmations: i64,
    /// The amount of the fee in BTC. This is negative and only available for the
    /// 'send' category of transactions.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        serialize_with = "option_amount_to_btc_float",
        deserialize_with = "option_amount_from_btc_float"
    )]
    pub fee: Option<bitcoin::Amount>,
    /// Only present if the transaction's only input is a coinbase one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated: Option<bool>,
    /// A comment for the address/transaction, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Transactions in the mempool that directly conflict with either this transaction or an ancestor transaction
    /// Wire JSON key: `mempoolconflicts` (Rust field `mempool_conflicts`).
    #[serde(rename = "mempoolconflicts")]
    pub mempool_conflicts: Vec<String>,
    /// Only if 'category' is 'receive'. List of parent descriptors for the output script of this coin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_descs: Option<Vec<String>>,
    /// Only if 'category' is 'send'. The txid if this tx was replaced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replaced_by_txid: Option<String>,
    /// Only if 'category' is 'send'. The txid if this tx replaces another.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replaces_txid: Option<String>,
    /// The transaction time expressed in UNIX epoch time.
    pub time: u64,
    /// The time received expressed in UNIX epoch time.
    /// Wire JSON key: `timereceived` (Rust field `time_received`).
    #[serde(rename = "timereceived")]
    pub time_received: u64,
    /// If a comment to is associated with the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    /// Whether we consider the transaction to be trusted and safe to spend from.
    /// Only present when the transaction has 0 confirmations (or negative confirmations, if conflicted).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted: Option<bool>,
    /// The transaction id.
    pub txid: bitcoin::Txid,
    /// the vout value
    pub vout: u64,
    /// Confirmed transactions that have been detected by the wallet to conflict with this transaction.
    /// Wire JSON key: `walletconflicts` (Rust field `wallet_conflicts`).
    #[serde(rename = "walletconflicts")]
    pub wallet_conflicts: Vec<String>,
    /// The hash of serialized transaction, including witness data.
    /// Wire JSON key: `wtxid` (Rust field `w_txid`).
    #[serde(rename = "wtxid")]
    pub w_txid: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ListUnspentResultItem {
    /// the bitcoin address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// the transaction output amount in BTC
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub amount: bitcoin::Amount,
    /// The number of in-mempool ancestor transactions, including this one (if transaction is in the mempool)
    /// Wire JSON key: `ancestorcount` (Rust field `ancestor_count`).
    #[serde(rename = "ancestorcount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ancestor_count: Option<u64>,
    /// The total fees of in-mempool ancestors (including this one) with fee deltas used for mining priority in sat (if transaction is in the mempool)
    /// Wire JSON key: `ancestorfees` (Rust field `ancestor_fees`).
    #[serde(rename = "ancestorfees")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ancestor_fees: Option<u64>,
    /// The virtual transaction size of in-mempool ancestors, including this one (if transaction is in the mempool)
    /// Wire JSON key: `ancestorsize` (Rust field `ancestor_size`).
    #[serde(rename = "ancestorsize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ancestor_size: Option<u64>,
    /// The number of confirmations
    pub confirmations: i64,
    /// (only when solvable) A descriptor for spending this output
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    /// The associated label, or "" for the default label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// List of parent descriptors for the output script of this coin.
    pub parent_descs: Vec<String>,
    /// The redeem script if the output script is P2SH
    /// Wire JSON key: `redeemScript` (Rust field `redeem_script`).
    #[serde(rename = "redeemScript")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redeem_script: Option<bitcoin::ScriptBuf>,
    /// (only present if avoid_reuse is set) Whether this output is reused/dirty (sent to an address that was previously spent from)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reused: Option<bool>,
    /// Whether this output is considered safe to spend. Unconfirmed transactions
    /// from outside keys and unconfirmed replacement transactions are considered unsafe
    /// and are not eligible for spending by fundrawtransaction and sendtoaddress.
    pub safe: bool,
    /// the output script
    /// Wire JSON key: `scriptPubKey` (Rust field `script_pubkey`).
    #[serde(rename = "scriptPubKey")]
    pub script_pubkey: bitcoin::ScriptBuf,
    /// Whether we know how to spend this output, ignoring the lack of keys
    pub solvable: bool,
    /// (DEPRECATED) Always true
    pub spendable: bool,
    /// the transaction id
    pub txid: bitcoin::Txid,
    /// the vout value
    pub vout: u64,
    /// witness script if the output script is P2WSH or P2SH-P2WSH
    /// Wire JSON key: `witnessScript` (Rust field `witness_script`).
    #[serde(rename = "witnessScript")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_script: Option<bitcoin::ScriptBuf>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ListWalletDirResult {
    pub wallets: Vec<ListWalletDirWallets>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ListWalletDirWallets {
    /// The wallet name
    pub name: String,
    /// Warning messages related to loading the wallet (may be empty).
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct LoadTxOutSetResult {
    /// the height of the base of the snapshot
    pub base_height: u64,
    /// the number of coins loaded from the snapshot
    pub coins_loaded: u64,
    /// the absolute path that the snapshot was loaded from
    pub path: String,
    /// the hash of the base of the snapshot
    pub tip_hash: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct LoadWalletResult {
    /// The wallet name if loaded successfully.
    pub name: String,
    /// Warning messages, if any, related to loading the wallet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

/// Type alias for LoggingResultMap
pub type LoggingResultMap = BTreeMap<String, bool>;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct MigrateWalletResult {
    /// The location of the backup of the original wallet
    pub backup_path: String,
    /// The name of the migrated wallet containing solvable but not watched scripts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solvables_name: Option<String>,
    /// The name of the primary migrated wallet
    pub wallet_name: String,
    /// The name of the migrated wallet containing the watchonly scripts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watchonly_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PsbtBumpFeeResult {
    /// Errors encountered during processing (may be empty).
    pub errors: Vec<String>,
    /// The fee of the new transaction.
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub fee: bitcoin::Amount,
    /// The fee of the replaced transaction.
    /// Wire JSON key: `origfee` (Rust field `orig_fee`).
    #[serde(rename = "origfee")]
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub orig_fee: bitcoin::Amount,
    /// The base64-encoded unsigned PSBT of the new transaction.
    pub psbt: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RescanBlockchainResult {
    /// The block height where the rescan started (the requested height or 0)
    pub start_height: u64,
    /// The height of the last rescanned block. May be null in rare cases if there was a reorg and the call didn't scan any blocks because they were already scanned in the background.
    pub stop_height: u64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RestoreWalletResult {
    /// The wallet name if restored successfully.
    pub name: String,
    /// Warning messages, if any, related to restoring and loading the wallet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

/// Metadata about this JSON-RPC interface.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RpcDiscoverInfo {
    /// API description.
    pub description: String,
    /// API title.
    pub title: String,
    /// Bitcoin Core version string.
    pub version: String,
}

/// Documented RPC methods.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RpcDiscoverMethods {
    /// Method description.
    pub description: String,
    /// Method name.
    pub name: String,
    /// Method parameters.
    pub params: Vec<RpcDiscoverParams>,
    /// Method result.
    pub result: RpcDiscoverMethodsResult,
    /// RPC category.
    /// Wire JSON key: `x-bitcoin-category` (Rust field `x_bitcoin_category`).
    #[serde(rename = "x-bitcoin-category")]
    pub x_bitcoin_category: String,
}

/// Method result.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RpcDiscoverMethodsResult {
    /// Result name.
    pub name: String,
    /// JSON Schema for the result. Numeric schemas may include "x-bitcoin-unit" property: "amount" which denotes a Bitcoin amount in BTC.
    pub schema: serde_json::Value,
}

/// Method parameters.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RpcDiscoverParams {
    /// Parameter description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Parameter name.
    pub name: String,
    /// Whether the parameter is required.
    pub required: bool,
    /// JSON Schema for the parameter.
    pub schema: serde_json::Value,
    /// Alternative parameter names.
    /// Wire JSON key: `x-bitcoin-aliases` (Rust field `x_bitcoin_aliases`).
    #[serde(rename = "x-bitcoin-aliases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_bitcoin_aliases: Option<Vec<String>>,
    /// Whether the parameter can also be passed positionally.
    /// Wire JSON key: `x-bitcoin-also-positional` (Rust field `x_bitcoin_also_positional`).
    #[serde(rename = "x-bitcoin-also-positional")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_bitcoin_also_positional: Option<bool>,
    /// Whether the parameter is retained only for compatibility.
    /// Wire JSON key: `x-bitcoin-placeholder` (Rust field `x_bitcoin_placeholder`).
    #[serde(rename = "x-bitcoin-placeholder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_bitcoin_placeholder: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RpcDiscoverResult {
    /// Metadata about this JSON-RPC interface.
    pub info: RpcDiscoverInfo,
    /// Documented RPC methods.
    pub methods: Vec<RpcDiscoverMethods>,
    /// OpenRPC specification version.
    pub openrpc: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SaveMempoolResult {
    /// the directory and file where the mempool was saved
    pub filename: String,
}

/// Type alias for ScanBlocksObjectArray1
pub type ScanBlocksObjectArray1 = String;

/// Type alias for ScanTxOutSetObjectArray1
pub type ScanTxOutSetObjectArray1 = String;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ScanTxOutSetUnspents {
    /// The total amount in BTC of the unspent output
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub amount: bitcoin::Amount,
    /// Blockhash of the unspent transaction output
    /// Wire JSON key: `blockhash` (Rust field `block_hash`).
    #[serde(rename = "blockhash")]
    pub block_hash: bitcoin::BlockHash,
    /// Whether this is a coinbase output
    pub coinbase: bool,
    /// Number of confirmations of the unspent transaction output when the scan was done
    pub confirmations: i64,
    /// A specialized descriptor for the matched output script
    pub desc: String,
    /// Height of the unspent transaction output
    pub height: u64,
    /// The output script
    /// Wire JSON key: `scriptPubKey` (Rust field `script_pubkey`).
    #[serde(rename = "scriptPubKey")]
    pub script_pubkey: bitcoin::ScriptBuf,
    /// The transaction id
    pub txid: bitcoin::Txid,
    /// The vout value
    pub vout: u64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SendAllResult {
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// If add_to_wallet is false, the hex-encoded raw transaction with signature(s)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<String>,
    /// If more signatures are needed, or if add_to_wallet is false, the base64-encoded (partially) signed transaction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psbt: Option<String>,
    /// The transaction id for the send. Only 1 transaction is created regardless of the number of addresses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txid: Option<bitcoin::Txid>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SendMsgToPeerResult {}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SendResult {
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// If add_to_wallet is false, the hex-encoded raw transaction with signature(s)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<String>,
    /// If more signatures are needed, or if add_to_wallet is false, the base64-encoded (partially) signed transaction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psbt: Option<String>,
    /// The transaction id for the send. Only 1 transaction is created regardless of the number of addresses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txid: Option<bitcoin::Txid>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SetWalletFlagResult {
    /// The name of the flag that was modified
    pub flag_name: String,
    /// The new state of the flag
    pub flag_state: bool,
    /// Any warnings associated with the change
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<String>,
}

/// Script verification errors (if there are any)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SignRawTransactionWithKeyErrors {
    /// Verification or signing error related to the input
    pub error: String,
    /// The hex-encoded signature script
    /// Wire JSON key: `scriptSig` (Rust field `script_sig`).
    #[serde(rename = "scriptSig")]
    pub script_sig: String,
    /// Script sequence number
    pub sequence: u64,
    /// The hash of the referenced, previous transaction
    pub txid: bitcoin::Txid,
    /// The index of the output to spent and used as input
    pub vout: u64,
    pub witness: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SignRawTransactionWithKeyResult {
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// Script verification errors (if there are any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<SignRawTransactionWithKeyErrors>>,
    /// The hex-encoded raw transaction with signature(s)
    pub hex: String,
}

/// Script verification errors (if there are any)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SignRawTransactionWithWalletErrors {
    /// Verification or signing error related to the input
    pub error: String,
    /// The hex-encoded signature script
    /// Wire JSON key: `scriptSig` (Rust field `script_sig`).
    #[serde(rename = "scriptSig")]
    pub script_sig: String,
    /// Script sequence number
    pub sequence: u64,
    /// The hash of the referenced, previous transaction
    pub txid: bitcoin::Txid,
    /// The index of the output to spent and used as input
    pub vout: u64,
    pub witness: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SignRawTransactionWithWalletResult {
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// Script verification errors (if there are any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<SignRawTransactionWithWalletErrors>>,
    /// The hex-encoded raw transaction with signature(s)
    pub hex: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SimulateRawTransactionResult {
    /// The wallet balance change (negative means decrease).
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub balance_change: bitcoin::Amount,
}

/// Transaction fees
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SubmitPackageFees {
    /// transaction fee in BTC
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub base: bitcoin::Amount,
    /// if the transaction was not already in the mempool, the effective feerate in BTC per KvB. For example, the package feerate and/or feerate with modified fees from prioritisetransaction.
    /// Wire JSON key: `effective-feerate` (Rust field `effective_feerate`).
    #[serde(rename = "effective-feerate")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        serialize_with = "option_amount_to_btc_float",
        deserialize_with = "option_amount_from_btc_float"
    )]
    pub effective_feerate: Option<bitcoin::Amount>,
    /// if effective-feerate is provided, the wtxids of the transactions whose fees and vsizes are included in effective-feerate.
    /// Wire JSON key: `effective-includes` (Rust field `effective_includes`).
    #[serde(rename = "effective-includes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_includes: Option<Vec<String>>,
}

/// The transaction results keyed by wtxid. An entry is returned for every submitted wtxid.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SubmitPackageMapValue {
    /// Error string if rejected from mempool, or "package-not-validated" when the package aborts before any per-tx processing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Transaction fees
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fees: Option<SubmitPackageFees>,
    /// The wtxid of a different transaction with the same txid but different witness found in the mempool. This means the submitted transaction was ignored.
    /// Wire JSON key: `other-wtxid` (Rust field `other_wtxid`).
    #[serde(rename = "other-wtxid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_wtxid: Option<String>,
    /// The transaction hash in hex
    pub txid: bitcoin::Txid,
    /// (DEPRECATED) Was previously erroneously described as the BIP 141 vsize, but is actually sigops-adjusted vsize.
    /// Use vsize_bip141 to actually get that behavior or switch to the explicit vsize_adjusted for retained behavior.
    /// Wire JSON key: `vsize` (Rust field `v_size`).
    #[serde(rename = "vsize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_size: Option<u64>,
    /// Maximum of sigop-adjusted size (-bytespersigop) and virtual transaction size as defined in BIP 141.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vsize_adjusted: Option<u64>,
    /// Virtual transaction size as defined in BIP 141.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vsize_bip141: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SubmitPackageResult {
    /// The transaction package result message. "success" indicates all transactions were accepted into or are already in the mempool.
    pub package_msg: String,
    /// List of txids of replaced transactions
    /// Wire JSON key: `replaced-transactions` (Rust field `replaced_transactions`).
    #[serde(rename = "replaced-transactions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replaced_transactions: Option<Vec<String>>,
    /// The transaction results keyed by wtxid. An entry is returned for every submitted wtxid.
    /// Wire JSON key: `tx-results` (Rust field `tx_results`).
    #[serde(rename = "tx-results")]
    pub tx_results: BTreeMap<String, SubmitPackageMapValue>,
}

/// Type alias for SubmitPackageTxResults
pub type SubmitPackageTxResults = BTreeMap<String, SubmitPackageMapValue>;

/// Transaction fees (only present if 'allowed' is true)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct TestMempoolAcceptFees {
    /// transaction fee in BTC
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub base: bitcoin::Amount,
    /// the effective feerate in BTC per KvB. May differ from the base feerate if, for example, there are modified fees from prioritisetransaction or a package feerate was used.
    /// Wire JSON key: `effective-feerate` (Rust field `effective_feerate`).
    #[serde(rename = "effective-feerate")]
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub effective_feerate: bitcoin::Amount,
    /// transactions whose fees and vsizes are included in effective-feerate.
    /// Wire JSON key: `effective-includes` (Rust field `effective_includes`).
    #[serde(rename = "effective-includes")]
    pub effective_includes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct TestMempoolAcceptResultItem {
    /// Whether this tx would be accepted to the mempool and pass client-specified maxfeerate. If not present, the tx was not fully validated due to a failure in another tx in the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed: Option<bool>,
    /// Transaction fees (only present if 'allowed' is true)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fees: Option<TestMempoolAcceptFees>,
    /// Package validation error, if any (only possible if rawtxs had more than 1 transaction).
    /// Wire JSON key: `package-error` (Rust field `package_error`).
    #[serde(rename = "package-error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_error: Option<String>,
    /// Rejection details (only present when 'allowed' is false and rejection details exist)
    /// Wire JSON key: `reject-details` (Rust field `reject_details`).
    #[serde(rename = "reject-details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject_details: Option<String>,
    /// Rejection reason (only present when 'allowed' is false)
    /// Wire JSON key: `reject-reason` (Rust field `reject_reason`).
    #[serde(rename = "reject-reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject_reason: Option<String>,
    /// The transaction hash in hex
    pub txid: bitcoin::Txid,
    /// (DEPRECATED) Was previously erroneously described as the BIP 141 vsize, but is actually sigops-adjusted vsize.
    /// Use vsize_bip141 to actually get that behavior or switch to the explicit vsize_adjusted for retained behavior.
    /// Wire JSON key: `vsize` (Rust field `v_size`).
    #[serde(rename = "vsize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_size: Option<u64>,
    /// Maximum of sigop-adjusted size (-bytespersigop) and virtual transaction size as defined in BIP 141 (only present when 'allowed' is true).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vsize_adjusted: Option<u64>,
    /// Virtual transaction size as defined in BIP 141.
    /// This is different from actual serialized size for witness transactions as witness data is discounted (only present when 'allowed' is true).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vsize_bip141: Option<u64>,
    /// The transaction witness hash in hex
    /// Wire JSON key: `wtxid` (Rust field `w_txid`).
    #[serde(rename = "wtxid")]
    pub w_txid: String,
}

/// Type alias for Txid
pub type Txid = bitcoin::Txid;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct UnloadWalletResult {
    /// Warning messages, if any, related to unloading the wallet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ValidateAddressResult {
    /// The bitcoin address validated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Error message, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Indices of likely error locations in address, if known (e.g. Bech32 errors)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_locations: Option<Vec<u64>>,
    /// If the key is a script
    /// Wire JSON key: `isscript` (Rust field `is_script`).
    #[serde(rename = "isscript")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_script: Option<bool>,
    /// If the address is valid or not
    /// Wire JSON key: `isvalid` (Rust field `is_valid`).
    #[serde(rename = "isvalid")]
    pub is_valid: bool,
    /// If the address is a witness address
    /// Wire JSON key: `iswitness` (Rust field `is_witness`).
    #[serde(rename = "iswitness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_witness: Option<bool>,
    /// The hex-encoded output script generated by the address
    /// Wire JSON key: `scriptPubKey` (Rust field `script_pubkey`).
    #[serde(rename = "scriptPubKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_pubkey: Option<bitcoin::ScriptBuf>,
    /// The hex value of the witness program
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_program: Option<String>,
    /// The version number of the witness program
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_version: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct WaitForBlockHeightResult {
    /// The blockhash
    pub hash: String,
    /// Block height
    pub height: u64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct WaitForBlockResult {
    /// The blockhash
    pub hash: String,
    /// Block height
    pub height: u64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct WaitForNewBlockResult {
    /// The blockhash
    pub hash: String,
    /// Block height
    pub height: u64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct WalletCreateFundedPsbtResult {
    /// The position of the added change output, or -1
    pub changepos: i64,
    /// Fee in BTC the resulting transaction pays
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub fee: bitcoin::Amount,
    /// The resulting raw transaction (base64-encoded string)
    pub psbt: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct WalletDisplayAddressResult {
    /// The address as confirmed by the signer
    pub address: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct WalletProcessPsbtResult {
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// The hex-encoded network transaction if complete
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<String>,
    /// The base64-encoded partially signed transaction
    pub psbt: String,
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for AbandonTransactionResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<()> for AbandonTransactionResponse {
    fn as_ref(&self) -> &() {
        &self.value
    }
}

impl From<()> for AbandonTransactionResponse {
    fn from(value: ()) -> Self {
        Self { value }
    }
}

impl From<AbandonTransactionResponse> for () {
    fn from(wrapper: AbandonTransactionResponse) -> Self {
        wrapper.value
    }
}

/// Response for the `AbortPrivateBroadcast` RPC method
///
/// Wire method: `abortprivatebroadcast`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct AbortPrivateBroadcastResponse {
    /// Transactions removed from the private broadcast queue
    pub removed_transactions: Vec<AbortPrivateBroadcastRemovedTransactions>,
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for AbortRescanResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<bool> for AbortRescanResponse {
    fn as_ref(&self) -> &bool {
        &self.value
    }
}

impl From<bool> for AbortRescanResponse {
    fn from(value: bool) -> Self {
        Self { value }
    }
}

impl From<AbortRescanResponse> for bool {
    fn from(wrapper: AbortRescanResponse) -> Self {
        wrapper.value
    }
}

/// Response for the `AddConnection` RPC method
///
/// Wire method: `addconnection`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct AddConnectionResponse {
    /// Address of newly added connection.
    pub address: String,
    /// Type of connection opened.
    pub connection_type: String,
}

/// Response for the `AddHdKey` RPC method
///
/// Wire method: `addhdkey`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct AddHdKeyResponse {
    /// The xpub of the HD key that was added to the wallet
    pub xpub: String,
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for AddNodeResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<()> for AddNodeResponse {
    fn as_ref(&self) -> &() {
        &self.value
    }
}

impl From<()> for AddNodeResponse {
    fn from(value: ()) -> Self {
        Self { value }
    }
}

impl From<AddNodeResponse> for () {
    fn from(wrapper: AddNodeResponse) -> Self {
        wrapper.value
    }
}

/// Response for the `AddPeerAddress` RPC method
///
/// Wire method: `addpeeraddress`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct AddPeerAddressResponse {
    /// error description, if the address could not be added
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// whether the peer address was successfully added to the address manager table
    pub success: bool,
}

/// Response for the `AnalyzePsbt` RPC method
///
/// Wire method: `analyzepsbt`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct AnalyzePsbtResponse {
    /// Error message (if there is one)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Estimated feerate of the final signed transaction in BTC/kvB. Shown only if all UTXO slots in the PSBT have been filled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_feerate: Option<f64>,
    /// Estimated vsize of the final signed transaction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_vsize: Option<u64>,
    /// The transaction fee paid. Shown only if all UTXO slots in the PSBT have been filled
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        serialize_with = "option_amount_to_btc_float",
        deserialize_with = "option_amount_from_btc_float"
    )]
    pub fee: Option<bitcoin::Amount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<AnalyzePsbtInputs>>,
    /// Role of the next person that this psbt needs to go to
    pub next: String,
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for BackupWalletResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<()> for BackupWalletResponse {
    fn as_ref(&self) -> &() {
        &self.value
    }
}

impl From<()> for BackupWalletResponse {
    fn from(value: ()) -> Self {
        Self { value }
    }
}

impl From<BackupWalletResponse> for () {
    fn from(wrapper: BackupWalletResponse) -> Self {
        wrapper.value
    }
}

/// Response for the `BumpFee` RPC method
///
/// Wire method: `bumpfee`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct BumpFeeResponse {
    /// Errors encountered during processing (may be empty).
    pub errors: Vec<String>,
    /// The fee of the new transaction.
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub fee: bitcoin::Amount,
    /// The fee of the replaced transaction.
    /// Wire JSON key: `origfee` (Rust field `orig_fee`).
    #[serde(rename = "origfee")]
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub orig_fee: bitcoin::Amount,
    /// The id of the new transaction.
    pub txid: bitcoin::Txid,
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for ClearBannedResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<()> for ClearBannedResponse {
    fn as_ref(&self) -> &() {
        &self.value
    }
}

impl From<()> for ClearBannedResponse {
    fn from(value: ()) -> Self {
        Self { value }
    }
}

impl From<ClearBannedResponse> for () {
    fn from(wrapper: ClearBannedResponse) -> Self {
        wrapper.value
    }
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
                Ok(CombinePsbtResponse {
                    value: v.to_string(),
                })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CombinePsbtResponse {
                    value: v.to_string(),
                })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CombinePsbtResponse {
                    value: v.to_string(),
                })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CombinePsbtResponse {
                    value: v.to_string(),
                })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CombinePsbtResponse {
                    value: v.to_string(),
                })
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for CombinePsbtResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<String> for CombinePsbtResponse {
    fn as_ref(&self) -> &String {
        &self.value
    }
}

impl From<String> for CombinePsbtResponse {
    fn from(value: String) -> Self {
        Self { value }
    }
}

impl From<CombinePsbtResponse> for String {
    fn from(wrapper: CombinePsbtResponse) -> Self {
        wrapper.value
    }
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
                Ok(CombineRawTransactionResponse {
                    value: v.to_string(),
                })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CombineRawTransactionResponse {
                    value: v.to_string(),
                })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CombineRawTransactionResponse {
                    value: v.to_string(),
                })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CombineRawTransactionResponse {
                    value: v.to_string(),
                })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CombineRawTransactionResponse {
                    value: v.to_string(),
                })
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for CombineRawTransactionResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<String> for CombineRawTransactionResponse {
    fn as_ref(&self) -> &String {
        &self.value
    }
}

impl From<String> for CombineRawTransactionResponse {
    fn from(value: String) -> Self {
        Self { value }
    }
}

impl From<CombineRawTransactionResponse> for String {
    fn from(wrapper: CombineRawTransactionResponse) -> Self {
        wrapper.value
    }
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
                Ok(ConvertToPsbtResponse {
                    value: v.to_string(),
                })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ConvertToPsbtResponse {
                    value: v.to_string(),
                })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ConvertToPsbtResponse {
                    value: v.to_string(),
                })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ConvertToPsbtResponse {
                    value: v.to_string(),
                })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ConvertToPsbtResponse {
                    value: v.to_string(),
                })
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for ConvertToPsbtResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<String> for ConvertToPsbtResponse {
    fn as_ref(&self) -> &String {
        &self.value
    }
}

impl From<String> for ConvertToPsbtResponse {
    fn from(value: String) -> Self {
        Self { value }
    }
}

impl From<ConvertToPsbtResponse> for String {
    fn from(wrapper: ConvertToPsbtResponse) -> Self {
        wrapper.value
    }
}

/// Response for the `CreateMultisig` RPC method
///
/// Wire method: `createmultisig`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct CreateMultisigResponse {
    /// The value of the new multisig address.
    pub address: String,
    /// The descriptor for this multisig
    pub descriptor: String,
    /// The string value of the hex-encoded redemption script.
    /// Wire JSON key: `redeemScript` (Rust field `redeem_script`).
    #[serde(rename = "redeemScript")]
    pub redeem_script: bitcoin::ScriptBuf,
    /// Any warnings resulting from the creation of this multisig
    #[serde(skip_serializing_if = "Option::is_none")]
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
                Ok(CreatePsbtResponse {
                    value: v.to_string(),
                })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CreatePsbtResponse {
                    value: v.to_string(),
                })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CreatePsbtResponse {
                    value: v.to_string(),
                })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CreatePsbtResponse {
                    value: v.to_string(),
                })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CreatePsbtResponse {
                    value: v.to_string(),
                })
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for CreatePsbtResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<String> for CreatePsbtResponse {
    fn as_ref(&self) -> &String {
        &self.value
    }
}

impl From<String> for CreatePsbtResponse {
    fn from(value: String) -> Self {
        Self { value }
    }
}

impl From<CreatePsbtResponse> for String {
    fn from(wrapper: CreatePsbtResponse) -> Self {
        wrapper.value
    }
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
                Ok(CreateRawTransactionResponse {
                    value: v.to_string(),
                })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CreateRawTransactionResponse {
                    value: v.to_string(),
                })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CreateRawTransactionResponse {
                    value: v.to_string(),
                })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CreateRawTransactionResponse {
                    value: v.to_string(),
                })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CreateRawTransactionResponse {
                    value: v.to_string(),
                })
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for CreateRawTransactionResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<String> for CreateRawTransactionResponse {
    fn as_ref(&self) -> &String {
        &self.value
    }
}

impl From<String> for CreateRawTransactionResponse {
    fn from(value: String) -> Self {
        Self { value }
    }
}

impl From<CreateRawTransactionResponse> for String {
    fn from(wrapper: CreateRawTransactionResponse) -> Self {
        wrapper.value
    }
}

/// Response for the `CreateWallet` RPC method
///
/// Wire method: `createwallet`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct CreateWalletResponse {
    /// The wallet name if created successfully. If the wallet was created using a full path, the wallet_name will be the full path.
    pub name: String,
    /// Warning messages, if any, related to creating and loading the wallet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

/// Response for the `CreateWalletDescriptor` RPC method
///
/// Wire method: `createwalletdescriptor`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct CreateWalletDescriptorResponse {
    /// The public descriptors that were added to the wallet
    pub descs: Vec<String>,
}

/// Response for the `DecodePsbt` RPC method
///
/// Wire method: `decodepsbt`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtResponse {
    /// The locktime to fallback to if no inputs specify a required locktime.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback_locktime: Option<u64>,
    /// The transaction fee paid if all UTXOs slots in the PSBT have been filled.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        serialize_with = "option_amount_to_btc_float",
        deserialize_with = "option_amount_from_btc_float"
    )]
    pub fee: Option<bitcoin::Amount>,
    pub global_xpubs: Vec<DecodePsbtGlobalXpubs>,
    /// Whether this PSBT has SIGHASH_SINGLE inputs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_sighash_single: Option<bool>,
    /// The number of inputs in this psbt
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_count: Option<u64>,
    pub inputs: Vec<DecodePsbtInputs>,
    /// Whether inputs can be modified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs_modifiable: Option<bool>,
    /// The number of outputs in this psbt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_count: Option<u64>,
    pub outputs: Vec<DecodePsbtOutputs>,
    /// Whether outputs can be modified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs_modifiable: Option<bool>,
    /// The global proprietary map
    pub proprietary: Vec<DecodePsbtProprietary>,
    /// The PSBT version number. Not to be confused with the unsigned transaction version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psbt_version: Option<u64>,
    /// The decoded network-serialized unsigned transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx: Option<DecodePsbtTx>,
    /// The version number of the unsigned transaction. Not to be confused with PSBT version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_version: Option<u64>,
    /// The unknown global fields
    pub unknown: BTreeMap<String, String>,
}

/// Response for the `DecodeRawTransaction` RPC method
///
/// Wire method: `decoderawtransaction`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodeRawTransactionResponse {
    /// The transaction hash (differs from txid for witness transactions)
    pub hash: String,
    /// The lock time
    /// Wire JSON key: `locktime` (Rust field `lock_time`).
    #[serde(rename = "locktime")]
    pub lock_time: u64,
    /// The serialized transaction size
    pub size: u64,
    /// The transaction id
    pub txid: bitcoin::Txid,
    /// The version
    pub version: u32,
    pub vin: Vec<DecodeRawTransactionVin>,
    pub vout: Vec<DecodeRawTransactionVout>,
    /// The virtual transaction size (differs from size for witness transactions)
    /// Wire JSON key: `vsize` (Rust field `v_size`).
    #[serde(rename = "vsize")]
    pub v_size: u64,
    /// The transaction's weight (between vsize*4-3 and vsize*4)
    pub weight: u64,
}

/// Response for the `DecodeScript` RPC method
///
/// Wire method: `decodescript`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodeScriptResponse {
    /// The Bitcoin address (only if a well-defined address exists)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Disassembly of the script
    pub asm: String,
    /// Inferred descriptor for the script
    pub desc: String,
    /// address of P2SH script wrapping this redeem script (not returned for types that should not be wrapped)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p2sh: Option<String>,
    /// Result of a witness output script wrapping this redeem script (not returned for types that should not be wrapped)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segwit: Option<DecodeScriptSegwit>,
    /// The output type (e.g. nonstandard, anchor, pubkey, pubkeyhash, scripthash, multisig, nulldata, witness_v0_scripthash, witness_v0_keyhash, witness_v1_taproot, witness_unknown)
    /// Wire JSON key: `type` (Rust field `r#type`).
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Response for the `DeriveAddresses` RPC method
///
/// Wire method: `deriveaddresses`
///
/// | Arm | Rust payload |
/// | --- | --- |
/// | `Array` | `Vec&lt;String&gt;` |
/// | `Array2` | `Vec&lt;Vec&lt;String&gt;&gt;` |
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeriveAddressesResponse {
    Array(Vec<String>),
    Array2(Vec<Vec<String>>),
}

/// Response for the `DeriveHdKey` RPC method
///
/// Wire method: `derivehdkey`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DeriveHdKeyResponse {
    /// Fingerprint and path for use in descriptors
    pub origin: String,
    /// The extended private key if "private" is true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xprv: Option<String>,
    /// The extended public key
    pub xpub: String,
}

/// Response for the `DescriptorProcessPsbt` RPC method
///
/// Wire method: `descriptorprocesspsbt`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DescriptorProcessPsbtResponse {
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// The hex-encoded network transaction if complete
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<String>,
    /// The base64-encoded partially signed transaction
    pub psbt: String,
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for DisconnectNodeResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<()> for DisconnectNodeResponse {
    fn as_ref(&self) -> &() {
        &self.value
    }
}

impl From<()> for DisconnectNodeResponse {
    fn from(value: ()) -> Self {
        Self { value }
    }
}

impl From<DisconnectNodeResponse> for () {
    fn from(wrapper: DisconnectNodeResponse) -> Self {
        wrapper.value
    }
}

/// Response for the `DumpTxOutSet` RPC method
///
/// Wire method: `dumptxoutset`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DumpTxOutSetResponse {
    /// the hash of the base of the snapshot
    pub base_hash: String,
    /// the height of the base of the snapshot
    pub base_height: u64,
    /// the number of coins written in the snapshot
    pub coins_written: u64,
    /// the number of transactions in the chain up to and including the base block
    /// Wire JSON key: `nchaintx` (Rust field `n_chain_tx`).
    #[serde(rename = "nchaintx")]
    pub n_chain_tx: u64,
    /// the absolute path that the snapshot was written to
    pub path: String,
    /// the hash of the UTXO set contents
    pub txoutset_hash: String,
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
    fn from(value: serde_json::Value) -> Self {
        Self { value }
    }
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
                Ok(EchoipcResponse {
                    value: v.to_string(),
                })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(EchoipcResponse {
                    value: v.to_string(),
                })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(EchoipcResponse {
                    value: v.to_string(),
                })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(EchoipcResponse {
                    value: v.to_string(),
                })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(EchoipcResponse {
                    value: v.to_string(),
                })
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for EchoipcResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<String> for EchoipcResponse {
    fn as_ref(&self) -> &String {
        &self.value
    }
}

impl From<String> for EchoipcResponse {
    fn from(value: String) -> Self {
        Self { value }
    }
}

impl From<EchoipcResponse> for String {
    fn from(wrapper: EchoipcResponse) -> Self {
        wrapper.value
    }
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
    fn from(value: serde_json::Value) -> Self {
        Self { value }
    }
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
                Ok(EncryptWalletResponse {
                    value: v.to_string(),
                })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(EncryptWalletResponse {
                    value: v.to_string(),
                })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(EncryptWalletResponse {
                    value: v.to_string(),
                })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(EncryptWalletResponse {
                    value: v.to_string(),
                })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(EncryptWalletResponse {
                    value: v.to_string(),
                })
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for EncryptWalletResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<String> for EncryptWalletResponse {
    fn as_ref(&self) -> &String {
        &self.value
    }
}

impl From<String> for EncryptWalletResponse {
    fn from(value: String) -> Self {
        Self { value }
    }
}

impl From<EncryptWalletResponse> for String {
    fn from(wrapper: EncryptWalletResponse) -> Self {
        wrapper.value
    }
}

/// Response for the `EnumerateSigners` RPC method
///
/// Wire method: `enumeratesigners`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct EnumerateSignersResponse {
    pub signers: Vec<EnumerateSignersSigners>,
}

/// Response for the `EstimateRawFee` RPC method
///
/// Wire method: `estimaterawfee`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct EstimateRawFeeResponse {
    /// estimate for long time horizon
    #[serde(skip_serializing_if = "Option::is_none")]
    pub long: Option<EstimateRawFeeLong>,
    /// estimate for medium time horizon
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium: Option<EstimateRawFeeMedium>,
    /// estimate for short time horizon
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short: Option<EstimateRawFeeShort>,
}

/// Response for the `EstimateSmartFee` RPC method
///
/// Wire method: `estimatesmartfee`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct EstimateSmartFeeResponse {
    /// the confirmation target in blocks for the returned fee rate estimate.
    /// For the block policy fee rate estimator, this is the target the estimate was found at, clamped to at
    /// least 2 and at most the estimator's maximum usable target. For the mempool fee rate
    /// estimator, it is always 2.
    pub blocks: u64,
    /// Errors encountered during processing (if there are any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,
    /// the fee estimator used to produce the result (only present for successful estimates when fee_rate_estimator is "none")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimator: Option<String>,
    /// estimate fee rate in BTC/kvB (only present if no errors were encountered)
    /// Wire JSON key: `feerate` (Rust field `fee_rate`).
    #[serde(rename = "feerate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_rate: Option<f64>,
    /// Health statistics for the most recently mined blocks tracked by the mempool fee rate estimator (only present when verbosity &gt;= 2)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mempool_health_statistics: Option<Vec<EstimateSmartFeeMempoolHealthStatistics>>,
}

/// Response for the `ExportAsMap` RPC method
///
/// Wire method: `exportasmap`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ExportAsMapResponse {
    /// the number of bytes written to the file
    pub bytes_written: u64,
    /// the SHA256 hash of the exported ASMap data
    pub file_hash: String,
    /// the absolute path that the ASMap data was written to
    pub path: String,
}

/// Response for the `ExportWatchOnlyWallet` RPC method
///
/// Wire method: `exportwatchonlywallet`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ExportWatchOnlyWalletResponse {
    /// The full path that the file has been exported to
    pub exported_file: String,
}

/// Response for the `FinalizePsbt` RPC method
///
/// Wire method: `finalizepsbt`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct FinalizePsbtResponse {
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// The hex-encoded network transaction if extracted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<String>,
    /// The base64-encoded partially signed transaction if not extracted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psbt: Option<String>,
}

/// Response for the `FundRawTransaction` RPC method
///
/// Wire method: `fundrawtransaction`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct FundRawTransactionResponse {
    /// The position of the added change output, or -1
    pub changepos: i64,
    /// Fee in BTC the resulting transaction pays
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub fee: bitcoin::Amount,
    /// The resulting raw transaction (hex-encoded string)
    pub hex: String,
}

/// Response for the `Generate` RPC method
///
/// This method returns arbitrary JSON (e.g. string, object, array) as a single value.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GenerateResponse {
    /// Wrapped JSON value
    pub value: serde_json::Value,
}

impl<'de> serde::Deserialize<'de> for GenerateResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<serde_json::Value> for GenerateResponse {
    fn from(value: serde_json::Value) -> Self {
        Self { value }
    }
}

/// Response for the `GenerateBlock` RPC method
///
/// Wire method: `generateblock`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GenerateBlockResponse {
    /// hash of generated block
    pub hash: String,
    /// hex of generated block, only present when submit=false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<String>,
}

/// Response for the `GenerateToAddress` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GenerateToAddressResponse {
    /// Wrapped array value
    pub value: Vec<String>,
}

impl<'de> serde::Deserialize<'de> for GenerateToAddressResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<String>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<String>> for GenerateToAddressResponse {
    fn from(value: Vec<String>) -> Self {
        Self { value }
    }
}

impl From<GenerateToAddressResponse> for Vec<String> {
    fn from(wrapper: GenerateToAddressResponse) -> Self {
        wrapper.value
    }
}

/// Response for the `GenerateToDescriptor` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GenerateToDescriptorResponse {
    /// Wrapped array value
    pub value: Vec<String>,
}

impl<'de> serde::Deserialize<'de> for GenerateToDescriptorResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<String>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<String>> for GenerateToDescriptorResponse {
    fn from(value: Vec<String>) -> Self {
        Self { value }
    }
}

impl From<GenerateToDescriptorResponse> for Vec<String> {
    fn from(wrapper: GenerateToDescriptorResponse) -> Self {
        wrapper.value
    }
}

/// Response for the `GetAddedNodeInfo` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetAddedNodeInfoResponse {
    /// Wrapped array value
    pub value: Vec<GetAddedNodeInfoResultItem>,
}

impl<'de> serde::Deserialize<'de> for GetAddedNodeInfoResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<GetAddedNodeInfoResultItem>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<GetAddedNodeInfoResultItem>> for GetAddedNodeInfoResponse {
    fn from(value: Vec<GetAddedNodeInfoResultItem>) -> Self {
        Self { value }
    }
}

impl From<GetAddedNodeInfoResponse> for Vec<GetAddedNodeInfoResultItem> {
    fn from(wrapper: GetAddedNodeInfoResponse) -> Self {
        wrapper.value
    }
}

/// Response for the `GetAddressesByLabel` RPC method
///
/// This method returns a dynamic-key object wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GetAddressesByLabelResponse(pub BTreeMap<String, GetAddressesByLabelMapValue>);

impl std::ops::Deref for GetAddressesByLabelResponse {
    type Target = BTreeMap<String, GetAddressesByLabelMapValue>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for GetAddressesByLabelResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<BTreeMap<String, GetAddressesByLabelMapValue>> for GetAddressesByLabelResponse {
    fn from(value: BTreeMap<String, GetAddressesByLabelMapValue>) -> Self {
        Self(value)
    }
}

impl From<GetAddressesByLabelResponse> for BTreeMap<String, GetAddressesByLabelMapValue> {
    fn from(wrapper: GetAddressesByLabelResponse) -> Self {
        wrapper.0
    }
}

/// Response for the `GetAddressInfo` RPC method
///
/// Wire method: `getaddressinfo`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetAddressInfoResponse {
    /// The bitcoin address validated.
    pub address: String,
    /// A descriptor for spending coins sent to this address (only when solvable).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    /// Information about the address embedded in P2SH or P2WSH, if relevant and known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedded: Option<GetAddressInfoEmbedded>,
    /// The HD keypath, if the key is HD and available.
    /// Wire JSON key: `hdkeypath` (Rust field `hd_key_path`).
    #[serde(rename = "hdkeypath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hd_key_path: Option<String>,
    /// The fingerprint of the master key.
    /// Wire JSON key: `hdmasterfingerprint` (Rust field `hd_master_fingerprint`).
    #[serde(rename = "hdmasterfingerprint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hd_master_fingerprint: Option<String>,
    /// The Hash160 of the HD seed.
    /// Wire JSON key: `hdseedid` (Rust field `hd_seed_id`).
    #[serde(rename = "hdseedid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hd_seed_id: Option<String>,
    /// The redeemscript for the p2sh address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<String>,
    /// If the address was used for change output.
    pub ischange: bool,
    /// If the pubkey is compressed.
    /// Wire JSON key: `iscompressed` (Rust field `is_compressed`).
    #[serde(rename = "iscompressed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_compressed: Option<bool>,
    /// If the address is yours.
    /// Wire JSON key: `ismine` (Rust field `is_mine`).
    #[serde(rename = "ismine")]
    pub is_mine: bool,
    /// If the key is a script.
    /// Wire JSON key: `isscript` (Rust field `is_script`).
    #[serde(rename = "isscript")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_script: Option<bool>,
    /// (DEPRECATED) Always false.
    /// Wire JSON key: `iswatchonly` (Rust field `is_watch_only`).
    #[serde(rename = "iswatchonly")]
    pub is_watch_only: bool,
    /// If the address is a witness address.
    /// Wire JSON key: `iswitness` (Rust field `is_witness`).
    #[serde(rename = "iswitness")]
    pub is_witness: bool,
    /// Array of labels associated with the address. Currently limited to one label but returned
    /// as an array to keep the API stable if multiple labels are enabled in the future.
    pub labels: Vec<String>,
    /// The descriptor used to derive this address if this is a descriptor wallet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_desc: Option<String>,
    /// The hex value of the raw public key for single-key addresses (possibly embedded in P2SH or P2WSH).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pubkey: Option<String>,
    /// Array of pubkeys associated with the known redeemscript (only if script is multisig).
    /// Wire JSON key: `pubkeys` (Rust field `pub_keys`).
    #[serde(rename = "pubkeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pub_keys: Option<Vec<String>>,
    /// The output script type. Only if isscript is true and the redeemscript is known. Possible
    /// types: nonstandard, pubkey, pubkeyhash, scripthash, multisig, nulldata, witness_v0_keyhash,
    /// witness_v0_scripthash, witness_unknown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<bitcoin::ScriptBuf>,
    /// The hex-encoded output script generated by the address.
    /// Wire JSON key: `scriptPubKey` (Rust field `script_pubkey`).
    #[serde(rename = "scriptPubKey")]
    pub script_pubkey: bitcoin::ScriptBuf,
    /// The number of signatures required to spend multisig output (only if script is multisig).
    /// Wire JSON key: `sigsrequired` (Rust field `sigs_required`).
    #[serde(rename = "sigsrequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sigs_required: Option<u64>,
    /// If we know how to spend coins sent to this address, ignoring the possible lack of private keys.
    pub solvable: bool,
    /// The creation time of the key, if available, expressed in UNIX epoch time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<u64>,
    /// The hex value of the witness program.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_program: Option<String>,
    /// The version number of the witness program.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_version: Option<u64>,
}

/// Response for the `GetAddrManInfo` RPC method
///
/// This method returns a dynamic-key object wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GetAddrManInfoResponse(pub BTreeMap<String, GetAddrManInfoMapValue>);

impl std::ops::Deref for GetAddrManInfoResponse {
    type Target = BTreeMap<String, GetAddrManInfoMapValue>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for GetAddrManInfoResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<BTreeMap<String, GetAddrManInfoMapValue>> for GetAddrManInfoResponse {
    fn from(value: BTreeMap<String, GetAddrManInfoMapValue>) -> Self {
        Self(value)
    }
}

impl From<GetAddrManInfoResponse> for BTreeMap<String, GetAddrManInfoMapValue> {
    fn from(wrapper: GetAddrManInfoResponse) -> Self {
        wrapper.0
    }
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
                Ok(GetBalanceResponse {
                    value: bitcoin::Amount::from_sat(v),
                })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if v < 0 {
                    return Err(de::Error::custom(format!(
                        "Amount cannot be negative: {}",
                        v
                    )));
                }
                Ok(GetBalanceResponse {
                    value: bitcoin::Amount::from_sat(v as u64),
                })
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for GetBalanceResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<bitcoin::Amount> for GetBalanceResponse {
    fn as_ref(&self) -> &bitcoin::Amount {
        &self.value
    }
}

impl From<bitcoin::Amount> for GetBalanceResponse {
    fn from(value: bitcoin::Amount) -> Self {
        Self { value }
    }
}

impl From<GetBalanceResponse> for bitcoin::Amount {
    fn from(wrapper: GetBalanceResponse) -> Self {
        wrapper.value
    }
}

/// Response for the `GetBalances` RPC method
///
/// Wire method: `getbalances`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBalancesResponse {
    /// hash and height of the block this information was generated on
    /// Wire JSON key: `lastprocessedblock` (Rust field `last_processed_block`).
    #[serde(rename = "lastprocessedblock")]
    pub last_processed_block: GetBalancesLastProcessedBlock,
    /// balances from outputs that the wallet can sign
    pub mine: GetBalancesMine,
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
                Ok(GetBestBlockHashResponse {
                    value: v.to_string(),
                })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetBestBlockHashResponse {
                    value: v.to_string(),
                })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetBestBlockHashResponse {
                    value: v.to_string(),
                })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetBestBlockHashResponse {
                    value: v.to_string(),
                })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetBestBlockHashResponse {
                    value: v.to_string(),
                })
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for GetBestBlockHashResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<String> for GetBestBlockHashResponse {
    fn as_ref(&self) -> &String {
        &self.value
    }
}

impl From<String> for GetBestBlockHashResponse {
    fn from(value: String) -> Self {
        Self { value }
    }
}

impl From<GetBestBlockHashResponse> for String {
    fn from(wrapper: GetBestBlockHashResponse) -> Self {
        wrapper.value
    }
}

/// Response for the `GetBlock` RPC method
///
/// Wire method: `getblock`
///
/// | Arm | Rust payload |
/// | --- | --- |
/// | `String` | `String` |
/// | `Object` | `GetBlockResponseGetBlockObject` |
/// | `Object2` | `GetBlockResponseGetBlockObject2` |
/// | `Object3` | `GetBlockResponseGetBlockObject3` |
/// Verbose JSON object (union variant) for `getblock` RPC
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockResponseGetBlockObject {
    /// nBits: compact representation of the block difficulty target
    pub bits: String,
    /// Expected number of hashes required to produce the chain up to this block (in hex)
    /// Wire JSON key: `chainwork` (Rust field `chain_work`).
    #[serde(rename = "chainwork")]
    pub chain_work: String,
    /// Coinbase transaction metadata
    pub coinbase_tx: GetBlockCoinbaseTx,
    /// The number of confirmations, or -1 if the block is not on the main chain
    pub confirmations: i64,
    /// The difficulty
    pub difficulty: f64,
    /// the block hash (same as provided)
    pub hash: String,
    /// The block height or index
    pub height: u64,
    /// The median block time expressed in UNIX epoch time
    /// Wire JSON key: `mediantime` (Rust field `median_time`).
    #[serde(rename = "mediantime")]
    pub median_time: u64,
    /// The merkle root
    /// Wire JSON key: `merkleroot` (Rust field `merkle_root`).
    #[serde(rename = "merkleroot")]
    pub merkle_root: String,
    /// The number of transactions in the block
    /// Wire JSON key: `nTx` (Rust field `n_tx`).
    #[serde(rename = "nTx")]
    pub n_tx: u64,
    /// The hash of the next block (if available)
    /// Wire JSON key: `nextblockhash` (Rust field `next_block_hash`).
    #[serde(rename = "nextblockhash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_block_hash: Option<String>,
    /// The nonce
    pub nonce: u64,
    /// The hash of the previous block (if available)
    /// Wire JSON key: `previousblockhash` (Rust field `previous_block_hash`).
    #[serde(rename = "previousblockhash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_block_hash: Option<String>,
    /// The block size
    pub size: u64,
    /// The block size excluding witness data
    /// Wire JSON key: `strippedsize` (Rust field `stripped_size`).
    #[serde(rename = "strippedsize")]
    pub stripped_size: u64,
    /// The difficulty target
    pub target: String,
    /// The block time expressed in UNIX epoch time
    pub time: u64,
    /// The transaction ids
    pub tx: Vec<String>,
    /// The block version
    pub version: u32,
    /// The block version formatted in hexadecimal
    /// Wire JSON key: `versionHex` (Rust field `version_hex`).
    #[serde(rename = "versionHex")]
    pub version_hex: String,
    /// The block weight as defined in BIP 141
    pub weight: u64,
}

/// Verbose JSON object (union variant) for `getblock` RPC
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockResponseGetBlockObject2 {
    /// nBits: compact representation of the block difficulty target
    pub bits: String,
    /// Expected number of hashes required to produce the chain up to this block (in hex)
    /// Wire JSON key: `chainwork` (Rust field `chain_work`).
    #[serde(rename = "chainwork")]
    pub chain_work: String,
    /// Coinbase transaction metadata
    pub coinbase_tx: GetBlockCoinbaseTx,
    /// The number of confirmations, or -1 if the block is not on the main chain
    pub confirmations: i64,
    /// The difficulty
    pub difficulty: f64,
    /// the block hash (same as provided)
    pub hash: String,
    /// The block height or index
    pub height: u64,
    /// The median block time expressed in UNIX epoch time
    /// Wire JSON key: `mediantime` (Rust field `median_time`).
    #[serde(rename = "mediantime")]
    pub median_time: u64,
    /// The merkle root
    /// Wire JSON key: `merkleroot` (Rust field `merkle_root`).
    #[serde(rename = "merkleroot")]
    pub merkle_root: String,
    /// The number of transactions in the block
    /// Wire JSON key: `nTx` (Rust field `n_tx`).
    #[serde(rename = "nTx")]
    pub n_tx: u64,
    /// The hash of the next block (if available)
    /// Wire JSON key: `nextblockhash` (Rust field `next_block_hash`).
    #[serde(rename = "nextblockhash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_block_hash: Option<String>,
    /// The nonce
    pub nonce: u64,
    /// The hash of the previous block (if available)
    /// Wire JSON key: `previousblockhash` (Rust field `previous_block_hash`).
    #[serde(rename = "previousblockhash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_block_hash: Option<String>,
    /// The block size
    pub size: u64,
    /// The block size excluding witness data
    /// Wire JSON key: `strippedsize` (Rust field `stripped_size`).
    #[serde(rename = "strippedsize")]
    pub stripped_size: u64,
    /// The difficulty target
    pub target: String,
    /// The block time expressed in UNIX epoch time
    pub time: u64,
    pub tx: Vec<GetBlockTx>,
    /// The block version
    pub version: u32,
    /// The block version formatted in hexadecimal
    /// Wire JSON key: `versionHex` (Rust field `version_hex`).
    #[serde(rename = "versionHex")]
    pub version_hex: String,
    /// The block weight as defined in BIP 141
    pub weight: u64,
}

/// Verbose JSON object (union variant) for `getblock` RPC
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockResponseGetBlockObject3 {
    /// nBits: compact representation of the block difficulty target
    pub bits: String,
    /// Expected number of hashes required to produce the chain up to this block (in hex)
    /// Wire JSON key: `chainwork` (Rust field `chain_work`).
    #[serde(rename = "chainwork")]
    pub chain_work: String,
    /// Coinbase transaction metadata
    pub coinbase_tx: GetBlockCoinbaseTx,
    /// The number of confirmations, or -1 if the block is not on the main chain
    pub confirmations: i64,
    /// The difficulty
    pub difficulty: f64,
    /// the block hash (same as provided)
    pub hash: String,
    /// The block height or index
    pub height: u64,
    /// The median block time expressed in UNIX epoch time
    /// Wire JSON key: `mediantime` (Rust field `median_time`).
    #[serde(rename = "mediantime")]
    pub median_time: u64,
    /// The merkle root
    /// Wire JSON key: `merkleroot` (Rust field `merkle_root`).
    #[serde(rename = "merkleroot")]
    pub merkle_root: String,
    /// The number of transactions in the block
    /// Wire JSON key: `nTx` (Rust field `n_tx`).
    #[serde(rename = "nTx")]
    pub n_tx: u64,
    /// The hash of the next block (if available)
    /// Wire JSON key: `nextblockhash` (Rust field `next_block_hash`).
    #[serde(rename = "nextblockhash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_block_hash: Option<String>,
    /// The nonce
    pub nonce: u64,
    /// The hash of the previous block (if available)
    /// Wire JSON key: `previousblockhash` (Rust field `previous_block_hash`).
    #[serde(rename = "previousblockhash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_block_hash: Option<String>,
    /// The block size
    pub size: u64,
    /// The block size excluding witness data
    /// Wire JSON key: `strippedsize` (Rust field `stripped_size`).
    #[serde(rename = "strippedsize")]
    pub stripped_size: u64,
    /// The difficulty target
    pub target: String,
    /// The block time expressed in UNIX epoch time
    pub time: u64,
    pub tx: Vec<GetBlockTx>,
    /// The block version
    pub version: u32,
    /// The block version formatted in hexadecimal
    /// Wire JSON key: `versionHex` (Rust field `version_hex`).
    #[serde(rename = "versionHex")]
    pub version_hex: String,
    /// The block weight as defined in BIP 141
    pub weight: u64,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBlockResponse {
    String(String),
    Object(GetBlockResponseGetBlockObject),
    Object2(GetBlockResponseGetBlockObject2),
    Object3(GetBlockResponseGetBlockObject3),
}

/// Response for the `GetBlockchainInfo` RPC method
///
/// Wire method: `getblockchaininfo`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockchainInfoResponse {
    /// whether automatic pruning is enabled (only present if pruning is enabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_pruning: Option<bool>,
    /// state info regarding background validation process
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backgroundvalidation: Option<GetBlockchainInfoBackgroundvalidation>,
    /// the hash of the currently best block
    /// Wire JSON key: `bestblockhash` (Rust field `best_block_hash`).
    #[serde(rename = "bestblockhash")]
    pub best_block_hash: String,
    /// nBits: compact representation of the block difficulty target
    pub bits: String,
    /// the height of the most-work fully-validated chain. The genesis block has height 0
    pub blocks: u64,
    /// current network name (main, test, testnet4, signet, regtest)
    pub chain: String,
    /// total amount of work in active chain, in hexadecimal
    /// Wire JSON key: `chainwork` (Rust field `chain_work`).
    #[serde(rename = "chainwork")]
    pub chain_work: String,
    /// the current difficulty
    pub difficulty: f64,
    /// the current number of headers we have validated
    pub headers: u64,
    /// (debug information) estimate of whether this node is in Initial Block Download mode
    /// Wire JSON key: `initialblockdownload` (Rust field `initial_block_download`).
    #[serde(rename = "initialblockdownload")]
    pub initial_block_download: bool,
    /// the median block time expressed in UNIX epoch time
    /// Wire JSON key: `mediantime` (Rust field `median_time`).
    #[serde(rename = "mediantime")]
    pub median_time: u64,
    /// the target size used by pruning (only present if automatic pruning is enabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prune_target_size: Option<u64>,
    /// if the blocks are subject to pruning
    pub pruned: bool,
    /// the first block unpruned, all previous blocks were pruned (only present if pruning is enabled)
    /// Wire JSON key: `pruneheight` (Rust field `prune_height`).
    #[serde(rename = "pruneheight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prune_height: Option<u64>,
    /// the block challenge (aka. block script), in hexadecimal (only present if the current network is a signet)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signet_challenge: Option<String>,
    /// the estimated size of the block and undo files on disk
    pub size_on_disk: u64,
    /// the difficulty target
    pub target: String,
    /// the block time expressed in UNIX epoch time
    pub time: u64,
    /// estimate of verification progress \[0..1\]
    /// Wire JSON key: `verificationprogress` (Rust field `verification_progress`).
    #[serde(rename = "verificationprogress")]
    pub verification_progress: f64,
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for GetBlockCountResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<u64> for GetBlockCountResponse {
    fn as_ref(&self) -> &u64 {
        &self.value
    }
}

impl From<u64> for GetBlockCountResponse {
    fn from(value: u64) -> Self {
        Self { value }
    }
}

impl From<GetBlockCountResponse> for u64 {
    fn from(wrapper: GetBlockCountResponse) -> Self {
        wrapper.value
    }
}

/// Response for the `GetBlockFilter` RPC method
///
/// Wire method: `getblockfilter`
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
                Ok(GetBlockHashResponse {
                    value: v.to_string(),
                })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetBlockHashResponse {
                    value: v.to_string(),
                })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetBlockHashResponse {
                    value: v.to_string(),
                })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetBlockHashResponse {
                    value: v.to_string(),
                })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetBlockHashResponse {
                    value: v.to_string(),
                })
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for GetBlockHashResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<String> for GetBlockHashResponse {
    fn as_ref(&self) -> &String {
        &self.value
    }
}

impl From<String> for GetBlockHashResponse {
    fn from(value: String) -> Self {
        Self { value }
    }
}

impl From<GetBlockHashResponse> for String {
    fn from(wrapper: GetBlockHashResponse) -> Self {
        wrapper.value
    }
}

/// Response for the `GetBlockHeader` RPC method
///
/// Wire method: `getblockheader`
///
/// | Arm | Rust payload |
/// | --- | --- |
/// | `Object` | `GetBlockHeaderResponseGetBlockHeaderObject` |
/// | `String` | `String` |
/// Verbose JSON object (union variant) for `getblockheader` RPC
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockHeaderResponseGetBlockHeaderObject {
    /// nBits: compact representation of the block difficulty target
    pub bits: String,
    /// Expected number of hashes required to produce the current chain
    /// Wire JSON key: `chainwork` (Rust field `chain_work`).
    #[serde(rename = "chainwork")]
    pub chain_work: String,
    /// The number of confirmations, or -1 if the block is not on the main chain
    pub confirmations: i64,
    /// The difficulty
    pub difficulty: f64,
    /// the block hash (same as provided)
    pub hash: String,
    /// The block height or index
    pub height: u64,
    /// The median block time expressed in UNIX epoch time
    /// Wire JSON key: `mediantime` (Rust field `median_time`).
    #[serde(rename = "mediantime")]
    pub median_time: u64,
    /// The merkle root
    /// Wire JSON key: `merkleroot` (Rust field `merkle_root`).
    #[serde(rename = "merkleroot")]
    pub merkle_root: String,
    /// The number of transactions in the block
    /// Wire JSON key: `nTx` (Rust field `n_tx`).
    #[serde(rename = "nTx")]
    pub n_tx: u64,
    /// The hash of the next block (if available)
    /// Wire JSON key: `nextblockhash` (Rust field `next_block_hash`).
    #[serde(rename = "nextblockhash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_block_hash: Option<String>,
    /// The nonce
    pub nonce: u64,
    /// The hash of the previous block (if available)
    /// Wire JSON key: `previousblockhash` (Rust field `previous_block_hash`).
    #[serde(rename = "previousblockhash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_block_hash: Option<String>,
    /// The difficulty target
    pub target: String,
    /// The block time expressed in UNIX epoch time
    pub time: u64,
    /// The block version
    pub version: u32,
    /// The block version formatted in hexadecimal
    /// Wire JSON key: `versionHex` (Rust field `version_hex`).
    #[serde(rename = "versionHex")]
    pub version_hex: String,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBlockHeaderResponse {
    String(String),
    Object(GetBlockHeaderResponseGetBlockHeaderObject),
}

/// Response for the `GetBlockStats` RPC method
///
/// Wire method: `getblockstats`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockStatsResponse {
    /// Average fee in the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avgfee: Option<u64>,
    /// Average feerate (in satoshis per virtual byte)
    /// Wire JSON key: `avgfeerate` (Rust field `avg_fee_rate`).
    #[serde(rename = "avgfeerate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_fee_rate: Option<u64>,
    /// Average transaction size
    /// Wire JSON key: `avgtxsize` (Rust field `avg_tx_size`).
    #[serde(rename = "avgtxsize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_tx_size: Option<u64>,
    /// The block hash (to check for potential reorgs)
    /// Wire JSON key: `blockhash` (Rust field `block_hash`).
    #[serde(rename = "blockhash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_hash: Option<bitcoin::BlockHash>,
    /// Feerates at the 10th, 25th, 50th, 75th, and 90th percentile weight unit (in satoshis per virtual byte)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feerate_percentiles: Option<Vec<u64>>,
    /// The height of the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u64>,
    /// The number of inputs (excluding coinbase)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ins: Option<u64>,
    /// Maximum fee in the block
    /// Wire JSON key: `maxfee` (Rust field `max_fee`).
    #[serde(rename = "maxfee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_fee: Option<u64>,
    /// Maximum feerate (in satoshis per virtual byte)
    /// Wire JSON key: `maxfeerate` (Rust field `max_fee_rate`).
    #[serde(rename = "maxfeerate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_fee_rate: Option<f64>,
    /// Maximum transaction size
    /// Wire JSON key: `maxtxsize` (Rust field `max_tx_size`).
    #[serde(rename = "maxtxsize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tx_size: Option<u64>,
    /// Truncated median fee in the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medianfee: Option<u64>,
    /// The block median time past
    /// Wire JSON key: `mediantime` (Rust field `median_time`).
    #[serde(rename = "mediantime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub median_time: Option<u64>,
    /// Truncated median transaction size
    /// Wire JSON key: `mediantxsize` (Rust field `median_tx_size`).
    #[serde(rename = "mediantxsize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub median_tx_size: Option<u64>,
    /// Minimum fee in the block
    /// Wire JSON key: `minfee` (Rust field `min_fee`).
    #[serde(rename = "minfee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_fee: Option<u64>,
    /// Minimum feerate (in satoshis per virtual byte)
    /// Wire JSON key: `minfeerate` (Rust field `min_fee_rate`).
    #[serde(rename = "minfeerate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_fee_rate: Option<u64>,
    /// Minimum transaction size
    /// Wire JSON key: `mintxsize` (Rust field `min_tx_size`).
    #[serde(rename = "mintxsize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_tx_size: Option<u64>,
    /// The number of outputs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outs: Option<u64>,
    /// The block subsidy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subsidy: Option<u64>,
    /// Total size of all segwit transactions
    /// Wire JSON key: `swtotal_size` (Rust field `sw_total_size`).
    #[serde(rename = "swtotal_size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sw_total_size: Option<u64>,
    /// Total weight of all segwit transactions
    /// Wire JSON key: `swtotal_weight` (Rust field `sw_total_weight`).
    #[serde(rename = "swtotal_weight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sw_total_weight: Option<u64>,
    /// The number of segwit transactions
    /// Wire JSON key: `swtxs` (Rust field `sw_txs`).
    #[serde(rename = "swtxs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sw_txs: Option<u64>,
    /// The block time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<u64>,
    /// Total amount in all outputs (excluding coinbase and thus reward \[ie subsidy + totalfee\])
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_out: Option<u64>,
    /// Total size of all non-coinbase transactions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_size: Option<u64>,
    /// Total weight of all non-coinbase transactions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_weight: Option<u64>,
    /// The fee total
    /// Wire JSON key: `totalfee` (Rust field `total_fee`).
    #[serde(rename = "totalfee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_fee: Option<u64>,
    /// The number of transactions (including coinbase)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txs: Option<u64>,
    /// The increase/decrease in the number of unspent outputs (not discounting op_return and similar)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utxo_increase: Option<u64>,
    /// The increase/decrease in the number of unspent outputs, not counting unspendables
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utxo_increase_actual: Option<u64>,
    /// The increase/decrease in size for the utxo index (not discounting op_return and similar)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utxo_size_inc: Option<u64>,
    /// The increase/decrease in size for the utxo index, not counting unspendables
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utxo_size_inc_actual: Option<u64>,
}

/// Response for the `GetBlockTemplate` RPC method
///
/// Wire method: `getblocktemplate`
///
/// | Arm | Rust payload |
/// | --- | --- |
/// | `Null` | `()` |
/// | `String` | `String` |
/// | `Object` | `GetBlockTemplateResponseGetBlockTemplateObject` |
/// Verbose JSON object (union variant) for `getblocktemplate` RPC
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockTemplateResponseGetBlockTemplateObject {
    /// compressed target of next block
    pub bits: String,
    pub capabilities: Vec<String>,
    /// data that should be included in the coinbase's scriptSig content
    /// Wire JSON key: `coinbaseaux` (Rust field `coinbase_aux`).
    #[serde(rename = "coinbaseaux")]
    pub coinbase_aux: BTreeMap<String, String>,
    /// maximum allowable input to coinbase transaction, including the generation award and transaction fees (in satoshis)
    /// Wire JSON key: `coinbasevalue` (Rust field `coinbase_value`).
    #[serde(rename = "coinbasevalue")]
    pub coinbase_value: u64,
    /// current timestamp in UNIX epoch time. Adjusted for the proposed BIP94 timewarp rule.
    /// Wire JSON key: `curtime` (Rust field `cur_time`).
    #[serde(rename = "curtime")]
    pub cur_time: u64,
    /// a valid witness commitment for the unmodified block template
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_witness_commitment: Option<String>,
    /// The height of the next block
    pub height: u64,
    /// an id to include with a request to longpoll on an update to this template
    /// Wire JSON key: `longpollid` (Rust field `longpoll_id`).
    #[serde(rename = "longpollid")]
    pub longpoll_id: String,
    /// The minimum timestamp appropriate for the next block time, expressed in UNIX epoch time. Adjusted for the proposed BIP94 timewarp rule.
    pub mintime: u64,
    /// list of ways the block template may be changed
    pub mutable: Vec<String>,
    /// A range of valid nonces
    /// Wire JSON key: `noncerange` (Rust field `nonce_range`).
    #[serde(rename = "noncerange")]
    pub nonce_range: String,
    /// The hash of current highest block
    /// Wire JSON key: `previousblockhash` (Rust field `previous_block_hash`).
    #[serde(rename = "previousblockhash")]
    pub previous_block_hash: String,
    /// specific block rules that are to be enforced
    pub rules: Vec<String>,
    /// Only on signet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signet_challenge: Option<String>,
    /// limit of sigops in blocks
    /// Wire JSON key: `sigoplimit` (Rust field `sigop_limit`).
    #[serde(rename = "sigoplimit")]
    pub sigop_limit: u64,
    /// limit of block size
    /// Wire JSON key: `sizelimit` (Rust field `size_limit`).
    #[serde(rename = "sizelimit")]
    pub size_limit: u64,
    /// The hash target
    pub target: String,
    /// contents of non-coinbase transactions that should be included in the next block
    pub transactions: Vec<GetBlockTemplateTransactions>,
    /// set of pending, supported versionbit (BIP 9) softfork deployments
    /// Wire JSON key: `vbavailable` (Rust field `vb_available`).
    #[serde(rename = "vbavailable")]
    pub vb_available: BTreeMap<String, u64>,
    /// bit mask of versionbits the server requires set in submissions
    /// Wire JSON key: `vbrequired` (Rust field `vb_required`).
    #[serde(rename = "vbrequired")]
    pub vb_required: u64,
    /// The preferred block version
    pub version: u32,
    /// limit of block weight
    /// Wire JSON key: `weightlimit` (Rust field `weight_limit`).
    #[serde(rename = "weightlimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight_limit: Option<u64>,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBlockTemplateResponse {
    Null(()),
    String(String),
    Object(GetBlockTemplateResponseGetBlockTemplateObject),
}

/// Response for the `GetChainStates` RPC method
///
/// Wire method: `getchainstates`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetChainStatesResponse {
    /// list of the chainstates ordered by work, with the most-work (active) chainstate last
    pub chainstates: Vec<GetChainStatesChainstates>,
    /// the number of headers seen so far
    pub headers: u64,
}

/// Response for the `GetChainTips` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetChainTipsResponse {
    /// Wrapped array value
    pub value: Vec<GetChainTipsResultItem>,
}

impl<'de> serde::Deserialize<'de> for GetChainTipsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<GetChainTipsResultItem>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<GetChainTipsResultItem>> for GetChainTipsResponse {
    fn from(value: Vec<GetChainTipsResultItem>) -> Self {
        Self { value }
    }
}

impl From<GetChainTipsResponse> for Vec<GetChainTipsResultItem> {
    fn from(wrapper: GetChainTipsResponse) -> Self {
        wrapper.value
    }
}

/// Response for the `GetChainTxStats` RPC method
///
/// Wire method: `getchaintxstats`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetChainTxStatsResponse {
    /// The timestamp for the final block in the window, expressed in UNIX epoch time
    pub time: u64,
    /// The total number of transactions in the chain up to that point, if known. It may be unknown when using assumeutxo.
    /// Wire JSON key: `txcount` (Rust field `tx_count`).
    #[serde(rename = "txcount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_count: Option<u64>,
    /// The average rate of transactions per second in the window. Only returned if "window_interval" is &gt; 0 and if window_tx_count exists.
    /// Wire JSON key: `txrate` (Rust field `tx_rate`).
    #[serde(rename = "txrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_rate: Option<u64>,
    /// Size of the window in number of blocks
    pub window_block_count: u64,
    /// The hash of the final block in the window
    pub window_final_block_hash: String,
    /// The height of the final block in the window.
    pub window_final_block_height: u64,
    /// The elapsed time in the window in seconds. Only returned if "window_block_count" is &gt; 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_interval: Option<u64>,
    /// The number of transactions in the window. Only returned if "window_block_count" is &gt; 0 and if txcount exists for the start and end of the window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_tx_count: Option<u64>,
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for GetConnectionCountResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<u64> for GetConnectionCountResponse {
    fn as_ref(&self) -> &u64 {
        &self.value
    }
}

impl From<u64> for GetConnectionCountResponse {
    fn from(value: u64) -> Self {
        Self { value }
    }
}

impl From<GetConnectionCountResponse> for u64 {
    fn from(wrapper: GetConnectionCountResponse) -> Self {
        wrapper.value
    }
}

/// Response for the `GetDeploymentInfo` RPC method
///
/// Wire method: `getdeploymentinfo`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetDeploymentInfoResponse {
    pub deployments: BTreeMap<String, GetDeploymentInfoMapValue>,
    /// requested block hash (or tip)
    pub hash: String,
    /// requested block height (or tip)
    pub height: u64,
    /// script verify flags for the block
    pub script_flags: Vec<String>,
}

/// Response for the `GetDescriptorActivity` RPC method
///
/// Wire method: `getdescriptoractivity`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetDescriptorActivityResponse {
    /// events
    pub activity: Vec<GetDescriptorActivityResultActivityItem>,
}

/// Response for the `GetDescriptorInfo` RPC method
///
/// Wire method: `getdescriptorinfo`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetDescriptorInfoResponse {
    /// The checksum for the input descriptor
    pub checksum: String,
    /// The descriptor, without private keys. For a multipath descriptor, only the first will be returned.
    pub descriptor: String,
    /// Whether the input descriptor contained at least one private key
    /// Wire JSON key: `hasprivatekeys` (Rust field `has_private_keys`).
    #[serde(rename = "hasprivatekeys")]
    pub has_private_keys: bool,
    /// Whether the descriptor is ranged
    /// Wire JSON key: `isrange` (Rust field `is_range`).
    #[serde(rename = "isrange")]
    pub is_range: bool,
    /// Whether the descriptor is solvable
    /// Wire JSON key: `issolvable` (Rust field `is_solvable`).
    #[serde(rename = "issolvable")]
    pub is_solvable: bool,
    /// All descriptors produced by expanding multipath derivation elements. Only if the provided descriptor specifies multipath derivation elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multipath_expansion: Option<Vec<String>>,
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for GetDifficultyResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<u64> for GetDifficultyResponse {
    fn as_ref(&self) -> &u64 {
        &self.value
    }
}

impl From<u64> for GetDifficultyResponse {
    fn from(value: u64) -> Self {
        Self { value }
    }
}

impl From<GetDifficultyResponse> for u64 {
    fn from(wrapper: GetDifficultyResponse) -> Self {
        wrapper.value
    }
}

/// Response for the `GetHdKeys` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetHdKeysResponse {
    /// Wrapped array value
    pub value: Vec<GetHdKeysResultItem>,
}

impl<'de> serde::Deserialize<'de> for GetHdKeysResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<GetHdKeysResultItem>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<GetHdKeysResultItem>> for GetHdKeysResponse {
    fn from(value: Vec<GetHdKeysResultItem>) -> Self {
        Self { value }
    }
}

impl From<GetHdKeysResponse> for Vec<GetHdKeysResultItem> {
    fn from(wrapper: GetHdKeysResponse) -> Self {
        wrapper.value
    }
}

/// Response for the `GetIndexInfo` RPC method
///
/// This method returns a dynamic-key object wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GetIndexInfoResponse(pub BTreeMap<String, GetIndexInfoMapValue>);

impl std::ops::Deref for GetIndexInfoResponse {
    type Target = BTreeMap<String, GetIndexInfoMapValue>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for GetIndexInfoResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<BTreeMap<String, GetIndexInfoMapValue>> for GetIndexInfoResponse {
    fn from(value: BTreeMap<String, GetIndexInfoMapValue>) -> Self {
        Self(value)
    }
}

impl From<GetIndexInfoResponse> for BTreeMap<String, GetIndexInfoMapValue> {
    fn from(wrapper: GetIndexInfoResponse) -> Self {
        wrapper.0
    }
}

/// Response for the `GetMemoryInfo` RPC method
///
/// Wire method: `getmemoryinfo`
///
/// | Arm | Rust payload |
/// | --- | --- |
/// | `Object` | `GetMemoryInfoResponseGetMemoryInfoObject` |
/// | `String` | `String` |
/// Verbose JSON object (union variant) for `getmemoryinfo` RPC
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMemoryInfoResponseGetMemoryInfoObject {
    /// Information about locked memory manager
    pub locked: GetMemoryInfoLocked,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMemoryInfoResponse {
    String(String),
    Object(GetMemoryInfoResponseGetMemoryInfoObject),
}

/// Response for the `GetMempoolAncestors` RPC method
///
/// Wire method: `getmempoolancestors`
///
/// | Arm | Rust payload |
/// | --- | --- |
/// | `Array` | `Vec&lt;String&gt;` |
/// | `Object` | `BTreeMap&lt;String, GetMempoolAncestorsMapValue&gt;` |
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMempoolAncestorsResponse {
    Array(Vec<String>),
    Object(BTreeMap<String, GetMempoolAncestorsMapValue>),
}

/// Response for the `GetMempoolCluster` RPC method
///
/// Wire method: `getmempoolcluster`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolClusterResponse {
    /// chunks in this cluster (in mining order)
    pub chunks: Vec<GetMempoolClusterChunks>,
    /// total sigops-adjusted weight (as defined in BIP 141 and modified by '-bytespersigop')
    pub clusterweight: u64,
    /// number of transactions
    /// Wire JSON key: `txcount` (Rust field `tx_count`).
    #[serde(rename = "txcount")]
    pub tx_count: u64,
}

/// Response for the `GetMempoolDescendants` RPC method
///
/// Wire method: `getmempooldescendants`
///
/// | Arm | Rust payload |
/// | --- | --- |
/// | `Array` | `Vec&lt;String&gt;` |
/// | `Object` | `BTreeMap&lt;String, GetMempoolDescendantsMapValue&gt;` |
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMempoolDescendantsResponse {
    Array(Vec<String>),
    Object(BTreeMap<String, GetMempoolDescendantsMapValue>),
}

/// Response for the `GetMempoolEntry` RPC method
///
/// Wire method: `getmempoolentry`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolEntryResponse {
    /// number of in-mempool ancestor transactions (including this one)
    /// Wire JSON key: `ancestorcount` (Rust field `ancestor_count`).
    #[serde(rename = "ancestorcount")]
    pub ancestor_count: u64,
    /// virtual transaction size of in-mempool ancestors (including this one)
    /// Wire JSON key: `ancestorsize` (Rust field `ancestor_size`).
    #[serde(rename = "ancestorsize")]
    pub ancestor_size: u64,
    /// sigops-adjusted weight (as defined in BIP 141 and modified by '-bytespersigop') of this transaction's chunk
    /// Wire JSON key: `chunkweight` (Rust field `chunk_weight`).
    #[serde(rename = "chunkweight")]
    pub chunk_weight: u64,
    /// unconfirmed transactions used as inputs for this transaction
    pub depends: Vec<String>,
    /// number of in-mempool descendant transactions (including this one)
    /// Wire JSON key: `descendantcount` (Rust field `descendant_count`).
    #[serde(rename = "descendantcount")]
    pub descendant_count: u64,
    /// virtual transaction size of in-mempool descendants (including this one)
    /// Wire JSON key: `descendantsize` (Rust field `descendant_size`).
    #[serde(rename = "descendantsize")]
    pub descendant_size: u64,
    pub fees: GetMempoolEntryFees,
    /// block height when transaction entered pool
    pub height: u64,
    /// unconfirmed transactions spending outputs from this transaction
    /// Wire JSON key: `spentby` (Rust field `spent_by`).
    #[serde(rename = "spentby")]
    pub spent_by: Vec<String>,
    /// local time transaction entered pool in seconds since 1 Jan 1970 GMT
    pub time: u64,
    /// Whether this transaction is currently unbroadcast (initial broadcast not yet acknowledged by any peers)
    pub unbroadcast: bool,
    /// (DEPRECATED) Was previously erroneously described as the BIP 141 vsize, but is actually sigops-adjusted vsize.
    /// Use vsize_bip141 to actually get that behavior or switch to the explicit vsize_adjusted for retained behavior.
    /// Wire JSON key: `vsize` (Rust field `v_size`).
    #[serde(rename = "vsize")]
    pub v_size: u64,
    /// Maximum of sigop-adjusted size (-bytespersigop) and virtual transaction size as defined in BIP 141.
    pub vsize_adjusted: u64,
    /// Virtual transaction size as defined in BIP 141.
    /// This is different from actual serialized size for witness transactions as witness data is discounted.
    pub vsize_bip141: u64,
    /// transaction weight as defined in BIP 141.
    pub weight: u64,
    /// hash of serialized transaction, including witness data
    /// Wire JSON key: `wtxid` (Rust field `w_txid`).
    #[serde(rename = "wtxid")]
    pub w_txid: String,
}

/// Response for the `GetMempoolFeeRateDiagram` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetMempoolFeeRateDiagramResponse {
    /// Wrapped array value
    pub value: Vec<GetMempoolFeeRateDiagramResultItem>,
}

impl<'de> serde::Deserialize<'de> for GetMempoolFeeRateDiagramResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<GetMempoolFeeRateDiagramResultItem>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<GetMempoolFeeRateDiagramResultItem>> for GetMempoolFeeRateDiagramResponse {
    fn from(value: Vec<GetMempoolFeeRateDiagramResultItem>) -> Self {
        Self { value }
    }
}

impl From<GetMempoolFeeRateDiagramResponse> for Vec<GetMempoolFeeRateDiagramResultItem> {
    fn from(wrapper: GetMempoolFeeRateDiagramResponse) -> Self {
        wrapper.value
    }
}

/// Response for the `GetMempoolInfo` RPC method
///
/// Wire method: `getmempoolinfo`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolInfoResponse {
    /// Sum of all virtual transaction sizes as defined in BIP 141. Differs from actual serialized size because witness data is discounted
    pub bytes: u64,
    /// minimum fee rate increment for mempool limiting or replacement in BTC/kvB
    /// Wire JSON key: `incrementalrelayfee` (Rust field `incremental_relay_fee`).
    #[serde(rename = "incrementalrelayfee")]
    pub incremental_relay_fee: f64,
    /// Maximum number of transactions that can be in a cluster (configured by -limitclustercount)
    /// Wire JSON key: `limitclustercount` (Rust field `limit_cluster_count`).
    #[serde(rename = "limitclustercount")]
    pub limit_cluster_count: u64,
    /// Maximum size of a cluster in virtual bytes (configured by -limitclustersize)
    /// Wire JSON key: `limitclustersize` (Rust field `limit_cluster_size`).
    #[serde(rename = "limitclustersize")]
    pub limit_cluster_size: u64,
    /// True if the initial load attempt of the persisted mempool finished
    pub loaded: bool,
    /// Maximum number of bytes that can be used by OP_RETURN outputs in the mempool
    /// Wire JSON key: `maxdatacarriersize` (Rust field `max_data_carrier_size`).
    #[serde(rename = "maxdatacarriersize")]
    pub max_data_carrier_size: u64,
    /// Maximum memory usage for the mempool
    /// Wire JSON key: `maxmempool` (Rust field `max_mempool`).
    #[serde(rename = "maxmempool")]
    pub max_mempool: u64,
    /// Minimum fee rate in BTC/kvB for tx to be accepted. Is the maximum of minrelaytxfee and minimum mempool fee
    /// Wire JSON key: `mempoolminfee` (Rust field `mempool_min_fee`).
    #[serde(rename = "mempoolminfee")]
    pub mempool_min_fee: f64,
    /// Current minimum relay fee for transactions
    /// Wire JSON key: `minrelaytxfee` (Rust field `min_relay_tx_fee`).
    #[serde(rename = "minrelaytxfee")]
    pub min_relay_tx_fee: f64,
    /// If the mempool is in a known-optimal transaction ordering
    pub optimal: bool,
    /// True if the mempool accepts transactions with bare multisig outputs
    /// Wire JSON key: `permitbaremultisig` (Rust field `permit_bare_multisig`).
    #[serde(rename = "permitbaremultisig")]
    pub permit_bare_multisig: bool,
    /// Current tx count
    pub size: u64,
    /// Total fees for the mempool in BTC, ignoring modified fees through prioritisetransaction
    pub total_fee: f64,
    /// Current number of transactions that haven't passed initial broadcast yet
    /// Wire JSON key: `unbroadcastcount` (Rust field `unbroadcast_count`).
    #[serde(rename = "unbroadcastcount")]
    pub unbroadcast_count: u64,
    /// Total memory usage for the mempool
    pub usage: u64,
}

/// Response for the `GetMiningInfo` RPC method
///
/// Wire method: `getmininginfo`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMiningInfoResponse {
    /// The hash of the current best block
    /// Wire JSON key: `bestblockhash` (Rust field `best_block_hash`).
    #[serde(rename = "bestblockhash")]
    pub best_block_hash: String,
    /// The current nBits, compact representation of the block difficulty target
    pub bits: String,
    /// Minimum feerate of packages selected for block inclusion in BTC/kvB
    /// Wire JSON key: `blockmintxfee` (Rust field `block_min_tx_fee`).
    #[serde(rename = "blockmintxfee")]
    pub block_min_tx_fee: f64,
    /// The current block
    pub blocks: u64,
    /// current network name (main, test, testnet4, signet, regtest)
    pub chain: String,
    /// The number of block transactions (excluding coinbase) of the last assembled block (only present if a block was ever assembled)
    /// Wire JSON key: `currentblocktx` (Rust field `current_block_tx`).
    #[serde(rename = "currentblocktx")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_block_tx: Option<u64>,
    /// The block weight (including reserved weight for block header, txs count and coinbase tx) of the last assembled block (only present if a block was ever assembled)
    /// Wire JSON key: `currentblockweight` (Rust field `current_block_weight`).
    #[serde(rename = "currentblockweight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_block_weight: Option<u64>,
    /// The current difficulty
    pub difficulty: f64,
    /// The network hashes per second
    /// Wire JSON key: `networkhashps` (Rust field `network_hashps`).
    #[serde(rename = "networkhashps")]
    pub network_hashps: f64,
    /// The next block
    pub next: GetMiningInfoNext,
    /// The size of the mempool
    /// Wire JSON key: `pooledtx` (Rust field `pooled_tx`).
    #[serde(rename = "pooledtx")]
    pub pooled_tx: u64,
    /// The block challenge (aka. block script), in hexadecimal (only present if the current network is a signet)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signet_challenge: Option<String>,
    /// The current target
    pub target: String,
    /// any network and blockchain warnings (run with `-deprecatedrpc=warnings` to return the latest warning as a single string)
    pub warnings: Vec<String>,
}

/// Response for the `GetNetTotals` RPC method
///
/// Wire method: `getnettotals`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetNetTotalsResponse {
    /// Current system UNIX epoch time in milliseconds
    /// Wire JSON key: `timemillis` (Rust field `time_millis`).
    #[serde(rename = "timemillis")]
    pub time_millis: u64,
    /// Total bytes received
    /// Wire JSON key: `totalbytesrecv` (Rust field `total_bytes_recv`).
    #[serde(rename = "totalbytesrecv")]
    pub total_bytes_recv: u64,
    /// Total bytes sent
    /// Wire JSON key: `totalbytessent` (Rust field `total_bytes_sent`).
    #[serde(rename = "totalbytessent")]
    pub total_bytes_sent: u64,
    /// Wire JSON key: `uploadtarget` (Rust field `upload_target`).
    #[serde(rename = "uploadtarget")]
    pub upload_target: GetNetTotalsUploadtarget,
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for GetNetworkHashPsResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<u64> for GetNetworkHashPsResponse {
    fn as_ref(&self) -> &u64 {
        &self.value
    }
}

impl From<u64> for GetNetworkHashPsResponse {
    fn from(value: u64) -> Self {
        Self { value }
    }
}

impl From<GetNetworkHashPsResponse> for u64 {
    fn from(wrapper: GetNetworkHashPsResponse) -> Self {
        wrapper.value
    }
}

/// Response for the `GetNetworkInfo` RPC method
///
/// Wire method: `getnetworkinfo`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetNetworkInfoResponse {
    /// the SHA256 hash of the asmap data used for IP bucketing (only displayed if the -asmap config option is set)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asmap_version: Option<String>,
    /// the total number of connections
    pub connections: u64,
    /// the number of inbound connections
    pub connections_in: u64,
    /// the number of outbound connections
    pub connections_out: u64,
    /// minimum fee rate increment for mempool limiting or replacement in BTC/kvB
    /// Wire JSON key: `incrementalfee` (Rust field `incremental_fee`).
    #[serde(rename = "incrementalfee")]
    pub incremental_fee: f64,
    pub inv_buckets: BTreeMap<String, GetNetworkInfoMapValue>,
    /// list of local addresses
    /// Wire JSON key: `localaddresses` (Rust field `local_addresses`).
    #[serde(rename = "localaddresses")]
    pub local_addresses: Vec<GetNetworkInfoLocaladdresses>,
    /// true if transaction relay is requested from peers
    /// Wire JSON key: `localrelay` (Rust field `local_relay`).
    #[serde(rename = "localrelay")]
    pub local_relay: bool,
    /// the services we offer to the network
    /// Wire JSON key: `localservices` (Rust field `local_services`).
    #[serde(rename = "localservices")]
    pub local_services: String,
    /// the services we offer to the network, in human-readable form
    /// Wire JSON key: `localservicesnames` (Rust field `local_services_names`).
    #[serde(rename = "localservicesnames")]
    pub local_services_names: Vec<String>,
    /// whether p2p networking is enabled
    /// Wire JSON key: `networkactive` (Rust field `network_active`).
    #[serde(rename = "networkactive")]
    pub network_active: bool,
    /// information per network
    pub networks: Vec<GetNetworkInfoNetworks>,
    /// the protocol version
    /// Wire JSON key: `protocolversion` (Rust field `protocol_version`).
    #[serde(rename = "protocolversion")]
    pub protocol_version: u64,
    /// minimum relay fee rate for transactions in BTC/kvB
    /// Wire JSON key: `relayfee` (Rust field `relay_fee`).
    #[serde(rename = "relayfee")]
    pub relay_fee: f64,
    /// the server subversion string
    pub subversion: String,
    /// the time offset
    /// Wire JSON key: `timeoffset` (Rust field `time_offset`).
    #[serde(rename = "timeoffset")]
    pub time_offset: u64,
    /// configured target for maximum number of transactions per second to send to inbound peers
    pub tx_send_rate: u64,
    /// the server version
    pub version: u32,
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
                Ok(GetNewAddressResponse {
                    value: v.to_string(),
                })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetNewAddressResponse {
                    value: v.to_string(),
                })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetNewAddressResponse {
                    value: v.to_string(),
                })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetNewAddressResponse {
                    value: v.to_string(),
                })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetNewAddressResponse {
                    value: v.to_string(),
                })
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for GetNewAddressResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<String> for GetNewAddressResponse {
    fn as_ref(&self) -> &String {
        &self.value
    }
}

impl From<String> for GetNewAddressResponse {
    fn from(value: String) -> Self {
        Self { value }
    }
}

impl From<GetNewAddressResponse> for String {
    fn from(wrapper: GetNewAddressResponse) -> Self {
        wrapper.value
    }
}

/// Response for the `GetNodeAddresses` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetNodeAddressesResponse {
    /// Wrapped array value
    pub value: Vec<GetNodeAddressesResultItem>,
}

impl<'de> serde::Deserialize<'de> for GetNodeAddressesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<GetNodeAddressesResultItem>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<GetNodeAddressesResultItem>> for GetNodeAddressesResponse {
    fn from(value: Vec<GetNodeAddressesResultItem>) -> Self {
        Self { value }
    }
}

impl From<GetNodeAddressesResponse> for Vec<GetNodeAddressesResultItem> {
    fn from(wrapper: GetNodeAddressesResponse) -> Self {
        wrapper.value
    }
}

/// Response for the `GetOpenRpcInfo` RPC method
///
/// Wire method: `getopenrpcinfo`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetOpenRpcInfoResponse {
    /// Metadata about this JSON-RPC interface.
    pub info: GetOpenRpcInfoInfo,
    /// Documented RPC methods.
    pub methods: Vec<GetOpenRpcInfoMethods>,
    /// OpenRPC specification version.
    pub openrpc: String,
}

/// Response for the `GetOrphanTxs` RPC method
///
/// Wire method: `getorphantxs`
///
/// | Arm | Rust payload |
/// | --- | --- |
/// | `Array` | `Vec&lt;String&gt;` |
/// | `Array2` | `Vec&lt;GetOrphanTxsResponseGetOrphanTxsArray2&gt;` |
/// | `Array3` | `Vec&lt;GetOrphanTxsResponseGetOrphanTxsArray3&gt;` |
/// Verbose JSON object (union variant) for `getorphantxs` RPC
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetOrphanTxsResponseGetOrphanTxsArray2 {
    /// The serialized transaction size in bytes
    pub bytes: u64,
    pub from: Vec<u64>,
    /// The transaction hash in hex
    pub txid: bitcoin::Txid,
    /// (DEPRECATED) use vsize_bip141 instead. The virtual transaction size as defined in BIP 141. This is different from actual serialized size for witness transactions as witness data is discounted.
    /// Wire JSON key: `vsize` (Rust field `v_size`).
    #[serde(rename = "vsize")]
    pub v_size: u64,
    /// The virtual transaction size as defined in BIP 141. This is different from actual serialized size for witness transactions as witness data is discounted.
    pub vsize_bip141: u64,
    /// The transaction weight as defined in BIP 141.
    pub weight: u64,
    /// The transaction witness hash in hex
    /// Wire JSON key: `wtxid` (Rust field `w_txid`).
    #[serde(rename = "wtxid")]
    pub w_txid: String,
}

/// Verbose JSON object (union variant) for `getorphantxs` RPC
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetOrphanTxsResponseGetOrphanTxsArray3 {
    /// The serialized transaction size in bytes
    pub bytes: u64,
    pub from: Vec<u64>,
    /// The serialized, hex-encoded transaction data
    pub hex: String,
    /// The transaction hash in hex
    pub txid: bitcoin::Txid,
    /// (DEPRECATED) use vsize_bip141 instead. The virtual transaction size as defined in BIP 141. This is different from actual serialized size for witness transactions as witness data is discounted.
    /// Wire JSON key: `vsize` (Rust field `v_size`).
    #[serde(rename = "vsize")]
    pub v_size: u64,
    /// The virtual transaction size as defined in BIP 141. This is different from actual serialized size for witness transactions as witness data is discounted.
    pub vsize_bip141: u64,
    /// The transaction weight as defined in BIP 141.
    pub weight: u64,
    /// The transaction witness hash in hex
    /// Wire JSON key: `wtxid` (Rust field `w_txid`).
    #[serde(rename = "wtxid")]
    pub w_txid: String,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOrphanTxsResponse {
    Array(Vec<String>),
    Array2(Vec<GetOrphanTxsResponseGetOrphanTxsArray2>),
    Array3(Vec<GetOrphanTxsResponseGetOrphanTxsArray3>),
}

/// Response for the `GetPeerInfo` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetPeerInfoResponse {
    /// Wrapped array value
    pub value: Vec<GetPeerInfoResultItem>,
}

impl<'de> serde::Deserialize<'de> for GetPeerInfoResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<GetPeerInfoResultItem>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<GetPeerInfoResultItem>> for GetPeerInfoResponse {
    fn from(value: Vec<GetPeerInfoResultItem>) -> Self {
        Self { value }
    }
}

impl From<GetPeerInfoResponse> for Vec<GetPeerInfoResultItem> {
    fn from(wrapper: GetPeerInfoResponse) -> Self {
        wrapper.value
    }
}

/// Response for the `GetPrioritisedTransactions` RPC method
///
/// This method returns a dynamic-key object wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GetPrioritisedTransactionsResponse(
    pub BTreeMap<String, GetPrioritisedTransactionsMapValue>,
);

impl std::ops::Deref for GetPrioritisedTransactionsResponse {
    type Target = BTreeMap<String, GetPrioritisedTransactionsMapValue>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for GetPrioritisedTransactionsResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<BTreeMap<String, GetPrioritisedTransactionsMapValue>>
    for GetPrioritisedTransactionsResponse
{
    fn from(value: BTreeMap<String, GetPrioritisedTransactionsMapValue>) -> Self {
        Self(value)
    }
}

impl From<GetPrioritisedTransactionsResponse>
    for BTreeMap<String, GetPrioritisedTransactionsMapValue>
{
    fn from(wrapper: GetPrioritisedTransactionsResponse) -> Self {
        wrapper.0
    }
}

/// Response for the `GetPrivateBroadcastInfo` RPC method
///
/// Wire method: `getprivatebroadcastinfo`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetPrivateBroadcastInfoResponse {
    pub transactions: Vec<GetPrivateBroadcastInfoTransactions>,
}

/// Response for the `GetRawAddrMan` RPC method
///
/// This method returns a dynamic-key object wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GetRawAddrManResponse(pub BTreeMap<String, BTreeMap<String, GetRawAddrManMapValue>>);

impl std::ops::Deref for GetRawAddrManResponse {
    type Target = BTreeMap<String, BTreeMap<String, GetRawAddrManMapValue>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for GetRawAddrManResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<BTreeMap<String, BTreeMap<String, GetRawAddrManMapValue>>> for GetRawAddrManResponse {
    fn from(value: BTreeMap<String, BTreeMap<String, GetRawAddrManMapValue>>) -> Self {
        Self(value)
    }
}

impl From<GetRawAddrManResponse> for BTreeMap<String, BTreeMap<String, GetRawAddrManMapValue>> {
    fn from(wrapper: GetRawAddrManResponse) -> Self {
        wrapper.0
    }
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
                Ok(GetRawChangeAddressResponse {
                    value: v.to_string(),
                })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetRawChangeAddressResponse {
                    value: v.to_string(),
                })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetRawChangeAddressResponse {
                    value: v.to_string(),
                })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetRawChangeAddressResponse {
                    value: v.to_string(),
                })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetRawChangeAddressResponse {
                    value: v.to_string(),
                })
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for GetRawChangeAddressResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<String> for GetRawChangeAddressResponse {
    fn as_ref(&self) -> &String {
        &self.value
    }
}

impl From<String> for GetRawChangeAddressResponse {
    fn from(value: String) -> Self {
        Self { value }
    }
}

impl From<GetRawChangeAddressResponse> for String {
    fn from(wrapper: GetRawChangeAddressResponse) -> Self {
        wrapper.value
    }
}

/// Response for the `GetRawMempool` RPC method
///
/// Wire method: `getrawmempool`
///
/// | Arm | Rust payload |
/// | --- | --- |
/// | `Array` | `Vec&lt;String&gt;` |
/// | `Object` | `BTreeMap&lt;String, GetRawMempoolMapValue&gt;` |
/// | `Object2` | `GetRawMempoolResponseGetRawMempoolObject2` |
/// Verbose JSON object (union variant) for `getrawmempool` RPC
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawMempoolResponseGetRawMempoolObject2 {
    /// The mempool sequence value.
    pub mempool_sequence: u64,
    pub txids: Vec<String>,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRawMempoolResponse {
    Array(Vec<String>),
    Object(BTreeMap<String, GetRawMempoolMapValue>),
    Object2(GetRawMempoolResponseGetRawMempoolObject2),
}

/// Response for the `GetRawTransaction` RPC method
///
/// Wire method: `getrawtransaction`
///
/// | Arm | Rust payload |
/// | --- | --- |
/// | `String` | `String` |
/// | `Object` | `GetRawTransactionResponseGetRawTransactionObject` |
/// | `Object2` | `GetRawTransactionResponseGetRawTransactionObject2` |
/// Verbose JSON object (union variant) for `getrawtransaction` RPC
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawTransactionResponseGetRawTransactionObject {
    /// the block hash
    /// Wire JSON key: `blockhash` (Rust field `block_hash`).
    #[serde(rename = "blockhash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_hash: Option<bitcoin::BlockHash>,
    /// The block time expressed in UNIX epoch time
    /// Wire JSON key: `blocktime` (Rust field `block_time`).
    #[serde(rename = "blocktime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_time: Option<u64>,
    /// The confirmations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmations: Option<i64>,
    /// The transaction hash (differs from txid for witness transactions)
    pub hash: String,
    /// The serialized, hex-encoded data for 'txid'
    pub hex: String,
    /// Whether specified block is in the active chain or not (only present with explicit "blockhash" argument)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_active_chain: Option<bool>,
    /// The lock time
    /// Wire JSON key: `locktime` (Rust field `lock_time`).
    #[serde(rename = "locktime")]
    pub lock_time: u64,
    /// The serialized transaction size
    pub size: u64,
    /// Same as "blocktime"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<u64>,
    /// The transaction id (same as provided)
    pub txid: bitcoin::Txid,
    /// The version
    pub version: u32,
    pub vin: Vec<GetRawTransactionVin>,
    pub vout: Vec<GetRawTransactionVout>,
    /// The virtual transaction size (differs from size for witness transactions)
    /// Wire JSON key: `vsize` (Rust field `v_size`).
    #[serde(rename = "vsize")]
    pub v_size: u64,
    /// Sigop-adjusted virtual size in bytes, present for mempool transactions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vsize_adjusted: Option<u64>,
    /// The transaction's weight (between vsize*4-3 and vsize*4)
    pub weight: u64,
}

/// Verbose JSON object (union variant) for `getrawtransaction` RPC
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawTransactionResponseGetRawTransactionObject2 {
    /// the block hash
    /// Wire JSON key: `blockhash` (Rust field `block_hash`).
    #[serde(rename = "blockhash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_hash: Option<bitcoin::BlockHash>,
    /// The block time expressed in UNIX epoch time
    /// Wire JSON key: `blocktime` (Rust field `block_time`).
    #[serde(rename = "blocktime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_time: Option<u64>,
    /// The confirmations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmations: Option<i64>,
    /// transaction fee in BTC, omitted if block undo data is not available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<f64>,
    /// The transaction hash (differs from txid for witness transactions)
    pub hash: String,
    /// The serialized, hex-encoded data for 'txid'
    pub hex: String,
    /// Whether specified block is in the active chain or not (only present with explicit "blockhash" argument)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_active_chain: Option<bool>,
    /// The lock time
    /// Wire JSON key: `locktime` (Rust field `lock_time`).
    #[serde(rename = "locktime")]
    pub lock_time: u64,
    /// The serialized transaction size
    pub size: u64,
    /// Same as "blocktime"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<u64>,
    /// The transaction id
    pub txid: bitcoin::Txid,
    /// The version
    pub version: u32,
    pub vin: Vec<GetRawTransactionVin>,
    pub vout: Vec<GetRawTransactionVout>,
    /// The virtual transaction size (differs from size for witness transactions)
    /// Wire JSON key: `vsize` (Rust field `v_size`).
    #[serde(rename = "vsize")]
    pub v_size: u64,
    /// Sigop-adjusted virtual size in bytes, present for mempool transactions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vsize_adjusted: Option<u64>,
    /// The transaction's weight (between vsize*4-3 and vsize*4)
    pub weight: u64,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRawTransactionResponse {
    String(String),
    Object(GetRawTransactionResponseGetRawTransactionObject),
    Object2(GetRawTransactionResponseGetRawTransactionObject2),
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
                Ok(GetReceivedByAddressResponse {
                    value: bitcoin::Amount::from_sat(v),
                })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if v < 0 {
                    return Err(de::Error::custom(format!(
                        "Amount cannot be negative: {}",
                        v
                    )));
                }
                Ok(GetReceivedByAddressResponse {
                    value: bitcoin::Amount::from_sat(v as u64),
                })
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for GetReceivedByAddressResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<bitcoin::Amount> for GetReceivedByAddressResponse {
    fn as_ref(&self) -> &bitcoin::Amount {
        &self.value
    }
}

impl From<bitcoin::Amount> for GetReceivedByAddressResponse {
    fn from(value: bitcoin::Amount) -> Self {
        Self { value }
    }
}

impl From<GetReceivedByAddressResponse> for bitcoin::Amount {
    fn from(wrapper: GetReceivedByAddressResponse) -> Self {
        wrapper.value
    }
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
                Ok(GetReceivedByLabelResponse {
                    value: bitcoin::Amount::from_sat(v),
                })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if v < 0 {
                    return Err(de::Error::custom(format!(
                        "Amount cannot be negative: {}",
                        v
                    )));
                }
                Ok(GetReceivedByLabelResponse {
                    value: bitcoin::Amount::from_sat(v as u64),
                })
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for GetReceivedByLabelResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<bitcoin::Amount> for GetReceivedByLabelResponse {
    fn as_ref(&self) -> &bitcoin::Amount {
        &self.value
    }
}

impl From<bitcoin::Amount> for GetReceivedByLabelResponse {
    fn from(value: bitcoin::Amount) -> Self {
        Self { value }
    }
}

impl From<GetReceivedByLabelResponse> for bitcoin::Amount {
    fn from(wrapper: GetReceivedByLabelResponse) -> Self {
        wrapper.value
    }
}

/// Response for the `GetRpcInfo` RPC method
///
/// Wire method: `getrpcinfo`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRpcInfoResponse {
    /// All active commands
    pub active_commands: Vec<GetRpcInfoActiveCommands>,
    /// The complete file path to the debug log
    pub logpath: String,
}

/// Response for the `GetTransaction` RPC method
///
/// Wire method: `gettransaction`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTransactionResponse {
    /// The wtxids of transactions with different witness data but the same txid.
    pub alternate_wtxids: Vec<String>,
    /// The amount in BTC
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub amount: bitcoin::Amount,
    /// ("yes|no|unknown") (DEPRECATED) Whether this transaction signals BIP125 replaceability or has an unconfirmed ancestor signaling BIP125 replaceability.
    /// May be unknown for unconfirmed transactions not in the mempool because their unconfirmed ancestors are unknown.
    /// Wire JSON key: `bip125-replaceable` (Rust field `bip125_replaceable`).
    #[serde(rename = "bip125-replaceable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bip125_replaceable: Option<String>,
    /// The block hash containing the transaction.
    /// Wire JSON key: `blockhash` (Rust field `block_hash`).
    #[serde(rename = "blockhash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_hash: Option<bitcoin::BlockHash>,
    /// The block height containing the transaction.
    /// Wire JSON key: `blockheight` (Rust field `block_height`).
    #[serde(rename = "blockheight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_height: Option<u64>,
    /// The index of the transaction in the block that includes it.
    /// Wire JSON key: `blockindex` (Rust field `block_index`).
    #[serde(rename = "blockindex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_index: Option<u64>,
    /// The block time expressed in UNIX epoch time.
    /// Wire JSON key: `blocktime` (Rust field `block_time`).
    #[serde(rename = "blocktime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_time: Option<u64>,
    /// If a comment is associated with the transaction, only present if not empty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// The number of confirmations for the transaction. Negative confirmations means the
    /// transaction conflicted that many blocks ago.
    pub confirmations: i64,
    /// The decoded transaction (only present when `verbose` is passed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decoded: Option<GetTransactionDecoded>,
    pub details: Vec<GetTransactionDetails>,
    /// The amount of the fee in BTC. This is negative and only available for the
    /// 'send' category of transactions.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        serialize_with = "option_amount_to_btc_float",
        deserialize_with = "option_amount_from_btc_float"
    )]
    pub fee: Option<bitcoin::Amount>,
    /// Only present if the transaction's only input is a coinbase one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated: Option<bool>,
    /// Raw data for transaction
    pub hex: String,
    /// hash and height of the block this information was generated on
    /// Wire JSON key: `lastprocessedblock` (Rust field `last_processed_block`).
    #[serde(rename = "lastprocessedblock")]
    pub last_processed_block: GetTransactionLastProcessedBlock,
    /// Transactions in the mempool that directly conflict with either this transaction or an ancestor transaction
    /// Wire JSON key: `mempoolconflicts` (Rust field `mempool_conflicts`).
    #[serde(rename = "mempoolconflicts")]
    pub mempool_conflicts: Vec<String>,
    /// Only if 'category' is 'receive'. List of parent descriptors for the output script of this coin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_descs: Option<Vec<String>>,
    /// Only if 'category' is 'send'. The txid if this tx was replaced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replaced_by_txid: Option<String>,
    /// Only if 'category' is 'send'. The txid if this tx replaces another.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replaces_txid: Option<String>,
    /// The transaction time expressed in UNIX epoch time.
    pub time: u64,
    /// The time received expressed in UNIX epoch time.
    /// Wire JSON key: `timereceived` (Rust field `time_received`).
    #[serde(rename = "timereceived")]
    pub time_received: u64,
    /// If a comment to is associated with the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    /// Whether we consider the transaction to be trusted and safe to spend from.
    /// Only present when the transaction has 0 confirmations (or negative confirmations, if conflicted).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted: Option<bool>,
    /// The transaction id.
    pub txid: bitcoin::Txid,
    /// Confirmed transactions that have been detected by the wallet to conflict with this transaction.
    /// Wire JSON key: `walletconflicts` (Rust field `wallet_conflicts`).
    #[serde(rename = "walletconflicts")]
    pub wallet_conflicts: Vec<String>,
    /// The hash of serialized transaction, including witness data.
    /// Wire JSON key: `wtxid` (Rust field `w_txid`).
    #[serde(rename = "wtxid")]
    pub w_txid: String,
}

/// Response for the `GetTxOut` RPC method
///
/// Wire method: `gettxout`
/// Result shape: lookup-or-null. JSON `null` is `None` on the client return type `Option&lt;Self&gt;`.
///
/// | Arm | Rust |
/// | --- | --- |
/// | Null | `None` |
/// | Object | \[`GetTxOutResponse`\] |
///
/// Otherwise
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTxOutResponse {
    /// The hash of the block at the tip of the chain
    /// Wire JSON key: `bestblock` (Rust field `best_block`).
    #[serde(rename = "bestblock")]
    pub best_block: String,
    /// Coinbase or not
    pub coinbase: bool,
    /// The number of confirmations
    pub confirmations: i64,
    /// Wire JSON key: `scriptPubKey` (Rust field `script_pubkey`).
    #[serde(rename = "scriptPubKey")]
    pub script_pubkey: GetTxOutScriptPubKey,
    /// The transaction value in BTC
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub value: bitcoin::Amount,
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
                Ok(GetTxOutProofResponse {
                    value: v.to_string(),
                })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetTxOutProofResponse {
                    value: v.to_string(),
                })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetTxOutProofResponse {
                    value: v.to_string(),
                })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetTxOutProofResponse {
                    value: v.to_string(),
                })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetTxOutProofResponse {
                    value: v.to_string(),
                })
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for GetTxOutProofResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<String> for GetTxOutProofResponse {
    fn as_ref(&self) -> &String {
        &self.value
    }
}

impl From<String> for GetTxOutProofResponse {
    fn from(value: String) -> Self {
        Self { value }
    }
}

impl From<GetTxOutProofResponse> for String {
    fn from(wrapper: GetTxOutProofResponse) -> Self {
        wrapper.value
    }
}

/// Response for the `GetTxOutSetInfo` RPC method
///
/// Wire method: `gettxoutsetinfo`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTxOutSetInfoResponse {
    /// The hash of the block at which these statistics are calculated
    /// Wire JSON key: `bestblock` (Rust field `best_block`).
    #[serde(rename = "bestblock")]
    pub best_block: String,
    /// Info on amounts in the block at this block height (only available if coinstatsindex is used)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_info: Option<GetTxOutSetInfoBlockInfo>,
    /// Database-independent, meaningless metric indicating the UTXO set size
    /// Wire JSON key: `bogosize` (Rust field `bogo_size`).
    #[serde(rename = "bogosize")]
    pub bogo_size: u64,
    /// The estimated size of the chainstate on disk (not available when coinstatsindex is used)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_size: Option<u64>,
    /// The serialized hash (only present if 'hash_serialized_3' hash_type is chosen)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash_serialized_3: Option<String>,
    /// The block height (index) of the returned statistics
    pub height: u64,
    /// The serialized hash (only present if 'muhash' hash_type is chosen)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub muhash: Option<String>,
    /// The total amount of coins in the UTXO set
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub total_amount: bitcoin::Amount,
    /// The total amount of coins permanently excluded from the UTXO set (only available if coinstatsindex is used)
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        serialize_with = "option_amount_to_btc_float",
        deserialize_with = "option_amount_from_btc_float"
    )]
    pub total_unspendable_amount: Option<bitcoin::Amount>,
    /// The number of transactions with unspent outputs (not available when coinstatsindex is used)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactions: Option<u64>,
    /// The number of unspent transaction outputs
    pub txouts: u64,
}

/// Response for the `GetTxSpendingPrevOut` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetTxSpendingPrevOutResponse {
    /// Wrapped array value
    pub value: Vec<GetTxSpendingPrevOutResultItem>,
}

impl<'de> serde::Deserialize<'de> for GetTxSpendingPrevOutResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<GetTxSpendingPrevOutResultItem>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<GetTxSpendingPrevOutResultItem>> for GetTxSpendingPrevOutResponse {
    fn from(value: Vec<GetTxSpendingPrevOutResultItem>) -> Self {
        Self { value }
    }
}

impl From<GetTxSpendingPrevOutResponse> for Vec<GetTxSpendingPrevOutResultItem> {
    fn from(wrapper: GetTxSpendingPrevOutResponse) -> Self {
        wrapper.value
    }
}

/// Response for the `GetWalletInfo` RPC method
///
/// Wire method: `getwalletinfo`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetWalletInfoResponse {
    /// whether this wallet tracks clean/dirty coins in terms of reuse
    pub avoid_reuse: bool,
    /// The start time for blocks scanning. It could be modified by (re)importing any descriptor with an earlier timestamp.
    /// Wire JSON key: `birthtime` (Rust field `birth_time`).
    #[serde(rename = "birthtime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_time: Option<u64>,
    /// Whether this wallet intentionally does not contain any keys, scripts, or descriptors
    pub blank: bool,
    /// whether this wallet uses descriptors for output script management
    pub descriptors: bool,
    /// whether this wallet is configured to use an external signer such as a hardware wallet
    pub external_signer: bool,
    /// The flags currently set on the wallet
    pub flags: Vec<String>,
    /// the database format (only sqlite)
    pub format: String,
    /// how many new keys are pre-generated (only counts external keys)
    /// Wire JSON key: `keypoolsize` (Rust field `key_pool_size`).
    #[serde(rename = "keypoolsize")]
    pub key_pool_size: u64,
    /// how many new keys are pre-generated for internal use (used for change outputs; 0 if external keys are used for change)
    pub keypoolsize_hd_internal: u64,
    /// hash and height of the block this information was generated on
    /// Wire JSON key: `lastprocessedblock` (Rust field `last_processed_block`).
    #[serde(rename = "lastprocessedblock")]
    pub last_processed_block: GetWalletInfoLastProcessedBlock,
    /// false if privatekeys are disabled for this wallet (enforced watch-only wallet)
    pub private_keys_enabled: bool,
    /// current scanning details, or false if no scan is in progress
    pub scanning: GetWalletInfoResultScanning,
    /// the total number of transactions in the wallet
    /// Wire JSON key: `txcount` (Rust field `tx_count`).
    #[serde(rename = "txcount")]
    pub tx_count: u64,
    /// the UNIX epoch time until which the wallet is unlocked for transfers, or 0 if the wallet is locked (only present for passphrase-encrypted wallets)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlocked_until: Option<u64>,
    /// the wallet name
    /// Wire JSON key: `walletname` (Rust field `wallet_name`).
    #[serde(rename = "walletname")]
    pub wallet_name: String,
    /// (DEPRECATED) only related to unsupported legacy wallet, returns the latest version 169900 for backwards compatibility
    /// Wire JSON key: `walletversion` (Rust field `wallet_version`).
    #[serde(rename = "walletversion")]
    pub wallet_version: u64,
}

/// Response for the `GetZmqNotifications` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetZmqNotificationsResponse {
    /// Wrapped array value
    pub value: Vec<GetZmqNotificationsResultItem>,
}

impl<'de> serde::Deserialize<'de> for GetZmqNotificationsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<GetZmqNotificationsResultItem>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<GetZmqNotificationsResultItem>> for GetZmqNotificationsResponse {
    fn from(value: Vec<GetZmqNotificationsResultItem>) -> Self {
        Self { value }
    }
}

impl From<GetZmqNotificationsResponse> for Vec<GetZmqNotificationsResultItem> {
    fn from(wrapper: GetZmqNotificationsResponse) -> Self {
        wrapper.value
    }
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
                Ok(HelpResponse {
                    value: v.to_string(),
                })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(HelpResponse {
                    value: v.to_string(),
                })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(HelpResponse {
                    value: v.to_string(),
                })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(HelpResponse {
                    value: v.to_string(),
                })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(HelpResponse {
                    value: v.to_string(),
                })
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for HelpResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<String> for HelpResponse {
    fn as_ref(&self) -> &String {
        &self.value
    }
}

impl From<String> for HelpResponse {
    fn from(value: String) -> Self {
        Self { value }
    }
}

impl From<HelpResponse> for String {
    fn from(wrapper: HelpResponse) -> Self {
        wrapper.value
    }
}

/// Response for the `ImportDescriptors` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ImportDescriptorsResponse {
    /// Wrapped array value
    pub value: Vec<ImportDescriptorsResultItem>,
}

impl<'de> serde::Deserialize<'de> for ImportDescriptorsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<ImportDescriptorsResultItem>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<ImportDescriptorsResultItem>> for ImportDescriptorsResponse {
    fn from(value: Vec<ImportDescriptorsResultItem>) -> Self {
        Self { value }
    }
}

impl From<ImportDescriptorsResponse> for Vec<ImportDescriptorsResultItem> {
    fn from(wrapper: ImportDescriptorsResponse) -> Self {
        wrapper.value
    }
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for ImportPrunedFundsResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<()> for ImportPrunedFundsResponse {
    fn as_ref(&self) -> &() {
        &self.value
    }
}

impl From<()> for ImportPrunedFundsResponse {
    fn from(value: ()) -> Self {
        Self { value }
    }
}

impl From<ImportPrunedFundsResponse> for () {
    fn from(wrapper: ImportPrunedFundsResponse) -> Self {
        wrapper.value
    }
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for InvalidateBlockResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<()> for InvalidateBlockResponse {
    fn as_ref(&self) -> &() {
        &self.value
    }
}

impl From<()> for InvalidateBlockResponse {
    fn from(value: ()) -> Self {
        Self { value }
    }
}

impl From<InvalidateBlockResponse> for () {
    fn from(wrapper: InvalidateBlockResponse) -> Self {
        wrapper.value
    }
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
                Ok(JoinPsbtsResponse {
                    value: v.to_string(),
                })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(JoinPsbtsResponse {
                    value: v.to_string(),
                })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(JoinPsbtsResponse {
                    value: v.to_string(),
                })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(JoinPsbtsResponse {
                    value: v.to_string(),
                })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(JoinPsbtsResponse {
                    value: v.to_string(),
                })
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for JoinPsbtsResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<String> for JoinPsbtsResponse {
    fn as_ref(&self) -> &String {
        &self.value
    }
}

impl From<String> for JoinPsbtsResponse {
    fn from(value: String) -> Self {
        Self { value }
    }
}

impl From<JoinPsbtsResponse> for String {
    fn from(wrapper: JoinPsbtsResponse) -> Self {
        wrapper.value
    }
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for KeypoolRefillResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<()> for KeypoolRefillResponse {
    fn as_ref(&self) -> &() {
        &self.value
    }
}

impl From<()> for KeypoolRefillResponse {
    fn from(value: ()) -> Self {
        Self { value }
    }
}

impl From<KeypoolRefillResponse> for () {
    fn from(wrapper: KeypoolRefillResponse) -> Self {
        wrapper.value
    }
}

/// Response for the `ListAddressGroupings` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ListAddressGroupingsResponse {
    /// Wrapped array value
    pub value: Vec<Vec<Vec<serde_json::Value>>>,
}

impl<'de> serde::Deserialize<'de> for ListAddressGroupingsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<Vec<Vec<serde_json::Value>>>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<Vec<Vec<serde_json::Value>>>> for ListAddressGroupingsResponse {
    fn from(value: Vec<Vec<Vec<serde_json::Value>>>) -> Self {
        Self { value }
    }
}

impl From<ListAddressGroupingsResponse> for Vec<Vec<Vec<serde_json::Value>>> {
    fn from(wrapper: ListAddressGroupingsResponse) -> Self {
        wrapper.value
    }
}

/// Response for the `ListBanned` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ListBannedResponse {
    /// Wrapped array value
    pub value: Vec<ListBannedResultItem>,
}

impl<'de> serde::Deserialize<'de> for ListBannedResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<ListBannedResultItem>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<ListBannedResultItem>> for ListBannedResponse {
    fn from(value: Vec<ListBannedResultItem>) -> Self {
        Self { value }
    }
}

impl From<ListBannedResponse> for Vec<ListBannedResultItem> {
    fn from(wrapper: ListBannedResponse) -> Self {
        wrapper.value
    }
}

/// Response for the `ListDescriptors` RPC method
///
/// Wire method: `listdescriptors`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ListDescriptorsResponse {
    /// Array of descriptor objects (sorted by descriptor string representation)
    pub descriptors: Vec<ListDescriptorsDescriptors>,
    /// Name of wallet this operation was performed on
    pub wallet_name: String,
}

/// Response for the `ListLabels` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ListLabelsResponse {
    /// Wrapped array value
    pub value: Vec<String>,
}

impl<'de> serde::Deserialize<'de> for ListLabelsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<String>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<String>> for ListLabelsResponse {
    fn from(value: Vec<String>) -> Self {
        Self { value }
    }
}

impl From<ListLabelsResponse> for Vec<String> {
    fn from(wrapper: ListLabelsResponse) -> Self {
        wrapper.value
    }
}

/// Response for the `ListLockUnspent` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ListLockUnspentResponse {
    /// Wrapped array value
    pub value: Vec<ListLockUnspentResultItem>,
}

impl<'de> serde::Deserialize<'de> for ListLockUnspentResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<ListLockUnspentResultItem>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<ListLockUnspentResultItem>> for ListLockUnspentResponse {
    fn from(value: Vec<ListLockUnspentResultItem>) -> Self {
        Self { value }
    }
}

impl From<ListLockUnspentResponse> for Vec<ListLockUnspentResultItem> {
    fn from(wrapper: ListLockUnspentResponse) -> Self {
        wrapper.value
    }
}

/// Response for the `ListReceivedByAddress` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ListReceivedByAddressResponse {
    /// Wrapped array value
    pub value: Vec<ListReceivedByAddressResultItem>,
}

impl<'de> serde::Deserialize<'de> for ListReceivedByAddressResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<ListReceivedByAddressResultItem>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<ListReceivedByAddressResultItem>> for ListReceivedByAddressResponse {
    fn from(value: Vec<ListReceivedByAddressResultItem>) -> Self {
        Self { value }
    }
}

impl From<ListReceivedByAddressResponse> for Vec<ListReceivedByAddressResultItem> {
    fn from(wrapper: ListReceivedByAddressResponse) -> Self {
        wrapper.value
    }
}

/// Response for the `ListReceivedByLabel` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ListReceivedByLabelResponse {
    /// Wrapped array value
    pub value: Vec<ListReceivedByLabelResultItem>,
}

impl<'de> serde::Deserialize<'de> for ListReceivedByLabelResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<ListReceivedByLabelResultItem>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<ListReceivedByLabelResultItem>> for ListReceivedByLabelResponse {
    fn from(value: Vec<ListReceivedByLabelResultItem>) -> Self {
        Self { value }
    }
}

impl From<ListReceivedByLabelResponse> for Vec<ListReceivedByLabelResultItem> {
    fn from(wrapper: ListReceivedByLabelResponse) -> Self {
        wrapper.value
    }
}

/// Response for the `ListSinceBlock` RPC method
///
/// Wire method: `listsinceblock`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ListSinceBlockResponse {
    /// The hash of the block (target_confirmations-1) from the best block on the main chain, or the genesis hash if the referenced block does not exist yet. This is typically used to feed back into listsinceblock the next time you call it. So you would generally use a target_confirmations of say 6, so you will be continually re-notified of transactions until they've reached 6 confirmations plus any new ones
    /// Wire JSON key: `lastblock` (Rust field `last_block`).
    #[serde(rename = "lastblock")]
    pub last_block: String,
    /// &lt;structure is the same as "transactions" above, only present if include_removed=true&gt;
    /// Note: transactions that were re-added in the active chain will appear as-is in this array, and may thus have a positive confirmation count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed: Option<Vec<ListSinceBlockRemoved>>,
    pub transactions: Vec<ListSinceBlockTransactions>,
}

/// Response for the `ListTransactions` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ListTransactionsResponse {
    /// Wrapped array value
    pub value: Vec<ListTransactionsResultItem>,
}

impl<'de> serde::Deserialize<'de> for ListTransactionsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<ListTransactionsResultItem>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<ListTransactionsResultItem>> for ListTransactionsResponse {
    fn from(value: Vec<ListTransactionsResultItem>) -> Self {
        Self { value }
    }
}

impl From<ListTransactionsResponse> for Vec<ListTransactionsResultItem> {
    fn from(wrapper: ListTransactionsResponse) -> Self {
        wrapper.value
    }
}

/// Response for the `ListUnspent` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ListUnspentResponse {
    /// Wrapped array value
    pub value: Vec<ListUnspentResultItem>,
}

impl<'de> serde::Deserialize<'de> for ListUnspentResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<ListUnspentResultItem>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<ListUnspentResultItem>> for ListUnspentResponse {
    fn from(value: Vec<ListUnspentResultItem>) -> Self {
        Self { value }
    }
}

impl From<ListUnspentResponse> for Vec<ListUnspentResultItem> {
    fn from(wrapper: ListUnspentResponse) -> Self {
        wrapper.value
    }
}

/// Response for the `ListWalletDir` RPC method
///
/// Wire method: `listwalletdir`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ListWalletDirResponse {
    pub wallets: Vec<ListWalletDirWallets>,
}

/// Response for the `ListWallets` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ListWalletsResponse {
    /// Wrapped array value
    pub value: Vec<String>,
}

impl<'de> serde::Deserialize<'de> for ListWalletsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<String>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<String>> for ListWalletsResponse {
    fn from(value: Vec<String>) -> Self {
        Self { value }
    }
}

impl From<ListWalletsResponse> for Vec<String> {
    fn from(wrapper: ListWalletsResponse) -> Self {
        wrapper.value
    }
}

/// Response for the `LoadTxOutSet` RPC method
///
/// Wire method: `loadtxoutset`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct LoadTxOutSetResponse {
    /// the height of the base of the snapshot
    pub base_height: u64,
    /// the number of coins loaded from the snapshot
    pub coins_loaded: u64,
    /// the absolute path that the snapshot was loaded from
    pub path: String,
    /// the hash of the base of the snapshot
    pub tip_hash: String,
}

/// Response for the `LoadWallet` RPC method
///
/// Wire method: `loadwallet`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct LoadWalletResponse {
    /// The wallet name if loaded successfully.
    pub name: String,
    /// Warning messages, if any, related to loading the wallet.
    #[serde(skip_serializing_if = "Option::is_none")]
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for LockUnspentResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<bool> for LockUnspentResponse {
    fn as_ref(&self) -> &bool {
        &self.value
    }
}

impl From<bool> for LockUnspentResponse {
    fn from(value: bool) -> Self {
        Self { value }
    }
}

impl From<LockUnspentResponse> for bool {
    fn from(wrapper: LockUnspentResponse) -> Self {
        wrapper.value
    }
}

/// Response for the `Logging` RPC method
///
/// This method returns a dynamic-key object wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LoggingResponse(pub BTreeMap<String, bool>);

impl std::ops::Deref for LoggingResponse {
    type Target = BTreeMap<String, bool>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for LoggingResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<BTreeMap<String, bool>> for LoggingResponse {
    fn from(value: BTreeMap<String, bool>) -> Self {
        Self(value)
    }
}

impl From<LoggingResponse> for BTreeMap<String, bool> {
    fn from(wrapper: LoggingResponse) -> Self {
        wrapper.0
    }
}

/// Response for the `MigrateWallet` RPC method
///
/// Wire method: `migratewallet`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct MigrateWalletResponse {
    /// The location of the backup of the original wallet
    pub backup_path: String,
    /// The name of the migrated wallet containing solvable but not watched scripts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solvables_name: Option<String>,
    /// The name of the primary migrated wallet
    pub wallet_name: String,
    /// The name of the migrated wallet containing the watchonly scripts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watchonly_name: Option<String>,
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for MockSchedulerResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<()> for MockSchedulerResponse {
    fn as_ref(&self) -> &() {
        &self.value
    }
}

impl From<()> for MockSchedulerResponse {
    fn from(value: ()) -> Self {
        Self { value }
    }
}

impl From<MockSchedulerResponse> for () {
    fn from(wrapper: MockSchedulerResponse) -> Self {
        wrapper.value
    }
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for PingResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<()> for PingResponse {
    fn as_ref(&self) -> &() {
        &self.value
    }
}

impl From<()> for PingResponse {
    fn from(value: ()) -> Self {
        Self { value }
    }
}

impl From<PingResponse> for () {
    fn from(wrapper: PingResponse) -> Self {
        wrapper.value
    }
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for PreciousBlockResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<()> for PreciousBlockResponse {
    fn as_ref(&self) -> &() {
        &self.value
    }
}

impl From<()> for PreciousBlockResponse {
    fn from(value: ()) -> Self {
        Self { value }
    }
}

impl From<PreciousBlockResponse> for () {
    fn from(wrapper: PreciousBlockResponse) -> Self {
        wrapper.value
    }
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for PrioritiseTransactionResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<bool> for PrioritiseTransactionResponse {
    fn as_ref(&self) -> &bool {
        &self.value
    }
}

impl From<bool> for PrioritiseTransactionResponse {
    fn from(value: bool) -> Self {
        Self { value }
    }
}

impl From<PrioritiseTransactionResponse> for bool {
    fn from(wrapper: PrioritiseTransactionResponse) -> Self {
        wrapper.value
    }
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for PruneBlockchainResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<u64> for PruneBlockchainResponse {
    fn as_ref(&self) -> &u64 {
        &self.value
    }
}

impl From<u64> for PruneBlockchainResponse {
    fn from(value: u64) -> Self {
        Self { value }
    }
}

impl From<PruneBlockchainResponse> for u64 {
    fn from(wrapper: PruneBlockchainResponse) -> Self {
        wrapper.value
    }
}

/// Response for the `PsbtBumpFee` RPC method
///
/// Wire method: `psbtbumpfee`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct PsbtBumpFeeResponse {
    /// Errors encountered during processing (may be empty).
    pub errors: Vec<String>,
    /// The fee of the new transaction.
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub fee: bitcoin::Amount,
    /// The fee of the replaced transaction.
    /// Wire JSON key: `origfee` (Rust field `orig_fee`).
    #[serde(rename = "origfee")]
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub orig_fee: bitcoin::Amount,
    /// The base64-encoded unsigned PSBT of the new transaction.
    pub psbt: String,
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for ReconsiderBlockResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<()> for ReconsiderBlockResponse {
    fn as_ref(&self) -> &() {
        &self.value
    }
}

impl From<()> for ReconsiderBlockResponse {
    fn from(value: ()) -> Self {
        Self { value }
    }
}

impl From<ReconsiderBlockResponse> for () {
    fn from(wrapper: ReconsiderBlockResponse) -> Self {
        wrapper.value
    }
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for RemovePrunedFundsResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<()> for RemovePrunedFundsResponse {
    fn as_ref(&self) -> &() {
        &self.value
    }
}

impl From<()> for RemovePrunedFundsResponse {
    fn from(value: ()) -> Self {
        Self { value }
    }
}

impl From<RemovePrunedFundsResponse> for () {
    fn from(wrapper: RemovePrunedFundsResponse) -> Self {
        wrapper.value
    }
}

/// Response for the `RescanBlockchain` RPC method
///
/// Wire method: `rescanblockchain`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct RescanBlockchainResponse {
    /// The block height where the rescan started (the requested height or 0)
    pub start_height: u64,
    /// The height of the last rescanned block. May be null in rare cases if there was a reorg and the call didn't scan any blocks because they were already scanned in the background.
    pub stop_height: u64,
}

/// Response for the `RestoreWallet` RPC method
///
/// Wire method: `restorewallet`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct RestoreWalletResponse {
    /// The wallet name if restored successfully.
    pub name: String,
    /// Warning messages, if any, related to restoring and loading the wallet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

/// Response for the `RpcDiscover` RPC method
///
/// Wire method: `rpc.discover`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct RpcDiscoverResponse {
    /// Metadata about this JSON-RPC interface.
    pub info: RpcDiscoverInfo,
    /// Documented RPC methods.
    pub methods: Vec<RpcDiscoverMethods>,
    /// OpenRPC specification version.
    pub openrpc: String,
}

/// Response for the `SaveMempool` RPC method
///
/// Wire method: `savemempool`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SaveMempoolResponse {
    /// the directory and file where the mempool was saved
    pub filename: String,
}

/// Response for the `ScanBlocks` RPC method
///
/// Wire method: `scanblocks`
///
/// | Arm | Rust payload |
/// | --- | --- |
/// | `Null` | `()` |
/// | `Object` | `ScanBlocksResponseScanBlocksObject` |
/// | `Object2` | `ScanBlocksResponseScanBlocksObject2` |
/// | `Boolean` | `bool` |
/// Verbose JSON object (union variant) for `scanblocks` RPC
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ScanBlocksResponseScanBlocksObject {
    /// true if the scan process was not aborted
    pub completed: bool,
    /// The height we started the scan from
    pub from_height: u64,
    /// Blocks that may have matched a scanobject.
    pub relevant_blocks: Vec<String>,
    /// The height we ended the scan at
    pub to_height: u64,
}

/// Verbose JSON object (union variant) for `scanblocks` RPC
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ScanBlocksResponseScanBlocksObject2 {
    /// Height of the block currently being scanned
    pub current_height: u64,
    /// Approximate percent complete
    pub progress: u64,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ScanBlocksResponse {
    Null(()),
    Boolean(bool),
    Object(ScanBlocksResponseScanBlocksObject),
    Object2(ScanBlocksResponseScanBlocksObject2),
}

/// Response for the `ScanTxOutSet` RPC method
///
/// Wire method: `scantxoutset`
///
/// | Arm | Rust payload |
/// | --- | --- |
/// | `Object` | `ScanTxOutSetResponseScanTxOutSetObject` |
/// | `Boolean` | `bool` |
/// | `Object2` | `ScanTxOutSetResponseScanTxOutSetObject2` |
/// | `Null` | `()` |
/// Verbose JSON object (union variant) for `scantxoutset` RPC
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ScanTxOutSetResponseScanTxOutSetObject {
    /// The hash of the block at the tip of the chain
    /// Wire JSON key: `bestblock` (Rust field `best_block`).
    #[serde(rename = "bestblock")]
    pub best_block: String,
    /// The block height at which the scan was done
    pub height: u64,
    /// Whether the scan was completed
    pub success: bool,
    /// The total amount of all found unspent outputs in BTC
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub total_amount: bitcoin::Amount,
    /// The number of unspent transaction outputs scanned
    pub txouts: u64,
    pub unspents: Vec<ScanTxOutSetUnspents>,
}

/// Verbose JSON object (union variant) for `scantxoutset` RPC
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ScanTxOutSetResponseScanTxOutSetObject2 {
    /// Approximate percent complete
    pub progress: u64,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ScanTxOutSetResponse {
    Boolean(bool),
    Null(()),
    Object(ScanTxOutSetResponseScanTxOutSetObject),
    Object2(ScanTxOutSetResponseScanTxOutSetObject2),
}

/// Response for the `Send` RPC method
///
/// Wire method: `send`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SendResponse {
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// If add_to_wallet is false, the hex-encoded raw transaction with signature(s)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<String>,
    /// If more signatures are needed, or if add_to_wallet is false, the base64-encoded (partially) signed transaction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psbt: Option<String>,
    /// The transaction id for the send. Only 1 transaction is created regardless of the number of addresses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txid: Option<bitcoin::Txid>,
}

/// Response for the `SendAll` RPC method
///
/// Wire method: `sendall`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SendAllResponse {
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// If add_to_wallet is false, the hex-encoded raw transaction with signature(s)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<String>,
    /// If more signatures are needed, or if add_to_wallet is false, the base64-encoded (partially) signed transaction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psbt: Option<String>,
    /// The transaction id for the send. Only 1 transaction is created regardless of the number of addresses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txid: Option<bitcoin::Txid>,
}

/// Response for the `SendMany` RPC method
///
/// Wire method: `sendmany`
///
/// | Arm | Rust payload |
/// | --- | --- |
/// | `String` | `String` |
/// | `Object` | `SendManyResponseSendManyObject` |
/// Verbose JSON object (union variant) for `sendmany` RPC
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SendManyResponseSendManyObject {
    /// The reason the wallet selected this fee rate (e.g. fee rate estimator, mempool minimum, fallback, or minimum required).
    pub fee_reason: String,
    /// The transaction id for the send. Only 1 transaction is created regardless of
    /// the number of addresses.
    pub txid: bitcoin::Txid,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendManyResponse {
    String(String),
    Object(SendManyResponseSendManyObject),
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
                Ok(SendRawTransactionResponse {
                    value: v.to_string(),
                })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SendRawTransactionResponse {
                    value: v.to_string(),
                })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SendRawTransactionResponse {
                    value: v.to_string(),
                })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SendRawTransactionResponse {
                    value: v.to_string(),
                })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SendRawTransactionResponse {
                    value: v.to_string(),
                })
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for SendRawTransactionResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<String> for SendRawTransactionResponse {
    fn as_ref(&self) -> &String {
        &self.value
    }
}

impl From<String> for SendRawTransactionResponse {
    fn from(value: String) -> Self {
        Self { value }
    }
}

impl From<SendRawTransactionResponse> for String {
    fn from(wrapper: SendRawTransactionResponse) -> Self {
        wrapper.value
    }
}

/// Response for the `SendToAddress` RPC method
///
/// Wire method: `sendtoaddress`
///
/// | Arm | Rust payload |
/// | --- | --- |
/// | `String` | `String` |
/// | `Object` | `SendToAddressResponseSendToAddressObject` |
/// Verbose JSON object (union variant) for `sendtoaddress` RPC
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SendToAddressResponseSendToAddressObject {
    /// The reason the wallet selected this fee rate (e.g. fee rate estimator, mempool minimum, fallback, or minimum required).
    pub fee_reason: String,
    /// The transaction id.
    pub txid: bitcoin::Txid,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendToAddressResponse {
    String(String),
    Object(SendToAddressResponseSendToAddressObject),
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for SetBanResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<()> for SetBanResponse {
    fn as_ref(&self) -> &() {
        &self.value
    }
}

impl From<()> for SetBanResponse {
    fn from(value: ()) -> Self {
        Self { value }
    }
}

impl From<SetBanResponse> for () {
    fn from(wrapper: SetBanResponse) -> Self {
        wrapper.value
    }
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for SetLabelResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<()> for SetLabelResponse {
    fn as_ref(&self) -> &() {
        &self.value
    }
}

impl From<()> for SetLabelResponse {
    fn from(value: ()) -> Self {
        Self { value }
    }
}

impl From<SetLabelResponse> for () {
    fn from(wrapper: SetLabelResponse) -> Self {
        wrapper.value
    }
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for SetMockTimeResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<()> for SetMockTimeResponse {
    fn as_ref(&self) -> &() {
        &self.value
    }
}

impl From<()> for SetMockTimeResponse {
    fn from(value: ()) -> Self {
        Self { value }
    }
}

impl From<SetMockTimeResponse> for () {
    fn from(wrapper: SetMockTimeResponse) -> Self {
        wrapper.value
    }
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for SetNetworkActiveResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<bool> for SetNetworkActiveResponse {
    fn as_ref(&self) -> &bool {
        &self.value
    }
}

impl From<bool> for SetNetworkActiveResponse {
    fn from(value: bool) -> Self {
        Self { value }
    }
}

impl From<SetNetworkActiveResponse> for bool {
    fn from(wrapper: SetNetworkActiveResponse) -> Self {
        wrapper.value
    }
}

/// Response for the `SetWalletFlag` RPC method
///
/// Wire method: `setwalletflag`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SetWalletFlagResponse {
    /// The name of the flag that was modified
    pub flag_name: String,
    /// The new state of the flag
    pub flag_state: bool,
    /// Any warnings associated with the change
    #[serde(skip_serializing_if = "Option::is_none")]
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
                Ok(SignMessageResponse {
                    value: v.to_string(),
                })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SignMessageResponse {
                    value: v.to_string(),
                })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SignMessageResponse {
                    value: v.to_string(),
                })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SignMessageResponse {
                    value: v.to_string(),
                })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SignMessageResponse {
                    value: v.to_string(),
                })
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for SignMessageResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<String> for SignMessageResponse {
    fn as_ref(&self) -> &String {
        &self.value
    }
}

impl From<String> for SignMessageResponse {
    fn from(value: String) -> Self {
        Self { value }
    }
}

impl From<SignMessageResponse> for String {
    fn from(wrapper: SignMessageResponse) -> Self {
        wrapper.value
    }
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
                Ok(SignMessageWithPrivKeyResponse {
                    value: v.to_string(),
                })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SignMessageWithPrivKeyResponse {
                    value: v.to_string(),
                })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SignMessageWithPrivKeyResponse {
                    value: v.to_string(),
                })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SignMessageWithPrivKeyResponse {
                    value: v.to_string(),
                })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SignMessageWithPrivKeyResponse {
                    value: v.to_string(),
                })
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for SignMessageWithPrivKeyResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<String> for SignMessageWithPrivKeyResponse {
    fn as_ref(&self) -> &String {
        &self.value
    }
}

impl From<String> for SignMessageWithPrivKeyResponse {
    fn from(value: String) -> Self {
        Self { value }
    }
}

impl From<SignMessageWithPrivKeyResponse> for String {
    fn from(wrapper: SignMessageWithPrivKeyResponse) -> Self {
        wrapper.value
    }
}

/// Response for the `SignRawTransactionWithKey` RPC method
///
/// Wire method: `signrawtransactionwithkey`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SignRawTransactionWithKeyResponse {
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// Script verification errors (if there are any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<SignRawTransactionWithKeyErrors>>,
    /// The hex-encoded raw transaction with signature(s)
    pub hex: String,
}

/// Response for the `SignRawTransactionWithWallet` RPC method
///
/// Wire method: `signrawtransactionwithwallet`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SignRawTransactionWithWalletResponse {
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// Script verification errors (if there are any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<SignRawTransactionWithWalletErrors>>,
    /// The hex-encoded raw transaction with signature(s)
    pub hex: String,
}

/// Response for the `SimulateRawTransaction` RPC method
///
/// Wire method: `simulaterawtransaction`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SimulateRawTransactionResponse {
    /// The wallet balance change (negative means decrease).
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
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
                Ok(StopResponse {
                    value: v.to_string(),
                })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(StopResponse {
                    value: v.to_string(),
                })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(StopResponse {
                    value: v.to_string(),
                })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(StopResponse {
                    value: v.to_string(),
                })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(StopResponse {
                    value: v.to_string(),
                })
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for StopResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<String> for StopResponse {
    fn as_ref(&self) -> &String {
        &self.value
    }
}

impl From<String> for StopResponse {
    fn from(value: String) -> Self {
        Self { value }
    }
}

impl From<StopResponse> for String {
    fn from(wrapper: StopResponse) -> Self {
        wrapper.value
    }
}

/// Response for the `SubmitBlock` RPC method
///
/// Wire method: `submitblock`
///
/// | Arm | Rust payload |
/// | --- | --- |
/// | `Null` | `()` |
/// | `String` | `String` |
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SubmitBlockResponse {
    Null(()),
    String(String),
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for SubmitHeaderResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<()> for SubmitHeaderResponse {
    fn as_ref(&self) -> &() {
        &self.value
    }
}

impl From<()> for SubmitHeaderResponse {
    fn from(value: ()) -> Self {
        Self { value }
    }
}

impl From<SubmitHeaderResponse> for () {
    fn from(wrapper: SubmitHeaderResponse) -> Self {
        wrapper.value
    }
}

/// Response for the `SubmitPackage` RPC method
///
/// Wire method: `submitpackage`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SubmitPackageResponse {
    /// The transaction package result message. "success" indicates all transactions were accepted into or are already in the mempool.
    pub package_msg: String,
    /// List of txids of replaced transactions
    /// Wire JSON key: `replaced-transactions` (Rust field `replaced_transactions`).
    #[serde(rename = "replaced-transactions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replaced_transactions: Option<Vec<String>>,
    /// The transaction results keyed by wtxid. An entry is returned for every submitted wtxid.
    /// Wire JSON key: `tx-results` (Rust field `tx_results`).
    #[serde(rename = "tx-results")]
    pub tx_results: BTreeMap<String, SubmitPackageMapValue>,
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for SyncWithValidationInterfaceQueueResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<()> for SyncWithValidationInterfaceQueueResponse {
    fn as_ref(&self) -> &() {
        &self.value
    }
}

impl From<()> for SyncWithValidationInterfaceQueueResponse {
    fn from(value: ()) -> Self {
        Self { value }
    }
}

impl From<SyncWithValidationInterfaceQueueResponse> for () {
    fn from(wrapper: SyncWithValidationInterfaceQueueResponse) -> Self {
        wrapper.value
    }
}

/// Response for the `TestMempoolAccept` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct TestMempoolAcceptResponse {
    /// Wrapped array value
    pub value: Vec<TestMempoolAcceptResultItem>,
}

impl<'de> serde::Deserialize<'de> for TestMempoolAcceptResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<TestMempoolAcceptResultItem>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<TestMempoolAcceptResultItem>> for TestMempoolAcceptResponse {
    fn from(value: Vec<TestMempoolAcceptResultItem>) -> Self {
        Self { value }
    }
}

impl From<TestMempoolAcceptResponse> for Vec<TestMempoolAcceptResultItem> {
    fn from(wrapper: TestMempoolAcceptResponse) -> Self {
        wrapper.value
    }
}

/// Response for the `UnloadWallet` RPC method
///
/// Wire method: `unloadwallet`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct UnloadWalletResponse {
    /// Warning messages, if any, related to unloading the wallet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for UptimeResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<u64> for UptimeResponse {
    fn as_ref(&self) -> &u64 {
        &self.value
    }
}

impl From<u64> for UptimeResponse {
    fn from(value: u64) -> Self {
        Self { value }
    }
}

impl From<UptimeResponse> for u64 {
    fn from(wrapper: UptimeResponse) -> Self {
        wrapper.value
    }
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
                Ok(UtxoUpdatePsbtResponse {
                    value: v.to_string(),
                })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(UtxoUpdatePsbtResponse {
                    value: v.to_string(),
                })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(UtxoUpdatePsbtResponse {
                    value: v.to_string(),
                })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(UtxoUpdatePsbtResponse {
                    value: v.to_string(),
                })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(UtxoUpdatePsbtResponse {
                    value: v.to_string(),
                })
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for UtxoUpdatePsbtResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<String> for UtxoUpdatePsbtResponse {
    fn as_ref(&self) -> &String {
        &self.value
    }
}

impl From<String> for UtxoUpdatePsbtResponse {
    fn from(value: String) -> Self {
        Self { value }
    }
}

impl From<UtxoUpdatePsbtResponse> for String {
    fn from(wrapper: UtxoUpdatePsbtResponse) -> Self {
        wrapper.value
    }
}

/// Response for the `ValidateAddress` RPC method
///
/// Wire method: `validateaddress`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ValidateAddressResponse {
    /// The bitcoin address validated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Error message, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Indices of likely error locations in address, if known (e.g. Bech32 errors)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_locations: Option<Vec<u64>>,
    /// If the key is a script
    /// Wire JSON key: `isscript` (Rust field `is_script`).
    #[serde(rename = "isscript")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_script: Option<bool>,
    /// If the address is valid or not
    /// Wire JSON key: `isvalid` (Rust field `is_valid`).
    #[serde(rename = "isvalid")]
    pub is_valid: bool,
    /// If the address is a witness address
    /// Wire JSON key: `iswitness` (Rust field `is_witness`).
    #[serde(rename = "iswitness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_witness: Option<bool>,
    /// The hex-encoded output script generated by the address
    /// Wire JSON key: `scriptPubKey` (Rust field `script_pubkey`).
    #[serde(rename = "scriptPubKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_pubkey: Option<bitcoin::ScriptBuf>,
    /// The hex value of the witness program
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_program: Option<String>,
    /// The version number of the witness program
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_version: Option<u64>,
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for VerifyChainResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<bool> for VerifyChainResponse {
    fn as_ref(&self) -> &bool {
        &self.value
    }
}

impl From<bool> for VerifyChainResponse {
    fn from(value: bool) -> Self {
        Self { value }
    }
}

impl From<VerifyChainResponse> for bool {
    fn from(wrapper: VerifyChainResponse) -> Self {
        wrapper.value
    }
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for VerifyMessageResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<bool> for VerifyMessageResponse {
    fn as_ref(&self) -> &bool {
        &self.value
    }
}

impl From<bool> for VerifyMessageResponse {
    fn from(value: bool) -> Self {
        Self { value }
    }
}

impl From<VerifyMessageResponse> for bool {
    fn from(wrapper: VerifyMessageResponse) -> Self {
        wrapper.value
    }
}

/// Response for the `VerifyTxOutProof` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct VerifyTxOutProofResponse {
    /// Wrapped array value
    pub value: Vec<String>,
}

impl<'de> serde::Deserialize<'de> for VerifyTxOutProofResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<String>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<String>> for VerifyTxOutProofResponse {
    fn from(value: Vec<String>) -> Self {
        Self { value }
    }
}

impl From<VerifyTxOutProofResponse> for Vec<String> {
    fn from(wrapper: VerifyTxOutProofResponse) -> Self {
        wrapper.value
    }
}

/// Response for the `WaitForBlock` RPC method
///
/// Wire method: `waitforblock`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct WaitForBlockResponse {
    /// The blockhash
    pub hash: String,
    /// Block height
    pub height: u64,
}

/// Response for the `WaitForBlockHeight` RPC method
///
/// Wire method: `waitforblockheight`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct WaitForBlockHeightResponse {
    /// The blockhash
    pub hash: String,
    /// Block height
    pub height: u64,
}

/// Response for the `WaitForNewBlock` RPC method
///
/// Wire method: `waitfornewblock`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct WaitForNewBlockResponse {
    /// The blockhash
    pub hash: String,
    /// Block height
    pub height: u64,
}

/// Response for the `WalletCreateFundedPsbt` RPC method
///
/// Wire method: `walletcreatefundedpsbt`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct WalletCreateFundedPsbtResponse {
    /// The position of the added change output, or -1
    pub changepos: i64,
    /// Fee in BTC the resulting transaction pays
    #[serde(
        serialize_with = "amount_to_btc_float",
        deserialize_with = "amount_from_btc_float"
    )]
    pub fee: bitcoin::Amount,
    /// The resulting raw transaction (base64-encoded string)
    pub psbt: String,
}

/// Response for the `WalletDisplayAddress` RPC method
///
/// Wire method: `walletdisplayaddress`
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for WalletLockResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<()> for WalletLockResponse {
    fn as_ref(&self) -> &() {
        &self.value
    }
}

impl From<()> for WalletLockResponse {
    fn from(value: ()) -> Self {
        Self { value }
    }
}

impl From<WalletLockResponse> for () {
    fn from(wrapper: WalletLockResponse) -> Self {
        wrapper.value
    }
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for WalletPassphraseResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<()> for WalletPassphraseResponse {
    fn as_ref(&self) -> &() {
        &self.value
    }
}

impl From<()> for WalletPassphraseResponse {
    fn from(value: ()) -> Self {
        Self { value }
    }
}

impl From<WalletPassphraseResponse> for () {
    fn from(wrapper: WalletPassphraseResponse) -> Self {
        wrapper.value
    }
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
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for WalletPassphraseChangeResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<()> for WalletPassphraseChangeResponse {
    fn as_ref(&self) -> &() {
        &self.value
    }
}

impl From<()> for WalletPassphraseChangeResponse {
    fn from(value: ()) -> Self {
        Self { value }
    }
}

impl From<WalletPassphraseChangeResponse> for () {
    fn from(wrapper: WalletPassphraseChangeResponse) -> Self {
        wrapper.value
    }
}

/// Response for the `WalletProcessPsbt` RPC method
///
/// Wire method: `walletprocesspsbt`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct WalletProcessPsbtResponse {
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// The hex-encoded network transaction if complete
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<String>,
    /// The base64-encoded partially signed transaction
    pub psbt: String,
}

/// Short aliases for common RPC result shapes (generated; do not hand-edit).
///
/// Prefer `use ethos_bitcoind::{GetBlockVerboseOne, GetTxOut, …}` or
/// `use ethos_bitcoind::aliases::*` / `rpc_prelude::*` instead of consumer shim modules.
pub mod rpc_prelude {
    /// Re-export of [`super::GetAddrManInfoMapValue`].
    pub use super::GetAddrManInfoMapValue as AddrManInfoNetwork;
    /// Re-export of [`super::GetAddrManInfoResponse`].
    pub use super::GetAddrManInfoResponse as GetAddrManInfo;
    /// Re-export of [`super::GetBlockHeaderResponse`].
    pub use super::GetBlockHeaderResponse as GetBlockHeader;
    /// Re-export of [`super::GetBlockHeaderResponseGetBlockHeaderObject`].
    pub use super::GetBlockHeaderResponseGetBlockHeaderObject as GetBlockHeaderObject;
    /// Re-export of [`super::GetBlockHeaderResponseGetBlockHeaderObject`].
    pub use super::GetBlockHeaderResponseGetBlockHeaderObject as GetBlockHeaderVerbose;
    /// Re-export of [`super::GetBlockResponse`].
    pub use super::GetBlockResponse as GetBlock;
    /// Re-export of [`super::GetBlockResponseGetBlockObject`].
    pub use super::GetBlockResponseGetBlockObject as GetBlockObject;
    /// Re-export of [`super::GetBlockResponseGetBlockObject`].
    pub use super::GetBlockResponseGetBlockObject as GetBlockVerboseOne;
    /// Re-export of [`super::GetBlockResponseGetBlockObject2`].
    pub use super::GetBlockResponseGetBlockObject2 as GetBlockObject2;
    /// Re-export of [`super::GetBlockResponseGetBlockObject3`].
    pub use super::GetBlockResponseGetBlockObject3 as GetBlockObject3;
    /// Re-export of [`super::GetBlockchainInfoResponse`].
    pub use super::GetBlockchainInfoResponse as GetBlockchainInfo;
    /// Re-export of [`super::GetDeploymentInfoMapValue`].
    pub use super::GetDeploymentInfoMapValue as DeploymentInfo;
    /// Re-export of [`super::GetDeploymentInfoResponse`].
    pub use super::GetDeploymentInfoResponse as GetDeploymentInfo;
    /// Re-export of [`super::GetNetworkInfoNetworks`].
    pub use super::GetNetworkInfoNetworks as GetNetworkInfoNetwork;
    /// Re-export of [`super::GetNetworkInfoResponse`].
    pub use super::GetNetworkInfoResponse as GetNetworkInfo;
    /// Re-export of [`super::GetRawTransactionResponse`].
    pub use super::GetRawTransactionResponse as GetRawTransaction;
    /// Re-export of [`super::GetRawTransactionResponseGetRawTransactionObject`].
    pub use super::GetRawTransactionResponseGetRawTransactionObject as GetRawTransactionObject;
    /// Re-export of [`super::GetRawTransactionResponseGetRawTransactionObject`].
    pub use super::GetRawTransactionResponseGetRawTransactionObject as GetRawTransactionVerbose;
    /// Re-export of [`super::GetRawTransactionResponseGetRawTransactionObject2`].
    pub use super::GetRawTransactionResponseGetRawTransactionObject2 as GetRawTransactionObject2;
    /// Re-export of [`super::GetRawTransactionScriptPubKey`].
    pub use super::GetRawTransactionScriptPubKey as RawTransactionScriptPubKey;
    /// Re-export of [`super::GetRawTransactionScriptSig`].
    pub use super::GetRawTransactionScriptSig as ScriptSig;
    /// Re-export of [`super::GetRawTransactionVin`].
    pub use super::GetRawTransactionVin as RawTransactionInput;
    /// Re-export of [`super::GetRawTransactionVout`].
    pub use super::GetRawTransactionVout as RawTransactionOutput;
    /// Re-export of [`super::GetTxOutResponse`].
    pub use super::GetTxOutResponse as GetTxOut;
    /// Re-export of [`super::GetTxOutScriptPubKey`].
    pub use super::GetTxOutScriptPubKey as ScriptPubKey;
}

/// Alias of [`rpc_prelude`] (Floresta-oriented name).
pub use rpc_prelude as aliases;

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

/// Serialize [`bitcoin::Amount`] as a BTC float (Core JSON-RPC wire).
fn amount_to_btc_float<S>(amount: &bitcoin::Amount, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_f64(amount.to_btc())
}

/// Serialize `Option<bitcoin::Amount>` as a BTC float when `Some`.
fn option_amount_to_btc_float<S>(
    amount: &Option<bitcoin::Amount>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match amount {
        Some(a) => amount_to_btc_float(a, serializer),
        None => serializer.serialize_none(),
    }
}
