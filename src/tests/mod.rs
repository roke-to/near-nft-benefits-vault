/// Different tools for testing.
mod environment;

mod integration;
mod view_balance_of;
mod withdraw_all;

// Precompiled smart contracts locations.
const WASMS_LOCATION: &str = "dist";
const WRAP_NEAR_WASM: &str = "wrap.wasm";
const NFT_WASM: &str = "non_fungible_token.wasm";

// NEAR is 10^24 yoctoNEAR.
const NEAR: u128 = 10u128.pow(24);

// Constants related to the wrap NEAR FT contract.
const WRAP_NEAR_TESTNET_ACCOUNT_ID: &str = "wrap.testnet";
const WRAP_NEAR_STORAGE_DEPOSIT_CALL: &str = "storage_deposit";
const WRAP_NEAR_STORAGE_DEPOSIT: u128 = 12_500_000_000_000_000_000_000;
const WRAP_NEAR_DEPOSIT_CALL: &str = "near_deposit";
const WRAP_NEAR_DEPOSIT: u128 = 10 * NEAR;
const WRAP_NEAR_FT_BALANCE_OF: &str = "ft_balance_of";

// Constants related to the NFT contract.
const NFT_NEW_DEFAULT_META_CALL: &str = "new_default_meta";
const NFT_MINT_CALL: &str = "nft_mint";
const NFT_MINT_STORAGE_DEPOSIT: u128 = 8_400_000_000_000_000_000_000;
const NFT_TOKEN_ID: &str = "awesome_test_pic_666";
const NFT_TRANSFER_CALL: &str = "nft_transfer";

// Constants related to the nft-benefits-vault contract.
const VAULT_BALANCE_OF_CALL: &str = "balance_of";
const VAULT_WITHDRAW_ALL_CALL: &str = "withdraw_all";
const VAULT_VIEW_CALL: &str = "vault";
const VAULT_TEST_DEPOSIT: u128 = NEAR;
