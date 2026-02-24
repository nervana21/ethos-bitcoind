// Generated client trait for Bitcoin Core v30.2

use std::future::Future;

use async_trait::async_trait;
use serde::de::DeserializeOwned;

use crate::transport::core::TransportExt;
use crate::transport::{TransportError, TransportTrait};
use crate::types::*;

#[doc = r#"A versioned client trait for Bitcoin Core v30.2"#]
#[async_trait]
pub trait BitcoinClient: Send + Sync + TransportTrait + TransportExt + RpcDispatchExt {
    type Error;

    /// Mark in-wallet transaction &lt;txid&gt; as abandoned
    /// This will mark this transaction and all its in-wallet descendants as abandoned which will allow
    /// for their inputs to be respent.  It can be used to replace "stuck" or evicted transactions.
    /// It only works on transactions which are not included in a block and are not currently in the mempool.
    /// It has no effect on transactions which are already abandoned.
    async fn abandon_transaction(
        &self,
        txid: bitcoin::Txid,
    ) -> Result<AbandonTransactionResponse, Self::Error>;

    /// Stops current wallet rescan triggered by an RPC call, e.g. by a rescanblockchain call.
    /// Note: Use "getwalletinfo" to query the scanning progress.
    async fn abort_rescan(&self) -> Result<AbortRescanResponse, Self::Error>;

    /// Open an outbound connection to a specified node. This RPC is for testing only.
    async fn add_connection(
        &self,
        address: bitcoin::Address,
        connection_type: String,
        v2transport: bool,
    ) -> Result<AddConnectionResponse, Self::Error>;

    /// Attempts to add or remove a node from the addnode list.
    /// Or try a connection to a node once.
    /// Nodes added using addnode (or -connect) are protected from DoS disconnection and are not required to be
    /// full nodes/support SegWit as other outbound peers are (though such peers will not be synced from).
    /// Addnode connections are limited to 8 at a time and are counted separately from the -maxconnections limit.
    async fn add_node(
        &self,
        node: String,
        command: String,
        v2transport: Option<bool>,
    ) -> Result<AddNodeResponse, Self::Error>;

    /// Add the address of a potential peer to an address manager table. This RPC is for testing only.
    async fn add_peer_address(
        &self,
        address: bitcoin::Address,
        port: i64,
        tried: Option<bool>,
    ) -> Result<AddPeerAddressResponse, Self::Error>;

    /// Analyzes and provides information about the current status of a PSBT and its inputs
    async fn analyze_psbt(&self, psbt: String) -> Result<AnalyzePsbtResponse, Self::Error>;

    /// Safely copies the current wallet file to the specified destination, which can either be a directory or a path with a filename.
    async fn backup_wallet(&self, destination: String)
        -> Result<BackupWalletResponse, Self::Error>;

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
    async fn bump_fee(
        &self,
        txid: bitcoin::Txid,
        options: Option<serde_json::Value>,
    ) -> Result<BumpFeeResponse, Self::Error>;

    /// Clear all banned IPs.
    async fn clear_banned(&self) -> Result<ClearBannedResponse, Self::Error>;

    /// Combine multiple partially signed Bitcoin transactions into one transaction.
    /// Implements the Combiner role.
    async fn combine_psbt(
        &self,
        txs: Vec<serde_json::Value>,
    ) -> Result<CombinePsbtResponse, Self::Error>;

    /// Combine multiple partially signed transactions into one transaction.
    /// The combined transaction may be another partially signed transaction or a
    /// fully signed transaction.
    async fn combine_raw_transaction(
        &self,
        txs: Vec<serde_json::Value>,
    ) -> Result<CombineRawTransactionResponse, Self::Error>;

    /// Converts a network serialized transaction to a PSBT. This should be used only with createrawtransaction and fundrawtransaction
    /// createpsbt and walletcreatefundedpsbt should be used for new applications.
    async fn convert_to_psbt(
        &self,
        hexstring: String,
        permitsigdata: Option<bool>,
        iswitness: Option<bool>,
    ) -> Result<ConvertToPsbtResponse, Self::Error>;

    /// Creates a multi-signature address with n signatures of m keys required.
    /// It returns a json object with the address and redeemScript.
    async fn create_multisig(
        &self,
        nrequired: i64,
        keys: Vec<serde_json::Value>,
        address_type: Option<String>,
    ) -> Result<CreateMultisigResponse, Self::Error>;

    /// Creates a transaction in the Partially Signed Transaction format.
    /// Implements the Creator role.
    /// Note that the transaction's inputs are not signed, and
    /// it is not stored in the wallet or transmitted to the network.
    async fn create_psbt(
        &self,
        inputs: Vec<serde_json::Value>,
        outputs: Vec<serde_json::Value>,
        locktime: Option<i64>,
        replaceable: Option<bool>,
        version: Option<i64>,
    ) -> Result<CreatePsbtResponse, Self::Error>;

    /// Create a transaction spending the given inputs and creating new outputs.
    /// Outputs can be addresses or data.
    /// Returns hex-encoded raw transaction.
    /// Note that the transaction's inputs are not signed, and
    /// it is not stored in the wallet or transmitted to the network.
    async fn create_raw_transaction(
        &self,
        inputs: Vec<serde_json::Value>,
        outputs: Vec<serde_json::Value>,
        locktime: Option<i64>,
        replaceable: Option<bool>,
        version: Option<i64>,
    ) -> Result<CreateRawTransactionResponse, Self::Error>;

    /// Creates and loads a new wallet.
    async fn create_wallet(
        &self,
        wallet_name: String,
        disable_private_keys: Option<bool>,
        blank: Option<bool>,
        passphrase: Option<String>,
        avoid_reuse: Option<bool>,
        descriptors: Option<bool>,
        load_on_startup: Option<bool>,
        external_signer: Option<bool>,
    ) -> Result<CreateWalletResponse, Self::Error>;

    /// Creates the wallet's descriptor for the given address type. The address type must be one that the wallet does not already have a descriptor for.
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    async fn create_wallet_descriptor(
        &self,
        r#type: String,
        options: Option<serde_json::Value>,
    ) -> Result<CreateWalletDescriptorResponse, Self::Error>;

    /// Return a JSON object representing the serialized, base64-encoded partially signed Bitcoin transaction.
    async fn decode_psbt(&self, psbt: String) -> Result<DecodePsbtResponse, Self::Error>;

    /// Return a JSON object representing the serialized, hex-encoded transaction.
    async fn decode_raw_transaction(
        &self,
        hexstring: String,
        iswitness: Option<bool>,
    ) -> Result<DecodeRawTransactionResponse, Self::Error>;

    /// Decode a hex-encoded script.
    async fn decode_script(&self, hexstring: String) -> Result<DecodeScriptResponse, Self::Error>;

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
    async fn derive_addresses(
        &self,
        descriptor: String,
        range: Option<serde_json::Value>,
    ) -> Result<DeriveAddressesResponse, Self::Error>;

    /// Update all segwit inputs in a PSBT with information from output descriptors, the UTXO set or the mempool.
    /// Then, sign the inputs we are able to with information from the output descriptors.
    async fn descriptor_process_psbt(
        &self,
        psbt: String,
        descriptors: Vec<serde_json::Value>,
        sighashtype: Option<String>,
        bip32derivs: Option<bool>,
        finalize: Option<bool>,
    ) -> Result<DescriptorProcessPsbtResponse, Self::Error>;

    /// Immediately disconnects from the specified peer node.
    /// Strictly one out of 'address' and 'nodeid' can be provided to identify the node.
    /// To disconnect by nodeid, either set 'address' to the empty string, or call using the named 'nodeid' argument only.
    async fn disconnect_node(
        &self,
        address: Option<bitcoin::Address>,
        nodeid: Option<i64>,
    ) -> Result<DisconnectNodeResponse, Self::Error>;

