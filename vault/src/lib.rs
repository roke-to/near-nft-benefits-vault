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

type VaultId = AccountId;
type NftId = TokenId;
type NftContractId = AccountId;

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
        todo!("store vault smart contract to storage");
        // Self {
        //     vaults: UnorderedMap::new(b"a"),
        // }
    }

    pub fn create_vault(&mut self, nft_id: NftId, nft_contract: NftContractId) -> VaultId {
        todo!("deploy vault contract and return it's account Id")
    }

    pub fn close_vault(&mut self, nft_id: NftId, nft_contract: NftContractId) {
        todo!("delete vault contract and transfer all remaining assets to onwer or issuer");
    }

    pub fn update_internal_vault_code(&mut self, code: Vec<u8>) {
        todo!("update internal vault contract code");
    }

    pub fn update_vault_code(&mut self, vault: VaultId) {
        todo!("update specified vault contract code");
    }
}

impl Default for Contract {
    fn default() -> Self {
        Self::new()
    }
}
