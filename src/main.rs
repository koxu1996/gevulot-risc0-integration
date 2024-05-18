use clap::Parser;
use gevulot_test::{cli, proof_logic};

fn main() -> anyhow::Result<()> {
    let args = cli::Args::parse();

    proof_logic(&args.guest, &args.input, &args.output).unwrap();

    Ok(())
}
