## Compile prover program

```sh
rustup target add x86_64-unknown-linux-gnu
cargo build --release --target x86_64-unknown-linux-gnu
```

## Pack into unikernel

```sh
cp ./target/x86_64-unknown-linux-gnu/release/gevulot-test ./gevulot-test
ops build ./gevulot-test -c prover.json
```

## Risc0 workload

```sh
cd ./risc0-hello-world
cargo run --release
```

---

## Workload - offchain demo

### Build Risc0 guest

```sh
$ cd ./example-workload-guest
$ cargo build --release
$ cp ./target/riscv-guest/riscv32im-risc0-zkvm-elf/release/square_check_guest /tmp/workload-guest.bin
$ cd ..
```

### Prepare input data

```sh
$ cargo run --release -p example-workload-input > /tmp/workload-input.json
```

### Run local prover

This will make sure you have correct input, that can be proved.

```sh
cargo run -p prover --bin prover -- --guest /tmp/workload-guest.bin --input /tmp/workload-input.json --output /tmp/workload-receipt.bin
```

**NOTE:** This command might take a while, because it's generating proof locally on your machine.

### Run local verifier

```sh
cargo run -p verifier --bin verifier -- --input /tmp/workload-receipt.json
```
