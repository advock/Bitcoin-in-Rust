use std::{hash::Hash, ptr::hash};
mod hashable;

use blockchainlib::*;
fn main() {
    let mut block = Block::new(
        0,
        now(),
        vec![0; 32],
        0,
        "another block".to_owned(),
        0x000fffffffffffffffffffffffffffff,
    );
    block.mine();

    let mut blockchain = Blockchain {
        blocks: vec![block],
    };
    let mut last_hashh = block.hash.clone();

    for i in 1..=10 {
        let mut block = Block::new(
            i,
            now(),
            last_hashh,
            0,
            "another block".to_owned(),
            0x000fffffffffffffffffffffffffffff,
        );
        block.mine();

        println!("{:?}", &block);
        last_hashh = block.hash.clone();

        let mut blockchain = Blockchain {
            blocks: vec![block],
        };
    }
}
