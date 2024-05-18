use std::time::SystemTime;
use std::{error::Error, result::Result};

use clap::Parser;
use gevulot_shim::{Task, TaskResult};
use serde::Serialize;
use serde_json::json;
use verifier::{cli, verify_logic};

fn main() -> Result<(), Box<dyn Error>> {
    gevulot_shim::run(run_task)
}

#[derive(Serialize)]
struct TaskOutputData {
    is_valid: bool,
    timestamp: u64,
}

fn run_task(task: Task) -> Result<TaskResult, Box<dyn Error>> {
    let args = cli::Args::parse_from(&task.args);

    let is_valid = verify_logic(&args.guest, &args.receipt);

    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    let result = TaskOutputData {
        is_valid,
        timestamp,
    };

    let jresult = json!(result).to_string();
    let data = jresult.as_bytes().to_vec();

    task.result(data, vec![])
}
