use super::*;
use std::fmt::{self, Debug, Formatter};

pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub hash: BlockHash,
    pub previous_block_hash: BlockHash,
    pub nonce: u64,
    pub playload: String,
    pub difficulty: u128,
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Block[{}]: {} at: {}  with:  {} and nonce {}",
            &self.index,
            &hex::encode(&self.hash),
            &self.timestamp,
            &self.playload,
            &self.nonce
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
        difficulty: u128,
    ) -> Self {
        Block {
            index,
            timestamp,
            hash: vec![0; 32],
            previous_block_hash,
            nonce,
            playload,
            difficulty,
        }
    }
    pub fn mine(&mut self) {
        for nonce_attempt in 0..(u64::max_value()) {
            self.nonce = nonce_attempt;
            let hash = self.hash();
            if check_difficulty(&hash, self.difficulty) {
                self.hash = hash;
            }
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
        bytes.extend(&u128_bytes(&self.difficulty));

        bytes
    }
}

pub fn check_difficulty(hash: &BlockHash, difficulty: u128) -> bool {
    difficulty > difficulty_bytes_as_u128(&hash)
}
