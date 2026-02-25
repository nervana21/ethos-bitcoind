//! wallet RPC method wrappers
//!
//! This module contains transport wrappers for wallet methods.

use serde_json::{json, Value};

use crate::transport::core::{TransportError, TransportTrait};

/// Mark in-wallet transaction &lt;txid&gt; as abandoned
/// This will mark this transaction and all its in-wallet descendants as abandoned which will allow
/// for their inputs to be respent.  It can be used to replace "stuck" or evicted transactions.
/// It only works on transactions which are not included in a block and are not currently in the mempool.
/// It has no effect on transactions which are already abandoned.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.abandontransaction(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::abandontransaction(&transport, ...).await`
///
/// Calls the `abandontransaction` RPC method.
pub async fn abandon_transaction(
    transport: &dyn TransportTrait,
    txid: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(txid)];
    let raw = transport.send_request("abandontransaction", &params).await?;
    Ok(raw)
}

/// Stops current wallet rescan triggered by an RPC call, e.g. by a rescanblockchain call.
/// Note: Use "getwalletinfo" to query the scanning progress.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.abortrescan(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::abortrescan(&transport, ...).await`
///
/// Calls the `abortrescan` RPC method.
pub async fn abort_rescan(transport: &dyn TransportTrait) -> Result<Value, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("abortrescan", &params).await?;
    Ok(raw)
}

/// Safely copies the current wallet file to the specified destination, which can either be a directory or a path with a filename.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.backupwallet(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::backupwallet(&transport, ...).await`
///
/// Calls the `backupwallet` RPC method.
pub async fn backup_wallet(
    transport: &dyn TransportTrait,
    destination: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(destination)];
    let raw = transport.send_request("backupwallet", &params).await?;
    Ok(raw)
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
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.bumpfee(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::bumpfee(&transport, ...).await`
///
/// Calls the `bumpfee` RPC method.
pub async fn bump_fee(
    transport: &dyn TransportTrait,
    txid: serde_json::Value,
    options: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(txid), json!(options)];
    let raw = transport.send_request("bumpfee", &params).await?;
    Ok(raw)
}

/// Creates and loads a new wallet.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.createwallet(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::createwallet(&transport, ...).await`
///
/// Calls the `createwallet` RPC method.
#[allow(clippy::too_many_arguments)]
pub async fn create_wallet(
    transport: &dyn TransportTrait,
    wallet_name: serde_json::Value,
    disable_private_keys: serde_json::Value,
    blank: serde_json::Value,
    passphrase: serde_json::Value,
    avoid_reuse: serde_json::Value,
    descriptors: serde_json::Value,
    load_on_startup: serde_json::Value,
    external_signer: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![
        json!(wallet_name),
        json!(disable_private_keys),
        json!(blank),
        json!(passphrase),
        json!(avoid_reuse),
        json!(descriptors),
        json!(load_on_startup),
        json!(external_signer),
    ];
    let raw = transport.send_request("createwallet", &params).await?;
    Ok(raw)
}

/// Creates the wallet's descriptor for the given address type. The address type must be one that the wallet does not already have a descriptor for.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.createwalletdescriptor(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::createwalletdescriptor(&transport, ...).await`
///
/// Calls the `createwalletdescriptor` RPC method.
pub async fn create_wallet_descriptor(
    transport: &dyn TransportTrait,
    r#type: serde_json::Value,
    options: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(r#type), json!(options)];
    let raw = transport.send_request("createwalletdescriptor", &params).await?;
    Ok(raw)
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
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.encryptwallet(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::encryptwallet(&transport, ...).await`
///
/// Calls the `encryptwallet` RPC method.
pub async fn encrypt_wallet(
    transport: &dyn TransportTrait,
    passphrase: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(passphrase)];
    let raw = transport.send_request("encryptwallet", &params).await?;
    Ok(raw)
}

/// Returns the list of addresses assigned the specified label.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.getaddressesbylabel(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::getaddressesbylabel(&transport, ...).await`
///
/// Calls the `getaddressesbylabel` RPC method.
pub async fn get_addresses_by_label(
    transport: &dyn TransportTrait,
    label: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(label)];
    let raw = transport.send_request("getaddressesbylabel", &params).await?;
    Ok(raw)
}

/// Return information about the given bitcoin address.
/// Some of the information will only be present if the address is in the active wallet.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.getaddressinfo(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::getaddressinfo(&transport, ...).await`
///
/// Calls the `getaddressinfo` RPC method.
pub async fn get_address_info(
    transport: &dyn TransportTrait,
    address: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(address)];
    let raw = transport.send_request("getaddressinfo", &params).await?;
    Ok(raw)
}

/// Returns the total available balance.
/// The available balance is what the wallet considers currently spendable, and is
/// thus affected by options which limit spendability such as -spendzeroconfchange.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.getbalance(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::getbalance(&transport, ...).await`
///
/// Calls the `getbalance` RPC method.
pub async fn get_balance(
    transport: &dyn TransportTrait,
    dummy: serde_json::Value,
    minconf: serde_json::Value,
    include_watchonly: serde_json::Value,
    avoid_reuse: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(dummy), json!(minconf), json!(include_watchonly), json!(avoid_reuse)];
    let raw = transport.send_request("getbalance", &params).await?;
    Ok(raw)
}

/// Returns an object with all balances in BTC.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.getbalances(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::getbalances(&transport, ...).await`
///
/// Calls the `getbalances` RPC method.
pub async fn get_balances(transport: &dyn TransportTrait) -> Result<Value, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getbalances", &params).await?;
    Ok(raw)
}

