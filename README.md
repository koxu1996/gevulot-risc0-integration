# [ZKHack Kraków] Gevulot - Risc0 integration ✅

This is full integration of Risc0 zkVM with Gevulot project.

## TL;DR

Prover / Verifier deployment:

```
Prover hash:34948922b107eaa8672b3315c830638fc98ade760780dd2f5d9406a14ef6c10a
Verifier hash:891eece1d904f4948d6b84aa062d7462dd521f6361ed2b31ce703338ba31844c.
Tx Hash:50a1f17c966020b4f85cab002515f367473127372f8e777be4c4e09487d09cc6
```

Example transaction:

![](./tx.gif)

```
Programs send to execution correctly. Tx hash:3c90fd79ea28803f2d826d23b7c332c1c7be50c95e73fa85472a706ac979784a
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
The hash of the file is: f37b73d5b9c1108a4ee5276a880cdba0cd97c71997cd37884c4c7ab87f340517
$ gevulot-cli calculate-hash --file ~/.ops/images/verifier_gevulot
The hash of the file is: a44146c6110c61207062dfa87507d3b148e10d25e1be22182dc6f7962e3d8993
```

Next step is to upload files, so they are accessible via plain HTTP. I just used manually created bucket in Google Cloud Storage.

### Deploy 🔥

```
$ gevulot-cli --jsonurl "http://api.devnet.gevulot.com:9944" --keyfile /tmp/localkey.pki \
deploy \
    --name "Risc0 prover & verifier 3" \
    --prover f37b73d5b9c1108a4ee5276a880cdba0cd97c71997cd37884c4c7ab87f340517 \
    --provername 'risc0prover3' \
    --proverimgurl 'https://storage.googleapis.com/gevulot-test/new2/prover_gevulot' \
    --verifier a44146c6110c61207062dfa87507d3b148e10d25e1be22182dc6f7962e3d8993 \
    --verifiername 'risc0verifier3' \
    --verifierimgurl 'https://storage.googleapis.com/gevulot-test/new2/verifier_gevulot'
Start prover / verifier deployment
Prover / Verifier deployed correctly.
Prover hash:34948922b107eaa8672b3315c830638fc98ade760780dd2f5d9406a14ef6c10a
Verifier hash:891eece1d904f4948d6b84aa062d7462dd521f6361ed2b31ce703338ba31844c.
Tx Hash:50a1f17c966020b4f85cab002515f367473127372f8e777be4c4e09487d09cc6
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
exec --tasks '[{"program":"34948922b107eaa8672b3315c830638fc98ade760780dd2f5d9406a14ef6c10a","cmd_args":[{"name":"--guest","value":"/workspace/workload-guest.bin"},{"name":"--input","value":"/workspace/workload-input.json"},{"name":"--output","value":"/workspace/workload-receipt.bin"}],"inputs":[{"Input":{"local_path":"1e7d80754b7f9f8cf0bc5b423feb03baacd4e2a533333581f0ab713a75e52afb","vm_path":"/workspace/workload-guest.bin","file_url":"https://storage.googleapis.com/gevulot-test/workload-guest.bin"}},{"Input":{"local_path":"e51bf918d5d85b49283a096ccb25afb0d2089fec2701b5d9f79437b58cd39660","vm_path":"/workspace/workload-input.json","file_url":"https://storage.googleapis.com/gevulot-test/workload-input.json"}}]},{"program":"891eece1d904f4948d6b84aa062d7462dd521f6361ed2b31ce703338ba31844c","cmd_args":[{"name":"--guest","value":"/workspace/workload-guest2.bin"},{"name":"--receipt","value":"/workspace/workload-receipt.bin"}],"inputs":[{"Input":{"local_path":"1e7d80754b7f9f8cf0bc5b423feb03baacd4e2a533333581f0ab713a75e52afb","vm_path":"/workspace/workload-guest2.bin","file_url":"https://storage.googleapis.com/gevulot-test/workload-guest.bin"}},{"Output":{"source_program":"34948922b107eaa8672b3315c830638fc98ade760780dd2f5d9406a14ef6c10a","file_name":"/workspace/workload-receipt.bin"}}]}]'
```

**NOTE:** Pretty printed content of `--tasks` is available in `example-workload.json` in this repository.

Transaction hash:

- `3c90fd79ea28803f2d826d23b7c332c1c7be50c95e73fa85472a706ac979784a`

That's all!
