use super::*;
use std::collections::HashSet;
#[derive(Debug)]
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
        if let Some((coinbase, transactions)) = block.transactions.split_first() {
            if !coinbase.is_coinbase() {
                return Err(BlockValadiationErr::InvalidCoinbasetransction);
            }

            let mut block_spend: HashSet<Hash> = HashSet::new();
            let mut block_created: HashSet<Hash> = HashSet::new();

            let mut total_fee = 0;

            for transaction in transactions {
                let input_hashes = transaction.input_hashes();
                if !(&input_hashes - &self.unspeand_outputs).is_empty()
                    || !(&input_hashes & &block_spend).is_empty()
                {
                    return Err(BlockValadiationErr::invalidInput);
                }
                let input_value = transaction.input_value();
                let output_value = transaction.output_value();

                if output_value > input_value {
                    return Err(BlockValadiationErr::InsufficientInputValue);
                }
                let fee = input_value - output_value;

                total_fee += fee;

                block_spend.extend(input_hashes);
                block_created.extend(transaction.output_hashes())
            }
            if coinbase.output_value() < total_fee {
                return Err(BlockValadiationErr::InvalidCoinbasetransction);
            } else {
                block_created.extend(coinbase.output_hashes());
            }

            self.unspeand_outputs
                .retain(|output| !block_spend.contains(output));
            self.unspeand_outputs.extend(block_created);
        }
        self.blocks.push(block);

        Ok(())
    }
}
