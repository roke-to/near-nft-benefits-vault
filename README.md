near-nft-benefits-vault
=======================
[![Rust](https://github.com/roke-to/near-nft-benefits-vault/actions/workflows/rust.yml/badge.svg)](https://github.com/roke-to/near-nft-benefits-vault/actions/workflows/rust.yml)

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

To turn on logger start with env variable `RUST_LOG`:
```sh
RUST_LOG='nft_benefits_vault=debug' cargo test
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
near call $CONTRACT_ID new '' --accountId $CONTRACT_ID
```

## Standard deploy
You should create an account on NEAR.
For convenience you can save account Id for the contract to the env variable.
In the below command, replace MY_ACCOUNT_NAME with the contract account name.
```sh
export ID=MY_CONTRACT_ID`
```
Check env variable with:
```sh
echo $ID
```

Deploy:
```sh
near deploy --wasmFile dist/nft_benefits_vault.wasm --accountId $ID
```

Initialize:
```sh
near call $CONTRACT_ID new '' --accountId $CONTRACT_ID
```
Congrats! NFT Benefits Vault is ready to use.

## Contract methods
withdraw_all
============
### Declaration
```rust
pub fn withdraw_all(&mut self, nft_contract_id: AccountId, nft_id: TokenId) -> Promise {..}
```
### Description
Public call to withdraw all FTs of every type from the Vault.
The contract will check caller ownership of the NFT specified by the NFT contract Id and NFT Id.
Then it will try to find the corresponding vault with access via provided contract/id pair.
### Arguments
`nft_contract_id` - account id of the NFT contract.

`nft_id` - string with unique token Id.
### Deposit
Exactly 1 yoctoNEAR must be attached.
### Gas consumption
@TODO


withdraw
============
### Declaration
```rust
pub fn withdraw(
    &mut self,
    nft_contract_id: AccountId,
    nft_id: TokenId,
    fungible_token: AccountId,
) {..}
```
### Description
Public call to withdraw all tokens of a __single__ type of FT from the Vault.
The contract will check ownership of the NFT spectified by the arguments.
Than it will try to find the vault with access via provided contract/id pair.
And it makes `ft_transfer` with all available tokens on the provided `fungible_token`.
### Arguments
`nft_contract_id` - account id of the NFT contract.

`nft_id` - string with unique token Id.

`fungible_token` - contract account id of the FT caller wants to withdraw.
### Deposit
Exactly 1 yoctoNEAR must be attached.
### Gas consumption
@TODO


withdraw_amount
============
### Declaration
```rust
pub fn withdraw_amount(
    &mut self,
    nft_contract_id: AccountId,
    nft_id: TokenId,
    fungible_token: AccountId,
    amount: U128,
) {..}
```
### Description
Public call to withdraw specified `amount` of tokens of a single type of FT from the Vault.
The contract will check ownership of the NFT spectified by the arguments.
Than it will try to find the vault with access via provided contract/id pair.
And it makes `ft_transfer` with specified amount of tokens on the provided `fungible_token`.
### Arguments
`nft_contract_id` - account id of the NFT contract.

`nft_id` - string with unique token Id.

`fungible_token` - contract account id of the FT caller wants to withdraw.

`amount` - amount of FTs to be withdrawn, must be provided as string, eg: `"1000000000"`.
### Deposit
Exactly 1 yoctoNEAR must be attached.
### Gas consumption
@TODO


add_replenishment_callback
============
### Declaration
```rust
pub fn add_replenishment_callback(
    &mut self,
    nft_contract_id: AccountId,
    nft_id: TokenId,
    callback: String,
    args: String,
) {..}
```
### Description
This method is used to add replenishers for the Vault.
When NFT owner calls withdraw methods the Contract will transfer available tokens from its own balance and will request available benefits from registered replenishers.
This method MUST be called and signed by replenisher himself because future replenishment requests will be made to the signer account.
The Vault will make call `callback` with `args` on the `env::predecessor_account_id()`.
### Arguments
`nft_contract_id` - account id of the NFT contract.

`nft_id` - string with unique token Id.

`callback` - function name of the external contract.

`args` - serialized in json arguments will be passed to the `callback`.
### Deposit
Exactly 1 yoctoNEAR must be attached.
### Gas consumption
@TODO

## View methods
Structs returned by view methods
================================
Complete list of tokens in the Vault.
```rust
pub struct BalanceView {
    /// Unique identifier of the NFT.
    pub nft_id: NftId,
    /// List of FTs.
    pub tokens: Vec<Token>,
}
```
Unique identifier of the NFT to be used as the Key to the Vault.
Only [`TokenId`] is insufficient because there can be multiple NFT contracts containing the same [`TokenId`].
```rust
pub struct NftId {
    contract_id: AccountId,
    token_id: TokenId,
}
```
Info about FT.
```rust
pub struct Token {
    /// Account id of the FT contract.
    pub contract_id: AccountId,
    /// Amount of this tokens in the Vault.
    pub amount: u128,
}
```
Info about vault.
```rust
pub struct VaultView {
    /// Key NFT with access to the Vault.
    pub nft_id: NftId,
    /// The number of FT types in the Vault.
    pub assets_count: u64,
}
```

balance_of
============
### Declaration
```rust
pub fn balance_of(&self, nft_contract_id: AccountId, nft_id: TokenId) -> Option<BalanceView> {..}
```
### Description
Function to view balances of all assets in the vault.
### Arguments
`nft_contract_id` - account id of the NFT contract.

`nft_id` - string with unique token Id.

vault_assets_count
============
### Declaration
```rust
pub fn vault_assets_count(
    &self,
    nft_contract_id: AccountId,
    nft_id: TokenId,
) -> Option<VaultView> {..}
```
### Description
Function to view the amount of assets in the Vault.
### Arguments
`nft_contract_id` - account id of the NFT contract.

`nft_id` - string with unique token Id.

replenishers
============
### Declaration
```rust
pub fn replenishers(
    &self,
    nft_contract_id: AccountId,
    nft_id: TokenId,
) -> Option<Vec<Replenisher>> {..}
```
### Description
Function to view the replenishers of the vault.
### Arguments
`nft_contract_id` - account id of the NFT contract.

`nft_id` - string with unique token Id.