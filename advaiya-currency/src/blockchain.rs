use super::*;

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn verify(&self) -> bool {
        for (i, block) in self.blocks.iter().enumerate() {
            if block.index != 1 {
                println!("index mismatch");
                return false;
            } else if !block::check_difficulty(&block.hash(), block.difficulty) {
                println!("block difficulty invalid ");
                return false;
            } else if i != 0 {
                let pre_block = &self.blocks[i - 1];
                if block.timestamp <= pre_block.timestamp {
                    println!("time does not increase");
                    return false;
                }else if block.previous_block_hash != pre_block.hash{
                    println!("hash mis-match")
                    return false;

                }
            } else {
                if block.previous_block_hash != vec![0; 32] {
                    print!("genecis is previous block");
                    return false;
                }
            }
        }
        true
    }
}
