/*!
NFT Benefits Vault.
*/

mod asset;
mod interface;

mod vault;
pub mod views;

use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::UnorderedMap,
    env, near_bindgen, AccountId,
};
use vault::Vault;

/// Core structure of the smart contract.
#[near_bindgen]
#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct Contract {
    vaults: UnorderedMap<TokenId, Vault>,
}

#[near_bindgen]
impl Contract {
    /// Trivial init function.
    #[init]
    #[private]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            vaults: UnorderedMap::new(b"a"),
        }
    }

    /// Public function to withdraw tokens with access by NFT `TokenId`.
    #[payable]
    pub fn withdraw(token_id: TokenId) {
        todo!()
    }
}

impl Default for Contract {
    fn default() -> Self {
        Self::new()
    }
}
