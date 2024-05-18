use error::AppError;
use risc0_zkvm::{compute_image_id, Receipt};

use std::fs;

pub mod cli;

pub mod error;

/// Reads the specified file into a byte vector.
fn read_file_to_bytes(filename: &str) -> Result<Vec<u8>, AppError> {
    let data = fs::read(filename)?;
    Ok(data)
}

pub fn verify_logic(guest: &str, receipt: &str) -> bool {
    // Compute guest image ID.
    // NOTE: This could be moved to client-side.
    let guest_elf = read_file_to_bytes(guest).unwrap();
    let image_id = compute_image_id(&guest_elf).unwrap();

    // Read receipt.
    let receipt_bytes = read_file_to_bytes(receipt).unwrap();
    let receipt: Receipt = bincode::deserialize(&receipt_bytes).unwrap();

    // Perform validation.
    match receipt.verify(image_id) {
        Ok(()) => true,
        Err(_e) => false,
    }
}