    /// Write the serialized UTXO set to a file. This can be used in loadtxoutset afterwards if this snapshot height is supported in the chainparams as well.
    /// Unless the "latest" type is requested, the node will roll back to the requested height and network activity will be suspended during this process. Because of this it is discouraged to interact with the node in any other way during the execution of this call to avoid inconsistent results and race conditions, particularly RPCs that interact with blockstorage.
    /// This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    async fn dump_txout_set(
        &self,
        path: String,
        r#type: Option<String>,
        options: Option<serde_json::Value>,
    ) -> Result<DumpTxOutSetResponse, Self::Error>;

    /// Simply echo back the input arguments. This command is for testing.
    /// It will return an internal bug report when arg9='trigger_internal_bug' is passed.
    /// The difference between echo and echojson is that echojson has argument conversion enabled in the client-side table in bitcoin-cli and the GUI. There is no server-side difference.
    async fn echo(
        &self,
        arg0: Option<String>,
        arg1: Option<String>,
        arg2: Option<String>,
        arg3: Option<String>,
        arg4: Option<String>,
        arg5: Option<String>,
        arg6: Option<String>,
        arg7: Option<String>,
        arg8: Option<String>,
        arg9: Option<String>,
    ) -> Result<EchoResponse, Self::Error>;

    /// Echo back the input argument, passing it through a spawned process in a multiprocess build.
    /// This command is for testing.
    async fn echoipc(&self, arg: String) -> Result<EchoipcResponse, Self::Error>;

    /// Simply echo back the input arguments. This command is for testing.
    /// It will return an internal bug report when arg9='trigger_internal_bug' is passed.
    /// The difference between echo and echojson is that echojson has argument conversion enabled in the client-side table in bitcoin-cli and the GUI. There is no server-side difference.
    async fn echojson(
        &self,
        arg0: Option<String>,
        arg1: Option<String>,
        arg2: Option<String>,
        arg3: Option<String>,
        arg4: Option<String>,
        arg5: Option<String>,
        arg6: Option<String>,
        arg7: Option<String>,
        arg8: Option<String>,
        arg9: Option<String>,
    ) -> Result<EchojsonResponse, Self::Error>;

    /// Encrypts the wallet with 'passphrase'. This is for first time encryption.
    /// After this, any calls that interact with private keys such as sending or signing
    /// will require the passphrase to be set prior to making these calls.
    /// Use the walletpassphrase call for this, and then walletlock call.
    /// If the wallet is already encrypted, use the walletpassphrasechange call.
    /// ** IMPORTANT **
    /// For security reasons, the encryption process will generate a new HD seed, resulting
    /// in the creation of a fresh set of active descriptors. Therefore, it is crucial to
    /// securely back up the newly generated wallet file using the backupwallet RPC.
    ///
    /// Requires wallet private keys to be available (e.g. unlocked).
    async fn encrypt_wallet(
        &self,
        passphrase: String,
    ) -> Result<EncryptWalletResponse, Self::Error>;

    /// Returns a list of external signers from -signer.
    async fn enumerate_signers(&self) -> Result<EnumerateSignersResponse, Self::Error>;

    /// WARNING: This interface is unstable and may disappear or change!
    /// WARNING: This is an advanced API call that is tightly coupled to the specific
    /// implementation of fee estimation. The parameters it can be called with
    /// and the results it returns will change if the internal implementation changes.
    /// Estimates the approximate fee per kilobyte needed for a transaction to begin
    /// confirmation within conf_target blocks if possible. Uses virtual transaction size as
    /// defined in BIP 141 (witness data is discounted).
    async fn estimate_raw_fee(
        &self,
        conf_target: i64,
        threshold: Option<i64>,
    ) -> Result<EstimateRawFeeResponse, Self::Error>;

    /// Estimates the approximate fee per kilobyte needed for a transaction to begin
    /// confirmation within conf_target blocks if possible and return the number of blocks
    /// for which the estimate is valid. Uses virtual transaction size as defined
    /// in BIP 141 (witness data is discounted).
    async fn estimate_smart_fee(
        &self,
        conf_target: i64,
        estimate_mode: Option<String>,
    ) -> Result<EstimateSmartFeeResponse, Self::Error>;

    /// Finalize the inputs of a PSBT. If the transaction is fully signed, it will produce a
    /// network serialized transaction which can be broadcast with sendrawtransaction. Otherwise a PSBT will be
    /// created which has the final_scriptSig and final_scriptwitness fields filled for inputs that are complete.
    /// Implements the Finalizer and Extractor roles.
    async fn finalize_psbt(
        &self,
        psbt: String,
        extract: Option<bool>,
    ) -> Result<FinalizePsbtResponse, Self::Error>;

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
    async fn fund_raw_transaction(
        &self,
        hexstring: String,
        options: Option<serde_json::Value>,
        iswitness: Option<bool>,
    ) -> Result<FundRawTransactionResponse, Self::Error>;

    /// has been replaced by the -generate cli option. Refer to -help for more information.
    async fn generate(&self) -> Result<(), Self::Error>;

    /// Mine a set of ordered transactions to a specified address or descriptor and return the block hash.
    async fn generate_block(
        &self,
        output: String,
        transactions: Vec<serde_json::Value>,
        submit: Option<bool>,
    ) -> Result<GenerateBlockResponse, Self::Error>;

    /// Mine to a specified address and return the block hashes.
    async fn generate_to_address(
        &self,
        nblocks: i64,
        address: bitcoin::Address,
        maxtries: Option<i64>,
    ) -> Result<GenerateToAddressResponse, Self::Error>;

    /// Mine to a specified descriptor and return the block hashes.
    async fn generate_to_descriptor(
        &self,
        num_blocks: i64,
        descriptor: String,
        maxtries: Option<i64>,
    ) -> Result<GenerateToDescriptorResponse, Self::Error>;

    /// Returns information about the given added node, or all added nodes
    /// (note that onetry addnodes are not listed here)
    async fn get_added_node_info(
        &self,
        node: Option<String>,
    ) -> Result<GetAddedNodeInfoResponse, Self::Error>;

    /// Returns the list of addresses assigned the specified label.
    async fn get_addresses_by_label(
        &self,
        label: String,
    ) -> Result<GetAddressesByLabelResponse, Self::Error>;

    /// Return information about the given bitcoin address.
    /// Some of the information will only be present if the address is in the active wallet.
    async fn get_address_info(
        &self,
        address: bitcoin::Address,
    ) -> Result<GetAddressInfoResponse, Self::Error>;

    /// Provides information about the node's address manager by returning the number of addresses in the `new` and `tried` tables and their sum for all networks.
    async fn get_addrman_info(&self) -> Result<GetAddrManInfoResponse, Self::Error>;

    /// Returns the total available balance.
    /// The available balance is what the wallet considers currently spendable, and is
    /// thus affected by options which limit spendability such as -spendzeroconfchange.
    async fn get_balance(
        &self,
        dummy: Option<String>,
        minconf: Option<i64>,
        include_watchonly: Option<bool>,
        avoid_reuse: Option<bool>,
    ) -> Result<GetBalanceResponse, Self::Error>;

    /// Returns an object with all balances in BTC.
    async fn get_balances(&self) -> Result<GetBalancesResponse, Self::Error>;

    /// Returns the hash of the best (tip) block in the most-work fully-validated chain.
    async fn get_best_block_hash(&self) -> Result<GetBestBlockHashResponse, Self::Error>;

    /// If verbosity is 0, returns a string that is serialized, hex-encoded data for block 'hash'.
    /// If verbosity is 1, returns an Object with information about block &lt;hash&gt;.
    /// If verbosity is 2, returns an Object with information about block &lt;hash&gt; and information about each transaction.
    /// If verbosity is 3, returns an Object with information about block &lt;hash&gt; and information about each transaction, including prevout information for inputs (only for unpruned blocks in the current best chain).
    async fn get_block(
        &self,
        blockhash: bitcoin::BlockHash,
        verbosity: Option<i64>,
    ) -> Result<GetBlockResponse, Self::Error>;

    /// Returns an object containing various state info regarding blockchain processing.
    async fn get_blockchain_info(&self) -> Result<GetBlockchainInfoResponse, Self::Error>;

    /// Returns the height of the most-work fully-validated chain.
    /// The genesis block has height 0.
    async fn get_block_count(&self) -> Result<GetBlockCountResponse, Self::Error>;

    /// Retrieve a BIP 157 content filter for a particular block.
    async fn get_block_filter(
        &self,
        blockhash: bitcoin::BlockHash,
        filtertype: Option<String>,
    ) -> Result<GetBlockFilterResponse, Self::Error>;

    /// Attempt to fetch block from a given peer.
    /// We must have the header for this block, e.g. using submitheader.
    /// The block will not have any undo data which can limit the usage of the block data in a context where the undo data is needed.
    /// Subsequent calls for the same block may cause the response from the previous peer to be ignored.
    /// Peers generally ignore requests for a stale block that they never fully verified, or one that is more than a month old.
    /// When a peer does not respond with a block, we will disconnect.
    /// Note: The block could be re-pruned as soon as it is received.
    /// Returns an empty JSON object if the request was successfully scheduled.
    async fn get_block_from_peer(
        &self,
        blockhash: bitcoin::BlockHash,
        peer_id: i64,
    ) -> Result<GetBlockFromPeerResponse, Self::Error>;

    /// Returns hash of block in best-block-chain at height provided.
    async fn get_block_hash(&self, height: i64) -> Result<GetBlockHashResponse, Self::Error>;

    /// If verbose is false, returns a string that is serialized, hex-encoded data for blockheader 'hash'.
    /// If verbose is true, returns an Object with information about blockheader &lt;hash&gt;.
    async fn get_block_header(
        &self,
        blockhash: bitcoin::BlockHash,
        verbose: Option<bool>,
    ) -> Result<GetBlockHeaderResponse, Self::Error>;

    /// Compute per block statistics for a given window. All amounts are in satoshis.
    /// It won't work for some heights with pruning.
    async fn get_block_stats(
        &self,
        hash_or_height: i64,
        stats: Option<Vec<serde_json::Value>>,
    ) -> Result<GetBlockStatsResponse, Self::Error>;

    /// If the request parameters include a 'mode' key, that is used to explicitly select between the default 'template' request or a 'proposal'.
    /// It returns data needed to construct a block to work on.
    /// For full specification, see BIPs 22, 23, 9, and 145:
    /// <https://github.com/bitcoin/bips/blob/master/bip-0022.mediawiki>
    /// <https://github.com/bitcoin/bips/blob/master/bip-0023.mediawiki>
    /// <https://github.com/bitcoin/bips/blob/master/bip-0009.mediawiki#getblocktemplate_changes>
    /// <https://github.com/bitcoin/bips/blob/master/bip-0145.mediawiki>
    async fn get_block_template(
        &self,
        template_request: serde_json::Value,
    ) -> Result<GetBlockTemplateResponse, Self::Error>;

    /// Return information about chainstates.
    async fn get_chain_states(&self) -> Result<GetChainStatesResponse, Self::Error>;

    /// Return information about all known tips in the block tree, including the main chain as well as orphaned branches.
    async fn get_chain_tips(&self) -> Result<GetChainTipsResponse, Self::Error>;

    /// Compute statistics about the total number and rate of transactions in the chain.
    async fn get_chain_tx_stats(
        &self,
        nblocks: Option<i64>,
        blockhash: Option<bitcoin::BlockHash>,
    ) -> Result<GetChainTxStatsResponse, Self::Error>;

    /// Returns the number of connections to other nodes.
    async fn get_connection_count(&self) -> Result<GetConnectionCountResponse, Self::Error>;

    /// Returns an object containing various state info regarding deployments of consensus changes.
    async fn get_deployment_info(
        &self,
        blockhash: Option<bitcoin::BlockHash>,
    ) -> Result<GetDeploymentInfoResponse, Self::Error>;

    /// Get spend and receive activity associated with a set of descriptors for a set of blocks. This command pairs well with the `relevant_blocks` output of `scanblocks()`.
    /// This call may take several minutes. If you encounter timeouts, try specifying no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    async fn get_descriptor_activity(
        &self,
        blockhashes: Vec<serde_json::Value>,
        scanobjects: Vec<serde_json::Value>,
        include_mempool: Option<bool>,
    ) -> Result<GetDescriptorActivityResponse, Self::Error>;

    /// Analyses a descriptor.
    async fn get_descriptor_info(
        &self,
        descriptor: String,
    ) -> Result<GetDescriptorInfoResponse, Self::Error>;

    /// Returns the proof-of-work difficulty as a multiple of the minimum difficulty.
    async fn get_difficulty(&self) -> Result<GetDifficultyResponse, Self::Error>;

    /// List all BIP 32 HD keys in the wallet and which descriptors use them.
    async fn get_hd_keys(
        &self,
        options: Option<serde_json::Value>,
    ) -> Result<GetHdKeysResponse, Self::Error>;

    /// Returns the status of one or all available indices currently running in the node.
    async fn get_index_info(
        &self,
        index_name: Option<String>,
    ) -> Result<GetIndexInfoResponse, Self::Error>;

    /// Returns an object containing information about memory usage.
    async fn get_memory_info(
        &self,
        mode: Option<String>,
    ) -> Result<GetMemoryInfoResponse, Self::Error>;

    /// If txid is in the mempool, returns all in-mempool ancestors.
    async fn get_mempool_ancestors(
        &self,
        txid: bitcoin::Txid,
        verbose: Option<bool>,
    ) -> Result<GetMempoolAncestorsResponse, Self::Error>;

    /// Returns mempool data for given cluster
    async fn get_mempool_cluster(
        &self,
        txid: bitcoin::Txid,
    ) -> Result<GetMempoolClusterResponse, Self::Error>;

    /// If txid is in the mempool, returns all in-mempool descendants.
    async fn get_mempool_descendants(
        &self,
        txid: bitcoin::Txid,
        verbose: Option<bool>,
    ) -> Result<GetMempoolDescendantsResponse, Self::Error>;

    /// Returns mempool data for given transaction
    async fn get_mempool_entry(
        &self,
        txid: bitcoin::Txid,
    ) -> Result<GetMempoolEntryResponse, Self::Error>;

    /// Returns the feerate diagram for the whole mempool.
    async fn get_mempool_fee_rate_diagram(
        &self,
    ) -> Result<GetMempoolFeeRateDiagramResponse, Self::Error>;

    /// Returns details on the active state of the TX memory pool.
    async fn get_mempool_info(&self) -> Result<GetMempoolInfoResponse, Self::Error>;

    /// Returns a json object containing mining-related information.
    async fn get_mining_info(&self) -> Result<GetMiningInfoResponse, Self::Error>;

    /// Returns information about network traffic, including bytes in, bytes out,
    /// and current system time.
    async fn get_net_totals(&self) -> Result<GetNetTotalsResponse, Self::Error>;

    /// Returns the estimated network hashes per second based on the last n blocks.
    /// Pass in \[blocks\] to override # of blocks, -1 specifies since last difficulty change.
    /// Pass in \[height\] to estimate the network speed at the time when a certain block was found.
    async fn get_network_hashps(
        &self,
        nblocks: Option<i64>,
        height: Option<i64>,
    ) -> Result<GetNetworkHashPsResponse, Self::Error>;

    /// Returns an object containing various state info regarding P2P networking.
    async fn get_network_info(&self) -> Result<GetNetworkInfoResponse, Self::Error>;

    /// Returns a new Bitcoin address for receiving payments.
    /// If 'label' is specified, it is added to the address book
    /// so payments received with the address will be associated with 'label'.
    async fn get_new_address(
        &self,
        label: Option<String>,
        address_type: Option<String>,
    ) -> Result<GetNewAddressResponse, Self::Error>;

    /// Return known addresses, after filtering for quality and recency.
    /// These can potentially be used to find new peers in the network.
    /// The total number of addresses known to the node may be higher.
    async fn get_node_addresses(
        &self,
        count: Option<i64>,
        network: Option<String>,
    ) -> Result<GetNodeAddressesResponse, Self::Error>;

    /// Shows transactions in the tx orphanage.
    /// EXPERIMENTAL warning: this call may be changed in future releases.
    async fn get_orphan_txs(
        &self,
        verbosity: Option<i64>,
    ) -> Result<GetOrphanTxsResponse, Self::Error>;

    /// Returns data about each connected network peer as a json array of objects.
    async fn get_peer_info(&self) -> Result<GetPeerInfoResponse, Self::Error>;

    /// Returns a map of all user-created (see prioritisetransaction) fee deltas by txid, and whether the tx is present in mempool.
    async fn get_prioritised_transactions(
        &self,
    ) -> Result<GetPrioritisedTransactionsResponse, Self::Error>;

    /// EXPERIMENTAL warning: this call may be changed in future releases.
    /// Returns information on all address manager entries for the new and tried tables.
    async fn get_raw_addrman(&self) -> Result<GetRawAddrManResponse, Self::Error>;

    /// Returns a new Bitcoin address, for receiving change.
    /// This is for use with raw transactions, NOT normal use.
    async fn get_raw_change_address(
        &self,
        address_type: Option<String>,
    ) -> Result<GetRawChangeAddressResponse, Self::Error>;

    /// Returns all transaction ids in memory pool as a json array of string transaction ids.
    /// Hint: use getmempoolentry to fetch a specific transaction from the mempool.
    async fn get_raw_mempool(
        &self,
        verbose: Option<bool>,
        mempool_sequence: Option<bool>,
    ) -> Result<GetRawMempoolResponse, Self::Error>;

    /// By default, this call only returns a transaction if it is in the mempool. If -txindex is enabled
    /// and no blockhash argument is passed, it will return the transaction if it is in the mempool or any block.
    /// If a blockhash argument is passed, it will return the transaction if
    /// the specified block is available and the transaction is in that block.
    /// Hint: Use gettransaction for wallet transactions.
    /// If verbosity is 0 or omitted, returns the serialized transaction as a hex-encoded string.
    /// If verbosity is 1, returns a JSON Object with information about the transaction.
    /// If verbosity is 2, returns a JSON Object with information about the transaction, including fee and prevout information.
    async fn get_raw_transaction(
        &self,
        txid: bitcoin::Txid,
        verbosity: Option<i64>,
        blockhash: Option<bitcoin::BlockHash>,
    ) -> Result<GetRawTransactionResponse, Self::Error>;

    /// Returns the total amount received by the given address in transactions with at least minconf confirmations.
    async fn get_received_by_address(
        &self,
        address: bitcoin::Address,
        minconf: Option<i64>,
        include_immature_coinbase: Option<bool>,
    ) -> Result<GetReceivedByAddressResponse, Self::Error>;

    /// Returns the total amount received by addresses with &lt;label&gt; in transactions with at least \[minconf\] confirmations.
    async fn get_received_by_label(
        &self,
        label: String,
        minconf: Option<i64>,
        include_immature_coinbase: Option<bool>,
    ) -> Result<GetReceivedByLabelResponse, Self::Error>;

    /// Returns details of the RPC server.
    async fn get_rpc_info(&self) -> Result<GetRpcInfoResponse, Self::Error>;

    /// Get detailed information about in-wallet transaction &lt;txid&gt;
    async fn get_transaction(
        &self,
        txid: bitcoin::Txid,
        include_watchonly: Option<bool>,
        verbose: Option<bool>,
    ) -> Result<GetTransactionResponse, Self::Error>;

    /// Returns details about an unspent transaction output.
    async fn get_txout(
        &self,
        txid: bitcoin::Txid,
        n: i64,
        include_mempool: Option<bool>,
    ) -> Result<GetTxOutResponse, Self::Error>;

    /// Returns a hex-encoded proof that "txid" was included in a block.
    /// NOTE: By default this function only works sometimes. This is when there is an
    /// unspent output in the utxo for this transaction. To make it always work,
    /// you need to maintain a transaction index, using the -txindex command line option or
    /// specify the block in which the transaction is included manually (by blockhash).
    async fn get_txout_proof(
        &self,
        txids: Vec<serde_json::Value>,
        blockhash: Option<bitcoin::BlockHash>,
    ) -> Result<GetTxOutProofResponse, Self::Error>;

    /// Returns statistics about the unspent transaction output set.
    /// Note this call may take some time if you are not using coinstatsindex.
    async fn get_txout_set_info(
        &self,
        hash_type: Option<String>,
        hash_or_height: Option<i64>,
        use_index: Option<bool>,
    ) -> Result<GetTxOutSetInfoResponse, Self::Error>;

    /// Scans the mempool to find transactions spending any of the given outputs
    async fn get_tx_spending_prevout(
        &self,
        outputs: Vec<serde_json::Value>,
    ) -> Result<GetTxSpendingPrevOutResponse, Self::Error>;

    /// Returns an object containing various wallet state info.
    async fn get_wallet_info(&self) -> Result<GetWalletInfoResponse, Self::Error>;

    /// List all commands, or get help for a specified command.
    async fn help(&self, command: Option<String>) -> Result<HelpResponse, Self::Error>;

    /// Import descriptors. This will trigger a rescan of the blockchain based on the earliest timestamp of all descriptors being imported. Requires a new wallet backup.
    /// When importing descriptors with multipath key expressions, if the multipath specifier contains exactly two elements, the descriptor produced from the second element will be imported as an internal descriptor.
    /// Note: This call can take over an hour to complete if using an early timestamp; during that time, other rpc calls
    /// may report that the imported keys, addresses or scripts exist but related transactions are still missing.
    /// The rescan is significantly faster if block filters are available (using startup option "-blockfilterindex=1").
    async fn import_descriptors(
        &self,
        requests: Vec<serde_json::Value>,
    ) -> Result<ImportDescriptorsResponse, Self::Error>;

    /// Import a mempool.dat file and attempt to add its contents to the mempool.
    /// Warning: Importing untrusted files is dangerous, especially if metadata from the file is taken over.
    async fn import_mempool(
        &self,
        filepath: String,
        options: Option<serde_json::Value>,
    ) -> Result<ImportMempoolResponse, Self::Error>;

    /// Imports funds without rescan. Corresponding address or script must previously be included in wallet. Aimed towards pruned wallets. The end-user is responsible to import additional transactions that subsequently spend the imported outputs or rescan after the point in the blockchain the transaction is included.
    async fn import_pruned_funds(
        &self,
        rawtransaction: String,
        txoutproof: String,
    ) -> Result<ImportPrunedFundsResponse, Self::Error>;

    /// Permanently marks a block as invalid, as if it violated a consensus rule.
    async fn invalidate_block(
        &self,
        blockhash: bitcoin::BlockHash,
    ) -> Result<InvalidateBlockResponse, Self::Error>;

    /// Joins multiple distinct PSBTs with different inputs and outputs into one PSBT with inputs and outputs from all of the PSBTs
    /// No input in any of the PSBTs can be in more than one of the PSBTs.
    async fn join_psbts(
        &self,
        txs: Vec<serde_json::Value>,
    ) -> Result<JoinPsbtsResponse, Self::Error>;

    /// Refills each descriptor keypool in the wallet up to the specified number of new keys.
    /// By default, descriptor wallets have 4 active ranged descriptors ("legacy", "p2sh-segwit", "bech32", "bech32m"), each with 1000 entries.
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    async fn keypool_refill(
        &self,
        newsize: Option<i64>,
    ) -> Result<KeypoolRefillResponse, Self::Error>;

    /// Lists groups of addresses which have had their common ownership
    /// made public by common use as inputs or as the resulting change
    /// in past transactions
    async fn list_address_groupings(&self) -> Result<ListAddressGroupingsResponse, Self::Error>;

    /// List all manually banned IPs/Subnets.
    async fn list_banned(&self) -> Result<ListBannedResponse, Self::Error>;

    /// List all descriptors present in a wallet.
    async fn list_descriptors(
        &self,
        private: Option<bool>,
    ) -> Result<ListDescriptorsResponse, Self::Error>;

    /// Returns the list of all labels, or labels that are assigned to addresses with a specific purpose.
    async fn list_labels(&self, purpose: Option<String>)
        -> Result<ListLabelsResponse, Self::Error>;

    /// Returns list of temporarily unspendable outputs.
    /// See the lockunspent call to lock and unlock transactions for spending.
    async fn list_lock_unspent(&self) -> Result<ListLockUnspentResponse, Self::Error>;

    /// List balances by receiving address.
    async fn list_received_by_address(
        &self,
        minconf: Option<i64>,
        include_empty: Option<bool>,
        include_watchonly: Option<bool>,
        address_filter: Option<String>,
        include_immature_coinbase: Option<bool>,
    ) -> Result<ListReceivedByAddressResponse, Self::Error>;

    /// List received transactions by label.
    async fn list_received_by_label(
        &self,
        minconf: Option<i64>,
        include_empty: Option<bool>,
        include_watchonly: Option<bool>,
        include_immature_coinbase: Option<bool>,
    ) -> Result<ListReceivedByLabelResponse, Self::Error>;

    /// Get all transactions in blocks since block \[blockhash\], or all transactions if omitted.
    /// If "blockhash" is no longer a part of the main chain, transactions from the fork point onward are included.
    /// Additionally, if include_removed is set, transactions affecting the wallet which were removed are returned in the "removed" array.
    async fn list_since_block(
        &self,
        blockhash: Option<bitcoin::BlockHash>,
        target_confirmations: Option<i64>,
        include_watchonly: Option<bool>,
        include_removed: Option<bool>,
        include_change: Option<bool>,
        label: Option<String>,
    ) -> Result<ListSinceBlockResponse, Self::Error>;

    /// If a label name is provided, this will return only incoming transactions paying to addresses with the specified label.
    /// Returns up to 'count' most recent transactions ordered from oldest to newest while skipping the first number of
    /// transactions specified in the 'skip' argument. A transaction can have multiple entries in this RPC response.
    /// For instance, a wallet transaction that pays three addresses — one wallet-owned and two external — will produce
    /// four entries. The payment to the wallet-owned address appears both as a send entry and as a receive entry.
    /// As a result, the RPC response will contain one entry in the receive category and three entries in the send category.
    async fn list_transactions(
        &self,
        label: Option<String>,
        count: Option<i64>,
        skip: Option<i64>,
        include_watchonly: Option<bool>,
    ) -> Result<ListTransactionsResponse, Self::Error>;

    /// Returns array of unspent transaction outputs
    /// with between minconf and maxconf (inclusive) confirmations.
    /// Optionally filter to only include txouts paid to specified addresses.
    async fn list_unspent(
        &self,
        minconf: Option<i64>,
        maxconf: Option<i64>,
        addresses: Option<Vec<serde_json::Value>>,
        include_unsafe: Option<bool>,
        query_options: Option<serde_json::Value>,
    ) -> Result<ListUnspentResponse, Self::Error>;

    /// Returns a list of wallets in the wallet directory.
    async fn list_wallet_dir(&self) -> Result<ListWalletDirResponse, Self::Error>;

    /// Returns a list of currently loaded wallets.
    /// For full information on the wallet, use "getwalletinfo"
    async fn list_wallets(&self) -> Result<ListWalletsResponse, Self::Error>;

    /// Load the serialized UTXO set from a file.
    /// Once this snapshot is loaded, its contents will be deserialized into a second chainstate data structure, which is then used to sync to the network's tip. Meanwhile, the original chainstate will complete the initial block download process in the background, eventually validating up to the block that the snapshot is based upon.
    /// The result is a usable bitcoind instance that is current with the network tip in a matter of minutes rather than hours. UTXO snapshot are typically obtained from third-party sources (HTTP, torrent, etc.) which is reasonable since their contents are always checked by hash.
    /// You can find more information on this process in the `assumeutxo` design document (<https://github.com/bitcoin/bitcoin/blob/master/doc/design/assumeutxo.md>).
    async fn load_txout_set(&self, path: String) -> Result<LoadTxOutSetResponse, Self::Error>;

    /// Loads a wallet from a wallet file or directory.
    /// Note that all wallet command-line options used when starting bitcoind will be
    /// applied to the new wallet.
    async fn load_wallet(
        &self,
        filename: String,
        load_on_startup: Option<bool>,
    ) -> Result<LoadWalletResponse, Self::Error>;

    /// Updates list of temporarily unspendable outputs.
    /// Temporarily lock (unlock=false) or unlock (unlock=true) specified transaction outputs.
    /// If no transaction outputs are specified when unlocking then all current locked transaction outputs are unlocked.
    /// A locked transaction output will not be chosen by automatic coin selection, when spending bitcoins.
    /// Manually selected coins are automatically unlocked.
    /// Locks are stored in memory only, unless persistent=true, in which case they will be written to the
    /// wallet database and loaded on node start. Unwritten (persistent=false) locks are always cleared
    /// (by virtue of process exit) when a node stops or fails. Unlocking will clear both persistent and not.
    /// Also see the listunspent call
    async fn lock_unspent(
        &self,
        unlock: bool,
        transactions: Option<Vec<serde_json::Value>>,
        persistent: Option<bool>,
    ) -> Result<LockUnspentResponse, Self::Error>;

    /// Gets and sets the logging configuration.
    /// When called without an argument, returns the list of categories with status that are currently being debug logged or not.
    /// When called with arguments, adds or removes categories from debug logging and return the lists above.
    /// The arguments are evaluated in order "include", "exclude".
    /// If an item is both included and excluded, it will thus end up being excluded.
    /// The valid logging categories are: addrman, bench, blockstorage, cmpctblock, coindb, estimatefee, http, i2p, ipc, kernel, leveldb, libevent, mempool, mempoolrej, net, privatebroadcast, proxy, prune, qt, rand, reindex, rpc, scan, selectcoins, tor, txpackages, txreconciliation, validation, walletdb, zmq
    /// In addition, the following are available as category names with special meanings:
    /// - "all",  "1" : represent all logging categories.
    async fn logging(
        &self,
        include: Option<Vec<serde_json::Value>>,
        exclude: Option<Vec<serde_json::Value>>,
    ) -> Result<LoggingResponse, Self::Error>;

    /// Migrate the wallet to a descriptor wallet.
    /// A new wallet backup will need to be made.
    /// The migration process will create a backup of the wallet before migrating. This backup
    /// file will be named &lt;wallet name&gt;-&lt;timestamp&gt;.legacy.bak and can be found in the directory
    /// for this wallet. In the event of an incorrect migration, the backup can be restored using restorewallet.
    /// Encrypted wallets must have the passphrase provided as an argument to this call.
    /// This RPC may take a long time to complete. Increasing the RPC client timeout is recommended.
    async fn migrate_wallet(
        &self,
        wallet_name: Option<String>,
        passphrase: Option<String>,
    ) -> Result<MigrateWalletResponse, Self::Error>;

    /// Bump the scheduler into the future (-regtest only)
    async fn mock_scheduler(&self, delta_time: i64) -> Result<MockSchedulerResponse, Self::Error>;

    /// Requests that a ping be sent to all other nodes, to measure ping time.
    /// Results are provided in getpeerinfo.
    /// Ping command is handled in queue with all other commands, so it measures processing backlog, not just network ping.
    async fn ping(&self) -> Result<PingResponse, Self::Error>;

    /// Treats a block as if it were received before others with the same work.
    /// A later preciousblock call can override the effect of an earlier one.
    /// The effects of preciousblock are not retained across restarts.
    async fn precious_block(
        &self,
        blockhash: bitcoin::BlockHash,
    ) -> Result<PreciousBlockResponse, Self::Error>;

    /// Accepts the transaction into mined blocks at a higher (or lower) priority
    async fn prioritise_transaction(
        &self,
        txid: bitcoin::Txid,
        dummy: Option<i64>,
        fee_delta: i64,
    ) -> Result<PrioritiseTransactionResponse, Self::Error>;

    /// Attempts to delete block and undo data up to a specified height or timestamp, if eligible for pruning.
    /// Requires `-prune` to be enabled at startup. While pruned data may be re-fetched in some cases (e.g., via `getblockfrompeer`), local deletion is irreversible.
    async fn prune_blockchain(&self, height: i64) -> Result<PruneBlockchainResponse, Self::Error>;

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
    async fn psbt_bump_fee(
        &self,
        txid: bitcoin::Txid,
        options: Option<serde_json::Value>,
    ) -> Result<PsbtBumpFeeResponse, Self::Error>;

    /// Removes invalidity status of a block, its ancestors and its descendants, reconsider them for activation.
    /// This can be used to undo the effects of invalidateblock.
    async fn reconsider_block(
        &self,
        blockhash: bitcoin::BlockHash,
    ) -> Result<ReconsiderBlockResponse, Self::Error>;

    /// Deletes the specified transaction from the wallet. Meant for use with pruned wallets and as a companion to importprunedfunds. This will affect wallet balances.
    async fn remove_pruned_funds(
        &self,
        txid: bitcoin::Txid,
    ) -> Result<RemovePrunedFundsResponse, Self::Error>;

    /// Rescan the local blockchain for wallet related transactions.
    /// Note: Use "getwalletinfo" to query the scanning progress.
    /// The rescan is significantly faster if block filters are available
    /// (using startup option "-blockfilterindex=1").
    async fn rescan_blockchain(
        &self,
        start_height: Option<i64>,
        stop_height: Option<i64>,
    ) -> Result<RescanBlockchainResponse, Self::Error>;

    /// Restores and loads a wallet from backup.
    /// The rescan is significantly faster if block filters are available
    /// (using startup option "-blockfilterindex=1").
    async fn restore_wallet(
        &self,
        wallet_name: String,
        backup_file: String,
        load_on_startup: Option<bool>,
    ) -> Result<RestoreWalletResponse, Self::Error>;

    /// Dumps the mempool to disk. It will fail until the previous dump is fully loaded.
    async fn save_mempool(&self) -> Result<SaveMempoolResponse, Self::Error>;

    /// Return relevant blockhashes for given descriptors (requires blockfilterindex).
    /// This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    async fn scan_blocks(
        &self,
        action: String,
        scanobjects: Option<Vec<serde_json::Value>>,
        start_height: Option<i64>,
        stop_height: Option<i64>,
        filtertype: Option<String>,
        options: Option<serde_json::Value>,
    ) -> Result<ScanBlocksResponse, Self::Error>;

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
    async fn scan_txout_set(
        &self,
        action: String,
        scanobjects: Option<Vec<serde_json::Value>>,
    ) -> Result<ScanTxOutSetResponse, Self::Error>;

    /// Return RPC command JSON Schema descriptions.
    async fn schema(&self) -> Result<SchemaResponse, Self::Error>;

    /// EXPERIMENTAL warning: this call may be changed in future releases.
    /// Send a transaction.
    async fn send(
        &self,
        outputs: Vec<serde_json::Value>,
        conf_target: Option<i64>,
        estimate_mode: Option<String>,
        fee_rate: Option<serde_json::Value>,
        options: Option<serde_json::Value>,
        version: Option<i64>,
    ) -> Result<SendResponse, Self::Error>;

    /// EXPERIMENTAL warning: this call may be changed in future releases.
    /// Spend the value of all (or specific) confirmed UTXOs and unconfirmed change in the wallet to one or more recipients.
    /// Unconfirmed inbound UTXOs and locked UTXOs will not be spent. Sendall will respect the avoid_reuse wallet flag.
    /// If your wallet contains many small inputs, either because it received tiny payments or as a result of accumulating change, consider using `send_max` to exclude inputs that are worth less than the fees needed to spend them.
    async fn send_all(
        &self,
        recipients: Vec<serde_json::Value>,
        conf_target: Option<i64>,
        estimate_mode: Option<String>,
        fee_rate: Option<serde_json::Value>,
        options: Option<serde_json::Value>,
    ) -> Result<SendAllResponse, Self::Error>;

    /// Send multiple times. Amounts are double-precision floating point numbers.
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    async fn send_many(
        &self,
        dummy: Option<String>,
        amounts: serde_json::Value,
        minconf: Option<i64>,
        comment: Option<String>,
        subtractfeefrom: Option<Vec<serde_json::Value>>,
        replaceable: Option<bool>,
        conf_target: Option<i64>,
        estimate_mode: Option<String>,
        fee_rate: Option<serde_json::Value>,
        verbose: Option<bool>,
    ) -> Result<SendManyResponse, Self::Error>;

    /// Send a p2p message to a peer specified by id.
    /// The message type and body must be provided, the message header will be generated.
    /// This RPC is for testing only.
    async fn send_msg_to_peer(
        &self,
        peer_id: i64,
        msg_type: String,
        msg: String,
    ) -> Result<SendMsgToPeerResponse, Self::Error>;

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
    async fn send_raw_transaction(
        &self,
        hexstring: String,
        maxfeerate: Option<serde_json::Value>,
        maxburnamount: Option<serde_json::Value>,
    ) -> Result<SendRawTransactionResponse, Self::Error>;

    /// Send an amount to a given address.
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    async fn send_to_address(
        &self,
        address: bitcoin::Address,
        amount: serde_json::Value,
        comment: Option<String>,
        comment_to: Option<String>,
        subtractfeefromamount: Option<bool>,
        replaceable: Option<bool>,
        conf_target: Option<i64>,
        estimate_mode: Option<String>,
        avoid_reuse: Option<bool>,
        fee_rate: Option<serde_json::Value>,
        verbose: Option<bool>,
    ) -> Result<SendToAddressResponse, Self::Error>;

    /// Attempts to add or remove an IP/Subnet from the banned list.
    async fn set_ban(
        &self,
        subnet: String,
        command: String,
        bantime: Option<i64>,
        absolute: Option<bool>,
    ) -> Result<SetBanResponse, Self::Error>;

    /// Sets the label associated with the given address.
    async fn set_label(
        &self,
        address: bitcoin::Address,
        label: String,
    ) -> Result<SetLabelResponse, Self::Error>;

    /// Set the local time to given timestamp (-regtest only)
    async fn set_mock_time(&self, timestamp: i64) -> Result<SetMockTimeResponse, Self::Error>;

    /// Disable/enable all p2p network activity.
    async fn set_network_active(
        &self,
        state: bool,
    ) -> Result<SetNetworkActiveResponse, Self::Error>;

    /// Change the state of the given wallet flag for a wallet.
    async fn set_wallet_flag(
        &self,
        flag: String,
        value: Option<bool>,
    ) -> Result<SetWalletFlagResponse, Self::Error>;

    /// Sign a message with the private key of an address
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    ///
    /// Requires wallet private keys to be available (e.g. unlocked).
    async fn sign_message(
        &self,
        address: bitcoin::Address,
        message: String,
    ) -> Result<SignMessageResponse, Self::Error>;

    /// Sign a message with the private key of an address
    async fn sign_message_with_priv_key(
        &self,
        privkey: String,
        message: String,
    ) -> Result<SignMessageWithPrivKeyResponse, Self::Error>;

    /// Sign inputs for raw transaction (serialized, hex-encoded).
    /// The second argument is an array of base58-encoded private
    /// keys that will be the only keys used to sign the transaction.
    /// The third optional argument (may be null) is an array of previous transaction outputs that
    /// this transaction depends on but may not yet be in the block chain.
    async fn sign_raw_transaction_with_key(
        &self,
        hexstring: String,
        privkeys: Vec<serde_json::Value>,
        prevtxs: Option<Vec<serde_json::Value>>,
        sighashtype: Option<String>,
    ) -> Result<SignRawTransactionWithKeyResponse, Self::Error>;

    /// Sign inputs for raw transaction (serialized, hex-encoded).
    /// The second optional argument (may be null) is an array of previous transaction outputs that
    /// this transaction depends on but may not yet be in the block chain.
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    ///
    /// Requires wallet private keys to be available (e.g. unlocked).
    async fn sign_raw_transaction_with_wallet(
        &self,
        hexstring: String,
        prevtxs: Option<Vec<serde_json::Value>>,
        sighashtype: Option<String>,
    ) -> Result<SignRawTransactionWithWalletResponse, Self::Error>;

    /// Calculate the balance change resulting in the signing and broadcasting of the given transaction(s).
    async fn simulate_raw_transaction(
        &self,
        rawtxs: Option<Vec<serde_json::Value>>,
        options: Option<serde_json::Value>,
    ) -> Result<SimulateRawTransactionResponse, Self::Error>;

    /// Request a graceful shutdown of Bitcoin Core.
    async fn stop(&self, wait: Option<i64>) -> Result<StopResponse, Self::Error>;

    /// Attempts to submit new block to network.
    /// See <https://en.bitcoin.it/wiki/BIP_0022> for full specification.
    async fn submit_block(
        &self,
        hexdata: String,
        dummy: Option<String>,
    ) -> Result<SubmitBlockResponse, Self::Error>;

    /// Decode the given hexdata as a header and submit it as a candidate chain tip if valid.
    /// Throws when the header is invalid.
    async fn submit_header(&self, hexdata: String) -> Result<SubmitHeaderResponse, Self::Error>;

    /// Submit a package of raw transactions (serialized, hex-encoded) to local node.
    /// The package will be validated according to consensus and mempool policy rules. If any transaction passes, it will be accepted to mempool.
    /// This RPC is experimental and the interface may be unstable. Refer to doc/policy/packages.md for documentation on package policies.
    /// Warning: successful submission does not mean the transactions will propagate throughout the network.
    async fn submit_package(
        &self,
        package: Vec<serde_json::Value>,
        maxfeerate: Option<serde_json::Value>,
        maxburnamount: Option<serde_json::Value>,
    ) -> Result<SubmitPackageResponse, Self::Error>;

    /// Waits for the validation interface queue to catch up on everything that was there when we entered this function.
    async fn sync_with_validation_interface_queue(
        &self,
    ) -> Result<SyncWithValidationInterfaceQueueResponse, Self::Error>;

    /// Returns result of mempool acceptance tests indicating if raw transaction(s) (serialized, hex-encoded) would be accepted by mempool.
    /// If multiple transactions are passed in, parents must come before children and package policies apply: the transactions cannot conflict with any mempool transactions or each other.
    /// If one transaction fails, other transactions may not be fully validated (the 'allowed' key will be blank).
    /// The maximum number of transactions allowed is 25.
    /// This checks if transactions violate the consensus or policy rules.
    /// See sendrawtransaction call.
    async fn test_mempool_accept(
        &self,
        rawtxs: Vec<serde_json::Value>,
        maxfeerate: Option<serde_json::Value>,
    ) -> Result<TestMempoolAcceptResponse, Self::Error>;

    /// Unloads the wallet referenced by the request endpoint or the wallet_name argument.
    /// If both are specified, they must be identical.
    async fn unload_wallet(
        &self,
        wallet_name: Option<String>,
        load_on_startup: Option<bool>,
    ) -> Result<UnloadWalletResponse, Self::Error>;

    /// Returns the total uptime of the server.
    async fn uptime(&self) -> Result<UptimeResponse, Self::Error>;

    /// Updates all segwit inputs and outputs in a PSBT with data from output descriptors, the UTXO set, txindex, or the mempool.
    async fn utxo_update_psbt(
        &self,
        psbt: String,
        descriptors: Option<Vec<serde_json::Value>>,
    ) -> Result<UtxoUpdatePsbtResponse, Self::Error>;

    /// Return information about the given bitcoin address.
    async fn validate_address(
        &self,
        address: bitcoin::Address,
    ) -> Result<ValidateAddressResponse, Self::Error>;

    /// Verifies blockchain database.
    async fn verify_chain(
        &self,
        checklevel: Option<i64>,
        nblocks: Option<i64>,
    ) -> Result<VerifyChainResponse, Self::Error>;

    /// Verify a signed message.
    async fn verify_message(
        &self,
        address: bitcoin::Address,
        signature: String,
        message: String,
    ) -> Result<VerifyMessageResponse, Self::Error>;

    /// Verifies that a proof points to a transaction in a block, returning the transaction it commits to
    /// and throwing an RPC error if the block is not in our best chain
    async fn verify_txout_proof(
        &self,
        proof: String,
    ) -> Result<VerifyTxOutProofResponse, Self::Error>;

    /// Waits for a specific new block and returns useful info about it.
    /// Returns the current block on timeout or exit.
    /// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    async fn wait_for_block(
        &self,
        blockhash: bitcoin::BlockHash,
        timeout: Option<i64>,
    ) -> Result<WaitForBlockResponse, Self::Error>;

    /// Waits for (at least) block height and returns the height and hash
    /// of the current tip.
    /// Returns the current block on timeout or exit.
    /// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    async fn wait_for_block_height(
        &self,
        height: i64,
        timeout: Option<i64>,
    ) -> Result<WaitForBlockHeightResponse, Self::Error>;

    /// Waits for any new block and returns useful info about it.
    /// Returns the current block on timeout or exit.
    /// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    async fn wait_for_new_block(
        &self,
        timeout: Option<i64>,
        current_tip: Option<String>,
    ) -> Result<WaitForNewBlockResponse, Self::Error>;

    /// Creates and funds a transaction in the Partially Signed Transaction format.
    /// Implements the Creator and Updater roles.
    /// All existing inputs must either have their previous output transaction be in the wallet
    /// or be in the UTXO set. Solving data must be provided for non-wallet inputs.
    async fn wallet_create_funded_psbt(
        &self,
        inputs: Option<Vec<serde_json::Value>>,
        outputs: Vec<serde_json::Value>,
        locktime: Option<i64>,
        options: Option<serde_json::Value>,
        bip32derivs: Option<bool>,
        version: Option<i64>,
    ) -> Result<WalletCreateFundedPsbtResponse, Self::Error>;

    /// Display address on an external signer for verification.
    async fn wallet_display_address(
        &self,
        address: bitcoin::Address,
    ) -> Result<WalletDisplayAddressResponse, Self::Error>;

    /// Removes the wallet encryption key from memory, locking the wallet.
    /// After calling this method, you will need to call walletpassphrase again
    /// before being able to call any methods which require the wallet to be unlocked.
    ///
    /// Requires wallet private keys to be available (e.g. unlocked).
    async fn wallet_lock(&self) -> Result<WalletLockResponse, Self::Error>;

    /// Stores the wallet decryption key in memory for 'timeout' seconds.
    /// This is needed prior to performing transactions related to private keys such as sending bitcoins
    /// Note:
    /// Issuing the walletpassphrase command while the wallet is already unlocked will set a new unlock
    /// time that overrides the old one.
    ///
    /// Requires wallet private keys to be available (e.g. unlocked).
    async fn wallet_passphrase(
        &self,
        passphrase: String,
        timeout: i64,
    ) -> Result<WalletPassphraseResponse, Self::Error>;

    /// Changes the wallet passphrase from 'oldpassphrase' to 'newpassphrase'.
    ///
    /// Requires wallet private keys to be available (e.g. unlocked).
    async fn wallet_passphrase_change(
        &self,
        oldpassphrase: String,
        newpassphrase: String,
    ) -> Result<WalletPassphraseChangeResponse, Self::Error>;

    /// Update a PSBT with input information from our wallet and then sign inputs
    /// that we can sign for.
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    async fn wallet_process_psbt(
        &self,
        psbt: String,
        sign: Option<bool>,
        sighashtype: Option<String>,
        bip32derivs: Option<bool>,
        finalize: Option<bool>,
    ) -> Result<WalletProcessPsbtResponse, Self::Error>;
}

