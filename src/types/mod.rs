pub mod responses;
pub use bitcoin::PublicKey;
pub use responses::*;
#[derive(Debug, serde::Serialize)]
pub enum HashOrHeight {
    Hash(bitcoin::BlockHash),
    Height(u32),
}
pub type FeeRate = bitcoin_units::FeeRate;
