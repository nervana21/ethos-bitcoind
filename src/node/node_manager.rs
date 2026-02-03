//! Node module for Bitcoin Core RPC testing
//!
//! This module provides utilities for managing Bitcoin Core nodes in test environments.

use std::process::Stdio;
use std::sync::Arc;
use std::time::{Duration, Instant};

use async_trait::async_trait;
use tempfile::TempDir;
use tokio::io::AsyncBufReadExt;
use tokio::process::{Child, Command};
use tokio::sync::{Mutex, RwLock};
use tracing::{debug, error, info};

use crate::test_config::TestConfig;
use crate::transport::core::TransportExt;
use crate::transport::{DefaultTransport, TransportError};

/// Represents the current state of a node
#[derive(Debug, Default, Clone)]
pub struct NodeState {
    /// Whether the node is currently running
    pub is_running: bool,
}

/// Configuration for port selection behavior
#[derive(Debug, Clone)]
pub enum PortSelection {
    /// Use the specified port number
    Fixed(u16),
    /// Let the OS assign an available port
    Dynamic,
    /// Use port 0 (not recommended, may cause daemon to fail)
    Zero,
}

/// Trait defining the interface for a node manager
#[async_trait]
pub trait NodeManager: Send + Sync + std::any::Any + std::fmt::Debug {
    async fn start(&self) -> Result<(), TransportError>;
    async fn stop(&mut self) -> Result<(), TransportError>;
    async fn get_state(&self) -> Result<NodeState, TransportError>;
    /// Return the RPC port this manager was configured with
    fn rpc_port(&self) -> u16;
    /// Return the RPC username this manager was configured with
    fn rpc_username(&self) -> &str;
    /// Return the RPC password this manager was configured with
    fn rpc_password(&self) -> &str;
    /// Create a transport for communicating with the node
    async fn create_transport(
        &self,
    ) -> Result<std::sync::Arc<crate::transport::DefaultTransport>, TransportError>;
}

/// Implementation of the node manager
#[derive(Debug)]
pub struct BitcoinNodeManager {
    /// Shared state of the node
    state: Arc<RwLock<NodeState>>,
    /// Child process handle for the daemon
    child: Arc<Mutex<Option<Child>>>,
    /// RPC port for communication with the node
    pub rpc_port: u16,
    /// Test configuration for the node
    config: TestConfig,
    /// Temporary directory for node data (cleaned up on drop)
    _datadir: Option<TempDir>,
}

impl BitcoinNodeManager {
    /// Create a new node manager with default configuration
    pub fn new() -> Result<Self, TransportError> { Self::new_with_config(&TestConfig::default()) }

    /// Create a new node manager with custom configuration
    pub fn new_with_config(config: &TestConfig) -> Result<Self, TransportError> {
        let datadir = TempDir::new()?;

        // Handle automatic port selection
        let rpc_port = if config.rpc_port == 0 {
            // Get a random free port by binding to 127.0.0.1:0
            // The listener is dropped at the end of the block, freeing the port
            {
                let listener = std::net::TcpListener::bind("127.0.0.1:0")?;
                listener.local_addr()?.port()
            }
        } else {
            config.rpc_port
        };

        Ok(Self {
            state: Arc::new(RwLock::new(NodeState::default())),
            child: Arc::new(Mutex::new(None)),
            rpc_port,
            config: config.clone(),
            _datadir: Some(datadir),
        })
    }

    /// Get the RPC port for this node manager
    pub fn rpc_port(&self) -> u16 { self.rpc_port }

    /// Get the RPC username from the configuration
    pub fn rpc_username(&self) -> &str { &self.config.rpc_username }

    /// Get the RPC password from the configuration
    pub fn rpc_password(&self) -> &str { &self.config.rpc_password }
}

#[async_trait]
impl NodeManager for BitcoinNodeManager {
    async fn start(&self) -> Result<(), TransportError> {
        let mut state = self.state.write().await;
        if state.is_running {
            return Ok(());
        }

        let datadir = self._datadir.as_ref().unwrap().path();
        let mut cmd = Command::new("bitcoind");

        let chain = format!("-chain={}", self.config.as_chain_str());
        let data_dir = format!("-datadir={}", datadir.display());
        let rpc_port = format!("-rpcport={}", self.rpc_port);
        let rpc_bind = format!("-rpcbind=127.0.0.1:{}", self.rpc_port);
        let rpc_user = format!("-rpcuser={}", self.config.rpc_username);
        let rpc_password = format!("-rpcpassword={}", self.config.rpc_password);

        let mut args = vec![
            &chain,
            "-listen=0",
            &data_dir,
            &rpc_port,
            &rpc_bind,
            "-rpcallowip=127.0.0.1",
            "-fallbackfee=0.0002",
            "-server=1",
            "-prune=1",
            &rpc_user,
            &rpc_password,
        ];

        for arg in &self.config.extra_args {
            args.push(arg);
        }

        cmd.args(&args);

        // Capture both stdout and stderr for better error reporting
        cmd.stderr(Stdio::piped());
        cmd.stdout(Stdio::piped());

        let mut child = cmd.spawn()?;

        // Read stderr in a separate task
        let stderr = child.stderr.take().unwrap();
        let stderr_reader = tokio::io::BufReader::new(stderr);
        tokio::spawn(async move {
            let mut lines = stderr_reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                error!("bitcoind stderr: {}", line);
            }
        });

