use std::{hash::Hash, ptr::hash};
mod hashable;

use blockchainlib::*;
fn main() {
    let mut genesis_block = Block::new(
        0,
        now(),
        vec![0; 32],
        vec![Transaction {
            inputs: vec![],
            outputs: vec![
                transaction::Output {
                    to_addr: "Alice".to_owned(),
                    value: 50,
                },
                transaction::Output {
                    to_addr: "Bob".to_owned(),
                    value: 7,
                },
            ],
        }],
        0x000fffffffffffffffffffffffffffff,
    );
    genesis_block.mine();
    let mut last_hashh = genesis_block.hash.clone();

    let mut blockchain = Blockchain::new();

    blockchain
        .update_with_block(genesis_block)
        .expect("failed to add genesis block");
}
