## Compile prover program

```sh
rustup target add x86_64-unknown-linux-gnu
cargo build --release --target x86_64-unknown-linux-gnu
```

## Pack into unikernel

```sh
ops build ./target/x86_64-unknown-linux-gnu/release/gevulot-test -c prover.json
```
