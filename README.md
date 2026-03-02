This repository demonstrates a quick fix for secp256k1 on SP1 v6.

On Ubuntu / Debian / Mint:

Install LLVM 21 following notes in https://apt.llvm.org/, then:

```bash
$ git clone https://github.com/wakabat/secp256k1-sp1-v6-sample
$ cd secp256k1-sp1-v6-sample/script
$ TARGET_CC=clang-21 \
  TARGET_CFLAGS="--target=riscv64 -march=rv64im" \
  cargo run --release
```

On macOS:

```bash
$ brew install llvm@21
$ git clone https://github.com/wakabat/secp256k1-sp1-v6-sample
$ cd secp256k1-sp1-v6-sample/script
$ TARGET_CC=/opt/homebrew/opt/llvm@21/bin/clang \
  TARGET_CFLAGS="--target=riscv64 -march=rv64im" \
  cargo run --release
```
