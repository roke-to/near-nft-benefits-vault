use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::UnorderedMap,
    AccountId,
};

use crate::asset::Asset;

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct Vault {
    pub key: TokenId,
    pub assets: UnorderedMap<AccountId, Asset>,
}

impl Vault {
    pub fn new(key: TokenId) -> Self {
        let assets = UnorderedMap::new(b"b");
        Self { key, assets }
    }

    pub fn store(&mut self, ft_account_id: AccountId, amount: u128) {
        let asset = if let Some(mut asset) = self.assets.get(&ft_account_id) {
            asset.balance += amount;
            asset
        } else {
            Asset::new(amount)
        };
        self.assets.insert(&ft_account_id, &asset);
    }

    pub fn add_asset(&mut self, ft_account_id: AccountId, initial_balance: u128) {
        assert!(
            self.assets.get(&ft_account_id).is_none(),
            "Asset already exists"
        );
        let asset = Asset::new(initial_balance);
        self.assets.insert(&ft_account_id, &asset);
    }
}
