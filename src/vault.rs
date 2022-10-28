use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::UnorderedMap,
    AccountId,
};

use crate::asset::Asset;

/// Stores map with different FT assets.
/// FT contracts' account ids used as keys.
#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct Vault {
    pub assets: UnorderedMap<AccountId, Asset>,
}

impl Vault {
    /// Creates new vault.
    pub fn new() -> Self {
        let assets = UnorderedMap::new(b"b");
        Self { assets }
    }

    /// Increases balance of the FT by provided amount.
    /// If there is no Asset with `ft_contract_id` it will create a new one.
    /// `ft_contract_id`: account id of the contract with FT asset.
    /// `amount`: amount of tokens to be stored.
    pub fn store(&mut self, ft_contract_id: AccountId, amount: u128) {
        let mut asset = self.assets.get(&ft_contract_id).unwrap_or_else(Asset::new);
        asset.balance += amount;
        self.assets.insert(&ft_contract_id, &asset);
    }

    /// Adds FT asset to the inner map.
    /// Panics if asset already exists.
    /// `ft_contract_id`: account id of the contract with FT asset.
    pub fn add_asset(&mut self, ft_contract_id: AccountId) {
        assert!(
            self.assets.get(&ft_contract_id).is_none(),
            "Asset already exists"
        );
        let asset = Asset::new();
        self.assets.insert(&ft_contract_id, &asset);
    }

    /// Reduces balance of the given asset.
    /// Panics if there is no asset associated with the `ft_contract_id`.
    pub fn reduce_balance(&mut self, ft_contract_id: AccountId, amount: u128) {
        let mut asset = self.assets.get(&ft_contract_id).expect("unknown asset");
        asset.reduce_balance(amount);
        self.assets.insert(&ft_contract_id, &asset);
    }
}

impl Default for Vault {
    fn default() -> Self {
        Self::new()
    }
}
