use clap::Parser;
use verifier::{cli, verify_logic};

fn main() -> anyhow::Result<()> {
    let args = cli::Args::parse();

    let is_valid = verify_logic(&args.guest, &args.receipt);

    println!("Valid: {}", is_valid);

    Ok(())
}
