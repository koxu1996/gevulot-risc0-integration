use bincode::Error as BincodeError;
use clap::Parser;
use hex::decode as hex_decode;
use risc0_zkvm::{default_prover, ExecutorEnv};
use serde_json::from_str;
use std::fs::{self, File};
use std::io::{self, Read};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Hex decoding error: {0}")]
    Hex(#[from] hex::FromHexError),

    #[error("Bincode error: {0}")]
    Bincode(#[from] BincodeError),
}

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

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to Risc0 guest (ELF file).
    #[arg(short, long)]
    guest: String,

    /// Path to JSON with guest args.
    #[arg(short, long)]
    input: String,

    /// Destination path for saving proof.
    #[arg(short, long)]
    output: String,
}

fn main() -> anyhow::Result<()> {
    println!("Hello!");

    let args = Args::parse();

    let guest_elf = read_file_to_bytes(&args.guest)?;

    // Build env from input args.
    let mut env_builder = ExecutorEnv::builder();
    let hex_strings = read_json_file(&args.input)?;
    let decoded_bytes_arrays = decode_hex_strings(hex_strings)?;

    for bytes in decoded_bytes_arrays {
        env_builder.write_slice(&bytes);
    }

    let env = env_builder.build()?;

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove_elf(env, &guest_elf)?;

    let receipt_bytes = bincode::serialize(&receipt)?;
    fs::write(&args.output, receipt_bytes)?;

    Ok(())
}