/// Helper to route calls to the node or wallet namespace automatically.
pub trait RpcDispatchExt: TransportTrait + TransportExt {
    /// Dispatch JSON-RPC methods by name.
    fn dispatch_json<R: DeserializeOwned>(
        &self,
        method: &str,
        params: &[serde_json::Value],
    ) -> impl Future<Output = Result<R, TransportError>> + Send {
        async move { self.call(method, params).await }
    }
}

impl<T: TransportTrait + TransportExt + ?Sized> RpcDispatchExt for T {}

// helper trait, so any TransportTrait gets a wallet_call by default
pub trait WalletTransportExt: TransportTrait + TransportExt {
    fn wallet_call<T: serde::Serialize + std::marker::Sync, R: serde::de::DeserializeOwned>(
        &self,
        method: &str,
        params: &[T],
    ) -> impl std::future::Future<Output = Result<R, crate::transport::TransportError>> + Send {
        async {
            // Convert params to Value before passing to call
            let value_params: Vec<serde_json::Value> =
                params.iter().map(|p| serde_json::to_value(p).unwrap()).collect();
            self.call(method, &value_params).await
        }
    }
}

impl<T: TransportTrait + TransportExt + ?Sized> WalletTransportExt for T {}

// Provide default implementation for any type that implements TransportTrait + TransportExt
#[async_trait]
impl<T: TransportTrait + TransportExt + Send + Sync> BitcoinClient for T {
    type Error = TransportError;

