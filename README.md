# Distributed Lab Solana program library

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Library products:
- [![Crates.io](https://img.shields.io/crates/v/upgrade-program)](https://crates.io/crates/upgrade-program) [![Docs.rs](https://docs.rs/upgrade-program/badge.svg)](https://docs.rs/upgrade-program) [upgrade-program](./programs/upgrade-program) - used to upgrade contracts by ECDSA secp256k1 public key.
  

## How to build
```shell
cargo build-bpf --manifest-path=<CRATE Cargo.toml FILE> --bpf-out-dir=<OUTPUT DIR>
```
  
## How to publish
1. Change the version in crate `Cargo.toml` file.
   
2. Execute:
```shell
cargo publish -p <CRATE NAME>
```