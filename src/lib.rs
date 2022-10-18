mod asset;
mod interface;

#[cfg(test)]
mod tests;
mod vault;
mod view;

use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::UnorderedMap,
    env, near_bindgen, AccountId,
};
use vault::Vault;

#[near_bindgen]
#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct Contract {
    vaults: UnorderedMap<TokenId, Vault>,
}

#[near_bindgen]
impl Contract {
    #[init]
    #[private]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            vaults: UnorderedMap::new(b"v"),
        }
    }

    #[payable]
    pub fn withdraw(token_id: TokenId) {
        todo!()
    }
}

impl Contract {
    pub fn store(&mut self, token_id: TokenId, ft_account_id: AccountId, amount: u128) {
        let mut vault = if let Some(vault) = self.vaults.get(&token_id) {
            vault
        } else {
            let mut vault = Vault::new(token_id.clone());
            vault.add_asset(ft_account_id.clone(), amount);
            vault
        };
        vault.store(ft_account_id, amount);

        self.vaults.insert(&token_id, &vault);
    }
}

impl Default for Contract {
    fn default() -> Self {
        Self::new()
    }
}
