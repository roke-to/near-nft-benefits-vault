<<<<<<< HEAD
# near-nft-benefits-vault
[![Rust](https://github.com/roke-to/near-nft-benefits-vault/actions/workflows/rust.yml/badge.svg)](https://github.com/roke-to/near-nft-benefits-vault/actions/workflows/rust.yml)

=======
near-nft-benefits-vault
=======================
>>>>>>> b547e74 (add quick deploy)
This contract provides functionality to increase the value and utility of NFTs.
NFT Benefits Vault allows multiple users to own a private vault for Fungible Assets accessed via the corresponding NFT.

# Prerequisites
Make sure Rust is installed
Make sure `near-cli` is installed

# Building
To build run:
```sh
./build.sh
```
Compiled binary will be put in the `dist` directory:
```
./dist/nft_benefits_vault.wasm
```

# Testing this contract
To test just run:

```sh
./test.sh
```

To do it manualy, build and place the contract WASM file to the `dist` dir. Then:

```sh
cargo test [OPTIONS] [TESTNAME] [-- [args]...]
```

# Using this contract
## Quickest deploy
Build and deploy this contract to the dev account:

```sh
./build.sh
near dev-deploy --wasmFile dist/nft_benefits_vault.wasm
```

`near-cli` will create new dev account and deploy wasm code to it.
The next command will initialize contract using `new` method:
```sh
near call $CONTRACT_NAME new '' --accountId $CONTRACT_NAME
```

## Standard deploy

## Contract methods

## View methods