/// List all BIP 32 HD keys in the wallet and which descriptors use them.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.gethdkeys(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::gethdkeys(&transport, ...).await`
///
/// Calls the `gethdkeys` RPC method.
pub async fn get_hd_keys(
    transport: &dyn TransportTrait,
    options: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(options)];
    let raw = transport.send_request("gethdkeys", &params).await?;
    Ok(raw)
}

/// Returns a new Bitcoin address for receiving payments.
/// If 'label' is specified, it is added to the address book
/// so payments received with the address will be associated with 'label'.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.getnewaddress(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::getnewaddress(&transport, ...).await`
///
/// Calls the `getnewaddress` RPC method.
pub async fn get_new_address(
    transport: &dyn TransportTrait,
    label: serde_json::Value,
    address_type: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(label), json!(address_type)];
    let raw = transport.send_request("getnewaddress", &params).await?;
    Ok(raw)
}

/// Returns a new Bitcoin address, for receiving change.
/// This is for use with raw transactions, NOT normal use.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.getrawchangeaddress(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::getrawchangeaddress(&transport, ...).await`
///
/// Calls the `getrawchangeaddress` RPC method.
pub async fn get_raw_change_address(
    transport: &dyn TransportTrait,
    address_type: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(address_type)];
    let raw = transport.send_request("getrawchangeaddress", &params).await?;
    Ok(raw)
}

/// Returns the total amount received by the given address in transactions with at least minconf confirmations.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.getreceivedbyaddress(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::getreceivedbyaddress(&transport, ...).await`
///
/// Calls the `getreceivedbyaddress` RPC method.
pub async fn get_received_by_address(
    transport: &dyn TransportTrait,
    address: serde_json::Value,
    minconf: serde_json::Value,
    include_immature_coinbase: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(address), json!(minconf), json!(include_immature_coinbase)];
    let raw = transport.send_request("getreceivedbyaddress", &params).await?;
    Ok(raw)
}

/// Returns the total amount received by addresses with &lt;label&gt; in transactions with at least \[minconf\] confirmations.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.getreceivedbylabel(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::getreceivedbylabel(&transport, ...).await`
///
/// Calls the `getreceivedbylabel` RPC method.
pub async fn get_received_by_label(
    transport: &dyn TransportTrait,
    label: serde_json::Value,
    minconf: serde_json::Value,
    include_immature_coinbase: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(label), json!(minconf), json!(include_immature_coinbase)];
    let raw = transport.send_request("getreceivedbylabel", &params).await?;
    Ok(raw)
}

