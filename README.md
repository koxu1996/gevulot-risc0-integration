# [ZKHack] Gevulot - Risc0 integration

This is full integration of Risc0 zkVM with Gevulot project.

## Deploying prover/verifier

### Requirements

- You have to be on `x86_64-linux` machine.
- `gevulot-cli`, Ops and Rust installed.
- You have Gevulot API key.

### Building images

```sh
$ cargo build --release -p prover
$ cp ./target/release/prover_gevulot ./prover_gevulot
$ cargo build --release -p verifier
$ cp ./target/release/verifier_gevulot ./verifier_gevulot
```

### Preparing unikernel images

```sh
$ ops build ./prover_gevulot -c manifest_prover.json
$ ops build ./verifier_gevulot -c manifest_verifier.json
```

Ops will print the resulting program image file location:

```
Bootable image file:/home/johndoe/.ops/images/prover_gevulot
Bootable image file:/home/johndoe/.ops/images/verifier_gevulot
```

### Compute hash and make images available via plain HTTP

Image hash will be used later:

```sh
$ gevulot-cli calculate-hash --file ~/.ops/images/prover_gevulot
The hash of the file is: 0ad5637cd5c26b0d3f1f90a74a7f6c87c76e99e541e32e8d4df39981c5694141
$ gevulot-cli calculate-hash --file ~/.ops/images/verifier_gevulot
The hash of the file is: 508eea7b1d5f7776cde3b2e14f383baa0f8bdf13eedf67d52ba3a2ad076e3da4
```

Next step is to upload files, so they are accessible via plain HTTP. I just used manually created bucket in Google Cloud Storage.

### Deploy 🔥

```
$ gevulot-cli --jsonurl "http://api.devnet.gevulot.com:9944" --keyfile /tmp/localkey.pki \
deploy \
    --name "Risc0 prover & verifier" \
    --prover 0ad5637cd5c26b0d3f1f90a74a7f6c87c76e99e541e32e8d4df39981c5694141 \
    --provername '#risc0prover' \
    --proverimgurl 'https://storage.googleapis.com/...hidden.../prover_gevulot' \
    --verifier 508eea7b1d5f7776cde3b2e14f383baa0f8bdf13eedf67d52ba3a2ad076e3da4 \
    --verifiername '#risc0verifier' \
    --verifierimgurl 'https://storage.googleapis.com/...hidden.../verifier_gevulot'
Start prover / verifier deployment
Prover / Verifier deployed correctly.
Prover hash:c969b70e087dd2b12a414ea4c86d43f528d03544d3c01b4ec59003039005b1b3
Verifier hash:87fc88961dfa36bf60bafedd25f02a09e33a0adf8435825f7a952aa677f82b03.
Tx Hash:72c92afa6113e737ea4d4654ebbfc39826ba2a8a959e5ed8ad4916bad53fdaf9
```

Time for testing!

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
