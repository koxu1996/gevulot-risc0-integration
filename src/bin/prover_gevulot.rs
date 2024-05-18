use std::{error::Error, result::Result};

use gevulot_shim::{Task, TaskResult};
use gevulot_test::proof_logic;

fn main() -> Result<(), Box<dyn Error>> {
    gevulot_shim::run(run_task)
}

// The main function that executes the prover program.
fn run_task(task: Task) -> Result<TaskResult, Box<dyn Error>> {
    // Display program arguments we received. These could be used for
    // e.g. parsing CLI arguments with clap.
    println!("prover: task.args: {:?}", &task.args);

    // -----------------------------------------------------------------------
    // Here would be the control logic to run the prover with given arguments.
    // -----------------------------------------------------------------------
    proof_logic();

    // Write generated proof to a file.
    std::fs::write("/workspace/proof.dat", b"this is a proof.")?;

    // Return TaskResult with reference to the generated proof file.
    task.result(vec![], vec![String::from("/workspace/proof.dat")])
}