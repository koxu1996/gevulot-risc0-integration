use clap::Parser;
use gevulot_test::proof_logic;

fn main() -> anyhow::Result<()> {
    let args = gevulot_test::cli::Args::parse();

    proof_logic(&args.guest, &args.input, &args.output).unwrap();

    Ok(())
}
