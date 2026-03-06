//! Parameter structs for RPC method calls
use serde::Serialize;

use crate::types::FeeRate;

mod serde_fee_rate {
    pub mod maxfeerate_opt {
        use serde::{Deserialize, Deserializer, Serialize, Serializer};

        use crate::types::FeeRate;

        #[allow(clippy::ref_option)]
        pub fn serialize<S>(f: &Option<FeeRate>, s: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match f {
                Some(r) => s.serialize_some(&((r.to_sat_per_kvb_floor() as f64) / 100_000_000.0)),
                None => s.serialize_none(),
            }
        }

        pub fn deserialize<'d, D: Deserializer<'d>>(d: D) -> Result<Option<FeeRate>, D::Error> {
            let opt: Option<f64> = Option::deserialize(d)?;
            Ok(opt.map(|v| {
                let sat_per_kvb = (v * 100_000_000.0).round().clamp(0.0, u32::MAX as f64) as u32;
                FeeRate::from_sat_per_kvb(sat_per_kvb)
            }))
        }
    }
}

/// Mark in-wallet transaction &lt;txid&gt; as abandoned
/// This will mark this transaction and all its in-wallet descendants as abandoned which will allow
/// for their inputs to be respent.  It can be used to replace "stuck" or evicted transactions.
/// It only works on transactions which are not included in a block and are not currently in the mempool.
/// It has no effect on transactions which are already abandoned.
#[derive(Debug, Serialize)]
pub struct AbandontransactionParams {
    /// The transaction id
    pub txid: bitcoin::Txid,
}

/// Open an outbound connection to a specified node. This RPC is for testing only.
#[derive(Debug, Serialize)]
pub struct AddconnectionParams {
    /// The IP address and port to attempt connecting to.
    pub address: bitcoin::Address,
    /// Type of connection to open ("outbound-full-relay", "block-relay-only", "addr-fetch" or "feeler").
    pub connection_type: String,
    /// Attempt to connect using BIP324 v2 transport protocol
    pub v2transport: bool,
}

/// Attempts to add or remove a node from the addnode list.
/// Or try a connection to a node once.
/// Nodes added using addnode (or -connect) are protected from DoS disconnection and are not required to be
/// full nodes/support SegWit as other outbound peers are (though such peers will not be synced from).
/// Addnode connections are limited to 8 at a time and are counted separately from the -maxconnections limit.
#[derive(Debug, Serialize)]
pub struct AddnodeParams {
    /// The IP address/hostname optionally followed by :port of the peer to connect to
    pub node: String,
    /// 'add' to add a node to the list, 'remove' to remove a node from the list, 'onetry' to try a connection to the node once
    pub command: String,
    /// Attempt to connect using BIP324 v2 transport protocol (ignored for 'remove' command)
    pub v2transport: Option<bool>,
}

/// Add the address of a potential peer to an address manager table. This RPC is for testing only.
#[derive(Debug, Serialize)]
pub struct AddpeeraddressParams {
    /// The IP address of the peer
    pub address: bitcoin::Address,
    /// The port of the peer
    pub port: i64,
    /// If true, attempt to add the peer to the tried addresses table
    pub tried: Option<bool>,
}

/// Analyzes and provides information about the current status of a PSBT and its inputs
#[derive(Debug, Serialize)]
pub struct AnalyzepsbtParams {
    /// A base64 string of a PSBT
    pub psbt: String,
}

/// Safely copies the current wallet file to the specified destination, which can either be a directory or a path with a filename.
#[derive(Debug, Serialize)]
pub struct BackupwalletParams {
    /// The destination directory or file
    pub destination: String,
}

/// Bumps the fee of a transaction T, replacing it with a new transaction B.
/// A transaction with the given txid must be in the wallet.
/// The command will pay the additional fee by reducing change outputs or adding inputs when necessary.
/// It may add a new change output if one does not already exist.
/// All inputs in the original transaction will be included in the replacement transaction.
/// The command will fail if the wallet or mempool contains a transaction that spends one of T's outputs.
/// By default, the new fee will be calculated automatically using the estimatesmartfee RPC.
/// The user can specify a confirmation target for estimatesmartfee.
/// Alternatively, the user can specify a fee rate in sat/vB for the new transaction.
/// At a minimum, the new fee rate must be high enough to pay an additional new relay fee (incrementalfee
/// returned by getnetworkinfo) to enter the node's mempool.
/// * WARNING: before version 0.21, fee_rate was in BTC/kvB. As of 0.21, fee_rate is in sat/vB. *
#[derive(Debug, Serialize)]
pub struct BumpfeeParams {
    /// The txid to be bumped
    pub txid: bitcoin::Txid,
    pub options: Option<serde_json::Value>,
}

/// Combine multiple partially signed Bitcoin transactions into one transaction.
/// Implements the Combiner role.
#[derive(Debug, Serialize)]
pub struct CombinepsbtParams {
    /// The base64 strings of partially signed transactions
    pub txs: Vec<serde_json::Value>,
}

/// Combine multiple partially signed transactions into one transaction.
/// The combined transaction may be another partially signed transaction or a
/// fully signed transaction.
#[derive(Debug, Serialize)]
pub struct CombinerawtransactionParams {
    /// The hex strings of partially signed transactions
    pub txs: Vec<serde_json::Value>,
}

/// Converts a network serialized transaction to a PSBT. This should be used only with createrawtransaction and fundrawtransaction
/// createpsbt and walletcreatefundedpsbt should be used for new applications.
#[derive(Debug, Serialize)]
pub struct ConverttopsbtParams {
    /// The hex string of a raw transaction
    pub hexstring: String,
    /// If true, any signatures in the input will be discarded and conversion
    /// will continue. If false, RPC will fail if any signatures are present.
    pub permitsigdata: Option<bool>,
    /// Whether the transaction hex is a serialized witness transaction.
    /// If iswitness is not present, heuristic tests will be used in decoding.
    /// If true, only witness deserialization will be tried.
    /// If false, only non-witness deserialization will be tried.
    /// This boolean should reflect whether the transaction has inputs
    /// (e.g. fully valid, or on-chain transactions), if known by the caller.
    pub iswitness: Option<bool>,
}

/// Creates a multi-signature address with n signatures of m keys required.
/// It returns a json object with the address and redeemScript.
#[derive(Debug, Serialize)]
pub struct CreatemultisigParams {
    /// The number of required signatures out of the m keys.
    pub nrequired: i64,
    /// The hex-encoded public keys.
    pub keys: Vec<serde_json::Value>,
    /// The address type to use. Options are "legacy", "p2sh-segwit", and "bech32".
    pub address_type: Option<String>,
}

/// Creates a transaction in the Partially Signed Transaction format.
/// Implements the Creator role.
/// Note that the transaction's inputs are not signed, and
/// it is not stored in the wallet or transmitted to the network.
#[derive(Debug, Serialize)]
pub struct CreatepsbtParams {
    /// The inputs
    pub inputs: Vec<serde_json::Value>,
    /// The outputs specified as key-value pairs.
    /// Each key may only appear once, i.e. there can only be one 'data' output, and no address may be duplicated.
    /// At least one output of either type must be specified.
    /// For compatibility reasons, a dictionary, which holds the key-value pairs directly, is also
    /// accepted as second parameter.
    pub outputs: Vec<serde_json::Value>,
    /// Raw locktime. Non-0 value also locktime-activates inputs
    #[serde(rename = "locktime")]
    pub lock_time: Option<i64>,
    /// Marks this transaction as BIP125-replaceable.
    /// Allows this transaction to be replaced by a transaction with higher fees. If provided, it is an error if explicit sequence numbers are incompatible.
    pub replaceable: Option<bool>,
    /// Transaction version
    pub version: Option<i64>,
}

/// Create a transaction spending the given inputs and creating new outputs.
/// Outputs can be addresses or data.
/// Returns hex-encoded raw transaction.
/// Note that the transaction's inputs are not signed, and
/// it is not stored in the wallet or transmitted to the network.
#[derive(Debug, Serialize)]
pub struct CreaterawtransactionParams {
    /// The inputs
    pub inputs: Vec<serde_json::Value>,
    /// The outputs specified as key-value pairs.
    /// Each key may only appear once, i.e. there can only be one 'data' output, and no address may be duplicated.
    /// At least one output of either type must be specified.
    /// For compatibility reasons, a dictionary, which holds the key-value pairs directly, is also
    /// accepted as second parameter.
    pub outputs: Vec<serde_json::Value>,
    /// Raw locktime. Non-0 value also locktime-activates inputs
    #[serde(rename = "locktime")]
    pub lock_time: Option<i64>,
    /// Marks this transaction as BIP125-replaceable.
    /// Allows this transaction to be replaced by a transaction with higher fees. If provided, it is an error if explicit sequence numbers are incompatible.
    pub replaceable: Option<bool>,
    /// Transaction version
    pub version: Option<i64>,
}

/// Creates and loads a new wallet.
#[derive(Debug, Serialize)]
pub struct CreatewalletParams {
    /// The name for the new wallet. If this is a path, the wallet will be created at the path location.
    pub wallet_name: String,
    /// Disable the possibility of private keys (only watchonlys are possible in this mode).
    pub disable_private_keys: Option<bool>,
    /// Create a blank wallet. A blank wallet has no keys.
    pub blank: Option<bool>,
    /// Encrypt the wallet with this passphrase.
    pub passphrase: Option<String>,
    /// Keep track of coin reuse, and treat dirty and clean coins differently with privacy considerations in mind.
    pub avoid_reuse: Option<bool>,
    /// If set, must be "true"
    pub descriptors: Option<bool>,
    /// Save wallet name to persistent settings and load on startup. True to add wallet to startup list, false to remove, null to leave unchanged.
    pub load_on_startup: Option<bool>,
    /// Use an external signer such as a hardware wallet. Requires -signer to be configured. Wallet creation will fail if keys cannot be fetched. Requires disable_private_keys and descriptors set to true.
    pub external_signer: Option<bool>,
}

