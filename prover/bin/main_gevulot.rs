use std::{error::Error, result::Result};

use clap::Parser;
use gevulot_shim::{Task, TaskResult};
use prover::{cli, proof_logic};

fn main() -> Result<(), Box<dyn Error>> {
    gevulot_shim::run(run_task)
}

//Write proof into "/workspace/proof.dat"
//
fn run_task(task: Task) -> Result<TaskResult, Box<dyn Error>> {
    // Synchronize argument parsing
    let mut raw_args = vec!["dummy".to_string()];
    for a in task.args.clone() {
        raw_args.push(a);
    }

    let args = cli::Args::parse_from(&raw_args);

    proof_logic(&args.guest, &args.input, &args.output).unwrap();

    // Return TaskResult with reference to the generated proof file.
    task.result(vec![], vec![args.output])
}