    /// Mark in-wallet transaction &lt;txid&gt; as abandoned
    /// This will mark this transaction and all its in-wallet descendants as abandoned which will allow
    /// for their inputs to be respent.  It can be used to replace "stuck" or evicted transactions.
    /// It only works on transactions which are not included in a block and are not currently in the mempool.
    /// It has no effect on transactions which are already abandoned.
    async fn abandon_transaction(
        &self,
        txid: bitcoin::Txid,
    ) -> Result<AbandonTransactionResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(txid));
        self.call::<AbandonTransactionResponse>("abandontransaction", &rpc_params).await
    }

    /// Stops current wallet rescan triggered by an RPC call, e.g. by a rescanblockchain call.
    /// Note: Use "getwalletinfo" to query the scanning progress.
    async fn abort_rescan(&self) -> Result<AbortRescanResponse, Self::Error> {
        self.call::<AbortRescanResponse>("abortrescan", &[]).await
    }

    /// Open an outbound connection to a specified node. This RPC is for testing only.
    async fn add_connection(
        &self,
        address: bitcoin::Address,
        connection_type: String,
        v2transport: bool,
    ) -> Result<AddConnectionResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(address));
        rpc_params.push(serde_json::json!(connection_type));
        rpc_params.push(serde_json::json!(v2transport));
        self.call::<AddConnectionResponse>("addconnection", &rpc_params).await
    }

    /// Attempts to add or remove a node from the addnode list.
    /// Or try a connection to a node once.
    /// Nodes added using addnode (or -connect) are protected from DoS disconnection and are not required to be
    /// full nodes/support SegWit as other outbound peers are (though such peers will not be synced from).
    /// Addnode connections are limited to 8 at a time and are counted separately from the -maxconnections limit.
    async fn add_node(
        &self,
        node: String,
        command: String,
        v2transport: Option<bool>,
    ) -> Result<AddNodeResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(node));
        rpc_params.push(serde_json::json!(command));
        if let Some(val) = v2transport {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<AddNodeResponse>("addnode", &rpc_params).await
    }

    /// Add the address of a potential peer to an address manager table. This RPC is for testing only.
    async fn add_peer_address(
        &self,
        address: bitcoin::Address,
        port: i64,
        tried: Option<bool>,
    ) -> Result<AddPeerAddressResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(address));
        rpc_params.push(serde_json::json!(port));
        if let Some(val) = tried {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<AddPeerAddressResponse>("addpeeraddress", &rpc_params).await
    }

    /// Analyzes and provides information about the current status of a PSBT and its inputs
    async fn analyze_psbt(&self, psbt: String) -> Result<AnalyzePsbtResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(psbt));
        self.call::<AnalyzePsbtResponse>("analyzepsbt", &rpc_params).await
    }

    /// Safely copies the current wallet file to the specified destination, which can either be a directory or a path with a filename.
    async fn backup_wallet(
        &self,
        destination: String,
    ) -> Result<BackupWalletResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(destination));
        self.call::<BackupWalletResponse>("backupwallet", &rpc_params).await
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
    async fn bump_fee(
        &self,
        txid: bitcoin::Txid,
        options: Option<serde_json::Value>,
    ) -> Result<BumpFeeResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(txid));
        if let Some(val) = options {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<BumpFeeResponse>("bumpfee", &rpc_params).await
    }

    /// Clear all banned IPs.
    async fn clear_banned(&self) -> Result<ClearBannedResponse, Self::Error> {
        self.call::<ClearBannedResponse>("clearbanned", &[]).await
    }

    /// Combine multiple partially signed Bitcoin transactions into one transaction.
    /// Implements the Combiner role.
    async fn combine_psbt(
        &self,
        txs: Vec<serde_json::Value>,
    ) -> Result<CombinePsbtResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(txs));
        self.call::<CombinePsbtResponse>("combinepsbt", &rpc_params).await
    }

    /// Combine multiple partially signed transactions into one transaction.
    /// The combined transaction may be another partially signed transaction or a
    /// fully signed transaction.
    async fn combine_raw_transaction(
        &self,
        txs: Vec<serde_json::Value>,
    ) -> Result<CombineRawTransactionResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(txs));
        self.call::<CombineRawTransactionResponse>("combinerawtransaction", &rpc_params).await
    }

    /// Converts a network serialized transaction to a PSBT. This should be used only with createrawtransaction and fundrawtransaction
    /// createpsbt and walletcreatefundedpsbt should be used for new applications.
    async fn convert_to_psbt(
        &self,
        hexstring: String,
        permitsigdata: Option<bool>,
        iswitness: Option<bool>,
    ) -> Result<ConvertToPsbtResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(hexstring));
        if let Some(val) = permitsigdata {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = iswitness {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<ConvertToPsbtResponse>("converttopsbt", &rpc_params).await
    }

    /// Creates a multi-signature address with n signatures of m keys required.
    /// It returns a json object with the address and redeemScript.
    async fn create_multisig(
        &self,
        nrequired: i64,
        keys: Vec<serde_json::Value>,
        address_type: Option<String>,
    ) -> Result<CreateMultisigResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(nrequired));
        rpc_params.push(serde_json::json!(keys));
        if let Some(val) = address_type {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<CreateMultisigResponse>("createmultisig", &rpc_params).await
    }

    /// Creates a transaction in the Partially Signed Transaction format.
    /// Implements the Creator role.
    /// Note that the transaction's inputs are not signed, and
    /// it is not stored in the wallet or transmitted to the network.
    async fn create_psbt(
        &self,
        inputs: Vec<serde_json::Value>,
        outputs: Vec<serde_json::Value>,
        locktime: Option<i64>,
        replaceable: Option<bool>,
        version: Option<i64>,
    ) -> Result<CreatePsbtResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(inputs));
        rpc_params.push(serde_json::json!(outputs));
        if let Some(val) = locktime {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = replaceable {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = version {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<CreatePsbtResponse>("createpsbt", &rpc_params).await
    }

    /// Create a transaction spending the given inputs and creating new outputs.
    /// Outputs can be addresses or data.
    /// Returns hex-encoded raw transaction.
    /// Note that the transaction's inputs are not signed, and
    /// it is not stored in the wallet or transmitted to the network.
    async fn create_raw_transaction(
        &self,
        inputs: Vec<serde_json::Value>,
        outputs: Vec<serde_json::Value>,
        locktime: Option<i64>,
        replaceable: Option<bool>,
        version: Option<i64>,
    ) -> Result<CreateRawTransactionResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(inputs));
        rpc_params.push(serde_json::json!(outputs));
        if let Some(val) = locktime {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = replaceable {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = version {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<CreateRawTransactionResponse>("createrawtransaction", &rpc_params).await
    }

    /// Creates and loads a new wallet.
    async fn create_wallet(
        &self,
        wallet_name: String,
        disable_private_keys: Option<bool>,
        blank: Option<bool>,
        passphrase: Option<String>,
        avoid_reuse: Option<bool>,
        descriptors: Option<bool>,
        load_on_startup: Option<bool>,
        external_signer: Option<bool>,
    ) -> Result<CreateWalletResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(wallet_name));
        if let Some(val) = disable_private_keys {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = blank {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = passphrase {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = avoid_reuse {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = descriptors {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = load_on_startup {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = external_signer {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<CreateWalletResponse>("createwallet", &rpc_params).await
    }

    /// Creates the wallet's descriptor for the given address type. The address type must be one that the wallet does not already have a descriptor for.
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    async fn create_wallet_descriptor(
        &self,
        r#type: String,
        options: Option<serde_json::Value>,
    ) -> Result<CreateWalletDescriptorResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(r#type));
        if let Some(val) = options {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<CreateWalletDescriptorResponse>("createwalletdescriptor", &rpc_params).await
    }

    /// Return a JSON object representing the serialized, base64-encoded partially signed Bitcoin transaction.
    async fn decode_psbt(&self, psbt: String) -> Result<DecodePsbtResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(psbt));
        self.call::<DecodePsbtResponse>("decodepsbt", &rpc_params).await
    }

    /// Return a JSON object representing the serialized, hex-encoded transaction.
    async fn decode_raw_transaction(
        &self,
        hexstring: String,
        iswitness: Option<bool>,
    ) -> Result<DecodeRawTransactionResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(hexstring));
        if let Some(val) = iswitness {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<DecodeRawTransactionResponse>("decoderawtransaction", &rpc_params).await
    }

    /// Decode a hex-encoded script.
    async fn decode_script(&self, hexstring: String) -> Result<DecodeScriptResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(hexstring));
        self.call::<DecodeScriptResponse>("decodescript", &rpc_params).await
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
    async fn derive_addresses(
        &self,
        descriptor: String,
        range: Option<serde_json::Value>,
    ) -> Result<DeriveAddressesResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(descriptor));
        if let Some(val) = range {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<DeriveAddressesResponse>("deriveaddresses", &rpc_params).await
    }

    /// Update all segwit inputs in a PSBT with information from output descriptors, the UTXO set or the mempool.
    /// Then, sign the inputs we are able to with information from the output descriptors.
    async fn descriptor_process_psbt(
        &self,
        psbt: String,
        descriptors: Vec<serde_json::Value>,
        sighashtype: Option<String>,
        bip32derivs: Option<bool>,
        finalize: Option<bool>,
    ) -> Result<DescriptorProcessPsbtResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(psbt));
        rpc_params.push(serde_json::json!(descriptors));
        if let Some(val) = sighashtype {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = bip32derivs {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = finalize {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<DescriptorProcessPsbtResponse>("descriptorprocesspsbt", &rpc_params).await
    }

    /// Immediately disconnects from the specified peer node.
    /// Strictly one out of 'address' and 'nodeid' can be provided to identify the node.
    /// To disconnect by nodeid, either set 'address' to the empty string, or call using the named 'nodeid' argument only.
    async fn disconnect_node(
        &self,
        address: Option<bitcoin::Address>,
        nodeid: Option<i64>,
    ) -> Result<DisconnectNodeResponse, Self::Error> {
        let mut rpc_params = vec![];
        if let Some(val) = address {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = nodeid {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<DisconnectNodeResponse>("disconnectnode", &rpc_params).await
    }

    /// Write the serialized UTXO set to a file. This can be used in loadtxoutset afterwards if this snapshot height is supported in the chainparams as well.
    /// Unless the "latest" type is requested, the node will roll back to the requested height and network activity will be suspended during this process. Because of this it is discouraged to interact with the node in any other way during the execution of this call to avoid inconsistent results and race conditions, particularly RPCs that interact with blockstorage.
    /// This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    async fn dump_txout_set(
        &self,
        path: String,
        r#type: Option<String>,
        options: Option<serde_json::Value>,
    ) -> Result<DumpTxOutSetResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(path));
        if let Some(val) = r#type {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = options {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<DumpTxOutSetResponse>("dumptxoutset", &rpc_params).await
    }

    /// Simply echo back the input arguments. This command is for testing.
    /// It will return an internal bug report when arg9='trigger_internal_bug' is passed.
    /// The difference between echo and echojson is that echojson has argument conversion enabled in the client-side table in bitcoin-cli and the GUI. There is no server-side difference.
    async fn echo(
        &self,
        arg0: Option<String>,
        arg1: Option<String>,
        arg2: Option<String>,
        arg3: Option<String>,
        arg4: Option<String>,
        arg5: Option<String>,
        arg6: Option<String>,
        arg7: Option<String>,
        arg8: Option<String>,
        arg9: Option<String>,
    ) -> Result<EchoResponse, Self::Error> {
        let mut rpc_params = vec![];
        if let Some(val) = arg0 {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = arg1 {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = arg2 {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = arg3 {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = arg4 {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = arg5 {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = arg6 {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = arg7 {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = arg8 {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = arg9 {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<EchoResponse>("echo", &rpc_params).await
    }

    /// Echo back the input argument, passing it through a spawned process in a multiprocess build.
    /// This command is for testing.
    async fn echoipc(&self, arg: String) -> Result<EchoipcResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(arg));
        self.call::<EchoipcResponse>("echoipc", &rpc_params).await
    }

    /// Simply echo back the input arguments. This command is for testing.
    /// It will return an internal bug report when arg9='trigger_internal_bug' is passed.
    /// The difference between echo and echojson is that echojson has argument conversion enabled in the client-side table in bitcoin-cli and the GUI. There is no server-side difference.
    async fn echojson(
        &self,
        arg0: Option<String>,
        arg1: Option<String>,
        arg2: Option<String>,
        arg3: Option<String>,
        arg4: Option<String>,
        arg5: Option<String>,
        arg6: Option<String>,
        arg7: Option<String>,
        arg8: Option<String>,
        arg9: Option<String>,
    ) -> Result<EchojsonResponse, Self::Error> {
        let mut rpc_params = vec![];
        if let Some(val) = arg0 {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = arg1 {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = arg2 {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = arg3 {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = arg4 {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = arg5 {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = arg6 {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = arg7 {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = arg8 {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = arg9 {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<EchojsonResponse>("echojson", &rpc_params).await
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
    ///
    /// Requires wallet private keys to be available (e.g. unlocked).
    async fn encrypt_wallet(
        &self,
        passphrase: String,
    ) -> Result<EncryptWalletResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(passphrase));
        self.call::<EncryptWalletResponse>("encryptwallet", &rpc_params).await
    }

    /// Returns a list of external signers from -signer.
    async fn enumerate_signers(&self) -> Result<EnumerateSignersResponse, Self::Error> {
        self.call::<EnumerateSignersResponse>("enumeratesigners", &[]).await
    }

    /// WARNING: This interface is unstable and may disappear or change!
    /// WARNING: This is an advanced API call that is tightly coupled to the specific
    /// implementation of fee estimation. The parameters it can be called with
    /// and the results it returns will change if the internal implementation changes.
    /// Estimates the approximate fee per kilobyte needed for a transaction to begin
    /// confirmation within conf_target blocks if possible. Uses virtual transaction size as
    /// defined in BIP 141 (witness data is discounted).
    async fn estimate_raw_fee(
        &self,
        conf_target: i64,
        threshold: Option<i64>,
    ) -> Result<EstimateRawFeeResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(conf_target));
        if let Some(val) = threshold {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<EstimateRawFeeResponse>("estimaterawfee", &rpc_params).await
    }

    /// Estimates the approximate fee per kilobyte needed for a transaction to begin
    /// confirmation within conf_target blocks if possible and return the number of blocks
    /// for which the estimate is valid. Uses virtual transaction size as defined
    /// in BIP 141 (witness data is discounted).
    async fn estimate_smart_fee(
        &self,
        conf_target: i64,
        estimate_mode: Option<String>,
    ) -> Result<EstimateSmartFeeResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(conf_target));
        if let Some(val) = estimate_mode {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<EstimateSmartFeeResponse>("estimatesmartfee", &rpc_params).await
    }

    /// Finalize the inputs of a PSBT. If the transaction is fully signed, it will produce a
    /// network serialized transaction which can be broadcast with sendrawtransaction. Otherwise a PSBT will be
    /// created which has the final_scriptSig and final_scriptwitness fields filled for inputs that are complete.
    /// Implements the Finalizer and Extractor roles.
    async fn finalize_psbt(
        &self,
        psbt: String,
        extract: Option<bool>,
    ) -> Result<FinalizePsbtResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(psbt));
        if let Some(val) = extract {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<FinalizePsbtResponse>("finalizepsbt", &rpc_params).await
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
    async fn fund_raw_transaction(
        &self,
        hexstring: String,
        options: Option<serde_json::Value>,
        iswitness: Option<bool>,
    ) -> Result<FundRawTransactionResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(hexstring));
        if let Some(val) = options {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = iswitness {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<FundRawTransactionResponse>("fundrawtransaction", &rpc_params).await
    }

    /// has been replaced by the -generate cli option. Refer to -help for more information.
    async fn generate(&self) -> Result<(), Self::Error> { self.call::<()>("generate", &[]).await }

    /// Mine a set of ordered transactions to a specified address or descriptor and return the block hash.
    async fn generate_block(
        &self,
        output: String,
        transactions: Vec<serde_json::Value>,
        submit: Option<bool>,
    ) -> Result<GenerateBlockResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(output));
        rpc_params.push(serde_json::json!(transactions));
        if let Some(val) = submit {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<GenerateBlockResponse>("generateblock", &rpc_params).await
    }

    /// Mine to a specified address and return the block hashes.
    async fn generate_to_address(
        &self,
        nblocks: i64,
        address: bitcoin::Address,
        maxtries: Option<i64>,
    ) -> Result<GenerateToAddressResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(nblocks));
        rpc_params.push(serde_json::json!(address));
        if let Some(val) = maxtries {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<GenerateToAddressResponse>("generatetoaddress", &rpc_params).await
    }

    /// Mine to a specified descriptor and return the block hashes.
    async fn generate_to_descriptor(
        &self,
        num_blocks: i64,
        descriptor: String,
        maxtries: Option<i64>,
    ) -> Result<GenerateToDescriptorResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(num_blocks));
        rpc_params.push(serde_json::json!(descriptor));
        if let Some(val) = maxtries {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<GenerateToDescriptorResponse>("generatetodescriptor", &rpc_params).await
    }

    /// Returns information about the given added node, or all added nodes
    /// (note that onetry addnodes are not listed here)
    async fn get_added_node_info(
        &self,
        node: Option<String>,
    ) -> Result<GetAddedNodeInfoResponse, Self::Error> {
        let mut rpc_params = vec![];
        if let Some(val) = node {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<GetAddedNodeInfoResponse>("getaddednodeinfo", &rpc_params).await
    }

    /// Returns the list of addresses assigned the specified label.
    async fn get_addresses_by_label(
        &self,
        label: String,
    ) -> Result<GetAddressesByLabelResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(label));
        self.call::<GetAddressesByLabelResponse>("getaddressesbylabel", &rpc_params).await
    }

    /// Return information about the given bitcoin address.
    /// Some of the information will only be present if the address is in the active wallet.
    async fn get_address_info(
        &self,
        address: bitcoin::Address,
    ) -> Result<GetAddressInfoResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(address));
        self.call::<GetAddressInfoResponse>("getaddressinfo", &rpc_params).await
    }

    /// Provides information about the node's address manager by returning the number of addresses in the `new` and `tried` tables and their sum for all networks.
    async fn get_addrman_info(&self) -> Result<GetAddrManInfoResponse, Self::Error> {
        self.call::<GetAddrManInfoResponse>("getaddrmaninfo", &[]).await
    }

    /// Returns the total available balance.
    /// The available balance is what the wallet considers currently spendable, and is
    /// thus affected by options which limit spendability such as -spendzeroconfchange.
    async fn get_balance(
        &self,
        dummy: Option<String>,
        minconf: Option<i64>,
        include_watchonly: Option<bool>,
        avoid_reuse: Option<bool>,
    ) -> Result<GetBalanceResponse, Self::Error> {
        let mut rpc_params = vec![];
        if let Some(val) = dummy {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = minconf {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = include_watchonly {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = avoid_reuse {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<GetBalanceResponse>("getbalance", &rpc_params).await
    }

    /// Returns an object with all balances in BTC.
    async fn get_balances(&self) -> Result<GetBalancesResponse, Self::Error> {
        self.call::<GetBalancesResponse>("getbalances", &[]).await
    }

    /// Returns the hash of the best (tip) block in the most-work fully-validated chain.
    async fn get_best_block_hash(&self) -> Result<GetBestBlockHashResponse, Self::Error> {
        self.call::<GetBestBlockHashResponse>("getbestblockhash", &[]).await
    }

    /// If verbosity is 0, returns a string that is serialized, hex-encoded data for block 'hash'.
    /// If verbosity is 1, returns an Object with information about block &lt;hash&gt;.
    /// If verbosity is 2, returns an Object with information about block &lt;hash&gt; and information about each transaction.
    /// If verbosity is 3, returns an Object with information about block &lt;hash&gt; and information about each transaction, including prevout information for inputs (only for unpruned blocks in the current best chain).
    async fn get_block(
        &self,
        blockhash: bitcoin::BlockHash,
        verbosity: Option<i64>,
    ) -> Result<GetBlockResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(blockhash));
        if let Some(val) = verbosity {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<GetBlockResponse>("getblock", &rpc_params).await
    }

    /// Returns an object containing various state info regarding blockchain processing.
    async fn get_blockchain_info(&self) -> Result<GetBlockchainInfoResponse, Self::Error> {
        self.call::<GetBlockchainInfoResponse>("getblockchaininfo", &[]).await
    }

    /// Returns the height of the most-work fully-validated chain.
    /// The genesis block has height 0.
    async fn get_block_count(&self) -> Result<GetBlockCountResponse, Self::Error> {
        self.call::<GetBlockCountResponse>("getblockcount", &[]).await
    }

    /// Retrieve a BIP 157 content filter for a particular block.
    async fn get_block_filter(
        &self,
        blockhash: bitcoin::BlockHash,
        filtertype: Option<String>,
    ) -> Result<GetBlockFilterResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(blockhash));
        if let Some(val) = filtertype {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<GetBlockFilterResponse>("getblockfilter", &rpc_params).await
    }

    /// Attempt to fetch block from a given peer.
    /// We must have the header for this block, e.g. using submitheader.
    /// The block will not have any undo data which can limit the usage of the block data in a context where the undo data is needed.
    /// Subsequent calls for the same block may cause the response from the previous peer to be ignored.
    /// Peers generally ignore requests for a stale block that they never fully verified, or one that is more than a month old.
    /// When a peer does not respond with a block, we will disconnect.
    /// Note: The block could be re-pruned as soon as it is received.
    /// Returns an empty JSON object if the request was successfully scheduled.
    async fn get_block_from_peer(
        &self,
        blockhash: bitcoin::BlockHash,
        peer_id: i64,
    ) -> Result<GetBlockFromPeerResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(blockhash));
        rpc_params.push(serde_json::json!(peer_id));
        self.call::<GetBlockFromPeerResponse>("getblockfrompeer", &rpc_params).await
    }

    /// Returns hash of block in best-block-chain at height provided.
    async fn get_block_hash(&self, height: i64) -> Result<GetBlockHashResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(height));
        self.call::<GetBlockHashResponse>("getblockhash", &rpc_params).await
    }

    /// If verbose is false, returns a string that is serialized, hex-encoded data for blockheader 'hash'.
    /// If verbose is true, returns an Object with information about blockheader &lt;hash&gt;.
    async fn get_block_header(
        &self,
        blockhash: bitcoin::BlockHash,
        verbose: Option<bool>,
    ) -> Result<GetBlockHeaderResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(blockhash));
        if let Some(val) = verbose {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<GetBlockHeaderResponse>("getblockheader", &rpc_params).await
    }

    /// Compute per block statistics for a given window. All amounts are in satoshis.
    /// It won't work for some heights with pruning.
    async fn get_block_stats(
        &self,
        hash_or_height: i64,
        stats: Option<Vec<serde_json::Value>>,
    ) -> Result<GetBlockStatsResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(hash_or_height));
        if let Some(val) = stats {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<GetBlockStatsResponse>("getblockstats", &rpc_params).await
    }

    /// If the request parameters include a 'mode' key, that is used to explicitly select between the default 'template' request or a 'proposal'.
    /// It returns data needed to construct a block to work on.
    /// For full specification, see BIPs 22, 23, 9, and 145:
    /// <https://github.com/bitcoin/bips/blob/master/bip-0022.mediawiki>
    /// <https://github.com/bitcoin/bips/blob/master/bip-0023.mediawiki>
    /// <https://github.com/bitcoin/bips/blob/master/bip-0009.mediawiki#getblocktemplate_changes>
    /// <https://github.com/bitcoin/bips/blob/master/bip-0145.mediawiki>
    async fn get_block_template(
        &self,
        template_request: serde_json::Value,
    ) -> Result<GetBlockTemplateResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(template_request));
        self.call::<GetBlockTemplateResponse>("getblocktemplate", &rpc_params).await
    }

    /// Return information about chainstates.
    async fn get_chain_states(&self) -> Result<GetChainStatesResponse, Self::Error> {
        self.call::<GetChainStatesResponse>("getchainstates", &[]).await
    }

    /// Return information about all known tips in the block tree, including the main chain as well as orphaned branches.
    async fn get_chain_tips(&self) -> Result<GetChainTipsResponse, Self::Error> {
        self.call::<GetChainTipsResponse>("getchaintips", &[]).await
    }

    /// Compute statistics about the total number and rate of transactions in the chain.
    async fn get_chain_tx_stats(
        &self,
        nblocks: Option<i64>,
        blockhash: Option<bitcoin::BlockHash>,
    ) -> Result<GetChainTxStatsResponse, Self::Error> {
        let mut rpc_params = vec![];
        if let Some(val) = nblocks {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = blockhash {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<GetChainTxStatsResponse>("getchaintxstats", &rpc_params).await
    }

    /// Returns the number of connections to other nodes.
    async fn get_connection_count(&self) -> Result<GetConnectionCountResponse, Self::Error> {
        self.call::<GetConnectionCountResponse>("getconnectioncount", &[]).await
    }

    /// Returns an object containing various state info regarding deployments of consensus changes.
    async fn get_deployment_info(
        &self,
        blockhash: Option<bitcoin::BlockHash>,
    ) -> Result<GetDeploymentInfoResponse, Self::Error> {
        let mut rpc_params = vec![];
        if let Some(val) = blockhash {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<GetDeploymentInfoResponse>("getdeploymentinfo", &rpc_params).await
    }

    /// Get spend and receive activity associated with a set of descriptors for a set of blocks. This command pairs well with the `relevant_blocks` output of `scanblocks()`.
    /// This call may take several minutes. If you encounter timeouts, try specifying no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    async fn get_descriptor_activity(
        &self,
        blockhashes: Vec<serde_json::Value>,
        scanobjects: Vec<serde_json::Value>,
        include_mempool: Option<bool>,
    ) -> Result<GetDescriptorActivityResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(blockhashes));
        rpc_params.push(serde_json::json!(scanobjects));
        if let Some(val) = include_mempool {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<GetDescriptorActivityResponse>("getdescriptoractivity", &rpc_params).await
    }

    /// Analyses a descriptor.
    async fn get_descriptor_info(
        &self,
        descriptor: String,
    ) -> Result<GetDescriptorInfoResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(descriptor));
        self.call::<GetDescriptorInfoResponse>("getdescriptorinfo", &rpc_params).await
    }

    /// Returns the proof-of-work difficulty as a multiple of the minimum difficulty.
    async fn get_difficulty(&self) -> Result<GetDifficultyResponse, Self::Error> {
        self.call::<GetDifficultyResponse>("getdifficulty", &[]).await
    }

    /// List all BIP 32 HD keys in the wallet and which descriptors use them.
    async fn get_hd_keys(
        &self,
        options: Option<serde_json::Value>,
    ) -> Result<GetHdKeysResponse, Self::Error> {
        let mut rpc_params = vec![];
        if let Some(val) = options {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<GetHdKeysResponse>("gethdkeys", &rpc_params).await
    }

    /// Returns the status of one or all available indices currently running in the node.
    async fn get_index_info(
        &self,
        index_name: Option<String>,
    ) -> Result<GetIndexInfoResponse, Self::Error> {
        let mut rpc_params = vec![];
        if let Some(val) = index_name {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<GetIndexInfoResponse>("getindexinfo", &rpc_params).await
    }

    /// Returns an object containing information about memory usage.
    async fn get_memory_info(
        &self,
        mode: Option<String>,
    ) -> Result<GetMemoryInfoResponse, Self::Error> {
        let mut rpc_params = vec![];
        if let Some(val) = mode {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<GetMemoryInfoResponse>("getmemoryinfo", &rpc_params).await
    }

    /// If txid is in the mempool, returns all in-mempool ancestors.
    async fn get_mempool_ancestors(
        &self,
        txid: bitcoin::Txid,
        verbose: Option<bool>,
    ) -> Result<GetMempoolAncestorsResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(txid));
        if let Some(val) = verbose {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<GetMempoolAncestorsResponse>("getmempoolancestors", &rpc_params).await
    }

    /// Returns mempool data for given cluster
    async fn get_mempool_cluster(
        &self,
        txid: bitcoin::Txid,
    ) -> Result<GetMempoolClusterResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(txid));
        self.call::<GetMempoolClusterResponse>("getmempoolcluster", &rpc_params).await
    }

    /// If txid is in the mempool, returns all in-mempool descendants.
    async fn get_mempool_descendants(
        &self,
        txid: bitcoin::Txid,
        verbose: Option<bool>,
    ) -> Result<GetMempoolDescendantsResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(txid));
        if let Some(val) = verbose {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<GetMempoolDescendantsResponse>("getmempooldescendants", &rpc_params).await
    }

    /// Returns mempool data for given transaction
    async fn get_mempool_entry(
        &self,
        txid: bitcoin::Txid,
    ) -> Result<GetMempoolEntryResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(txid));
        self.call::<GetMempoolEntryResponse>("getmempoolentry", &rpc_params).await
    }

    /// Returns the feerate diagram for the whole mempool.
    async fn get_mempool_fee_rate_diagram(
        &self,
    ) -> Result<GetMempoolFeeRateDiagramResponse, Self::Error> {
        self.call::<GetMempoolFeeRateDiagramResponse>("getmempoolfeeratediagram", &[]).await
    }

    /// Returns details on the active state of the TX memory pool.
    async fn get_mempool_info(&self) -> Result<GetMempoolInfoResponse, Self::Error> {
        self.call::<GetMempoolInfoResponse>("getmempoolinfo", &[]).await
    }

    /// Returns a json object containing mining-related information.
    async fn get_mining_info(&self) -> Result<GetMiningInfoResponse, Self::Error> {
        self.call::<GetMiningInfoResponse>("getmininginfo", &[]).await
    }

    /// Returns information about network traffic, including bytes in, bytes out,
    /// and current system time.
    async fn get_net_totals(&self) -> Result<GetNetTotalsResponse, Self::Error> {
        self.call::<GetNetTotalsResponse>("getnettotals", &[]).await
    }

    /// Returns the estimated network hashes per second based on the last n blocks.
    /// Pass in \[blocks\] to override # of blocks, -1 specifies since last difficulty change.
    /// Pass in \[height\] to estimate the network speed at the time when a certain block was found.
    async fn get_network_hashps(
        &self,
        nblocks: Option<i64>,
        height: Option<i64>,
    ) -> Result<GetNetworkHashPsResponse, Self::Error> {
        let mut rpc_params = vec![];
        if let Some(val) = nblocks {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = height {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<GetNetworkHashPsResponse>("getnetworkhashps", &rpc_params).await
    }

    /// Returns an object containing various state info regarding P2P networking.
    async fn get_network_info(&self) -> Result<GetNetworkInfoResponse, Self::Error> {
        self.call::<GetNetworkInfoResponse>("getnetworkinfo", &[]).await
    }

    /// Returns a new Bitcoin address for receiving payments.
    /// If 'label' is specified, it is added to the address book
    /// so payments received with the address will be associated with 'label'.
    async fn get_new_address(
        &self,
        label: Option<String>,
        address_type: Option<String>,
    ) -> Result<GetNewAddressResponse, Self::Error> {
        let mut rpc_params = vec![];
        if let Some(val) = label {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = address_type {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<GetNewAddressResponse>("getnewaddress", &rpc_params).await
    }

    /// Return known addresses, after filtering for quality and recency.
    /// These can potentially be used to find new peers in the network.
    /// The total number of addresses known to the node may be higher.
    async fn get_node_addresses(
        &self,
        count: Option<i64>,
        network: Option<String>,
    ) -> Result<GetNodeAddressesResponse, Self::Error> {
        let mut rpc_params = vec![];
        if let Some(val) = count {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = network {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<GetNodeAddressesResponse>("getnodeaddresses", &rpc_params).await
    }

    /// Shows transactions in the tx orphanage.
    /// EXPERIMENTAL warning: this call may be changed in future releases.
    async fn get_orphan_txs(
        &self,
        verbosity: Option<i64>,
    ) -> Result<GetOrphanTxsResponse, Self::Error> {
        let mut rpc_params = vec![];
        if let Some(val) = verbosity {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<GetOrphanTxsResponse>("getorphantxs", &rpc_params).await
    }

    /// Returns data about each connected network peer as a json array of objects.
    async fn get_peer_info(&self) -> Result<GetPeerInfoResponse, Self::Error> {
        self.call::<GetPeerInfoResponse>("getpeerinfo", &[]).await
    }

    /// Returns a map of all user-created (see prioritisetransaction) fee deltas by txid, and whether the tx is present in mempool.
    async fn get_prioritised_transactions(
        &self,
    ) -> Result<GetPrioritisedTransactionsResponse, Self::Error> {
        self.call::<GetPrioritisedTransactionsResponse>("getprioritisedtransactions", &[]).await
    }

    /// EXPERIMENTAL warning: this call may be changed in future releases.
    /// Returns information on all address manager entries for the new and tried tables.
    async fn get_raw_addrman(&self) -> Result<GetRawAddrManResponse, Self::Error> {
        self.call::<GetRawAddrManResponse>("getrawaddrman", &[]).await
    }

    /// Returns a new Bitcoin address, for receiving change.
    /// This is for use with raw transactions, NOT normal use.
    async fn get_raw_change_address(
        &self,
        address_type: Option<String>,
    ) -> Result<GetRawChangeAddressResponse, Self::Error> {
        let mut rpc_params = vec![];
        if let Some(val) = address_type {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<GetRawChangeAddressResponse>("getrawchangeaddress", &rpc_params).await
    }

    /// Returns all transaction ids in memory pool as a json array of string transaction ids.
    /// Hint: use getmempoolentry to fetch a specific transaction from the mempool.
    async fn get_raw_mempool(
        &self,
        verbose: Option<bool>,
        mempool_sequence: Option<bool>,
    ) -> Result<GetRawMempoolResponse, Self::Error> {
        let mut rpc_params = vec![];
        if let Some(val) = verbose {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = mempool_sequence {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<GetRawMempoolResponse>("getrawmempool", &rpc_params).await
    }

    /// By default, this call only returns a transaction if it is in the mempool. If -txindex is enabled
    /// and no blockhash argument is passed, it will return the transaction if it is in the mempool or any block.
    /// If a blockhash argument is passed, it will return the transaction if
    /// the specified block is available and the transaction is in that block.
    /// Hint: Use gettransaction for wallet transactions.
    /// If verbosity is 0 or omitted, returns the serialized transaction as a hex-encoded string.
    /// If verbosity is 1, returns a JSON Object with information about the transaction.
    /// If verbosity is 2, returns a JSON Object with information about the transaction, including fee and prevout information.
    async fn get_raw_transaction(
        &self,
        txid: bitcoin::Txid,
        verbosity: Option<i64>,
        blockhash: Option<bitcoin::BlockHash>,
    ) -> Result<GetRawTransactionResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(txid));
        if let Some(val) = verbosity {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = blockhash {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<GetRawTransactionResponse>("getrawtransaction", &rpc_params).await
    }

    /// Returns the total amount received by the given address in transactions with at least minconf confirmations.
    async fn get_received_by_address(
        &self,
        address: bitcoin::Address,
        minconf: Option<i64>,
        include_immature_coinbase: Option<bool>,
    ) -> Result<GetReceivedByAddressResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(address));
        if let Some(val) = minconf {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = include_immature_coinbase {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<GetReceivedByAddressResponse>("getreceivedbyaddress", &rpc_params).await
    }

    /// Returns the total amount received by addresses with &lt;label&gt; in transactions with at least \[minconf\] confirmations.
    async fn get_received_by_label(
        &self,
        label: String,
        minconf: Option<i64>,
        include_immature_coinbase: Option<bool>,
    ) -> Result<GetReceivedByLabelResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(label));
        if let Some(val) = minconf {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = include_immature_coinbase {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<GetReceivedByLabelResponse>("getreceivedbylabel", &rpc_params).await
    }

    /// Returns details of the RPC server.
    async fn get_rpc_info(&self) -> Result<GetRpcInfoResponse, Self::Error> {
        self.call::<GetRpcInfoResponse>("getrpcinfo", &[]).await
    }

    /// Get detailed information about in-wallet transaction &lt;txid&gt;
    async fn get_transaction(
        &self,
        txid: bitcoin::Txid,
        include_watchonly: Option<bool>,
        verbose: Option<bool>,
    ) -> Result<GetTransactionResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(txid));
        if let Some(val) = include_watchonly {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = verbose {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<GetTransactionResponse>("gettransaction", &rpc_params).await
    }

    /// Returns details about an unspent transaction output.
    async fn get_txout(
        &self,
        txid: bitcoin::Txid,
        n: i64,
        include_mempool: Option<bool>,
    ) -> Result<GetTxOutResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(txid));
        rpc_params.push(serde_json::json!(n));
        if let Some(val) = include_mempool {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<GetTxOutResponse>("gettxout", &rpc_params).await
    }

    /// Returns a hex-encoded proof that "txid" was included in a block.
    /// NOTE: By default this function only works sometimes. This is when there is an
    /// unspent output in the utxo for this transaction. To make it always work,
    /// you need to maintain a transaction index, using the -txindex command line option or
    /// specify the block in which the transaction is included manually (by blockhash).
    async fn get_txout_proof(
        &self,
        txids: Vec<serde_json::Value>,
        blockhash: Option<bitcoin::BlockHash>,
    ) -> Result<GetTxOutProofResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(txids));
        if let Some(val) = blockhash {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<GetTxOutProofResponse>("gettxoutproof", &rpc_params).await
    }

    /// Returns statistics about the unspent transaction output set.
    /// Note this call may take some time if you are not using coinstatsindex.
    async fn get_txout_set_info(
        &self,
        hash_type: Option<String>,
        hash_or_height: Option<i64>,
        use_index: Option<bool>,
    ) -> Result<GetTxOutSetInfoResponse, Self::Error> {
        let mut rpc_params = vec![];
        if let Some(val) = hash_type {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = hash_or_height {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = use_index {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<GetTxOutSetInfoResponse>("gettxoutsetinfo", &rpc_params).await
    }

    /// Scans the mempool to find transactions spending any of the given outputs
    async fn get_tx_spending_prevout(
        &self,
        outputs: Vec<serde_json::Value>,
    ) -> Result<GetTxSpendingPrevOutResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(outputs));
        self.call::<GetTxSpendingPrevOutResponse>("gettxspendingprevout", &rpc_params).await
    }

    /// Returns an object containing various wallet state info.
    async fn get_wallet_info(&self) -> Result<GetWalletInfoResponse, Self::Error> {
        self.call::<GetWalletInfoResponse>("getwalletinfo", &[]).await
    }

    /// List all commands, or get help for a specified command.
    async fn help(&self, command: Option<String>) -> Result<HelpResponse, Self::Error> {
        let mut rpc_params = vec![];
        if let Some(val) = command {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<HelpResponse>("help", &rpc_params).await
    }

    /// Import descriptors. This will trigger a rescan of the blockchain based on the earliest timestamp of all descriptors being imported. Requires a new wallet backup.
    /// When importing descriptors with multipath key expressions, if the multipath specifier contains exactly two elements, the descriptor produced from the second element will be imported as an internal descriptor.
    /// Note: This call can take over an hour to complete if using an early timestamp; during that time, other rpc calls
    /// may report that the imported keys, addresses or scripts exist but related transactions are still missing.
    /// The rescan is significantly faster if block filters are available (using startup option "-blockfilterindex=1").
    async fn import_descriptors(
        &self,
        requests: Vec<serde_json::Value>,
    ) -> Result<ImportDescriptorsResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(requests));
        self.call::<ImportDescriptorsResponse>("importdescriptors", &rpc_params).await
    }

    /// Import a mempool.dat file and attempt to add its contents to the mempool.
    /// Warning: Importing untrusted files is dangerous, especially if metadata from the file is taken over.
    async fn import_mempool(
        &self,
        filepath: String,
        options: Option<serde_json::Value>,
    ) -> Result<ImportMempoolResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(filepath));
        if let Some(val) = options {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<ImportMempoolResponse>("importmempool", &rpc_params).await
    }

    /// Imports funds without rescan. Corresponding address or script must previously be included in wallet. Aimed towards pruned wallets. The end-user is responsible to import additional transactions that subsequently spend the imported outputs or rescan after the point in the blockchain the transaction is included.
    async fn import_pruned_funds(
        &self,
        rawtransaction: String,
        txoutproof: String,
    ) -> Result<ImportPrunedFundsResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(rawtransaction));
        rpc_params.push(serde_json::json!(txoutproof));
        self.call::<ImportPrunedFundsResponse>("importprunedfunds", &rpc_params).await
    }

    /// Permanently marks a block as invalid, as if it violated a consensus rule.
    async fn invalidate_block(
        &self,
        blockhash: bitcoin::BlockHash,
    ) -> Result<InvalidateBlockResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(blockhash));
        self.call::<InvalidateBlockResponse>("invalidateblock", &rpc_params).await
    }

    /// Joins multiple distinct PSBTs with different inputs and outputs into one PSBT with inputs and outputs from all of the PSBTs
    /// No input in any of the PSBTs can be in more than one of the PSBTs.
    async fn join_psbts(
        &self,
        txs: Vec<serde_json::Value>,
    ) -> Result<JoinPsbtsResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(txs));
        self.call::<JoinPsbtsResponse>("joinpsbts", &rpc_params).await
    }

    /// Refills each descriptor keypool in the wallet up to the specified number of new keys.
    /// By default, descriptor wallets have 4 active ranged descriptors ("legacy", "p2sh-segwit", "bech32", "bech32m"), each with 1000 entries.
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    async fn keypool_refill(
        &self,
        newsize: Option<i64>,
    ) -> Result<KeypoolRefillResponse, Self::Error> {
        let mut rpc_params = vec![];
        if let Some(val) = newsize {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<KeypoolRefillResponse>("keypoolrefill", &rpc_params).await
    }

    /// Lists groups of addresses which have had their common ownership
    /// made public by common use as inputs or as the resulting change
    /// in past transactions
    async fn list_address_groupings(&self) -> Result<ListAddressGroupingsResponse, Self::Error> {
        self.call::<ListAddressGroupingsResponse>("listaddressgroupings", &[]).await
    }

    /// List all manually banned IPs/Subnets.
    async fn list_banned(&self) -> Result<ListBannedResponse, Self::Error> {
        self.call::<ListBannedResponse>("listbanned", &[]).await
    }

    /// List all descriptors present in a wallet.
    async fn list_descriptors(
        &self,
        private: Option<bool>,
    ) -> Result<ListDescriptorsResponse, Self::Error> {
        let mut rpc_params = vec![];
        if let Some(val) = private {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<ListDescriptorsResponse>("listdescriptors", &rpc_params).await
    }

    /// Returns the list of all labels, or labels that are assigned to addresses with a specific purpose.
    async fn list_labels(
        &self,
        purpose: Option<String>,
    ) -> Result<ListLabelsResponse, Self::Error> {
        let mut rpc_params = vec![];
        if let Some(val) = purpose {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<ListLabelsResponse>("listlabels", &rpc_params).await
    }

    /// Returns list of temporarily unspendable outputs.
    /// See the lockunspent call to lock and unlock transactions for spending.
    async fn list_lock_unspent(&self) -> Result<ListLockUnspentResponse, Self::Error> {
        self.call::<ListLockUnspentResponse>("listlockunspent", &[]).await
    }

    /// List balances by receiving address.
    async fn list_received_by_address(
        &self,
        minconf: Option<i64>,
        include_empty: Option<bool>,
        include_watchonly: Option<bool>,
        address_filter: Option<String>,
        include_immature_coinbase: Option<bool>,
    ) -> Result<ListReceivedByAddressResponse, Self::Error> {
        let mut rpc_params = vec![];
        if let Some(val) = minconf {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = include_empty {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = include_watchonly {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = address_filter {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = include_immature_coinbase {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<ListReceivedByAddressResponse>("listreceivedbyaddress", &rpc_params).await
    }

    /// List received transactions by label.
    async fn list_received_by_label(
        &self,
        minconf: Option<i64>,
        include_empty: Option<bool>,
        include_watchonly: Option<bool>,
        include_immature_coinbase: Option<bool>,
    ) -> Result<ListReceivedByLabelResponse, Self::Error> {
        let mut rpc_params = vec![];
        if let Some(val) = minconf {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = include_empty {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = include_watchonly {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = include_immature_coinbase {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<ListReceivedByLabelResponse>("listreceivedbylabel", &rpc_params).await
    }

    /// Get all transactions in blocks since block \[blockhash\], or all transactions if omitted.
    /// If "blockhash" is no longer a part of the main chain, transactions from the fork point onward are included.
    /// Additionally, if include_removed is set, transactions affecting the wallet which were removed are returned in the "removed" array.
    async fn list_since_block(
        &self,
        blockhash: Option<bitcoin::BlockHash>,
        target_confirmations: Option<i64>,
        include_watchonly: Option<bool>,
        include_removed: Option<bool>,
        include_change: Option<bool>,
        label: Option<String>,
    ) -> Result<ListSinceBlockResponse, Self::Error> {
        let mut rpc_params = vec![];
        if let Some(val) = blockhash {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = target_confirmations {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = include_watchonly {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = include_removed {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = include_change {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = label {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<ListSinceBlockResponse>("listsinceblock", &rpc_params).await
    }

    /// If a label name is provided, this will return only incoming transactions paying to addresses with the specified label.
    /// Returns up to 'count' most recent transactions ordered from oldest to newest while skipping the first number of
    /// transactions specified in the 'skip' argument. A transaction can have multiple entries in this RPC response.
    /// For instance, a wallet transaction that pays three addresses — one wallet-owned and two external — will produce
    /// four entries. The payment to the wallet-owned address appears both as a send entry and as a receive entry.
    /// As a result, the RPC response will contain one entry in the receive category and three entries in the send category.
    async fn list_transactions(
        &self,
        label: Option<String>,
        count: Option<i64>,
        skip: Option<i64>,
        include_watchonly: Option<bool>,
    ) -> Result<ListTransactionsResponse, Self::Error> {
        let mut rpc_params = vec![];
        if let Some(val) = label {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = count {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = skip {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = include_watchonly {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<ListTransactionsResponse>("listtransactions", &rpc_params).await
    }

    /// Returns array of unspent transaction outputs
    /// with between minconf and maxconf (inclusive) confirmations.
    /// Optionally filter to only include txouts paid to specified addresses.
    async fn list_unspent(
        &self,
        minconf: Option<i64>,
        maxconf: Option<i64>,
        addresses: Option<Vec<serde_json::Value>>,
        include_unsafe: Option<bool>,
        query_options: Option<serde_json::Value>,
    ) -> Result<ListUnspentResponse, Self::Error> {
        let mut rpc_params = vec![];
        if let Some(val) = minconf {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = maxconf {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = addresses {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = include_unsafe {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = query_options {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<ListUnspentResponse>("listunspent", &rpc_params).await
    }

    /// Returns a list of wallets in the wallet directory.
    async fn list_wallet_dir(&self) -> Result<ListWalletDirResponse, Self::Error> {
        self.call::<ListWalletDirResponse>("listwalletdir", &[]).await
    }

    /// Returns a list of currently loaded wallets.
    /// For full information on the wallet, use "getwalletinfo"
    async fn list_wallets(&self) -> Result<ListWalletsResponse, Self::Error> {
        self.call::<ListWalletsResponse>("listwallets", &[]).await
    }

    /// Load the serialized UTXO set from a file.
    /// Once this snapshot is loaded, its contents will be deserialized into a second chainstate data structure, which is then used to sync to the network's tip. Meanwhile, the original chainstate will complete the initial block download process in the background, eventually validating up to the block that the snapshot is based upon.
    /// The result is a usable bitcoind instance that is current with the network tip in a matter of minutes rather than hours. UTXO snapshot are typically obtained from third-party sources (HTTP, torrent, etc.) which is reasonable since their contents are always checked by hash.
    /// You can find more information on this process in the `assumeutxo` design document (<https://github.com/bitcoin/bitcoin/blob/master/doc/design/assumeutxo.md>).
    async fn load_txout_set(&self, path: String) -> Result<LoadTxOutSetResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(path));
        self.call::<LoadTxOutSetResponse>("loadtxoutset", &rpc_params).await
    }

    /// Loads a wallet from a wallet file or directory.
    /// Note that all wallet command-line options used when starting bitcoind will be
    /// applied to the new wallet.
    async fn load_wallet(
        &self,
        filename: String,
        load_on_startup: Option<bool>,
    ) -> Result<LoadWalletResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(filename));
        if let Some(val) = load_on_startup {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<LoadWalletResponse>("loadwallet", &rpc_params).await
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
    async fn lock_unspent(
        &self,
        unlock: bool,
        transactions: Option<Vec<serde_json::Value>>,
        persistent: Option<bool>,
    ) -> Result<LockUnspentResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(unlock));
        if let Some(val) = transactions {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = persistent {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<LockUnspentResponse>("lockunspent", &rpc_params).await
    }

    /// Gets and sets the logging configuration.
    /// When called without an argument, returns the list of categories with status that are currently being debug logged or not.
    /// When called with arguments, adds or removes categories from debug logging and return the lists above.
    /// The arguments are evaluated in order "include", "exclude".
    /// If an item is both included and excluded, it will thus end up being excluded.
    /// The valid logging categories are: addrman, bench, blockstorage, cmpctblock, coindb, estimatefee, http, i2p, ipc, kernel, leveldb, libevent, mempool, mempoolrej, net, privatebroadcast, proxy, prune, qt, rand, reindex, rpc, scan, selectcoins, tor, txpackages, txreconciliation, validation, walletdb, zmq
    /// In addition, the following are available as category names with special meanings:
    /// - "all",  "1" : represent all logging categories.
    async fn logging(
        &self,
        include: Option<Vec<serde_json::Value>>,
        exclude: Option<Vec<serde_json::Value>>,
    ) -> Result<LoggingResponse, Self::Error> {
        let mut rpc_params = vec![];
        if let Some(val) = include {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = exclude {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<LoggingResponse>("logging", &rpc_params).await
    }

    /// Migrate the wallet to a descriptor wallet.
    /// A new wallet backup will need to be made.
    /// The migration process will create a backup of the wallet before migrating. This backup
    /// file will be named &lt;wallet name&gt;-&lt;timestamp&gt;.legacy.bak and can be found in the directory
    /// for this wallet. In the event of an incorrect migration, the backup can be restored using restorewallet.
    /// Encrypted wallets must have the passphrase provided as an argument to this call.
    /// This RPC may take a long time to complete. Increasing the RPC client timeout is recommended.
    async fn migrate_wallet(
        &self,
        wallet_name: Option<String>,
        passphrase: Option<String>,
    ) -> Result<MigrateWalletResponse, Self::Error> {
        let mut rpc_params = vec![];
        if let Some(val) = wallet_name {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = passphrase {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<MigrateWalletResponse>("migratewallet", &rpc_params).await
    }

    /// Bump the scheduler into the future (-regtest only)
    async fn mock_scheduler(&self, delta_time: i64) -> Result<MockSchedulerResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(delta_time));
        self.call::<MockSchedulerResponse>("mockscheduler", &rpc_params).await
    }

    /// Requests that a ping be sent to all other nodes, to measure ping time.
    /// Results are provided in getpeerinfo.
    /// Ping command is handled in queue with all other commands, so it measures processing backlog, not just network ping.
    async fn ping(&self) -> Result<PingResponse, Self::Error> {
        self.call::<PingResponse>("ping", &[]).await
    }

    /// Treats a block as if it were received before others with the same work.
    /// A later preciousblock call can override the effect of an earlier one.
    /// The effects of preciousblock are not retained across restarts.
    async fn precious_block(
        &self,
        blockhash: bitcoin::BlockHash,
    ) -> Result<PreciousBlockResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(blockhash));
        self.call::<PreciousBlockResponse>("preciousblock", &rpc_params).await
    }

    /// Accepts the transaction into mined blocks at a higher (or lower) priority
    async fn prioritise_transaction(
        &self,
        txid: bitcoin::Txid,
        dummy: Option<i64>,
        fee_delta: i64,
    ) -> Result<PrioritiseTransactionResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(txid));
        if let Some(val) = dummy {
            rpc_params.push(serde_json::json!(val));
        }
        rpc_params.push(serde_json::json!(fee_delta));
        self.call::<PrioritiseTransactionResponse>("prioritisetransaction", &rpc_params).await
    }

    /// Attempts to delete block and undo data up to a specified height or timestamp, if eligible for pruning.
    /// Requires `-prune` to be enabled at startup. While pruned data may be re-fetched in some cases (e.g., via `getblockfrompeer`), local deletion is irreversible.
    async fn prune_blockchain(&self, height: i64) -> Result<PruneBlockchainResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(height));
        self.call::<PruneBlockchainResponse>("pruneblockchain", &rpc_params).await
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
    async fn psbt_bump_fee(
        &self,
        txid: bitcoin::Txid,
        options: Option<serde_json::Value>,
    ) -> Result<PsbtBumpFeeResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(txid));
        if let Some(val) = options {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<PsbtBumpFeeResponse>("psbtbumpfee", &rpc_params).await
    }

    /// Removes invalidity status of a block, its ancestors and its descendants, reconsider them for activation.
    /// This can be used to undo the effects of invalidateblock.
    async fn reconsider_block(
        &self,
        blockhash: bitcoin::BlockHash,
    ) -> Result<ReconsiderBlockResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(blockhash));
        self.call::<ReconsiderBlockResponse>("reconsiderblock", &rpc_params).await
    }

    /// Deletes the specified transaction from the wallet. Meant for use with pruned wallets and as a companion to importprunedfunds. This will affect wallet balances.
    async fn remove_pruned_funds(
        &self,
        txid: bitcoin::Txid,
    ) -> Result<RemovePrunedFundsResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(txid));
        self.call::<RemovePrunedFundsResponse>("removeprunedfunds", &rpc_params).await
    }

    /// Rescan the local blockchain for wallet related transactions.
    /// Note: Use "getwalletinfo" to query the scanning progress.
    /// The rescan is significantly faster if block filters are available
    /// (using startup option "-blockfilterindex=1").
    async fn rescan_blockchain(
        &self,
        start_height: Option<i64>,
        stop_height: Option<i64>,
    ) -> Result<RescanBlockchainResponse, Self::Error> {
        let mut rpc_params = vec![];
        if let Some(val) = start_height {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = stop_height {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<RescanBlockchainResponse>("rescanblockchain", &rpc_params).await
    }

    /// Restores and loads a wallet from backup.
    /// The rescan is significantly faster if block filters are available
    /// (using startup option "-blockfilterindex=1").
    async fn restore_wallet(
        &self,
        wallet_name: String,
        backup_file: String,
        load_on_startup: Option<bool>,
    ) -> Result<RestoreWalletResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(wallet_name));
        rpc_params.push(serde_json::json!(backup_file));
        if let Some(val) = load_on_startup {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<RestoreWalletResponse>("restorewallet", &rpc_params).await
    }

    /// Dumps the mempool to disk. It will fail until the previous dump is fully loaded.
    async fn save_mempool(&self) -> Result<SaveMempoolResponse, Self::Error> {
        self.call::<SaveMempoolResponse>("savemempool", &[]).await
    }

    /// Return relevant blockhashes for given descriptors (requires blockfilterindex).
    /// This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    async fn scan_blocks(
        &self,
        action: String,
        scanobjects: Option<Vec<serde_json::Value>>,
        start_height: Option<i64>,
        stop_height: Option<i64>,
        filtertype: Option<String>,
        options: Option<serde_json::Value>,
    ) -> Result<ScanBlocksResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(action));
        if let Some(val) = scanobjects {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = start_height {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = stop_height {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = filtertype {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = options {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<ScanBlocksResponse>("scanblocks", &rpc_params).await
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
    async fn scan_txout_set(
        &self,
        action: String,
        scanobjects: Option<Vec<serde_json::Value>>,
    ) -> Result<ScanTxOutSetResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(action));
        if let Some(val) = scanobjects {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<ScanTxOutSetResponse>("scantxoutset", &rpc_params).await
    }

    /// Return RPC command JSON Schema descriptions.
    async fn schema(&self) -> Result<SchemaResponse, Self::Error> {
        self.call::<SchemaResponse>("schema", &[]).await
    }

    /// EXPERIMENTAL warning: this call may be changed in future releases.
    /// Send a transaction.
    async fn send(
        &self,
        outputs: Vec<serde_json::Value>,
        conf_target: Option<i64>,
        estimate_mode: Option<String>,
        fee_rate: Option<serde_json::Value>,
        options: Option<serde_json::Value>,
        version: Option<i64>,
    ) -> Result<SendResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(outputs));
        if let Some(val) = conf_target {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = estimate_mode {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = fee_rate {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = options {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = version {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<SendResponse>("send", &rpc_params).await
    }

    /// EXPERIMENTAL warning: this call may be changed in future releases.
    /// Spend the value of all (or specific) confirmed UTXOs and unconfirmed change in the wallet to one or more recipients.
    /// Unconfirmed inbound UTXOs and locked UTXOs will not be spent. Sendall will respect the avoid_reuse wallet flag.
    /// If your wallet contains many small inputs, either because it received tiny payments or as a result of accumulating change, consider using `send_max` to exclude inputs that are worth less than the fees needed to spend them.
    async fn send_all(
        &self,
        recipients: Vec<serde_json::Value>,
        conf_target: Option<i64>,
        estimate_mode: Option<String>,
        fee_rate: Option<serde_json::Value>,
        options: Option<serde_json::Value>,
    ) -> Result<SendAllResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(recipients));
        if let Some(val) = conf_target {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = estimate_mode {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = fee_rate {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = options {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<SendAllResponse>("sendall", &rpc_params).await
    }

    /// Send multiple times. Amounts are double-precision floating point numbers.
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    async fn send_many(
        &self,
        dummy: Option<String>,
        amounts: serde_json::Value,
        minconf: Option<i64>,
        comment: Option<String>,
        subtractfeefrom: Option<Vec<serde_json::Value>>,
        replaceable: Option<bool>,
        conf_target: Option<i64>,
        estimate_mode: Option<String>,
        fee_rate: Option<serde_json::Value>,
        verbose: Option<bool>,
    ) -> Result<SendManyResponse, Self::Error> {
        let mut rpc_params = vec![];
        if let Some(val) = dummy {
            rpc_params.push(serde_json::json!(val));
        }
        rpc_params.push(serde_json::json!(amounts));
        if let Some(val) = minconf {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = comment {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = subtractfeefrom {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = replaceable {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = conf_target {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = estimate_mode {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = fee_rate {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = verbose {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<SendManyResponse>("sendmany", &rpc_params).await
    }

    /// Send a p2p message to a peer specified by id.
    /// The message type and body must be provided, the message header will be generated.
    /// This RPC is for testing only.
    async fn send_msg_to_peer(
        &self,
        peer_id: i64,
        msg_type: String,
        msg: String,
    ) -> Result<SendMsgToPeerResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(peer_id));
        rpc_params.push(serde_json::json!(msg_type));
        rpc_params.push(serde_json::json!(msg));
        self.call::<SendMsgToPeerResponse>("sendmsgtopeer", &rpc_params).await
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
    async fn send_raw_transaction(
        &self,
        hexstring: String,
        maxfeerate: Option<serde_json::Value>,
        maxburnamount: Option<serde_json::Value>,
    ) -> Result<SendRawTransactionResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(hexstring));
        if let Some(val) = maxfeerate {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = maxburnamount {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<SendRawTransactionResponse>("sendrawtransaction", &rpc_params).await
    }

    /// Send an amount to a given address.
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    async fn send_to_address(
        &self,
        address: bitcoin::Address,
        amount: serde_json::Value,
        comment: Option<String>,
        comment_to: Option<String>,
        subtractfeefromamount: Option<bool>,
        replaceable: Option<bool>,
        conf_target: Option<i64>,
        estimate_mode: Option<String>,
        avoid_reuse: Option<bool>,
        fee_rate: Option<serde_json::Value>,
        verbose: Option<bool>,
    ) -> Result<SendToAddressResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(address));
        rpc_params.push(serde_json::json!(amount));
        if let Some(val) = comment {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = comment_to {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = subtractfeefromamount {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = replaceable {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = conf_target {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = estimate_mode {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = avoid_reuse {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = fee_rate {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = verbose {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<SendToAddressResponse>("sendtoaddress", &rpc_params).await
    }

    /// Attempts to add or remove an IP/Subnet from the banned list.
    async fn set_ban(
        &self,
        subnet: String,
        command: String,
        bantime: Option<i64>,
        absolute: Option<bool>,
    ) -> Result<SetBanResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(subnet));
        rpc_params.push(serde_json::json!(command));
        if let Some(val) = bantime {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = absolute {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<SetBanResponse>("setban", &rpc_params).await
    }

    /// Sets the label associated with the given address.
    async fn set_label(
        &self,
        address: bitcoin::Address,
        label: String,
    ) -> Result<SetLabelResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(address));
        rpc_params.push(serde_json::json!(label));
        self.call::<SetLabelResponse>("setlabel", &rpc_params).await
    }

    /// Set the local time to given timestamp (-regtest only)
    async fn set_mock_time(&self, timestamp: i64) -> Result<SetMockTimeResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(timestamp));
        self.call::<SetMockTimeResponse>("setmocktime", &rpc_params).await
    }

    /// Disable/enable all p2p network activity.
    async fn set_network_active(
        &self,
        state: bool,
    ) -> Result<SetNetworkActiveResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(state));
        self.call::<SetNetworkActiveResponse>("setnetworkactive", &rpc_params).await
    }

    /// Change the state of the given wallet flag for a wallet.
    async fn set_wallet_flag(
        &self,
        flag: String,
        value: Option<bool>,
    ) -> Result<SetWalletFlagResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(flag));
        if let Some(val) = value {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<SetWalletFlagResponse>("setwalletflag", &rpc_params).await
    }

    /// Sign a message with the private key of an address
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    ///
    /// Requires wallet private keys to be available (e.g. unlocked).
    async fn sign_message(
        &self,
        address: bitcoin::Address,
        message: String,
    ) -> Result<SignMessageResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(address));
        rpc_params.push(serde_json::json!(message));
        self.call::<SignMessageResponse>("signmessage", &rpc_params).await
    }

    /// Sign a message with the private key of an address
    async fn sign_message_with_priv_key(
        &self,
        privkey: String,
        message: String,
    ) -> Result<SignMessageWithPrivKeyResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(privkey));
        rpc_params.push(serde_json::json!(message));
        self.call::<SignMessageWithPrivKeyResponse>("signmessagewithprivkey", &rpc_params).await
    }

    /// Sign inputs for raw transaction (serialized, hex-encoded).
    /// The second argument is an array of base58-encoded private
    /// keys that will be the only keys used to sign the transaction.
    /// The third optional argument (may be null) is an array of previous transaction outputs that
    /// this transaction depends on but may not yet be in the block chain.
    async fn sign_raw_transaction_with_key(
        &self,
        hexstring: String,
        privkeys: Vec<serde_json::Value>,
        prevtxs: Option<Vec<serde_json::Value>>,
        sighashtype: Option<String>,
    ) -> Result<SignRawTransactionWithKeyResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(hexstring));
        rpc_params.push(serde_json::json!(privkeys));
        if let Some(val) = prevtxs {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = sighashtype {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<SignRawTransactionWithKeyResponse>("signrawtransactionwithkey", &rpc_params)
            .await
    }

    /// Sign inputs for raw transaction (serialized, hex-encoded).
    /// The second optional argument (may be null) is an array of previous transaction outputs that
    /// this transaction depends on but may not yet be in the block chain.
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    ///
    /// Requires wallet private keys to be available (e.g. unlocked).
    async fn sign_raw_transaction_with_wallet(
        &self,
        hexstring: String,
        prevtxs: Option<Vec<serde_json::Value>>,
        sighashtype: Option<String>,
    ) -> Result<SignRawTransactionWithWalletResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(hexstring));
        if let Some(val) = prevtxs {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = sighashtype {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<SignRawTransactionWithWalletResponse>(
            "signrawtransactionwithwallet",
            &rpc_params,
        )
        .await
    }

    /// Calculate the balance change resulting in the signing and broadcasting of the given transaction(s).
    async fn simulate_raw_transaction(
        &self,
        rawtxs: Option<Vec<serde_json::Value>>,
        options: Option<serde_json::Value>,
    ) -> Result<SimulateRawTransactionResponse, Self::Error> {
        let mut rpc_params = vec![];
        if let Some(val) = rawtxs {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = options {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<SimulateRawTransactionResponse>("simulaterawtransaction", &rpc_params).await
    }

    /// Request a graceful shutdown of Bitcoin Core.
    async fn stop(&self, wait: Option<i64>) -> Result<StopResponse, Self::Error> {
        let mut rpc_params = vec![];
        if let Some(val) = wait {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<StopResponse>("stop", &rpc_params).await
    }

    /// Attempts to submit new block to network.
    /// See <https://en.bitcoin.it/wiki/BIP_0022> for full specification.
    async fn submit_block(
        &self,
        hexdata: String,
        dummy: Option<String>,
    ) -> Result<SubmitBlockResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(hexdata));
        if let Some(val) = dummy {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<SubmitBlockResponse>("submitblock", &rpc_params).await
    }

    /// Decode the given hexdata as a header and submit it as a candidate chain tip if valid.
    /// Throws when the header is invalid.
    async fn submit_header(&self, hexdata: String) -> Result<SubmitHeaderResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(hexdata));
        self.call::<SubmitHeaderResponse>("submitheader", &rpc_params).await
    }

    /// Submit a package of raw transactions (serialized, hex-encoded) to local node.
    /// The package will be validated according to consensus and mempool policy rules. If any transaction passes, it will be accepted to mempool.
    /// This RPC is experimental and the interface may be unstable. Refer to doc/policy/packages.md for documentation on package policies.
    /// Warning: successful submission does not mean the transactions will propagate throughout the network.
    async fn submit_package(
        &self,
        package: Vec<serde_json::Value>,
        maxfeerate: Option<serde_json::Value>,
        maxburnamount: Option<serde_json::Value>,
    ) -> Result<SubmitPackageResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(package));
        if let Some(val) = maxfeerate {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = maxburnamount {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<SubmitPackageResponse>("submitpackage", &rpc_params).await
    }

    /// Waits for the validation interface queue to catch up on everything that was there when we entered this function.
    async fn sync_with_validation_interface_queue(
        &self,
    ) -> Result<SyncWithValidationInterfaceQueueResponse, Self::Error> {
        self.call::<SyncWithValidationInterfaceQueueResponse>(
            "syncwithvalidationinterfacequeue",
            &[],
        )
        .await
    }

    /// Returns result of mempool acceptance tests indicating if raw transaction(s) (serialized, hex-encoded) would be accepted by mempool.
    /// If multiple transactions are passed in, parents must come before children and package policies apply: the transactions cannot conflict with any mempool transactions or each other.
    /// If one transaction fails, other transactions may not be fully validated (the 'allowed' key will be blank).
    /// The maximum number of transactions allowed is 25.
    /// This checks if transactions violate the consensus or policy rules.
    /// See sendrawtransaction call.
    async fn test_mempool_accept(
        &self,
        rawtxs: Vec<serde_json::Value>,
        maxfeerate: Option<serde_json::Value>,
    ) -> Result<TestMempoolAcceptResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(rawtxs));
        if let Some(val) = maxfeerate {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<TestMempoolAcceptResponse>("testmempoolaccept", &rpc_params).await
    }

    /// Unloads the wallet referenced by the request endpoint or the wallet_name argument.
    /// If both are specified, they must be identical.
    async fn unload_wallet(
        &self,
        wallet_name: Option<String>,
        load_on_startup: Option<bool>,
    ) -> Result<UnloadWalletResponse, Self::Error> {
        let mut rpc_params = vec![];
        if let Some(val) = wallet_name {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = load_on_startup {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<UnloadWalletResponse>("unloadwallet", &rpc_params).await
    }

    /// Returns the total uptime of the server.
    async fn uptime(&self) -> Result<UptimeResponse, Self::Error> {
        self.call::<UptimeResponse>("uptime", &[]).await
    }

    /// Updates all segwit inputs and outputs in a PSBT with data from output descriptors, the UTXO set, txindex, or the mempool.
    async fn utxo_update_psbt(
        &self,
        psbt: String,
        descriptors: Option<Vec<serde_json::Value>>,
    ) -> Result<UtxoUpdatePsbtResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(psbt));
        if let Some(val) = descriptors {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<UtxoUpdatePsbtResponse>("utxoupdatepsbt", &rpc_params).await
    }

    /// Return information about the given bitcoin address.
    async fn validate_address(
        &self,
        address: bitcoin::Address,
    ) -> Result<ValidateAddressResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(address));
        self.call::<ValidateAddressResponse>("validateaddress", &rpc_params).await
    }

    /// Verifies blockchain database.
    async fn verify_chain(
        &self,
        checklevel: Option<i64>,
        nblocks: Option<i64>,
    ) -> Result<VerifyChainResponse, Self::Error> {
        let mut rpc_params = vec![];
        if let Some(val) = checklevel {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = nblocks {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<VerifyChainResponse>("verifychain", &rpc_params).await
    }

    /// Verify a signed message.
    async fn verify_message(
        &self,
        address: bitcoin::Address,
        signature: String,
        message: String,
    ) -> Result<VerifyMessageResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(address));
        rpc_params.push(serde_json::json!(signature));
        rpc_params.push(serde_json::json!(message));
        self.call::<VerifyMessageResponse>("verifymessage", &rpc_params).await
    }

    /// Verifies that a proof points to a transaction in a block, returning the transaction it commits to
    /// and throwing an RPC error if the block is not in our best chain
    async fn verify_txout_proof(
        &self,
        proof: String,
    ) -> Result<VerifyTxOutProofResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(proof));
        self.call::<VerifyTxOutProofResponse>("verifytxoutproof", &rpc_params).await
    }

    /// Waits for a specific new block and returns useful info about it.
    /// Returns the current block on timeout or exit.
    /// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    async fn wait_for_block(
        &self,
        blockhash: bitcoin::BlockHash,
        timeout: Option<i64>,
    ) -> Result<WaitForBlockResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(blockhash));
        if let Some(val) = timeout {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<WaitForBlockResponse>("waitforblock", &rpc_params).await
    }

    /// Waits for (at least) block height and returns the height and hash
    /// of the current tip.
    /// Returns the current block on timeout or exit.
    /// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    async fn wait_for_block_height(
        &self,
        height: i64,
        timeout: Option<i64>,
    ) -> Result<WaitForBlockHeightResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(height));
        if let Some(val) = timeout {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<WaitForBlockHeightResponse>("waitforblockheight", &rpc_params).await
    }

    /// Waits for any new block and returns useful info about it.
    /// Returns the current block on timeout or exit.
    /// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    async fn wait_for_new_block(
        &self,
        timeout: Option<i64>,
        current_tip: Option<String>,
    ) -> Result<WaitForNewBlockResponse, Self::Error> {
        let mut rpc_params = vec![];
        if let Some(val) = timeout {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = current_tip {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<WaitForNewBlockResponse>("waitfornewblock", &rpc_params).await
    }

    /// Creates and funds a transaction in the Partially Signed Transaction format.
    /// Implements the Creator and Updater roles.
    /// All existing inputs must either have their previous output transaction be in the wallet
    /// or be in the UTXO set. Solving data must be provided for non-wallet inputs.
    async fn wallet_create_funded_psbt(
        &self,
        inputs: Option<Vec<serde_json::Value>>,
        outputs: Vec<serde_json::Value>,
        locktime: Option<i64>,
        options: Option<serde_json::Value>,
        bip32derivs: Option<bool>,
        version: Option<i64>,
    ) -> Result<WalletCreateFundedPsbtResponse, Self::Error> {
        let mut rpc_params = vec![];
        if let Some(val) = inputs {
            rpc_params.push(serde_json::json!(val));
        }
        rpc_params.push(serde_json::json!(outputs));
        if let Some(val) = locktime {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = options {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = bip32derivs {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = version {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<WalletCreateFundedPsbtResponse>("walletcreatefundedpsbt", &rpc_params).await
    }

    /// Display address on an external signer for verification.
    async fn wallet_display_address(
        &self,
        address: bitcoin::Address,
    ) -> Result<WalletDisplayAddressResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(address));
        self.call::<WalletDisplayAddressResponse>("walletdisplayaddress", &rpc_params).await
    }

    /// Removes the wallet encryption key from memory, locking the wallet.
    /// After calling this method, you will need to call walletpassphrase again
    /// before being able to call any methods which require the wallet to be unlocked.
    ///
    /// Requires wallet private keys to be available (e.g. unlocked).
    async fn wallet_lock(&self) -> Result<WalletLockResponse, Self::Error> {
        self.call::<WalletLockResponse>("walletlock", &[]).await
    }

    /// Stores the wallet decryption key in memory for 'timeout' seconds.
    /// This is needed prior to performing transactions related to private keys such as sending bitcoins
    /// Note:
    /// Issuing the walletpassphrase command while the wallet is already unlocked will set a new unlock
    /// time that overrides the old one.
    ///
    /// Requires wallet private keys to be available (e.g. unlocked).
    async fn wallet_passphrase(
        &self,
        passphrase: String,
        timeout: i64,
    ) -> Result<WalletPassphraseResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(passphrase));
        rpc_params.push(serde_json::json!(timeout));
        self.call::<WalletPassphraseResponse>("walletpassphrase", &rpc_params).await
    }

    /// Changes the wallet passphrase from 'oldpassphrase' to 'newpassphrase'.
    ///
    /// Requires wallet private keys to be available (e.g. unlocked).
    async fn wallet_passphrase_change(
        &self,
        oldpassphrase: String,
        newpassphrase: String,
    ) -> Result<WalletPassphraseChangeResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(oldpassphrase));
        rpc_params.push(serde_json::json!(newpassphrase));
        self.call::<WalletPassphraseChangeResponse>("walletpassphrasechange", &rpc_params).await
    }

    /// Update a PSBT with input information from our wallet and then sign inputs
    /// that we can sign for.
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    async fn wallet_process_psbt(
        &self,
        psbt: String,
        sign: Option<bool>,
        sighashtype: Option<String>,
        bip32derivs: Option<bool>,
        finalize: Option<bool>,
    ) -> Result<WalletProcessPsbtResponse, Self::Error> {
        let mut rpc_params = vec![];
        rpc_params.push(serde_json::json!(psbt));
        if let Some(val) = sign {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = sighashtype {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = bip32derivs {
            rpc_params.push(serde_json::json!(val));
        }
        if let Some(val) = finalize {
            rpc_params.push(serde_json::json!(val));
        }
        self.call::<WalletProcessPsbtResponse>("walletprocesspsbt", &rpc_params).await
    }
}
