# Upgrade program

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/upgrade-program)](https://crates.io/crates/upgrade-program)

Distributed Lab Solana upgrade program can be useful for managing program upgrades by ECDSA sekp256k1 signature 
and also provides an example how to execute program upgrades by other programs.

Here is [an article](https://medium.com/@oleg.fomenko2002/solana-program-trustful-upgrade-e6733bff4581) that describes how to use or implement the similar contract.

## Build

```shell
npm run build:upgrade-program
```

## Deploy
```shell
solana program deploy --program-id ./dist/program/upgrade-keypair.json ./dist/program/upgrade.so
```
