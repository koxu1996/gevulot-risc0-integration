use error::AppError;
use risc0_zkvm::{default_prover, ExecutorEnv};

use hex::decode as hex_decode;
use serde_json::from_str;
use std::fs::{self};

pub mod cli;

pub mod error;

/// Reads the specified file into a byte vector.
fn read_file_to_bytes(filename: &str) -> Result<Vec<u8>, AppError> {
    let data = fs::read(filename)?;
    Ok(data)
}

/// Reads a JSON file and returns a vector of strings.
fn read_json_file(file_path: &str) -> Result<Vec<String>, AppError> {
    let data = fs::read_to_string(file_path)?;
    let hex_strings: Vec<String> = from_str(&data)?;
    Ok(hex_strings)
}

/// Decodes a vector of hex strings into bytes.
fn decode_hex_strings(hex_strings: Vec<String>) -> Result<Vec<Vec<u8>>, AppError> {
    hex_strings
        .into_iter()
        .map(hex_decode)
        .map(|result| result.map_err(AppError::Hex))
        .collect()
}

pub fn proof_logic(guest: &str, input: &str, output: &str) -> anyhow::Result<()> {
    let guest_elf = read_file_to_bytes(guest)?;

    // Build env from input args.
    let mut env_builder = ExecutorEnv::builder();
    let hex_strings = read_json_file(&input)?;
    let decoded_bytes_arrays = decode_hex_strings(hex_strings)?;

    for bytes in decoded_bytes_arrays {
        env_builder.write_slice(&bytes);
    }

    let env = env_builder.build()?;

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove(env, &guest_elf)?;

    let receipt_bytes = bincode::serialize(&receipt)?;
    fs::write(output, receipt_bytes)?;

    println!("OK!");

    Ok(())
}
