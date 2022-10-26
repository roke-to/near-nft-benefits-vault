use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::{
    near_bindgen,
    serde::{Deserialize, Serialize},
    AccountId,
};

use crate::{nft_id::NftId, Contract, ContractExt};

/// Complete list of tokens in the Vault.
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct BalanceView {
    pub nft_id: NftId,
    pub tokens: Vec<Token>,
}

/// Info about FT.
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Token {
    pub account_id: AccountId,
    pub amount: u128,
}

/// Info about vault.
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct VaultView {
    pub nft_id: NftId,
    pub assets_count: u64,
}

#[near_bindgen]
impl Contract {
    /// Function to view the content of the vault.
    pub fn balance_of(&self, nft_contract_id: AccountId, nft_id: TokenId) -> Option<BalanceView> {
        let nft_id = NftId::new(nft_contract_id, nft_id);
        let vault = self.vaults.get(&nft_id)?;

        let tokens: Vec<_> = vault
            .assets
            .iter()
            .map(|(account_id, asset)| Token {
                account_id,
                amount: asset.balance,
            })
            .collect();

        Some(BalanceView { nft_id, tokens })
    }

    pub fn vault(&self, nft_contract_id: AccountId, nft_id: TokenId) -> Option<VaultView> {
        let nft_id = NftId::new(nft_contract_id, nft_id);
        let vault = self.get_vault(&nft_id);
        Some(VaultView {
            nft_id,
            assets_count: vault.assets.len(),
        })
    }
}
