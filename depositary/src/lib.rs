use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::UnorderedMap,
    near_bindgen, AccountId,
};

/// Core structure of the smart contract.
#[near_bindgen]
#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct Contract {
    // vaults: UnorderedMap<TokenId, Vault>,
}

impl Contract {
    /// Adds provided amount of tokens to the vault specified by NFT `token_id`.
    pub fn store(&mut self, token_id: TokenId, ft_account_id: AccountId, amount: u128) {
        // let mut vault = if let Some(vault) = self.vaults.get(&token_id) {
        //     vault
        // } else {
        //     let mut vault = Vault::new(token_id.clone());
        //     vault.add_asset(ft_account_id.clone(), amount);
        //     vault
        // };
        // vault.store(ft_account_id, amount);

        // self.vaults.insert(&token_id, &vault);
        todo!()
    }
}