/// Get detailed information about in-wallet transaction &lt;txid&gt;
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.gettransaction(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::gettransaction(&transport, ...).await`
///
/// Calls the `gettransaction` RPC method.
pub async fn get_transaction(
    transport: &dyn TransportTrait,
    txid: serde_json::Value,
    include_watchonly: serde_json::Value,
    verbose: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(txid), json!(include_watchonly), json!(verbose)];
    let raw = transport.send_request("gettransaction", &params).await?;
    Ok(raw)
}

/// Returns an object containing various wallet state info.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.getwalletinfo(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::getwalletinfo(&transport, ...).await`
///
/// Calls the `getwalletinfo` RPC method.
pub async fn get_wallet_info(transport: &dyn TransportTrait) -> Result<Value, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("getwalletinfo", &params).await?;
    Ok(raw)
}

/// Import descriptors. This will trigger a rescan of the blockchain based on the earliest timestamp of all descriptors being imported. Requires a new wallet backup.
/// When importing descriptors with multipath key expressions, if the multipath specifier contains exactly two elements, the descriptor produced from the second element will be imported as an internal descriptor.
/// Note: This call can take over an hour to complete if using an early timestamp; during that time, other rpc calls
/// may report that the imported keys, addresses or scripts exist but related transactions are still missing.
/// The rescan is significantly faster if block filters are available (using startup option "-blockfilterindex=1").
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.importdescriptors(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::importdescriptors(&transport, ...).await`
///
/// Calls the `importdescriptors` RPC method.
pub async fn import_descriptors(
    transport: &dyn TransportTrait,
    requests: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(requests)];
    let raw = transport.send_request("importdescriptors", &params).await?;
    Ok(raw)
}

/// Imports funds without rescan. Corresponding address or script must previously be included in wallet. Aimed towards pruned wallets. The end-user is responsible to import additional transactions that subsequently spend the imported outputs or rescan after the point in the blockchain the transaction is included.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.importprunedfunds(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::importprunedfunds(&transport, ...).await`
///
/// Calls the `importprunedfunds` RPC method.
pub async fn import_pruned_funds(
    transport: &dyn TransportTrait,
    rawtransaction: serde_json::Value,
    txoutproof: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(rawtransaction), json!(txoutproof)];
    let raw = transport.send_request("importprunedfunds", &params).await?;
    Ok(raw)
}

/// Refills each descriptor keypool in the wallet up to the specified number of new keys.
/// By default, descriptor wallets have 4 active ranged descriptors ("legacy", "p2sh-segwit", "bech32", "bech32m"), each with 1000 entries.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.keypoolrefill(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::keypoolrefill(&transport, ...).await`
///
/// Calls the `keypoolrefill` RPC method.
pub async fn keypool_refill(
    transport: &dyn TransportTrait,
    newsize: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(newsize)];
    let raw = transport.send_request("keypoolrefill", &params).await?;
    Ok(raw)
}

/// Lists groups of addresses which have had their common ownership
/// made public by common use as inputs or as the resulting change
/// in past transactions
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.listaddressgroupings(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::listaddressgroupings(&transport, ...).await`
///
/// Calls the `listaddressgroupings` RPC method.
pub async fn list_address_groupings(
    transport: &dyn TransportTrait,
) -> Result<Value, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("listaddressgroupings", &params).await?;
    Ok(raw)
}

/// List all descriptors present in a wallet.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.listdescriptors(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::listdescriptors(&transport, ...).await`
///
/// Calls the `listdescriptors` RPC method.
pub async fn list_descriptors(
    transport: &dyn TransportTrait,
    private: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(private)];
    let raw = transport.send_request("listdescriptors", &params).await?;
    Ok(raw)
}

