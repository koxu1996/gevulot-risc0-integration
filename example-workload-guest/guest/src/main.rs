#![no_main]
#![no_std]

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    // Read inputs.
    let public_input_square: u32 = env::read();
    let private_input_root: u32 = env::read();

    // Main logic: check user provided valid square root.
    let computed_square = private_input_root * private_input_root;
    assert_eq!(computed_square, public_input_square);

    // Write public output to journal.
    env::commit(&public_input_square);
}
