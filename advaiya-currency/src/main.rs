use std::{hash::Hash, ptr::hash};
mod hashable;

use blockchainlib::*;
fn main() {
    let mut block = Block::new(
        0,
        0,
        vec![0; 32],
        0,
        "Genesis block".to_owned(),
        0x000fffffffffffffffffffffffffffff,
    );

    block.hash = block.hash();
    print!("{:?}", &block);

    block.mine();

    println!("{:?}", &block);
}