/// Returns the list of all labels, or labels that are assigned to addresses with a specific purpose.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.listlabels(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::listlabels(&transport, ...).await`
///
/// Calls the `listlabels` RPC method.
pub async fn list_labels(
    transport: &dyn TransportTrait,
    purpose: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(purpose)];
    let raw = transport.send_request("listlabels", &params).await?;
    Ok(raw)
}

/// Returns list of temporarily unspendable outputs.
/// See the lockunspent call to lock and unlock transactions for spending.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.listlockunspent(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::listlockunspent(&transport, ...).await`
///
/// Calls the `listlockunspent` RPC method.
pub async fn list_lock_unspent(transport: &dyn TransportTrait) -> Result<Value, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("listlockunspent", &params).await?;
    Ok(raw)
}

/// List balances by receiving address.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.listreceivedbyaddress(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::listreceivedbyaddress(&transport, ...).await`
///
/// Calls the `listreceivedbyaddress` RPC method.
pub async fn list_received_by_address(
    transport: &dyn TransportTrait,
    minconf: serde_json::Value,
    include_empty: serde_json::Value,
    include_watchonly: serde_json::Value,
    address_filter: serde_json::Value,
    include_immature_coinbase: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![
        json!(minconf),
        json!(include_empty),
        json!(include_watchonly),
        json!(address_filter),
        json!(include_immature_coinbase),
    ];
    let raw = transport.send_request("listreceivedbyaddress", &params).await?;
    Ok(raw)
}

/// List received transactions by label.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.listreceivedbylabel(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::listreceivedbylabel(&transport, ...).await`
///
/// Calls the `listreceivedbylabel` RPC method.
pub async fn list_received_by_label(
    transport: &dyn TransportTrait,
    minconf: serde_json::Value,
    include_empty: serde_json::Value,
    include_watchonly: serde_json::Value,
    include_immature_coinbase: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![
        json!(minconf),
        json!(include_empty),
        json!(include_watchonly),
        json!(include_immature_coinbase),
    ];
    let raw = transport.send_request("listreceivedbylabel", &params).await?;
    Ok(raw)
}

/// Get all transactions in blocks since block \[blockhash\], or all transactions if omitted.
/// If "blockhash" is no longer a part of the main chain, transactions from the fork point onward are included.
/// Additionally, if include_removed is set, transactions affecting the wallet which were removed are returned in the "removed" array.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.listsinceblock(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::listsinceblock(&transport, ...).await`
///
/// Calls the `listsinceblock` RPC method.
pub async fn list_since_block(
    transport: &dyn TransportTrait,
    blockhash: serde_json::Value,
    target_confirmations: serde_json::Value,
    include_watchonly: serde_json::Value,
    include_removed: serde_json::Value,
    include_change: serde_json::Value,
    label: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![
        json!(blockhash),
        json!(target_confirmations),
        json!(include_watchonly),
        json!(include_removed),
        json!(include_change),
        json!(label),
    ];
    let raw = transport.send_request("listsinceblock", &params).await?;
    Ok(raw)
}

/// If a label name is provided, this will return only incoming transactions paying to addresses with the specified label.
/// Returns up to 'count' most recent transactions ordered from oldest to newest while skipping the first number of
/// transactions specified in the 'skip' argument. A transaction can have multiple entries in this RPC response.
/// For instance, a wallet transaction that pays three addresses — one wallet-owned and two external — will produce
/// four entries. The payment to the wallet-owned address appears both as a send entry and as a receive entry.
/// As a result, the RPC response will contain one entry in the receive category and three entries in the send category.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.listtransactions(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::listtransactions(&transport, ...).await`
///
/// Calls the `listtransactions` RPC method.
pub async fn list_transactions(
    transport: &dyn TransportTrait,
    label: serde_json::Value,
    count: serde_json::Value,
    skip: serde_json::Value,
    include_watchonly: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(label), json!(count), json!(skip), json!(include_watchonly)];
    let raw = transport.send_request("listtransactions", &params).await?;
    Ok(raw)
}