/// Creates the wallet's descriptor for the given address type. The address type must be one that the wallet does not already have a descriptor for.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
#[derive(Debug, Serialize)]
pub struct CreatewalletdescriptorParams {
    /// The address type the descriptor will produce. Options are "legacy", "p2sh-segwit", "bech32", "bech32m".
    #[serde(rename = "type")]
    pub r#type: String,
    pub options: Option<serde_json::Value>,
}

/// Return a JSON object representing the serialized, base64-encoded partially signed Bitcoin transaction.
#[derive(Debug, Serialize)]
pub struct DecodepsbtParams {
    /// The PSBT base64 string
    pub psbt: String,
}

/// Return a JSON object representing the serialized, hex-encoded transaction.
#[derive(Debug, Serialize)]
pub struct DecoderawtransactionParams {
    /// The transaction hex string
    pub hexstring: String,
    /// Whether the transaction hex is a serialized witness transaction.
    /// If iswitness is not present, heuristic tests will be used in decoding.
    /// If true, only witness deserialization will be tried.
    /// If false, only non-witness deserialization will be tried.
    /// This boolean should reflect whether the transaction has inputs
    /// (e.g. fully valid, or on-chain transactions), if known by the caller.
    pub iswitness: Option<bool>,
}

/// Decode a hex-encoded script.
#[derive(Debug, Serialize)]
pub struct DecodescriptParams {
    /// the hex-encoded script
    pub hexstring: String,
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
#[derive(Debug, Serialize)]
pub struct DeriveaddressesParams {
    /// The descriptor.
    pub descriptor: String,
    /// If a ranged descriptor is used, this specifies the end or the range (in \[begin,end\] notation) to derive.
    pub range: Option<serde_json::Value>,
}

/// Update all segwit inputs in a PSBT with information from output descriptors, the UTXO set or the mempool.
/// Then, sign the inputs we are able to with information from the output descriptors.
#[derive(Debug, Serialize)]
pub struct DescriptorprocesspsbtParams {
    /// The transaction base64 string
    pub psbt: String,
    /// An array of either strings or objects
    pub descriptors: Vec<serde_json::Value>,
    /// The signature hash type to sign with if not specified by the PSBT. Must be one of
    /// "DEFAULT"
    /// "ALL"
    /// "NONE"
    /// "SINGLE"
    /// "ALL|ANYONECANPAY"
    /// "NONE|ANYONECANPAY"
    /// "SINGLE|ANYONECANPAY"
    pub sighashtype: Option<String>,
    /// Include BIP 32 derivation paths for public keys if we know them
    pub bip32derivs: Option<bool>,
    /// Also finalize inputs if possible
    pub finalize: Option<bool>,
}

/// Immediately disconnects from the specified peer node.
/// Strictly one out of 'address' and 'nodeid' can be provided to identify the node.
/// To disconnect by nodeid, either set 'address' to the empty string, or call using the named 'nodeid' argument only.
#[derive(Debug, Serialize)]
pub struct DisconnectnodeParams {
    /// The IP address/port of the node
    pub address: Option<bitcoin::Address>,
    /// The node ID (see getpeerinfo for node IDs)
    pub nodeid: Option<i64>,
}

/// Write the serialized UTXO set to a file. This can be used in loadtxoutset afterwards if this snapshot height is supported in the chainparams as well.
/// Unless the "latest" type is requested, the node will roll back to the requested height and network activity will be suspended during this process. Because of this it is discouraged to interact with the node in any other way during the execution of this call to avoid inconsistent results and race conditions, particularly RPCs that interact with blockstorage.
/// This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[derive(Debug, Serialize)]
pub struct DumptxoutsetParams {
    /// Path to the output file. If relative, will be prefixed by datadir.
    pub path: String,
    /// The type of snapshot to create. Can be "latest" to create a snapshot of the current UTXO set or "rollback" to temporarily roll back the state of the node to a historical block before creating the snapshot of a historical UTXO set. This parameter can be omitted if a separate "rollback" named parameter is specified indicating the height or hash of a specific historical block. If "rollback" is specified and separate "rollback" named parameter is not specified, this will roll back to the latest valid snapshot block that can currently be loaded with loadtxoutset.
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    pub options: Option<serde_json::Value>,
}

/// Simply echo back the input arguments. This command is for testing.
/// It will return an internal bug report when arg9='trigger_internal_bug' is passed.
/// The difference between echo and echojson is that echojson has argument conversion enabled in the client-side table in bitcoin-cli and the GUI. There is no server-side difference.
#[derive(Debug, Serialize)]
pub struct EchoParams {
    pub arg0: Option<String>,
    pub arg1: Option<String>,
    pub arg2: Option<String>,
    pub arg3: Option<String>,
    pub arg4: Option<String>,
    pub arg5: Option<String>,
    pub arg6: Option<String>,
    pub arg7: Option<String>,
    pub arg8: Option<String>,
    pub arg9: Option<String>,
}

/// Echo back the input argument, passing it through a spawned process in a multiprocess build.
/// This command is for testing.
#[derive(Debug, Serialize)]
pub struct EchoipcParams {
    /// The string to echo
    pub arg: String,
}

/// Simply echo back the input arguments. This command is for testing.
/// It will return an internal bug report when arg9='trigger_internal_bug' is passed.
/// The difference between echo and echojson is that echojson has argument conversion enabled in the client-side table in bitcoin-cli and the GUI. There is no server-side difference.
#[derive(Debug, Serialize)]
pub struct EchojsonParams {
    pub arg0: Option<String>,
    pub arg1: Option<String>,
    pub arg2: Option<String>,
    pub arg3: Option<String>,
    pub arg4: Option<String>,
    pub arg5: Option<String>,
    pub arg6: Option<String>,
    pub arg7: Option<String>,
    pub arg8: Option<String>,
    pub arg9: Option<String>,
}

/// Encrypts the wallet with 'passphrase'. This is for first time encryption.
/// After this, any calls that interact with private keys such as sending or signing
/// will require the passphrase to be set prior to making these calls.
/// Use the walletpassphrase call for this, and then walletlock call.
/// If the wallet is already encrypted, use the walletpassphrasechange call.
/// ** IMPORTANT **
/// For security reasons, the encryption process will generate a new HD seed, resulting
/// in the creation of a fresh set of active descriptors. Therefore, it is crucial to
/// securely back up the newly generated wallet file using the backupwallet RPC.
#[derive(Debug, Serialize)]
pub struct EncryptwalletParams {
    /// The pass phrase to encrypt the wallet with. It must be at least 1 character, but should be long.
    pub passphrase: String,
}

/// WARNING: This interface is unstable and may disappear or change!
/// WARNING: This is an advanced API call that is tightly coupled to the specific
/// implementation of fee estimation. The parameters it can be called with
/// and the results it returns will change if the internal implementation changes.
/// Estimates the approximate fee per kilobyte needed for a transaction to begin
/// confirmation within conf_target blocks if possible. Uses virtual transaction size as
/// defined in BIP 141 (witness data is discounted).
#[derive(Debug, Serialize)]
pub struct EstimaterawfeeParams {
    /// Confirmation target in blocks (1 - 1008)
    pub conf_target: i64,
    /// The proportion of transactions in a given feerate range that must have been
    /// confirmed within conf_target in order to consider those feerates as high enough and proceed to check
    /// lower buckets.
    pub threshold: Option<i64>,
}

/// Estimates the approximate fee per kilobyte needed for a transaction to begin
/// confirmation within conf_target blocks if possible and return the number of blocks
/// for which the estimate is valid. Uses virtual transaction size as defined
/// in BIP 141 (witness data is discounted).
#[derive(Debug, Serialize)]
pub struct EstimatesmartfeeParams {
    /// Confirmation target in blocks (1 - 1008)
    pub conf_target: i64,
    /// The fee estimate mode.
    /// unset, economical, conservative
    /// unset means no mode set (default mode will be used).
    /// economical estimates use a shorter time horizon, making them more
    /// responsive to short-term drops in the prevailing fee market. This mode
    /// potentially returns a lower fee rate estimate.
    /// conservative estimates use a longer time horizon, making them
    /// less responsive to short-term drops in the prevailing fee market. This mode
    /// potentially returns a higher fee rate estimate.
    pub estimate_mode: Option<String>,
}

/// Finalize the inputs of a PSBT. If the transaction is fully signed, it will produce a
/// network serialized transaction which can be broadcast with sendrawtransaction. Otherwise a PSBT will be
/// created which has the final_scriptSig and final_scriptwitness fields filled for inputs that are complete.
/// Implements the Finalizer and Extractor roles.
#[derive(Debug, Serialize)]
pub struct FinalizepsbtParams {
    /// A base64 string of a PSBT
    pub psbt: String,
    /// If true and the transaction is complete,
    /// extract and return the complete transaction in normal network serialization instead of the PSBT.
    pub extract: Option<bool>,
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
#[derive(Debug, Serialize)]
pub struct FundrawtransactionParams {
    /// The hex string of the raw transaction
    pub hexstring: String,
    pub options: Option<serde_json::Value>,
    /// Whether the transaction hex is a serialized witness transaction.
    /// If iswitness is not present, heuristic tests will be used in decoding.
    /// If true, only witness deserialization will be tried.
    /// If false, only non-witness deserialization will be tried.
    /// This boolean should reflect whether the transaction has inputs
    /// (e.g. fully valid, or on-chain transactions), if known by the caller.
    pub iswitness: Option<bool>,
}

/// Mine a set of ordered transactions to a specified address or descriptor and return the block hash.
#[derive(Debug, Serialize)]
pub struct GenerateblockParams {
    /// The address or descriptor to send the newly generated bitcoin to.
    pub output: String,
    /// An array of hex strings which are either txids or raw transactions.
    /// Txids must reference transactions currently in the mempool.
    /// All transactions must be valid and in valid order, otherwise the block will be rejected.
    pub transactions: Vec<serde_json::Value>,
    /// Whether to submit the block before the RPC call returns or to return it as hex.
    pub submit: Option<bool>,
}

/// Mine to a specified address and return the block hashes.
#[derive(Debug, Serialize)]
pub struct GeneratetoaddressParams {
    /// How many blocks are generated.
    pub nblocks: i64,
    /// The address to send the newly generated bitcoin to.
    pub address: bitcoin::Address,
    /// How many iterations to try.
    pub maxtries: Option<i64>,
}

/// Mine to a specified descriptor and return the block hashes.
#[derive(Debug, Serialize)]
pub struct GeneratetodescriptorParams {
    /// How many blocks are generated.
    pub num_blocks: i64,
    /// The descriptor to send the newly generated bitcoin to.
    pub descriptor: String,
    /// How many iterations to try.
    pub maxtries: Option<i64>,
}

/// Returns information about the given added node, or all added nodes
/// (note that onetry addnodes are not listed here)
#[derive(Debug, Serialize)]
pub struct GetaddednodeinfoParams {
    /// If provided, return information about this specific node, otherwise all nodes are returned.
    pub node: Option<String>,
}

/// Returns the list of addresses assigned the specified label.
#[derive(Debug, Serialize)]
pub struct GetaddressesbylabelParams {
    /// The label.
    pub label: String,
}

/// Return information about the given bitcoin address.
/// Some of the information will only be present if the address is in the active wallet.
#[derive(Debug, Serialize)]
pub struct GetaddressinfoParams {
    /// The bitcoin address for which to get information.
    pub address: bitcoin::Address,
}

/// Returns the total available balance.
/// The available balance is what the wallet considers currently spendable, and is
/// thus affected by options which limit spendability such as -spendzeroconfchange.
#[derive(Debug, Serialize)]
pub struct GetbalanceParams {
    /// Remains for backward compatibility. Must be excluded or set to "*".
    pub dummy: Option<String>,
    /// Only include transactions confirmed at least this many times.
    #[serde(rename = "minconf")]
    pub min_conf: Option<i64>,
    /// No longer used
    pub include_watchonly: Option<bool>,
    /// (only available if avoid_reuse wallet flag is set) Do not include balance in dirty outputs; addresses are considered dirty if they have previously been used in a transaction.
    pub avoid_reuse: Option<bool>,
}

/// If verbosity is 0, returns a string that is serialized, hex-encoded data for block 'hash'.
/// If verbosity is 1, returns an Object with information about block &lt;hash&gt;.
/// If verbosity is 2, returns an Object with information about block &lt;hash&gt; and information about each transaction.
/// If verbosity is 3, returns an Object with information about block &lt;hash&gt; and information about each transaction, including prevout information for inputs (only for unpruned blocks in the current best chain).
#[derive(Debug, Serialize)]
pub struct GetblockParams {
    /// The block hash
    #[serde(rename = "blockhash")]
    pub block_hash: bitcoin::BlockHash,
    /// 0 for hex-encoded data, 1 for a JSON object, 2 for JSON object with transaction data, and 3 for JSON object with transaction data including prevout information for inputs
    pub verbosity: Option<i64>,
}

/// Retrieve a BIP 157 content filter for a particular block.
#[derive(Debug, Serialize)]
pub struct GetblockfilterParams {
    /// The hash of the block
    #[serde(rename = "blockhash")]
    pub block_hash: bitcoin::BlockHash,
    /// The type name of the filter
    pub filtertype: Option<String>,
}

/// Attempt to fetch block from a given peer.
/// We must have the header for this block, e.g. using submitheader.
/// The block will not have any undo data which can limit the usage of the block data in a context where the undo data is needed.
/// Subsequent calls for the same block may cause the response from the previous peer to be ignored.
/// Peers generally ignore requests for a stale block that they never fully verified, or one that is more than a month old.
/// When a peer does not respond with a block, we will disconnect.
/// Note: The block could be re-pruned as soon as it is received.
/// Returns an empty JSON object if the request was successfully scheduled.
#[derive(Debug, Serialize)]
pub struct GetblockfrompeerParams {
    /// The block hash to try to fetch
    #[serde(rename = "blockhash")]
    pub block_hash: bitcoin::BlockHash,
    /// The peer to fetch it from (see getpeerinfo for peer IDs)
    pub peer_id: i64,
}

/// Returns hash of block in best-block-chain at height provided.
#[derive(Debug, Serialize)]
pub struct GetblockhashParams {
    /// The height index
    pub height: i64,
}

/// If verbose is false, returns a string that is serialized, hex-encoded data for blockheader 'hash'.
/// If verbose is true, returns an Object with information about blockheader &lt;hash&gt;.
#[derive(Debug, Serialize)]
pub struct GetblockheaderParams {
    /// The block hash
    #[serde(rename = "blockhash")]
    pub block_hash: bitcoin::BlockHash,
    /// true for a json object, false for the hex-encoded data
    pub verbose: Option<bool>,
}

/// Compute per block statistics for a given window. All amounts are in satoshis.
/// It won't work for some heights with pruning.
#[derive(Debug, Serialize)]
pub struct GetblockstatsParams {
    /// The block hash or height of the target block
    pub hash_or_height: i64,
    /// Values to plot (see result below)
    pub stats: Option<Vec<serde_json::Value>>,
}

/// If the request parameters include a 'mode' key, that is used to explicitly select between the default 'template' request or a 'proposal'.
/// It returns data needed to construct a block to work on.
/// For full specification, see BIPs 22, 23, 9, and 145:
/// <https://github.com/bitcoin/bips/blob/master/bip-0022.mediawiki>
/// <https://github.com/bitcoin/bips/blob/master/bip-0023.mediawiki>
/// <https://github.com/bitcoin/bips/blob/master/bip-0009.mediawiki#getblocktemplate_changes>
/// <https://github.com/bitcoin/bips/blob/master/bip-0145.mediawiki>
#[derive(Debug, Serialize)]
pub struct GetblocktemplateParams {
    /// Format of the template
    pub template_request: GetBlockTemplateRequest,
}

/// Compute statistics about the total number and rate of transactions in the chain.
#[derive(Debug, Serialize)]
pub struct GetchaintxstatsParams {
    /// Size of the window in number of blocks
    pub nblocks: Option<i64>,
    /// The hash of the block that ends the window.
    #[serde(rename = "blockhash")]
    pub block_hash: Option<bitcoin::BlockHash>,
}

/// Returns an object containing various state info regarding deployments of consensus changes.
#[derive(Debug, Serialize)]
pub struct GetdeploymentinfoParams {
    /// The block hash at which to query deployment state
    #[serde(rename = "blockhash")]
    pub block_hash: Option<bitcoin::BlockHash>,
}

/// Get spend and receive activity associated with a set of descriptors for a set of blocks. This command pairs well with the `relevant_blocks` output of `scanblocks()`.
/// This call may take several minutes. If you encounter timeouts, try specifying no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[derive(Debug, Serialize)]
pub struct GetdescriptoractivityParams {
    /// The list of blockhashes to examine for activity. Order doesn't matter. Must be along main chain or an error is thrown.
    pub blockhashes: Vec<serde_json::Value>,
    /// The list of descriptors (scan objects) to examine for activity. Every scan object is either a string descriptor or an object:
    pub scanobjects: Vec<serde_json::Value>,
    /// Whether to include unconfirmed activity
    pub include_mempool: Option<bool>,
}

/// Analyses a descriptor.
#[derive(Debug, Serialize)]
pub struct GetdescriptorinfoParams {
    /// The descriptor.
    pub descriptor: String,
}

/// List all BIP 32 HD keys in the wallet and which descriptors use them.
#[derive(Debug, Serialize)]
pub struct GethdkeysParams {
    pub options: Option<serde_json::Value>,
}

/// Returns the status of one or all available indices currently running in the node.
#[derive(Debug, Serialize)]
pub struct GetindexinfoParams {
    /// Filter results for an index with a specific name.
    pub index_name: Option<String>,
}

/// Returns an object containing information about memory usage.
#[derive(Debug, Serialize)]
pub struct GetmemoryinfoParams {
    /// determines what kind of information is returned.
    /// - "stats" returns general statistics about memory usage in the daemon.
    /// - "mallocinfo" returns an XML string describing low-level heap state (only available if compiled with glibc).
    pub mode: Option<String>,
}

/// If txid is in the mempool, returns all in-mempool ancestors.
#[derive(Debug, Serialize)]
pub struct GetmempoolancestorsParams {
    /// The transaction id (must be in mempool)
    pub txid: bitcoin::Txid,
    /// True for a json object, false for array of transaction ids
    pub verbose: Option<bool>,
}

/// Returns mempool data for given cluster
#[derive(Debug, Serialize)]
pub struct GetmempoolclusterParams {
    /// The txid of a transaction in the cluster
    pub txid: bitcoin::Txid,
}

/// If txid is in the mempool, returns all in-mempool descendants.
#[derive(Debug, Serialize)]
pub struct GetmempooldescendantsParams {
    /// The transaction id (must be in mempool)
    pub txid: bitcoin::Txid,
    /// True for a json object, false for array of transaction ids
    pub verbose: Option<bool>,
}

/// Returns mempool data for given transaction
#[derive(Debug, Serialize)]
pub struct GetmempoolentryParams {
    /// The transaction id (must be in mempool)
    pub txid: bitcoin::Txid,
}

/// Returns the estimated network hashes per second based on the last n blocks.
/// Pass in \[blocks\] to override # of blocks, -1 specifies since last difficulty change.
/// Pass in \[height\] to estimate the network speed at the time when a certain block was found.
#[derive(Debug, Serialize)]
pub struct GetnetworkhashpsParams {
    /// The number of previous blocks to calculate estimate from, or -1 for blocks since last difficulty change.
    pub nblocks: Option<i64>,
    /// To estimate at the time of the given height.
    pub height: Option<i64>,
}

/// Returns a new Bitcoin address for receiving payments.
/// If 'label' is specified, it is added to the address book
/// so payments received with the address will be associated with 'label'.
#[derive(Debug, Serialize)]
pub struct GetnewaddressParams {
    /// The label name for the address to be linked to. It can also be set to the empty string "" to represent the default label. The label does not need to exist, it will be created if there is no label by the given name.
    pub label: Option<String>,
    /// The address type to use. Options are "legacy", "p2sh-segwit", "bech32", "bech32m".
    pub address_type: Option<String>,
}

/// Return known addresses, after filtering for quality and recency.
/// These can potentially be used to find new peers in the network.
/// The total number of addresses known to the node may be higher.
#[derive(Debug, Serialize)]
pub struct GetnodeaddressesParams {
    /// The maximum number of addresses to return. Specify 0 to return all known addresses.
    pub count: Option<i64>,
    /// Return only addresses of the specified network. Can be one of: ipv4, ipv6, onion, i2p, cjdns.
    pub network: Option<String>,
}

/// Shows transactions in the tx orphanage.
/// EXPERIMENTAL warning: this call may be changed in future releases.
#[derive(Debug, Serialize)]
pub struct GetorphantxsParams {
    /// 0 for an array of txids (may contain duplicates), 1 for an array of objects with tx details, and 2 for details from (1) and tx hex
    pub verbosity: Option<i64>,
}

/// Returns a new Bitcoin address, for receiving change.
/// This is for use with raw transactions, NOT normal use.
#[derive(Debug, Serialize)]
pub struct GetrawchangeaddressParams {
    /// The address type to use. Options are "legacy", "p2sh-segwit", "bech32", "bech32m".
    pub address_type: Option<String>,
}

/// Returns all transaction ids in memory pool as a json array of string transaction ids.
/// Hint: use getmempoolentry to fetch a specific transaction from the mempool.
#[derive(Debug, Serialize)]
pub struct GetrawmempoolParams {
    /// True for a json object, false for array of transaction ids
    pub verbose: Option<bool>,
    /// If verbose=false, returns a json object with transaction list and mempool sequence number attached.
    pub mempool_sequence: Option<bool>,
}

/// By default, this call only returns a transaction if it is in the mempool. If -txindex is enabled
/// and no blockhash argument is passed, it will return the transaction if it is in the mempool or any block.
/// If a blockhash argument is passed, it will return the transaction if
/// the specified block is available and the transaction is in that block.
/// Hint: Use gettransaction for wallet transactions.
/// If verbosity is 0 or omitted, returns the serialized transaction as a hex-encoded string.
/// If verbosity is 1, returns a JSON Object with information about the transaction.
/// If verbosity is 2, returns a JSON Object with information about the transaction, including fee and prevout information.
#[derive(Debug, Serialize)]
pub struct GetrawtransactionParams {
    /// The transaction id
    pub txid: bitcoin::Txid,
    /// 0 for hex-encoded data, 1 for a JSON object, and 2 for JSON object with fee and prevout
    pub verbosity: Option<i64>,
    /// The block in which to look for the transaction
    #[serde(rename = "blockhash")]
    pub block_hash: Option<bitcoin::BlockHash>,
}

/// Returns the total amount received by the given address in transactions with at least minconf confirmations.
#[derive(Debug, Serialize)]
pub struct GetreceivedbyaddressParams {
    /// The bitcoin address for transactions.
    pub address: bitcoin::Address,
    /// Only include transactions confirmed at least this many times.
    #[serde(rename = "minconf")]
    pub min_conf: Option<i64>,
    /// Include immature coinbase transactions.
    pub include_immature_coinbase: Option<bool>,
}

/// Returns the total amount received by addresses with &lt;label&gt; in transactions with at least \[minconf\] confirmations.
#[derive(Debug, Serialize)]
pub struct GetreceivedbylabelParams {
    /// The selected label, may be the default label using "".
    pub label: String,
    /// Only include transactions confirmed at least this many times.
    #[serde(rename = "minconf")]
    pub min_conf: Option<i64>,
    /// Include immature coinbase transactions.
    pub include_immature_coinbase: Option<bool>,
}

/// Get detailed information about in-wallet transaction &lt;txid&gt;
#[derive(Debug, Serialize)]
pub struct GettransactionParams {
    /// The transaction id
    pub txid: bitcoin::Txid,
    /// (DEPRECATED) No longer used
    pub include_watchonly: Option<bool>,
    /// Whether to include a `decoded` field containing the decoded transaction (equivalent to RPC decoderawtransaction)
    pub verbose: Option<bool>,
}

/// Returns details about an unspent transaction output.
#[derive(Debug, Serialize)]
pub struct GettxoutParams {
    /// The transaction id
    pub txid: bitcoin::Txid,
    /// vout number
    pub n: i64,
    /// Whether to include the mempool. Note that an unspent output that is spent in the mempool won't appear.
    pub include_mempool: Option<bool>,
}

/// Returns a hex-encoded proof that "txid" was included in a block.
/// NOTE: By default this function only works sometimes. This is when there is an
/// unspent output in the utxo for this transaction. To make it always work,
/// you need to maintain a transaction index, using the -txindex command line option or
/// specify the block in which the transaction is included manually (by blockhash).
#[derive(Debug, Serialize)]
pub struct GettxoutproofParams {
    /// The txids to filter
    pub txids: Vec<serde_json::Value>,
    /// If specified, looks for txid in the block with this hash
    #[serde(rename = "blockhash")]
    pub block_hash: Option<bitcoin::BlockHash>,
}

/// Returns statistics about the unspent transaction output set.
/// Note this call may take some time if you are not using coinstatsindex.
#[derive(Debug, Serialize)]
pub struct GettxoutsetinfoParams {
    /// Which UTXO set hash should be calculated. Options: 'hash_serialized_3' (the legacy algorithm), 'muhash', 'none'.
    pub hash_type: Option<String>,
    /// The block hash or height of the target height (only available with coinstatsindex).
    pub hash_or_height: Option<i64>,
    /// Use coinstatsindex, if available.
    pub use_index: Option<bool>,
}

/// Scans the mempool to find transactions spending any of the given outputs
#[derive(Debug, Serialize)]
pub struct GettxspendingprevoutParams {
    /// The transaction outputs that we want to check, and within each, the txid (string) vout (numeric).
    pub outputs: Vec<serde_json::Value>,
}

/// List all commands, or get help for a specified command.
#[derive(Debug, Serialize)]
pub struct HelpParams {
    /// The command to get help on
    pub command: Option<String>,
}

/// Import descriptors. This will trigger a rescan of the blockchain based on the earliest timestamp of all descriptors being imported. Requires a new wallet backup.
/// When importing descriptors with multipath key expressions, if the multipath specifier contains exactly two elements, the descriptor produced from the second element will be imported as an internal descriptor.
/// Note: This call can take over an hour to complete if using an early timestamp; during that time, other rpc calls
/// may report that the imported keys, addresses or scripts exist but related transactions are still missing.
/// The rescan is significantly faster if block filters are available (using startup option "-blockfilterindex=1").
#[derive(Debug, Serialize)]
pub struct ImportdescriptorsParams {
    /// Data to be imported
    pub requests: Vec<serde_json::Value>,
}

/// Import a mempool.dat file and attempt to add its contents to the mempool.
/// Warning: Importing untrusted files is dangerous, especially if metadata from the file is taken over.
#[derive(Debug, Serialize)]
pub struct ImportmempoolParams {
    /// The mempool file
    pub filepath: String,
    pub options: Option<serde_json::Value>,
}

/// Imports funds without rescan. Corresponding address or script must previously be included in wallet. Aimed towards pruned wallets. The end-user is responsible to import additional transactions that subsequently spend the imported outputs or rescan after the point in the blockchain the transaction is included.
#[derive(Debug, Serialize)]
pub struct ImportprunedfundsParams {
    /// A raw transaction in hex funding an already-existing address in wallet
    pub rawtransaction: String,
    /// The hex output from gettxoutproof that contains the transaction
    pub txoutproof: String,
}

/// Permanently marks a block as invalid, as if it violated a consensus rule.
#[derive(Debug, Serialize)]
pub struct InvalidateblockParams {
    /// the hash of the block to mark as invalid
    #[serde(rename = "blockhash")]
    pub block_hash: bitcoin::BlockHash,
}

/// Joins multiple distinct PSBTs with different inputs and outputs into one PSBT with inputs and outputs from all of the PSBTs
/// No input in any of the PSBTs can be in more than one of the PSBTs.
#[derive(Debug, Serialize)]
pub struct JoinpsbtsParams {
    /// The base64 strings of partially signed transactions
    pub txs: Vec<serde_json::Value>,
}

/// Refills each descriptor keypool in the wallet up to the specified number of new keys.
/// By default, descriptor wallets have 4 active ranged descriptors ("legacy", "p2sh-segwit", "bech32", "bech32m"), each with 1000 entries.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
#[derive(Debug, Serialize)]
pub struct KeypoolrefillParams {
    /// The new keypool size
    pub newsize: Option<i64>,
}

/// List all descriptors present in a wallet.
#[derive(Debug, Serialize)]
pub struct ListdescriptorsParams {
    /// Show private descriptors.
    pub private: Option<bool>,
}

/// Returns the list of all labels, or labels that are assigned to addresses with a specific purpose.
#[derive(Debug, Serialize)]
pub struct ListlabelsParams {
    /// Address purpose to list labels for ('send','receive'). An empty string is the same as not providing this argument.
    pub purpose: Option<String>,
}

/// List balances by receiving address.
#[derive(Debug, Serialize)]
pub struct ListreceivedbyaddressParams {
    /// The minimum number of confirmations before payments are included.
    #[serde(rename = "minconf")]
    pub min_conf: Option<i64>,
    /// Whether to include addresses that haven't received any payments.
    pub include_empty: Option<bool>,
    /// (DEPRECATED) No longer used
    pub include_watchonly: Option<bool>,
    /// If present and non-empty, only return information on this address.
    pub address_filter: Option<String>,
    /// Include immature coinbase transactions.
    pub include_immature_coinbase: Option<bool>,
}

/// List received transactions by label.
#[derive(Debug, Serialize)]
pub struct ListreceivedbylabelParams {
    /// The minimum number of confirmations before payments are included.
    #[serde(rename = "minconf")]
    pub min_conf: Option<i64>,
    /// Whether to include labels that haven't received any payments.
    pub include_empty: Option<bool>,
    /// (DEPRECATED) No longer used
    pub include_watchonly: Option<bool>,
    /// Include immature coinbase transactions.
    pub include_immature_coinbase: Option<bool>,
}

/// Get all transactions in blocks since block \[blockhash\], or all transactions if omitted.
/// If "blockhash" is no longer a part of the main chain, transactions from the fork point onward are included.
/// Additionally, if include_removed is set, transactions affecting the wallet which were removed are returned in the "removed" array.
#[derive(Debug, Serialize)]
pub struct ListsinceblockParams {
    /// If set, the block hash to list transactions since, otherwise list all transactions.
    #[serde(rename = "blockhash")]
    pub block_hash: Option<bitcoin::BlockHash>,
    /// Return the nth block hash from the main chain. e.g. 1 would mean the best block hash. Note: this is not used as a filter, but only affects \[lastblock\] in the return value
    pub target_confirmations: Option<i64>,
    /// (DEPRECATED) No longer used
    pub include_watchonly: Option<bool>,
    /// Show transactions that were removed due to a reorg in the "removed" array
    /// (not guaranteed to work on pruned nodes)
    pub include_removed: Option<bool>,
    /// Also add entries for change outputs.
    pub include_change: Option<bool>,
    /// Return only incoming transactions paying to addresses with the specified label.
    pub label: Option<String>,
}

/// If a label name is provided, this will return only incoming transactions paying to addresses with the specified label.
/// Returns up to 'count' most recent transactions ordered from oldest to newest while skipping the first number of
/// transactions specified in the 'skip' argument. A transaction can have multiple entries in this RPC response.
/// For instance, a wallet transaction that pays three addresses — one wallet-owned and two external — will produce
/// four entries. The payment to the wallet-owned address appears both as a send entry and as a receive entry.
/// As a result, the RPC response will contain one entry in the receive category and three entries in the send category.
#[derive(Debug, Serialize)]
pub struct ListtransactionsParams {
    /// If set, should be a valid label name to return only incoming transactions
    /// with the specified label, or "*" to disable filtering and return all transactions.
    pub label: Option<String>,
    /// The number of transactions to return
    pub count: Option<i64>,
    /// The number of transactions to skip
    pub skip: Option<i64>,
    /// (DEPRECATED) No longer used
    pub include_watchonly: Option<bool>,
}

/// Returns array of unspent transaction outputs
/// with between minconf and maxconf (inclusive) confirmations.
/// Optionally filter to only include txouts paid to specified addresses.
#[derive(Debug, Serialize)]
pub struct ListunspentParams {
    /// The minimum confirmations to filter
    #[serde(rename = "minconf")]
    pub min_conf: Option<i64>,
    /// The maximum confirmations to filter
    #[serde(rename = "maxconf")]
    pub max_conf: Option<i64>,
    /// The bitcoin addresses to filter
    pub addresses: Option<Vec<serde_json::Value>>,
    /// Include outputs that are not safe to spend
    /// See description of "safe" attribute below.
    pub include_unsafe: Option<bool>,
    pub query_options: Option<serde_json::Value>,
}

/// Load the serialized UTXO set from a file.
/// Once this snapshot is loaded, its contents will be deserialized into a second chainstate data structure, which is then used to sync to the network's tip. Meanwhile, the original chainstate will complete the initial block download process in the background, eventually validating up to the block that the snapshot is based upon.
/// The result is a usable bitcoind instance that is current with the network tip in a matter of minutes rather than hours. UTXO snapshot are typically obtained from third-party sources (HTTP, torrent, etc.) which is reasonable since their contents are always checked by hash.
/// You can find more information on this process in the `assumeutxo` design document (<https://github.com/bitcoin/bitcoin/blob/master/doc/design/assumeutxo.md>).
#[derive(Debug, Serialize)]
pub struct LoadtxoutsetParams {
    /// path to the snapshot file. If relative, will be prefixed by datadir.
    pub path: String,
}

/// Loads a wallet from a wallet file or directory.
/// Note that all wallet command-line options used when starting bitcoind will be
/// applied to the new wallet.
#[derive(Debug, Serialize)]
pub struct LoadwalletParams {
    /// The path to the directory of the wallet to be loaded, either absolute or relative to the "wallets" directory. The "wallets" directory is set by the -walletdir option and defaults to the "wallets" folder within the data directory.
    pub filename: String,
    /// Save wallet name to persistent settings and load on startup. True to add wallet to startup list, false to remove, null to leave unchanged.
    pub load_on_startup: Option<bool>,
}

/// Updates list of temporarily unspendable outputs.
/// Temporarily lock (unlock=false) or unlock (unlock=true) specified transaction outputs.
/// If no transaction outputs are specified when unlocking then all current locked transaction outputs are unlocked.
/// A locked transaction output will not be chosen by automatic coin selection, when spending bitcoins.
/// Manually selected coins are automatically unlocked.
/// Locks are stored in memory only, unless persistent=true, in which case they will be written to the
/// wallet database and loaded on node start. Unwritten (persistent=false) locks are always cleared
/// (by virtue of process exit) when a node stops or fails. Unlocking will clear both persistent and not.
/// Also see the listunspent call
#[derive(Debug, Serialize)]
pub struct LockunspentParams {
    /// Whether to unlock (true) or lock (false) the specified transactions
    pub unlock: bool,
    /// The transaction outputs and within each, the txid (string) vout (numeric).
    pub transactions: Option<Vec<serde_json::Value>>,
    /// Whether to write/erase this lock in the wallet database, or keep the change in memory only. Ignored for unlocking.
    pub persistent: Option<bool>,
}

/// Gets and sets the logging configuration.
/// When called without an argument, returns the list of categories with status that are currently being debug logged or not.
/// When called with arguments, adds or removes categories from debug logging and return the lists above.
/// The arguments are evaluated in order "include", "exclude".
/// If an item is both included and excluded, it will thus end up being excluded.
/// The valid logging categories are: addrman, bench, blockstorage, cmpctblock, coindb, estimatefee, http, i2p, ipc, kernel, leveldb, libevent, mempool, mempoolrej, net, privatebroadcast, proxy, prune, qt, rand, reindex, rpc, scan, selectcoins, tor, txpackages, txreconciliation, validation, walletdb, zmq
/// In addition, the following are available as category names with special meanings:
/// - "all",  "1" : represent all logging categories.
#[derive(Debug, Serialize)]
pub struct LoggingParams {
    /// The categories to add to debug logging
    pub include: Option<Vec<serde_json::Value>>,
    /// The categories to remove from debug logging
    pub exclude: Option<Vec<serde_json::Value>>,
}

/// Migrate the wallet to a descriptor wallet.
/// A new wallet backup will need to be made.
/// The migration process will create a backup of the wallet before migrating. This backup
/// file will be named &lt;wallet name&gt;-&lt;timestamp&gt;.legacy.bak and can be found in the directory
/// for this wallet. In the event of an incorrect migration, the backup can be restored using restorewallet.
/// Encrypted wallets must have the passphrase provided as an argument to this call.
/// This RPC may take a long time to complete. Increasing the RPC client timeout is recommended.
#[derive(Debug, Serialize)]
pub struct MigratewalletParams {
    /// The name of the wallet to migrate. If provided both here and in the RPC endpoint, the two must be identical.
    pub wallet_name: Option<String>,
    /// The wallet passphrase
    pub passphrase: Option<String>,
}

/// Bump the scheduler into the future (-regtest only)
#[derive(Debug, Serialize)]
pub struct MockschedulerParams {
    /// Number of seconds to forward the scheduler into the future.
    pub delta_time: i64,
}

/// Treats a block as if it were received before others with the same work.
/// A later preciousblock call can override the effect of an earlier one.
/// The effects of preciousblock are not retained across restarts.
#[derive(Debug, Serialize)]
pub struct PreciousblockParams {
    /// the hash of the block to mark as precious
    #[serde(rename = "blockhash")]
    pub block_hash: bitcoin::BlockHash,
}

/// Accepts the transaction into mined blocks at a higher (or lower) priority
#[derive(Debug, Serialize)]
pub struct PrioritisetransactionParams {
    /// The transaction id.
    pub txid: bitcoin::Txid,
    /// API-Compatibility for previous API. Must be zero or null.
    /// DEPRECATED. For forward compatibility use named arguments and omit this parameter.
    pub dummy: Option<i64>,
    /// The fee value (in satoshis) to add (or subtract, if negative).
    /// Note, that this value is not a fee rate. It is a value to modify absolute fee of the TX.
    /// The fee is not actually paid, only the algorithm for selecting transactions into a block
    /// considers the transaction as it would have paid a higher (or lower) fee.
    pub fee_delta: i64,
}

/// Attempts to delete block and undo data up to a specified height or timestamp, if eligible for pruning.
/// Requires `-prune` to be enabled at startup. While pruned data may be re-fetched in some cases (e.g., via `getblockfrompeer`), local deletion is irreversible.
#[derive(Debug, Serialize)]
pub struct PruneblockchainParams {
    /// The block height to prune up to. May be set to a discrete height, or to a UNIX epoch time
    /// to prune blocks whose block time is at least 2 hours older than the provided timestamp.
    pub height: i64,
}

/// Bumps the fee of a transaction T, replacing it with a new transaction B.
/// Returns a PSBT instead of creating and signing a new transaction.
/// A transaction with the given txid must be in the wallet.
/// The command will pay the additional fee by reducing change outputs or adding inputs when necessary.
/// It may add a new change output if one does not already exist.
/// All inputs in the original transaction will be included in the replacement transaction.
/// The command will fail if the wallet or mempool contains a transaction that spends one of T's outputs.
/// By default, the new fee will be calculated automatically using the estimatesmartfee RPC.
/// The user can specify a confirmation target for estimatesmartfee.
/// Alternatively, the user can specify a fee rate in sat/vB for the new transaction.
/// At a minimum, the new fee rate must be high enough to pay an additional new relay fee (incrementalfee
/// returned by getnetworkinfo) to enter the node's mempool.
/// * WARNING: before version 0.21, fee_rate was in BTC/kvB. As of 0.21, fee_rate is in sat/vB. *
#[derive(Debug, Serialize)]
pub struct PsbtbumpfeeParams {
    /// The txid to be bumped
    pub txid: bitcoin::Txid,
    pub options: Option<serde_json::Value>,
}

/// Removes invalidity status of a block, its ancestors and its descendants, reconsider them for activation.
/// This can be used to undo the effects of invalidateblock.
#[derive(Debug, Serialize)]
pub struct ReconsiderblockParams {
    /// the hash of the block to reconsider
    #[serde(rename = "blockhash")]
    pub block_hash: bitcoin::BlockHash,
}

/// Deletes the specified transaction from the wallet. Meant for use with pruned wallets and as a companion to importprunedfunds. This will affect wallet balances.
#[derive(Debug, Serialize)]
pub struct RemoveprunedfundsParams {
    /// The hex-encoded id of the transaction you are deleting
    pub txid: bitcoin::Txid,
}

/// Rescan the local blockchain for wallet related transactions.
/// Note: Use "getwalletinfo" to query the scanning progress.
/// The rescan is significantly faster if block filters are available
/// (using startup option "-blockfilterindex=1").
#[derive(Debug, Serialize)]
pub struct RescanblockchainParams {
    /// block height where the rescan should start
    pub start_height: Option<i64>,
    /// the last block height that should be scanned. If none is provided it will rescan up to the tip at return time of this call.
    pub stop_height: Option<i64>,
}

/// Restores and loads a wallet from backup.
/// The rescan is significantly faster if block filters are available
/// (using startup option "-blockfilterindex=1").
#[derive(Debug, Serialize)]
pub struct RestorewalletParams {
    /// The name that will be applied to the restored wallet
    pub wallet_name: String,
    /// The backup file that will be used to restore the wallet.
    pub backup_file: String,
    /// Save wallet name to persistent settings and load on startup. True to add wallet to startup list, false to remove, null to leave unchanged.
    pub load_on_startup: Option<bool>,
}

/// Return relevant blockhashes for given descriptors (requires blockfilterindex).
/// This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[derive(Debug, Serialize)]
pub struct ScanblocksParams {
    /// The action to execute
    /// "start" for starting a scan
    /// "abort" for aborting the current scan (returns true when abort was successful)
    /// "status" for progress report (in %) of the current scan
    pub action: String,
    /// Array of scan objects. Required for "start" action
    /// Every scan object is either a string descriptor or an object:
    pub scanobjects: Option<Vec<serde_json::Value>>,
    /// Height to start to scan from
    pub start_height: Option<i64>,
    /// Height to stop to scan
    pub stop_height: Option<i64>,
    /// The type name of the filter
    pub filtertype: Option<String>,
    pub options: Option<serde_json::Value>,
}

/// Scans the unspent transaction output set for entries that match certain output descriptors.
/// Examples of output descriptors are:
/// addr(&lt;address&gt;)                      Outputs whose output script corresponds to the specified address (does not include P2PK)
/// raw(&lt;hex script&gt;)                    Outputs whose output script equals the specified hex-encoded bytes
/// combo(&lt;pubkey&gt;)                      P2PK, P2PKH, P2WPKH, and P2SH-P2WPKH outputs for the given pubkey
/// pkh(&lt;pubkey&gt;)                        P2PKH outputs for the given pubkey
/// sh(multi(&lt;n&gt;,&lt;pubkey&gt;,&lt;pubkey&gt;,...)) P2SH-multisig outputs for the given threshold and pubkeys
/// tr(&lt;pubkey&gt;)                         P2TR
/// tr(&lt;pubkey&gt;,{pk(&lt;pubkey&gt;)})          P2TR with single fallback pubkey in tapscript
/// rawtr(&lt;pubkey&gt;)                      P2TR with the specified key as output key rather than inner
/// wsh(and_v(v:pk(&lt;pubkey&gt;),after(2)))  P2WSH miniscript with mandatory pubkey and a timelock
/// In the above, &lt;pubkey&gt; either refers to a fixed public key in hexadecimal notation, or to an xpub/xprv optionally followed by one
/// or more path elements separated by "/", and optionally ending in "/*" (unhardened), or "/*'" or "/*h" (hardened) to specify all
/// unhardened or hardened child keys.
/// In the latter case, a range needs to be specified by below if different from 1000.
/// For more information on output descriptors, see the documentation in the doc/descriptors.md file.
#[derive(Debug, Serialize)]
pub struct ScantxoutsetParams {
    /// The action to execute
    /// "start" for starting a scan
    /// "abort" for aborting the current scan (returns true when abort was successful)
    /// "status" for progress report (in %) of the current scan
    pub action: String,
    /// Array of scan objects. Required for "start" action
    /// Every scan object is either a string descriptor or an object:
    pub scanobjects: Option<Vec<serde_json::Value>>,
}

/// EXPERIMENTAL warning: this call may be changed in future releases.
/// Send a transaction.
#[derive(Debug, Serialize)]
pub struct SendParams {
    /// The outputs specified as key-value pairs.
    /// Each key may only appear once, i.e. there can only be one 'data' output, and no address may be duplicated.
    /// At least one output of either type must be specified.
    /// For convenience, a dictionary, which holds the key-value pairs directly, is also accepted.
    pub outputs: Vec<serde_json::Value>,
    /// Confirmation target in blocks
    pub conf_target: Option<i64>,
    /// The fee estimate mode, must be one of (case insensitive):
    /// unset, economical, conservative
    /// unset means no mode set (economical mode is used if the transaction is replaceable;
    /// otherwise, conservative mode is used).
    /// economical estimates use a shorter time horizon, making them more
    /// responsive to short-term drops in the prevailing fee market. This mode
    /// potentially returns a lower fee rate estimate.
    /// conservative estimates use a longer time horizon, making them
    /// less responsive to short-term drops in the prevailing fee market. This mode
    /// potentially returns a higher fee rate estimate.
    pub estimate_mode: Option<String>,
    /// Specify a fee rate in sat/vB.
    #[serde(with = "bitcoin_units::fee_rate::serde::as_sat_per_vb_floor::opt")]
    pub fee_rate: Option<FeeRate>,
    pub options: Option<serde_json::Value>,
    /// Transaction version
    pub version: Option<i64>,
}

/// EXPERIMENTAL warning: this call may be changed in future releases.
/// Spend the value of all (or specific) confirmed UTXOs and unconfirmed change in the wallet to one or more recipients.
/// Unconfirmed inbound UTXOs and locked UTXOs will not be spent. Sendall will respect the avoid_reuse wallet flag.
/// If your wallet contains many small inputs, either because it received tiny payments or as a result of accumulating change, consider using `send_max` to exclude inputs that are worth less than the fees needed to spend them.
#[derive(Debug, Serialize)]
pub struct SendallParams {
    /// The sendall destinations. Each address may only appear once.
    /// Optionally some recipients can be specified with an amount to perform payments, but at least one address must appear without a specified amount.
    pub recipients: Vec<SendallRecipient>,
    /// Confirmation target in blocks
    pub conf_target: Option<i64>,
    /// The fee estimate mode, must be one of (case insensitive):
    /// unset, economical, conservative
    /// unset means no mode set (economical mode is used if the transaction is replaceable;
    /// otherwise, conservative mode is used).
    /// economical estimates use a shorter time horizon, making them more
    /// responsive to short-term drops in the prevailing fee market. This mode
    /// potentially returns a lower fee rate estimate.
    /// conservative estimates use a longer time horizon, making them
    /// less responsive to short-term drops in the prevailing fee market. This mode
    /// potentially returns a higher fee rate estimate.
    pub estimate_mode: Option<String>,
    /// Specify a fee rate in sat/vB.
    #[serde(with = "bitcoin_units::fee_rate::serde::as_sat_per_vb_floor::opt")]
    pub fee_rate: Option<FeeRate>,
    pub options: Option<serde_json::Value>,
}

/// Send multiple times. Amounts are double-precision floating point numbers.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
#[derive(Debug, Serialize)]
pub struct SendmanyParams {
    /// Must be set to "" for backwards compatibility.
    pub dummy: Option<String>,
    /// The addresses and amounts
    #[serde(with = "crate::bitcoin_core_client::params::serde_amounts_map")]
    pub amounts: std::collections::HashMap<
        bitcoin::Address<bitcoin::address::NetworkUnchecked>,
        bitcoin::Amount,
    >,
    /// Ignored dummy value
    #[serde(rename = "minconf")]
    pub min_conf: Option<i64>,
    /// A comment
    pub comment: Option<String>,
    /// The addresses.
    /// The fee will be equally deducted from the amount of each selected address.
    /// Those recipients will receive less bitcoins than you enter in their corresponding amount field.
    /// If no addresses are specified here, the sender pays the fee.
    pub subtractfeefrom: Option<Vec<bitcoin::Address<bitcoin::address::NetworkUnchecked>>>,
    /// Signal that this transaction can be replaced by a transaction (BIP 125)
    pub replaceable: Option<bool>,
    /// Confirmation target in blocks
    pub conf_target: Option<i64>,
    /// The fee estimate mode, must be one of (case insensitive):
    /// unset, economical, conservative
    /// unset means no mode set (economical mode is used if the transaction is replaceable;
    /// otherwise, conservative mode is used).
    /// economical estimates use a shorter time horizon, making them more
    /// responsive to short-term drops in the prevailing fee market. This mode
    /// potentially returns a lower fee rate estimate.
    /// conservative estimates use a longer time horizon, making them
    /// less responsive to short-term drops in the prevailing fee market. This mode
    /// potentially returns a higher fee rate estimate.
    pub estimate_mode: Option<String>,
    /// Specify a fee rate in sat/vB.
    #[serde(with = "bitcoin_units::fee_rate::serde::as_sat_per_vb_floor::opt")]
    pub fee_rate: Option<FeeRate>,
    /// If true, return extra information about the transaction.
    pub verbose: Option<bool>,
}

/// Send a p2p message to a peer specified by id.
/// The message type and body must be provided, the message header will be generated.
/// This RPC is for testing only.
#[derive(Debug, Serialize)]
pub struct SendmsgtopeerParams {
    /// The peer to send the message to.
    pub peer_id: i64,
    /// The message type (maximum length 12)
    pub msg_type: String,
    /// The serialized message body to send, in hex, without a message header
    pub msg: String,
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
#[derive(Debug, Serialize)]
pub struct SendrawtransactionParams {
    /// The hex string of the raw transaction
    pub hexstring: String,
    /// Reject transactions whose fee rate is higher than the specified value, expressed in BTC/kvB.
    /// Fee rates larger than 1BTC/kvB are rejected.
    /// Set to 0 to accept any fee rate.
    #[serde(with = "crate::bitcoin_core_client::params::serde_fee_rate::maxfeerate_opt")]
    #[serde(rename = "maxfeerate")]
    pub max_fee_rate: Option<FeeRate>,
    /// Reject transactions with provably unspendable outputs (e.g. 'datacarrier' outputs that use the OP_RETURN opcode) greater than the specified value, expressed in BTC.
    /// If burning funds through unspendable outputs is desired, increase this value.
    /// This check is based on heuristics and does not guarantee spendability of outputs.
    #[serde(rename = "maxburnamount")]
    pub max_burn_amount: Option<bitcoin::Amount>,
}

/// Send an amount to a given address.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
#[derive(Debug, Serialize)]
pub struct SendtoaddressParams {
    /// The bitcoin address to send to.
    pub address: bitcoin::Address,
    /// The amount in BTC to send. eg 0.1
    pub amount: bitcoin::Amount,
    /// A comment used to store what the transaction is for.
    /// This is not part of the transaction, just kept in your wallet.
    pub comment: Option<String>,
    /// A comment to store the name of the person or organization
    /// to which you're sending the transaction. This is not part of the
    /// transaction, just kept in your wallet.
    pub comment_to: Option<String>,
    /// The fee will be deducted from the amount being sent.
    /// The recipient will receive less bitcoins than you enter in the amount field.
    #[serde(rename = "subtractfeefromamount")]
    pub subtract_fee_from_amount: Option<bool>,
    /// Signal that this transaction can be replaced by a transaction (BIP 125)
    pub replaceable: Option<bool>,
    /// Confirmation target in blocks
    pub conf_target: Option<i64>,
    /// The fee estimate mode, must be one of (case insensitive):
    /// unset, economical, conservative
    /// unset means no mode set (economical mode is used if the transaction is replaceable;
    /// otherwise, conservative mode is used).
    /// economical estimates use a shorter time horizon, making them more
    /// responsive to short-term drops in the prevailing fee market. This mode
    /// potentially returns a lower fee rate estimate.
    /// conservative estimates use a longer time horizon, making them
    /// less responsive to short-term drops in the prevailing fee market. This mode
    /// potentially returns a higher fee rate estimate.
    pub estimate_mode: Option<String>,
    /// (only available if avoid_reuse wallet flag is set) Avoid spending from dirty addresses; addresses are considered
    /// dirty if they have previously been used in a transaction. If true, this also activates avoidpartialspends, grouping outputs by their addresses.
    pub avoid_reuse: Option<bool>,
    /// Specify a fee rate in sat/vB.
    #[serde(with = "bitcoin_units::fee_rate::serde::as_sat_per_vb_floor::opt")]
    pub fee_rate: Option<FeeRate>,
    /// If true, return extra information about the transaction.
    pub verbose: Option<bool>,
}

/// Attempts to add or remove an IP/Subnet from the banned list.
#[derive(Debug, Serialize)]
pub struct SetbanParams {
    /// The IP/Subnet (see getpeerinfo for nodes IP) with an optional netmask (default is /32 = single IP)
    pub subnet: String,
    /// 'add' to add an IP/Subnet to the list, 'remove' to remove an IP/Subnet from the list
    pub command: String,
    /// time in seconds how long (or until when if \[absolute\] is set) the IP is banned (0 or empty means using the default time of 24h which can also be overwritten by the -bantime startup argument)
    pub bantime: Option<i64>,
    /// If set, the bantime must be an absolute timestamp expressed in UNIX epoch time
    pub absolute: Option<bool>,
}

/// Sets the label associated with the given address.
#[derive(Debug, Serialize)]
pub struct SetlabelParams {
    /// The bitcoin address to be associated with a label.
    pub address: bitcoin::Address,
    /// The label to assign to the address.
    pub label: String,
}

/// Set the local time to given timestamp (-regtest only)
#[derive(Debug, Serialize)]
pub struct SetmocktimeParams {
    /// UNIX epoch time
    /// Pass 0 to go back to using the system time.
    pub timestamp: i64,
}

/// Disable/enable all p2p network activity.
#[derive(Debug, Serialize)]
pub struct SetnetworkactiveParams {
    /// true to enable networking, false to disable
    pub state: bool,
}

/// Change the state of the given wallet flag for a wallet.
#[derive(Debug, Serialize)]
pub struct SetwalletflagParams {
    /// The name of the flag to change. Current available flags: avoid_reuse
    pub flag: String,
    /// The new state.
    pub value: Option<bool>,
}

/// Sign a message with the private key of an address
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
#[derive(Debug, Serialize)]
pub struct SignmessageParams {
    /// The bitcoin address to use for the private key.
    pub address: bitcoin::Address,
    /// The message to create a signature of.
    pub message: String,
}

/// Sign a message with the private key of an address
#[derive(Debug, Serialize)]
pub struct SignmessagewithprivkeyParams {
    /// The private key to sign the message with.
    pub privkey: String,
    /// The message to create a signature of.
    pub message: String,
}

/// Sign inputs for raw transaction (serialized, hex-encoded).
/// The second argument is an array of base58-encoded private
/// keys that will be the only keys used to sign the transaction.
/// The third optional argument (may be null) is an array of previous transaction outputs that
/// this transaction depends on but may not yet be in the block chain.
#[derive(Debug, Serialize)]
pub struct SignrawtransactionwithkeyParams {
    /// The transaction hex string
    pub hexstring: String,
    /// The base58-encoded private keys for signing
    pub privkeys: Vec<serde_json::Value>,
    /// The previous dependent transaction outputs
    pub prevtxs: Option<Vec<serde_json::Value>>,
    /// The signature hash type. Must be one of:
    /// "DEFAULT"
    /// "ALL"
    /// "NONE"
    /// "SINGLE"
    /// "ALL|ANYONECANPAY"
    /// "NONE|ANYONECANPAY"
    /// "SINGLE|ANYONECANPAY"
    pub sighashtype: Option<String>,
}

/// Sign inputs for raw transaction (serialized, hex-encoded).
/// The second optional argument (may be null) is an array of previous transaction outputs that
/// this transaction depends on but may not yet be in the block chain.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
#[derive(Debug, Serialize)]
pub struct SignrawtransactionwithwalletParams {
    /// The transaction hex string
    pub hexstring: String,
    /// The previous dependent transaction outputs
    pub prevtxs: Option<Vec<serde_json::Value>>,
    /// The signature hash type. Must be one of
    /// "DEFAULT"
    /// "ALL"
    /// "NONE"
    /// "SINGLE"
    /// "ALL|ANYONECANPAY"
    /// "NONE|ANYONECANPAY"
    /// "SINGLE|ANYONECANPAY"
    pub sighashtype: Option<String>,
}

/// Calculate the balance change resulting in the signing and broadcasting of the given transaction(s).
#[derive(Debug, Serialize)]
pub struct SimulaterawtransactionParams {
    /// An array of hex strings of raw transactions.
    pub rawtxs: Option<Vec<serde_json::Value>>,
    pub options: Option<serde_json::Value>,
}

/// Request a graceful shutdown of Bitcoin Core.
#[derive(Debug, Serialize)]
pub struct StopParams {
    /// how long to wait in ms
    pub wait: Option<i64>,
}

/// Attempts to submit new block to network.
/// See <https://en.bitcoin.it/wiki/BIP_0022> for full specification.
#[derive(Debug, Serialize)]
pub struct SubmitblockParams {
    /// the hex-encoded block data to submit
    pub hexdata: String,
    /// dummy value, for compatibility with BIP22. This value is ignored.
    pub dummy: Option<String>,
}

/// Decode the given hexdata as a header and submit it as a candidate chain tip if valid.
/// Throws when the header is invalid.
#[derive(Debug, Serialize)]
pub struct SubmitheaderParams {
    /// the hex-encoded block header data
    pub hexdata: String,
}

/// Submit a package of raw transactions (serialized, hex-encoded) to local node.
/// The package will be validated according to consensus and mempool policy rules. If any transaction passes, it will be accepted to mempool.
/// This RPC is experimental and the interface may be unstable. Refer to doc/policy/packages.md for documentation on package policies.
/// Warning: successful submission does not mean the transactions will propagate throughout the network.
#[derive(Debug, Serialize)]
pub struct SubmitpackageParams {
    /// An array of raw transactions.
    /// The package must consist of a transaction with (some, all, or none of) its unconfirmed parents. A single transaction is permitted.
    /// None of the parents may depend on each other. Parents that are already in mempool do not need to be present in the package.
    /// The package must be topologically sorted, with the child being the last element in the array if there are multiple elements.
    pub package: Vec<serde_json::Value>,
    /// Reject transactions whose fee rate is higher than the specified value, expressed in BTC/kvB.
    /// Fee rates larger than 1BTC/kvB are rejected.
    /// Set to 0 to accept any fee rate.
    #[serde(with = "crate::bitcoin_core_client::params::serde_fee_rate::maxfeerate_opt")]
    #[serde(rename = "maxfeerate")]
    pub max_fee_rate: Option<FeeRate>,
    /// Reject transactions with provably unspendable outputs (e.g. 'datacarrier' outputs that use the OP_RETURN opcode) greater than the specified value, expressed in BTC.
    /// If burning funds through unspendable outputs is desired, increase this value.
    /// This check is based on heuristics and does not guarantee spendability of outputs.
    #[serde(rename = "maxburnamount")]
    pub max_burn_amount: Option<bitcoin::Amount>,
}

/// Returns result of mempool acceptance tests indicating if raw transaction(s) (serialized, hex-encoded) would be accepted by mempool.
/// If multiple transactions are passed in, parents must come before children and package policies apply: the transactions cannot conflict with any mempool transactions or each other.
/// If one transaction fails, other transactions may not be fully validated (the 'allowed' key will be blank).
/// The maximum number of transactions allowed is 25.
/// This checks if transactions violate the consensus or policy rules.
/// See sendrawtransaction call.
#[derive(Debug, Serialize)]
pub struct TestmempoolacceptParams {
    /// An array of hex strings of raw transactions.
    pub rawtxs: Vec<serde_json::Value>,
    /// Reject transactions whose fee rate is higher than the specified value, expressed in BTC/kvB.
    /// Fee rates larger than 1BTC/kvB are rejected.
    /// Set to 0 to accept any fee rate.
    #[serde(with = "crate::bitcoin_core_client::params::serde_fee_rate::maxfeerate_opt")]
    #[serde(rename = "maxfeerate")]
    pub max_fee_rate: Option<FeeRate>,
}

/// Unloads the wallet referenced by the request endpoint or the wallet_name argument.
/// If both are specified, they must be identical.
#[derive(Debug, Serialize)]
pub struct UnloadwalletParams {
    /// The name of the wallet to unload. If provided both here and in the RPC endpoint, the two must be identical.
    pub wallet_name: Option<String>,
    /// Save wallet name to persistent settings and load on startup. True to add wallet to startup list, false to remove, null to leave unchanged.
    pub load_on_startup: Option<bool>,
}

/// Updates all segwit inputs and outputs in a PSBT with data from output descriptors, the UTXO set, txindex, or the mempool.
#[derive(Debug, Serialize)]
pub struct UtxoupdatepsbtParams {
    /// A base64 string of a PSBT
    pub psbt: String,
    /// An array of either strings or objects
    pub descriptors: Option<Vec<serde_json::Value>>,
}

/// Return information about the given bitcoin address.
#[derive(Debug, Serialize)]
pub struct ValidateaddressParams {
    /// The bitcoin address to validate
    pub address: bitcoin::Address,
}

/// Verifies blockchain database.
#[derive(Debug, Serialize)]
pub struct VerifychainParams {
    /// How thorough the block verification is:
    /// - level 0 reads the blocks from disk
    /// - level 1 verifies block validity
    /// - level 2 verifies undo data
    /// - level 3 checks disconnection of tip blocks
    /// - level 4 tries to reconnect the blocks
    /// - each level includes the checks of the previous levels
    pub checklevel: Option<i64>,
    /// The number of blocks to check.
    pub nblocks: Option<i64>,
}

/// Verify a signed message.
#[derive(Debug, Serialize)]
pub struct VerifymessageParams {
    /// The bitcoin address to use for the signature.
    pub address: bitcoin::Address,
    /// The signature provided by the signer in base 64 encoding (see signmessage).
    pub signature: String,
    /// The message that was signed.
    pub message: String,
}

/// Verifies that a proof points to a transaction in a block, returning the transaction it commits to
/// and throwing an RPC error if the block is not in our best chain
#[derive(Debug, Serialize)]
pub struct VerifytxoutproofParams {
    /// The hex-encoded proof generated by gettxoutproof
    pub proof: String,
}

/// Waits for a specific new block and returns useful info about it.
/// Returns the current block on timeout or exit.
/// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[derive(Debug, Serialize)]
pub struct WaitforblockParams {
    /// Block hash to wait for.
    #[serde(rename = "blockhash")]
    pub block_hash: bitcoin::BlockHash,
    /// Time in milliseconds to wait for a response. 0 indicates no timeout.
    pub timeout: Option<i64>,
}

/// Waits for (at least) block height and returns the height and hash
/// of the current tip.
/// Returns the current block on timeout or exit.
/// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[derive(Debug, Serialize)]
pub struct WaitforblockheightParams {
    /// Block height to wait for.
    pub height: i64,
    /// Time in milliseconds to wait for a response. 0 indicates no timeout.
    pub timeout: Option<i64>,
}

/// Waits for any new block and returns useful info about it.
/// Returns the current block on timeout or exit.
/// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[derive(Debug, Serialize)]
pub struct WaitfornewblockParams {
    /// Time in milliseconds to wait for a response. 0 indicates no timeout.
    pub timeout: Option<i64>,
    /// Method waits for the chain tip to differ from this.
    pub current_tip: Option<String>,
}

/// Creates and funds a transaction in the Partially Signed Transaction format.
/// Implements the Creator and Updater roles.
/// All existing inputs must either have their previous output transaction be in the wallet
/// or be in the UTXO set. Solving data must be provided for non-wallet inputs.
#[derive(Debug, Serialize)]
pub struct WalletcreatefundedpsbtParams {
    /// Leave empty to add inputs automatically. See add_inputs option.
    pub inputs: Option<Vec<serde_json::Value>>,
    /// The outputs specified as key-value pairs.
    /// Each key may only appear once, i.e. there can only be one 'data' output, and no address may be duplicated.
    /// At least one output of either type must be specified.
    /// For compatibility reasons, a dictionary, which holds the key-value pairs directly, is also
    /// accepted as second parameter.
    pub outputs: Vec<serde_json::Value>,
    /// Raw locktime. Non-0 value also locktime-activates inputs
    #[serde(rename = "locktime")]
    pub lock_time: Option<i64>,
    pub options: Option<serde_json::Value>,
    /// Include BIP 32 derivation paths for public keys if we know them
    pub bip32derivs: Option<bool>,
    /// Transaction version
    pub version: Option<i64>,
}

/// Display address on an external signer for verification.
#[derive(Debug, Serialize)]
pub struct WalletdisplayaddressParams {
    /// bitcoin address to display
    pub address: bitcoin::Address,
}

/// Stores the wallet decryption key in memory for 'timeout' seconds.
/// This is needed prior to performing transactions related to private keys such as sending bitcoins
/// Note:
/// Issuing the walletpassphrase command while the wallet is already unlocked will set a new unlock
/// time that overrides the old one.
#[derive(Debug, Serialize)]
pub struct WalletpassphraseParams {
    /// The wallet passphrase
    pub passphrase: String,
    /// The time to keep the decryption key in seconds; capped at 100000000 (~3 years).
    pub timeout: i64,
}

/// Changes the wallet passphrase from 'oldpassphrase' to 'newpassphrase'.
#[derive(Debug, Serialize)]
pub struct WalletpassphrasechangeParams {
    /// The current passphrase
    #[serde(rename = "oldpassphrase")]
    pub old_passphrase: String,
    /// The new passphrase
    #[serde(rename = "newpassphrase")]
    pub new_passphrase: String,
}

/// Update a PSBT with input information from our wallet and then sign inputs
/// that we can sign for.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
#[derive(Debug, Serialize)]
pub struct WalletprocesspsbtParams {
    /// The transaction base64 string
    pub psbt: String,
    /// Also sign the transaction when updating (requires wallet to be unlocked)
    pub sign: Option<bool>,
    /// The signature hash type to sign with if not specified by the PSBT. Must be one of
    /// "DEFAULT"
    /// "ALL"
    /// "NONE"
    /// "SINGLE"
    /// "ALL|ANYONECANPAY"
    /// "NONE|ANYONECANPAY"
    /// "SINGLE|ANYONECANPAY"
    pub sighashtype: Option<String>,
    /// Include BIP 32 derivation paths for public keys if we know them
    pub bip32derivs: Option<bool>,
    /// Also finalize inputs if possible
    pub finalize: Option<bool>,
}
