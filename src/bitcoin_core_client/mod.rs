//! Bitcoin Core test node client
//!
//! This module provides a test client for Bitcoin Core that works with any NodeManager
//! implementation via dependency injection.
pub mod client;
pub mod params;

// re-export common clients
pub use client::BitcoinTestClient;
