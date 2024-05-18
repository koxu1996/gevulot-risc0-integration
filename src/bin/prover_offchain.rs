use risc0_zkvm::{default_prover, ExecutorEnv};

use clap::Parser;

use std::fs::{self, File};
use std::io::Read;

fn get_file_as_byte_vec(filename: &str) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to Risc0 guest (ELF file).
    #[arg(short, long)]
    guest_path: String,
    // /// Path to JSON with guest args.
    // #[arg(short, long,)]
    // guest_input: String,

    // /// Destination path for saving proof.
    // #[arg(short, long)]
    // output: String,
}

fn main() {
    println!("Hello!");

    let args = Args::parse();

    let guest_elf = get_file_as_byte_vec(&args.guest_path);

    let env = ExecutorEnv::builder()
        .write(&(16 as u32))
        .unwrap() // env.write_slice(arg1.serialize())
        .write(&(4 as u32))
        .unwrap()
        // env.write_slice(arg2.serialize())
        // env.write_slice(arg3.serialize())
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove_elf(env, &guest_elf).unwrap();
    // println!("{:?}", receipt);
}
