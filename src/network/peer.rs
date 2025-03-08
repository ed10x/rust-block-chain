use crate::core::block::Block;
use std::net::SocketAddr;

#[derive(Debug)]
pub struct Peer {
    pub address: SocketAddr,
    pub protocol_version: u32,
}

impl Peer {
    pub fn new(addr: SocketAddr) -> Self {
        Self {
            address: addr,
            protocol_version: 1,
        }
    }

    pub fn send_block(&self, block: &Block) {
        // 实现区块同步逻辑
    }
}