        // Read stdout in a separate task
        let stdout = child.stdout.take().unwrap();
        let stdout_reader = tokio::io::BufReader::new(stdout);
        tokio::spawn(async move {
            let mut lines = stdout_reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                info!("bitcoind stdout: {}", line);
            }
        });

        // Store the child process
        let mut child_guard = self.child.lock().await;
        *child_guard = Some(child);

        info!("Waiting for bitcoind node to initialize...");
        tokio::time::sleep(Duration::from_millis(150)).await;

        // Create transport for RPC health check
        let transport = DefaultTransport::new(
            format!("http://127.0.0.1:{}/", self.rpc_port),
            Some((self.config.rpc_username.clone(), self.config.rpc_password.clone())),
        );

        // Wait for node to be ready
        let deadline = Instant::now() + Duration::from_secs(10);
        let mut attempts = 0;
        while Instant::now() < deadline {
            if let Some(child) = child_guard.as_mut() {
                if let Ok(Some(status)) = child.try_wait() {
                    let error = format!("bitcoind node exited early with status: {}", status);
                    error!("{}", error);
                    return Err(TransportError::Rpc(error));
                }
            }

            // Try to connect to RPC
            match transport.call::<serde_json::Value>("getnetworkinfo", &[]).await {
                Ok(_) => {
                    state.is_running = true;
                    info!("bitcoind node started successfully on port {}", self.rpc_port);
                    return Ok(());
                }
                Err(e) => {
                    debug!("Failed to connect to RPC (attempt {}): {}", attempts, e);
                }
            }

            attempts += 1;
            tokio::time::sleep(Duration::from_millis(200)).await;
        }

        let error = format!(
            "Timed out waiting for bitcoind node to start on port {} after {} attempts",
            self.rpc_port, attempts
        );
        error!("{}", error);
        return Err(TransportError::Rpc(error));
    }

    async fn stop(&mut self) -> Result<(), TransportError> {
        let mut state = self.state.write().await;
        if !state.is_running {
            return Ok(());
        }

        let mut child = self.child.lock().await;
        if let Some(mut child_process) = child.take() {
            info!("Stopping bitcoind node...");
            let _ = child_process.kill().await;
        }

        state.is_running = false;
        info!("bitcoind node stopped");
        Ok(())
    }

    async fn get_state(&self) -> Result<NodeState, TransportError> {
        Ok(self.state.read().await.clone())
    }

    fn rpc_port(&self) -> u16 { self.rpc_port }

    fn rpc_username(&self) -> &str { &self.config.rpc_username }

    fn rpc_password(&self) -> &str { &self.config.rpc_password }

    async fn create_transport(
        &self,
    ) -> Result<std::sync::Arc<crate::transport::DefaultTransport>, TransportError> {
        use std::sync::Arc;

        use crate::transport::DefaultTransport;

        // Create HTTP transport for Bitcoin Core
        let rpc_url = format!("http://127.0.0.1:{}", self.rpc_port());
        let auth = Some((self.rpc_username().to_string(), self.rpc_password().to_string()));
        let transport = Arc::new(DefaultTransport::new(rpc_url, auth));

        // Wait for node to be ready for RPC with Bitcoin Core specific initialization logic
        // Bitcoin Core initialization states that require waiting:
        // -28: RPC in warmup
        // -4:  RPC in warmup (alternative code)
        let init_states = ["\"code\":-28", "\"code\":-4"];

        let max_retries = 30;
        let mut retries = 0;

        loop {
            match transport.call::<serde_json::Value>("getnetworkinfo", &[]).await {
                Ok(_) => break,
                Err(TransportError::Rpc(e)) => {
                    // Check if the error matches any known initialization state
                    let is_init_state = init_states.iter().any(|state| e.contains(state));
                    if is_init_state && retries < max_retries {
                        tracing::debug!(
                            "Waiting for initialization: {} (attempt {}/{})",
                            e,
                            retries + 1,
                            max_retries
                        );
                        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                        retries += 1;
                        continue;
                    }
                    return Err(TransportError::Rpc(e));
                }
                Err(e) => return Err(e),
            }
        }

        if retries > 0 {
            tracing::debug!("Node initialization completed after {} attempts", retries);
        }

        Ok(transport)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extra_args() {
        let config = crate::test_config::TestConfig {
            extra_args: vec!["-debug=1".to_string()],
            ..crate::test_config::TestConfig::default()
        };

        let node_manager = BitcoinNodeManager::new_with_config(&config)
            .expect("Failed to create node manager with extra args");

        assert_eq!(node_manager.config.extra_args[0], "-debug=1");
    }
}
