/// Different tools for testing.
mod environment;

mod add_replenisher;
mod ft_on_transfer;
mod integration;
mod view_balance_of;
mod withdraw;
mod withdraw_all;
mod withdraw_all_callback;
mod withdraw_amount;
mod withdraw_callback;

// Precompiled smart contracts locations.
const WASMS_LOCATION: &str = "dist";
const WRAP_NEAR_WASM: &str = "wrap.wasm";
const NFT_WASM: &str = "non_fungible_token.wasm";
const FUNGIBLE_TOKEN_WASM: &str = "fungible_token.wasm";
const VAULT_TEST_REPLENISHER_WASM: &str = "vault_test_replenisher.wasm";

// NEAR is 10^24 yoctoNEAR.
const NEAR: u128 = 10u128.pow(24);

// Constants related to the wrap NEAR FT contract.
const WRAP_NEAR_TESTNET_ACCOUNT_ID: &str = "wrap.testnet";
const WRAP_NEAR_DEPOSIT_CALL: &str = "near_deposit";
const WRAP_NEAR_DEPOSIT: u128 = 10 * NEAR;

// Constants related to the NFT contract.
const NFT_NEW_DEFAULT_META_CALL: &str = "new_default_meta";
const NFT_MINT_CALL: &str = "nft_mint";
const NFT_MINT_STORAGE_DEPOSIT: u128 = 8_400_000_000_000_000_000_000;
const NFT_TOKEN_ID: &str = "awesome_test_pic_666";
const NFT_TRANSFER_CALL: &str = "nft_transfer";

// Constants related to the nft-benefits-vault contract.
const VAULT_BALANCE_OF_VIEW: &str = "balance_of";
const VAULT_WITHDRAW_ALL_CALL: &str = "withdraw_all";
const VAULT_WITHDRAW_CALL: &str = "withdraw";
const VAULT_WITHDRAW_AMOUNT_CALL: &str = "withdraw_amount";
const VAULT_ASSETS_COUNT_VIEW: &str = "vault_assets_count";
const VAULT_ADD_REPLENISHMENT_CALLBACK_CALL: &str = "add_replenishment_callback";
const VAULT_VIEW_REPLENISHERS_CALL: &str = "replenishers";
const VAULT_TEST_DEPOSIT: u128 = NEAR;
const VAULT_REPLENISH_CALLBACK: &str = "request_ft";
const VAULT_REPLENISH_ARGS: &str = "{arg: \"some value\"}";

// Constants related to the NEP-141 standard.
const FT_BALANCE_OF_CALL: &str = "ft_balance_of";
const FT_STORAGE_DEPOSIT_CALL: &str = "storage_deposit";
const FT_TRANSFER_CALL: &str = "ft_transfer";
const FT_TRANSFER_WITH_CALLBACK_CALL: &str = "ft_transfer_call";
const FT_STORAGE_DEPOSIT: u128 = 12_500_000_000_000_000_000_000;
