#![forbid(unsafe_code)]
#![allow(missing_docs)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::empty_docs)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(non_snake_case)]
//! Generated Bitcoin Core RPC client library.
//!
//! This library provides a strongly-typed interface to the Bitcoin Core RPC API.
//! It is generated from the Bitcoin Core RPC API documentation.
//!
//! Enable feature `client` (default) for transport / node manager. Types-only:
//! `default-features = false` plus the RPC category features you need.

// Core modules
#[cfg(feature = "client")]
pub mod bitcoin_core_client;
#[cfg(feature = "client")]
pub mod client_trait;
#[cfg(feature = "client")]
pub mod config;
#[cfg(feature = "client")]
pub mod node;
#[cfg(feature = "client")]
pub mod test_config;
#[cfg(feature = "client")]
pub mod transport;
pub mod types;

// Re-exports for ergonomic access
pub use bitcoin::{Address, Amount, BlockHash, Network, ScriptBuf, Txid};
#[cfg(feature = "client")]
pub use bitcoin_core_client::BitcoinTestClient;
#[cfg(feature = "client")]
pub use client_trait::BitcoinClient;
#[cfg(feature = "client")]
pub use config::Config;
#[cfg(feature = "client")]
pub use node::{BitcoinNodeManager, NodeManager};
#[cfg(feature = "client")]
pub use test_config::TestConfig;
#[cfg(feature = "client")]
pub use transport::{DefaultTransport, RpcClient, TransportError};
pub use types::rpc_prelude::*;
pub use types::{aliases, rpc_prelude, *};