/// Returns array of unspent transaction outputs
/// with between minconf and maxconf (inclusive) confirmations.
/// Optionally filter to only include txouts paid to specified addresses.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.listunspent(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::listunspent(&transport, ...).await`
///
/// Calls the `listunspent` RPC method.
pub async fn list_unspent(
    transport: &dyn TransportTrait,
    minconf: serde_json::Value,
    maxconf: serde_json::Value,
    addresses: serde_json::Value,
    include_unsafe: serde_json::Value,
    query_options: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![
        json!(minconf),
        json!(maxconf),
        json!(addresses),
        json!(include_unsafe),
        json!(query_options),
    ];
    let raw = transport.send_request("listunspent", &params).await?;
    Ok(raw)
}

/// Returns a list of wallets in the wallet directory.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.listwalletdir(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::listwalletdir(&transport, ...).await`
///
/// Calls the `listwalletdir` RPC method.
pub async fn list_wallet_dir(transport: &dyn TransportTrait) -> Result<Value, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("listwalletdir", &params).await?;
    Ok(raw)
}

/// Returns a list of currently loaded wallets.
/// For full information on the wallet, use "getwalletinfo"
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.listwallets(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::listwallets(&transport, ...).await`
///
/// Calls the `listwallets` RPC method.
pub async fn list_wallets(transport: &dyn TransportTrait) -> Result<Value, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("listwallets", &params).await?;
    Ok(raw)
}

/// Loads a wallet from a wallet file or directory.
/// Note that all wallet command-line options used when starting bitcoind will be
/// applied to the new wallet.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.loadwallet(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::loadwallet(&transport, ...).await`
///
/// Calls the `loadwallet` RPC method.
pub async fn load_wallet(
    transport: &dyn TransportTrait,
    filename: serde_json::Value,
    load_on_startup: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(filename), json!(load_on_startup)];
    let raw = transport.send_request("loadwallet", &params).await?;
    Ok(raw)
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
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.lockunspent(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::lockunspent(&transport, ...).await`
///
/// Calls the `lockunspent` RPC method.
pub async fn lock_unspent(
    transport: &dyn TransportTrait,
    unlock: serde_json::Value,
    transactions: serde_json::Value,
    persistent: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(unlock), json!(transactions), json!(persistent)];
    let raw = transport.send_request("lockunspent", &params).await?;
    Ok(raw)
}

/// Migrate the wallet to a descriptor wallet.
/// A new wallet backup will need to be made.
/// The migration process will create a backup of the wallet before migrating. This backup
/// file will be named &lt;wallet name&gt;-&lt;timestamp&gt;.legacy.bak and can be found in the directory
/// for this wallet. In the event of an incorrect migration, the backup can be restored using restorewallet.
/// Encrypted wallets must have the passphrase provided as an argument to this call.
/// This RPC may take a long time to complete. Increasing the RPC client timeout is recommended.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.migratewallet(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::migratewallet(&transport, ...).await`
///
/// Calls the `migratewallet` RPC method.
pub async fn migrate_wallet(
    transport: &dyn TransportTrait,
    wallet_name: serde_json::Value,
    passphrase: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(wallet_name), json!(passphrase)];
    let raw = transport.send_request("migratewallet", &params).await?;
    Ok(raw)
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
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.psbtbumpfee(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::psbtbumpfee(&transport, ...).await`
///
/// Calls the `psbtbumpfee` RPC method.
pub async fn psbt_bump_fee(
    transport: &dyn TransportTrait,
    txid: serde_json::Value,
    options: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(txid), json!(options)];
    let raw = transport.send_request("psbtbumpfee", &params).await?;
    Ok(raw)
}

/// Deletes the specified transaction from the wallet. Meant for use with pruned wallets and as a companion to importprunedfunds. This will affect wallet balances.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.removeprunedfunds(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::removeprunedfunds(&transport, ...).await`
///
/// Calls the `removeprunedfunds` RPC method.
pub async fn remove_pruned_funds(
    transport: &dyn TransportTrait,
    txid: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(txid)];
    let raw = transport.send_request("removeprunedfunds", &params).await?;
    Ok(raw)
}

/// Rescan the local blockchain for wallet related transactions.
/// Note: Use "getwalletinfo" to query the scanning progress.
/// The rescan is significantly faster if block filters are available
/// (using startup option "-blockfilterindex=1").
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.rescanblockchain(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::rescanblockchain(&transport, ...).await`
///
/// Calls the `rescanblockchain` RPC method.
pub async fn rescan_blockchain(
    transport: &dyn TransportTrait,
    start_height: serde_json::Value,
    stop_height: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(start_height), json!(stop_height)];
    let raw = transport.send_request("rescanblockchain", &params).await?;
    Ok(raw)
}

/// Restores and loads a wallet from backup.
/// The rescan is significantly faster if block filters are available
/// (using startup option "-blockfilterindex=1").
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.restorewallet(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::restorewallet(&transport, ...).await`
///
/// Calls the `restorewallet` RPC method.
pub async fn restore_wallet(
    transport: &dyn TransportTrait,
    wallet_name: serde_json::Value,
    backup_file: serde_json::Value,
    load_on_startup: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(wallet_name), json!(backup_file), json!(load_on_startup)];
    let raw = transport.send_request("restorewallet", &params).await?;
    Ok(raw)
}

/// EXPERIMENTAL warning: this call may be changed in future releases.
/// Send a transaction.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.send(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::send(&transport, ...).await`
///
/// Calls the `send` RPC method.
pub async fn send(
    transport: &dyn TransportTrait,
    outputs: serde_json::Value,
    conf_target: serde_json::Value,
    estimate_mode: serde_json::Value,
    fee_rate: serde_json::Value,
    options: serde_json::Value,
    version: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![
        json!(outputs),
        json!(conf_target),
        json!(estimate_mode),
        json!(fee_rate),
        json!(options),
        json!(version),
    ];
    let raw = transport.send_request("send", &params).await?;
    Ok(raw)
}

/// EXPERIMENTAL warning: this call may be changed in future releases.
/// Spend the value of all (or specific) confirmed UTXOs and unconfirmed change in the wallet to one or more recipients.
/// Unconfirmed inbound UTXOs and locked UTXOs will not be spent. Sendall will respect the avoid_reuse wallet flag.
/// If your wallet contains many small inputs, either because it received tiny payments or as a result of accumulating change, consider using `send_max` to exclude inputs that are worth less than the fees needed to spend them.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.sendall(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::sendall(&transport, ...).await`
///
/// Calls the `sendall` RPC method.
pub async fn send_all(
    transport: &dyn TransportTrait,
    recipients: serde_json::Value,
    conf_target: serde_json::Value,
    estimate_mode: serde_json::Value,
    fee_rate: serde_json::Value,
    options: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![
        json!(recipients),
        json!(conf_target),
        json!(estimate_mode),
        json!(fee_rate),
        json!(options),
    ];
    let raw = transport.send_request("sendall", &params).await?;
    Ok(raw)
}

/// Send multiple times. Amounts are double-precision floating point numbers.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.sendmany(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::sendmany(&transport, ...).await`
///
/// Calls the `sendmany` RPC method.
#[allow(clippy::too_many_arguments)]
pub async fn send_many(
    transport: &dyn TransportTrait,
    dummy: serde_json::Value,
    amounts: serde_json::Value,
    minconf: serde_json::Value,
    comment: serde_json::Value,
    subtractfeefrom: serde_json::Value,
    replaceable: serde_json::Value,
    conf_target: serde_json::Value,
    estimate_mode: serde_json::Value,
    fee_rate: serde_json::Value,
    verbose: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![
        json!(dummy),
        json!(amounts),
        json!(minconf),
        json!(comment),
        json!(subtractfeefrom),
        json!(replaceable),
        json!(conf_target),
        json!(estimate_mode),
        json!(fee_rate),
        json!(verbose),
    ];
    let raw = transport.send_request("sendmany", &params).await?;
    Ok(raw)
}

/// Send an amount to a given address.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.sendtoaddress(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::sendtoaddress(&transport, ...).await`
///
/// Calls the `sendtoaddress` RPC method.
#[allow(clippy::too_many_arguments)]
pub async fn send_to_address(
    transport: &dyn TransportTrait,
    address: serde_json::Value,
    amount: serde_json::Value,
    comment: serde_json::Value,
    comment_to: serde_json::Value,
    subtractfeefromamount: serde_json::Value,
    replaceable: serde_json::Value,
    conf_target: serde_json::Value,
    estimate_mode: serde_json::Value,
    avoid_reuse: serde_json::Value,
    fee_rate: serde_json::Value,
    verbose: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![
        json!(address),
        json!(amount),
        json!(comment),
        json!(comment_to),
        json!(subtractfeefromamount),
        json!(replaceable),
        json!(conf_target),
        json!(estimate_mode),
        json!(avoid_reuse),
        json!(fee_rate),
        json!(verbose),
    ];
    let raw = transport.send_request("sendtoaddress", &params).await?;
    Ok(raw)
}

/// Sets the label associated with the given address.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.setlabel(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::setlabel(&transport, ...).await`
///
/// Calls the `setlabel` RPC method.
pub async fn set_label(
    transport: &dyn TransportTrait,
    address: serde_json::Value,
    label: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(address), json!(label)];
    let raw = transport.send_request("setlabel", &params).await?;
    Ok(raw)
}

/// Change the state of the given wallet flag for a wallet.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.setwalletflag(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::setwalletflag(&transport, ...).await`
///
/// Calls the `setwalletflag` RPC method.
pub async fn set_wallet_flag(
    transport: &dyn TransportTrait,
    flag: serde_json::Value,
    value: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(flag), json!(value)];
    let raw = transport.send_request("setwalletflag", &params).await?;
    Ok(raw)
}

