use std::sync::Arc;

use crate::transport::core::TransportTrait;
use crate::transport::DefaultTransport;
#[derive(Debug)]
pub struct BitcoinTestClient {
    _transport: Arc<DefaultTransport>,
}
impl BitcoinTestClient {
    pub fn new(transport: Arc<DefaultTransport>) -> Self { Self { _transport: transport } }
    pub fn endpoint(&self) -> &str { self._transport.url() }
}
