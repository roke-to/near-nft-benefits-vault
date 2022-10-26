use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::UnorderedMap,
    AccountId,
};

use crate::asset::Asset;

/// Stores map with different FT assets.
#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct Vault {
    pub nft_id: TokenId,
    pub nft_contract_id: AccountId,
    pub assets: UnorderedMap<AccountId, Asset>,
}

impl Vault {
    /// Creates new vault.
    pub fn new(nft_id: TokenId, nft_contract_id: AccountId) -> Self {
        let assets = UnorderedMap::new(b"b");
        Self {
            nft_id,
            nft_contract_id,
            assets,
        }
    }

    /// Increases balance of the FT by provided amount.
    pub fn store(&mut self, ft_account_id: AccountId, amount: u128) {
        let asset = if let Some(mut asset) = self.assets.get(&ft_account_id) {
            asset.balance += amount;
            asset
        } else {
            Asset::new(amount)
        };
        self.assets.insert(&ft_account_id, &asset);
    }

    /// Adds FT asset to the inner map.
    pub fn add_asset(&mut self, ft_account_id: AccountId, initial_balance: u128) {
        assert!(
            self.assets.get(&ft_account_id).is_none(),
            "Asset already exists"
        );
        let asset = Asset::new(initial_balance);
        self.assets.insert(&ft_account_id, &asset);
    }

    pub fn reduce_balance(&mut self, ft_account_id: AccountId, amount: u128) {
        let mut asset = self.assets.get(&ft_account_id).expect("unknown asset");
        asset.reduce_balance(amount);
        self.assets.insert(&ft_account_id, &asset);
    }
}
