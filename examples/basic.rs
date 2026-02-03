use std::sync::Arc;

use ethos_bitcoind::{BitcoinClient, BitcoinNodeManager, DefaultTransport, NodeManager};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut manager = BitcoinNodeManager::new()?;
    manager.start().await?;
    let client: Arc<DefaultTransport> = manager.create_transport().await?;

    let result = client.get_blockchain_info().await?;
    println!("Blockchain info: {:?}", result);

    manager.stop().await?;

    Ok(())
}
