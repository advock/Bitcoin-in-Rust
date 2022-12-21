use super::*;
use std::fmt::{self, Debug, Formatter};

pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub hash: BlockHash,
    pub previous_block_hash: BlockHash,
    pub nonce: u64,
    pub playload: String,
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Block[{}]: {} at: {}  with:  {}",
            &self.index,
            &hex::encode(&self.hash),
            &self.timestamp,
            &self.playload,
        )
    }
}

impl Block {
    pub fn new(
        index: u32,
        timestamp: u128,
        previous_block_hash: BlockHash,
        nonce: u64,
        playload: String,
    ) -> Self {
        Block {
            index,
            timestamp,
            hash: vec![0; 32],
            previous_block_hash,
            nonce,
            playload,
        }
    }
}

impl Hashable for Block {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(&u32_bytes(&self.index));
        bytes.extend(&u128_bytes(&self.timestamp));
        bytes.extend(&self.previous_block_hash);
        bytes.extend(&u64_bytes(&self.nonce));
        bytes.extend(self.playload.as_bytes());

        bytes
    }
}