/// Sign a message with the private key of an address
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.signmessage(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::signmessage(&transport, ...).await`
///
/// Calls the `signmessage` RPC method.
pub async fn sign_message(
    transport: &dyn TransportTrait,
    address: serde_json::Value,
    message: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(address), json!(message)];
    let raw = transport.send_request("signmessage", &params).await?;
    Ok(raw)
}

/// Sign inputs for raw transaction (serialized, hex-encoded).
/// The second optional argument (may be null) is an array of previous transaction outputs that
/// this transaction depends on but may not yet be in the block chain.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.signrawtransactionwithwallet(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::signrawtransactionwithwallet(&transport, ...).await`
///
/// Calls the `signrawtransactionwithwallet` RPC method.
pub async fn sign_raw_transaction_with_wallet(
    transport: &dyn TransportTrait,
    hexstring: serde_json::Value,
    prevtxs: serde_json::Value,
    sighashtype: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(hexstring), json!(prevtxs), json!(sighashtype)];
    let raw = transport.send_request("signrawtransactionwithwallet", &params).await?;
    Ok(raw)
}

/// Calculate the balance change resulting in the signing and broadcasting of the given transaction(s).
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.simulaterawtransaction(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::simulaterawtransaction(&transport, ...).await`
///
/// Calls the `simulaterawtransaction` RPC method.
pub async fn simulate_raw_transaction(
    transport: &dyn TransportTrait,
    rawtxs: serde_json::Value,
    options: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(rawtxs), json!(options)];
    let raw = transport.send_request("simulaterawtransaction", &params).await?;
    Ok(raw)
}

/// Unloads the wallet referenced by the request endpoint or the wallet_name argument.
/// If both are specified, they must be identical.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.unloadwallet(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::unloadwallet(&transport, ...).await`
///
/// Calls the `unloadwallet` RPC method.
pub async fn unload_wallet(
    transport: &dyn TransportTrait,
    wallet_name: serde_json::Value,
    load_on_startup: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(wallet_name), json!(load_on_startup)];
    let raw = transport.send_request("unloadwallet", &params).await?;
    Ok(raw)
}

