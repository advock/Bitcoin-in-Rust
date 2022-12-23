use super::*;
use std::collections::HashSet;
pub enum BlockValadiationErr {
    MismatchedIndex,
    InvalidHash,
    AchronologicalTimestamp,
    MismatchPreviousIndex,
    InvalidGensisBlock,
    invalidInput,
    InsufficientInputValue,
    InvalidCoinbasetransction,
}
pub struct Blockchain {
    pub blocks: Vec<Block>,
    unspeand_outputs: HashSet<Hash>,
}

impl Blockchain {
    pub fn update_with_block(&mut self, block: Block) -> Result<(), BlockValadiationErr> {
        let i = self.blocks.len();

        if block.index != i as u32 {
            return Err(BlockValadiationErr::MismatchedIndex);
        } else if !block::check_difficulty(&block.hash(), block.difficulty) {
            return Err(BlockValadiationErr::InvalidHash);
        } else if i != 0 {
            let pre_block = &self.blocks[i - 1];
            if block.timestamp <= pre_block.timestamp {
                return Err(BlockValadiationErr::AchronologicalTimestamp);
            } else if block.previous_block_hash != pre_block.hash {
                return Err(BlockValadiationErr::MismatchPreviousIndex);
            }
        } else {
            if block.previous_block_hash != vec![0; 32] {
                return Err(BlockValadiationErr::InvalidGensisBlock);
            }
        }
        if let Some((coinbase, transaction)) = block.transactions.split_first() {
            if !coinbase.is_coinbase() {
                return Err(BlockValadiationErr::InvalidCoinbasetransction);
            }
            let mut block_spend: HashSet<Hash> = HashSet::new();
        }
        Ok(())
    }
}
