use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to Risc0 guest (ELF file).
    #[arg(short, long)]
    pub guest: String,

    /// Path to Risc0 receipt.
    #[arg(short, long)]
    pub receipt: String,
}
