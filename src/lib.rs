#![forbid(unsafe_code)]
#![allow(missing_docs)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::empty_docs)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
//! Generated Bitcoin Core RPC client library.
//!
//! This library provides a strongly-typed interface to the Bitcoin Core RPC API.
//! It is generated from the Bitcoin Core RPC API documentation.

// Core modules
pub mod bitcoin_core_client;
pub mod client_trait;
pub mod config;
pub mod node;
pub mod test_config;
pub mod transport;
pub mod types;

// Re-exports for ergonomic access
pub use bitcoin::{Address, Amount, BlockHash, Network, ScriptBuf, Txid};
pub use bitcoin_core_client::BitcoinTestClient;
pub use client_trait::BitcoinClient;
pub use config::Config;
pub use node::{BitcoinNodeManager, NodeManager};
pub use test_config::TestConfig;
pub use transport::{DefaultTransport, RpcClient, TransportError};
pub use types::*;
