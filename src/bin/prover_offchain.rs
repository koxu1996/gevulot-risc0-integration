use risc0_zkvm::{default_prover, ExecutorEnv};

use clap::Parser;

use hex::decode as hex_decode;
use serde_json::from_str;
use std::fs::{self, File};
use std::io::{self, Read};

fn get_file_as_byte_vec(filename: &str) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}

fn read_json_file(file_path: &str) -> io::Result<Vec<String>> {
    let data = fs::read_to_string(file_path).expect("Unable to read file");
    let hex_strings: Vec<String> = from_str(&data)?;
    Ok(hex_strings)
}

fn decode_hex_strings(hex_strings: Vec<String>) -> Result<Vec<Vec<u8>>, hex::FromHexError> {
    hex_strings.into_iter().map(hex_decode).collect()
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
    // /// Destination path for saving proof.
    // #[arg(short, long)]
    // output: String,
}

fn main() {
    println!("Hello!");

    let args = Args::parse();

    let guest_elf = get_file_as_byte_vec(&args.guest);

    // Encoding input.
    // println!(
    //     "{}",
    //     hex::encode(bincode::serialize(&(1787569 as u32)).unwrap())
    // );
    // println!(
    //     "{}",
    //     hex::encode(bincode::serialize(&(1337 as u32)).unwrap())
    // );

    // Build env from input args.
    let mut env_builder = ExecutorEnv::builder();
    match read_json_file(&args.input) {
        Ok(hex_strings) => match decode_hex_strings(hex_strings) {
            Ok(decoded_bytes_arrays) => {
                for (_index, bytes) in decoded_bytes_arrays.iter().enumerate() {
                    env_builder.write_slice(bytes);
                }
            }
            Err(e) => panic!("Failed to decode hex: {}", e),
        },
        Err(e) => panic!("Failed to read file: {}", e),
    }
    let env = env_builder.build().unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove_elf(env, &guest_elf).unwrap();
    // println!("{:?}", receipt);
}
