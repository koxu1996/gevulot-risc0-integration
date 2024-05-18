# [ZKHack] Gevulot - Risc0 integration

This is full integration of Risc0 zkVM with Gevulot project.

## Example usage

### Setup

#### 1. Build Risc0 guest

```sh
$ cd ./example-workload-guest
$ cargo build --release
$ cp ./target/riscv-guest/riscv32im-risc0-zkvm-elf/release/square_check_guest /tmp/workload-guest.bin
$ cd ..
```

#### 2. Prepare input data

```sh
$ cargo run --release -p example-workload-input > /tmp/workload-input.json
```

### Running offchain

#### 1. Run local prover

This will make sure you have correct input, that can be proved.

```sh
$ cargo run -p prover --bin prover -- --guest /tmp/workload-guest.bin --input /tmp/workload-input.json --output /tmp/workload-receipt.bin
```

**NOTE:** This command might take a while, because it's generating proof locally on your machine.

#### 2. Run local verifier

```sh
$ cargo run -p verifier --bin verifier -- --guest /tmp/workload-guest.bin --receipt /tmp/workload-receipt.bin
```

It should print:

```
Valid: true
```

### Running on-chain with Gevulot!

TODO
