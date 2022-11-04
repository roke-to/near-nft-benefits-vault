# near-nft-benefits-vault
[![Rust](https://github.com/roke-to/near-nft-benefits-vault/actions/workflows/rust.yml/badge.svg)](https://github.com/roke-to/near-nft-benefits-vault/actions/workflows/rust.yml)

This contract provides functionality to increase the value and utility of NFTs.
NFT Benefits Vault allows multiple users to own a private vault for Fungible Assets accessed via the corresponding NFT.

## Building
To build run:
```sh
./build.sh
```
Compiled binary will be put in the `dist` directory:
```
./dist/nft_benefits_vault.wasm
```

## Testing this contract
To test just run:
```sh
./test.sh
```
To do it manualy, build and place the contract WASM file to the `dist` dir. Then:
```
cargo test [OPTIONS] [TESTNAME] [-- [args]...]
```
