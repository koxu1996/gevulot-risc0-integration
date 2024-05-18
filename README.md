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

Prepare input data.

cargo run --release -p example-workload-input