/// Creates and funds a transaction in the Partially Signed Transaction format.
/// Implements the Creator and Updater roles.
/// All existing inputs must either have their previous output transaction be in the wallet
/// or be in the UTXO set. Solving data must be provided for non-wallet inputs.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.walletcreatefundedpsbt(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::walletcreatefundedpsbt(&transport, ...).await`
///
/// Calls the `walletcreatefundedpsbt` RPC method.
pub async fn wallet_create_funded_psbt(
    transport: &dyn TransportTrait,
    inputs: serde_json::Value,
    outputs: serde_json::Value,
    locktime: serde_json::Value,
    options: serde_json::Value,
    bip32derivs: serde_json::Value,
    version: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![
        json!(inputs),
        json!(outputs),
        json!(locktime),
        json!(options),
        json!(bip32derivs),
        json!(version),
    ];
    let raw = transport.send_request("walletcreatefundedpsbt", &params).await?;
    Ok(raw)
}

/// Display address on an external signer for verification.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.walletdisplayaddress(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::walletdisplayaddress(&transport, ...).await`
///
/// Calls the `walletdisplayaddress` RPC method.
pub async fn wallet_display_address(
    transport: &dyn TransportTrait,
    address: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(address)];
    let raw = transport.send_request("walletdisplayaddress", &params).await?;
    Ok(raw)
}

