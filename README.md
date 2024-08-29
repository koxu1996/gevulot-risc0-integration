# GevuRisc - Risc0 prover/verifier integration for Gevulot

This is full integration of Risc0 zkVM with Gevulot project. Done during [ZKHack Kraków](https://zkhack.dev/).

## TL;DR

Prover / Verifier deployment:

```
Prover hash:4a9549ce7b6f918c453771a18e80e6483896489245be2557d8f93b4a9f6076c3
Verifier hash:900580cbf68246692ef750083e78f743f0ce76c650b8f9fe610b7cf3a6c2bb97.
Tx Hash:eac0b10e873a1564ae625d7c755da934a4df9c6e49efe5ea71ac8705b6324dbf
```

Example transaction:

![](./tx.gif)

**NOTE:** Above screencast is slightly outdated, please refer to the output below:

```
Programs send to execution correctly. Tx hash:67aa482daeac5516dbd010b87720f255060ebc678f83041c8cd3caeb056c2144
```

---

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
The hash of the file is: b5f0398812f7239c00f7a634d34e23d67c71105151b6f1fe8ba9c37f724de5c5
$ gevulot-cli calculate-hash --file ~/.ops/images/verifier_gevulot
The hash of the file is: f858800c37e9a044fd707e708bcd1bbda6d0b7f8ba18e70d2082286a6dfce139
```

Next step is to upload files, so they are accessible via plain HTTP. I just used manually created bucket in Google Cloud Storage.

### Deploy 🔥

```
$ gevulot-cli --jsonurl "http://api.devnet.gevulot.com:9944" --keyfile /tmp/localkey.pki \
deploy \
    --name "Risc0" \
    --prover b5f0398812f7239c00f7a634d34e23d67c71105151b6f1fe8ba9c37f724de5c5 \
    --provername 'risc0_prover_004' \
    --proverimgurl 'https://storage.googleapis.com/gevulot/risc0/prover_gevulot' \
    --verifier f858800c37e9a044fd707e708bcd1bbda6d0b7f8ba18e70d2082286a6dfce139 \
    --verifiername 'risc0_verifier_004' \
    --verifierimgurl 'https://storage.googleapis.com/gevulot/risc0/verifier_gevulot'
Start prover / verifier deployment
Prover / Verifier deployed correctly.
Prover hash:4a9549ce7b6f918c453771a18e80e6483896489245be2557d8f93b4a9f6076c3
Verifier hash:900580cbf68246692ef750083e78f743f0ce76c650b8f9fe610b7cf3a6c2bb97.
Tx Hash:eac0b10e873a1564ae625d7c755da934a4df9c6e49efe5ea71ac8705b6324dbf
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

```sh
$ gevulot-cli --jsonurl "http://api.devnet.gevulot.com:9944" --keyfile /tmp/localkey.pki \
exec --tasks '[{"program":"4a9549ce7b6f918c453771a18e80e6483896489245be2557d8f93b4a9f6076c3","cmd_args":[{"name":"--guest","value":"/workspace/workload-guest.bin"},{"name":"--input","value":"/workspace/workload-input.json"},{"name":"--output","value":"/workspace/workload-receipt.bin"}],"inputs":[{"Input":{"local_path":"6b87037b1197ac732939c589687b3f04eb972440c3dccf9fc8e2f807902174c7","vm_path":"/workspace/workload-guest.bin","file_url":"https://storage.googleapis.com/gevulot/risc0/example-workload/workload-guest.bin"}},{"Input":{"local_path":"e51bf918d5d85b49283a096ccb25afb0d2089fec2701b5d9f79437b58cd39660","vm_path":"/workspace/workload-input.json","file_url":"https://storage.googleapis.com/gevulot/risc0/example-workload/workload-input.json"}}]},{"program":"900580cbf68246692ef750083e78f743f0ce76c650b8f9fe610b7cf3a6c2bb97","cmd_args":[{"name":"--guest","value":"/workspace/workload-guest2.bin"},{"name":"--receipt","value":"/workspace/workload-receipt.bin"}],"inputs":[{"Input":{"local_path":"6b87037b1197ac732939c589687b3f04eb972440c3dccf9fc8e2f807902174c7","vm_path":"/workspace/workload-guest2.bin","file_url":"https://storage.googleapis.com/gevulot/risc0/example-workload/workload-guest.bin"}},{"Output":{"source_program":"4a9549ce7b6f918c453771a18e80e6483896489245be2557d8f93b4a9f6076c3","file_name":"/workspace/workload-receipt.bin"}}]}]'
```

**NOTE:** Pretty printed content of `--tasks` is available in `example-workload.json` in this repository.

Transaction hash:

- `67aa482daeac5516dbd010b87720f255060ebc678f83041c8cd3caeb056c2144`

That's all!
