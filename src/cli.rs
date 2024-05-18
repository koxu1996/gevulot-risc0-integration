use clap::Parser;

use std::fs::{self};
use std::io::{self};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to Risc0 guest (ELF file).
    #[arg(short, long)]
    pub guest: String,

    /// Path to JSON with guest args.
    #[arg(short, long)]
    pub input: String,

    /// Destination path for saving proof.
    #[arg(short, long)]
    pub output: String,
}