/// Removes the wallet encryption key from memory, locking the wallet.
/// After calling this method, you will need to call walletpassphrase again
/// before being able to call any methods which require the wallet to be unlocked.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.walletlock(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::walletlock(&transport, ...).await`
///
/// Calls the `walletlock` RPC method.
pub async fn wallet_lock(transport: &dyn TransportTrait) -> Result<Value, TransportError> {
    let params = Vec::<Value>::new();
    let raw = transport.send_request("walletlock", &params).await?;
    Ok(raw)
}

/// Stores the wallet decryption key in memory for 'timeout' seconds.
/// This is needed prior to performing transactions related to private keys such as sending bitcoins
/// Note:
/// Issuing the walletpassphrase command while the wallet is already unlocked will set a new unlock
/// time that overrides the old one.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.walletpassphrase(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::walletpassphrase(&transport, ...).await`
///
/// Calls the `walletpassphrase` RPC method.
pub async fn wallet_passphrase(
    transport: &dyn TransportTrait,
    passphrase: serde_json::Value,
    timeout: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(passphrase), json!(timeout)];
    let raw = transport.send_request("walletpassphrase", &params).await?;
    Ok(raw)
}

/// Changes the wallet passphrase from 'oldpassphrase' to 'newpassphrase'.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.walletpassphrasechange(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::walletpassphrasechange(&transport, ...).await`
///
/// Calls the `walletpassphrasechange` RPC method.
pub async fn wallet_passphrase_change(
    transport: &dyn TransportTrait,
    oldpassphrase: serde_json::Value,
    newpassphrase: serde_json::Value,
) -> Result<Value, TransportError> {
    let params = vec![json!(oldpassphrase), json!(newpassphrase)];
    let raw = transport.send_request("walletpassphrasechange", &params).await?;
    Ok(raw)
}

/// Update a PSBT with input information from our wallet and then sign inputs
/// that we can sign for.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
///
/// # Usage
/// This method can be called using the high-level client interface:
/// - `client.walletprocesspsbt(...).await`
/// Or directly via the transport layer for advanced use cases:
/// - `transport::walletprocesspsbt(&transport, ...).await`
///
/// Calls the `walletprocesspsbt` RPC method.
pub async fn wallet_process_psbt(
    transport: &dyn TransportTrait,
    psbt: serde_json::Value,
    sign: serde_json::Value,
    sighashtype: serde_json::Value,
    bip32derivs: serde_json::Value,
    finalize: serde_json::Value,
) -> Result<Value, TransportError> {
    let params =
        vec![json!(psbt), json!(sign), json!(sighashtype), json!(bip32derivs), json!(finalize)];
    let raw = transport.send_request("walletprocesspsbt", &params).await?;
    Ok(raw)
}
