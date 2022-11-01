#![doc = include_str!("../README.md")]

mod asset;
mod interface;

pub mod calls;
mod nft_id;
#[cfg(test)]
mod tests;
mod vault;
mod views;

use interface::ft::ft;
use nft_id::NftId;
use vault::Vault;

use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::UnorderedMap,
    env,
    json_types::U128,
    log, near_bindgen, AccountId, Promise,
};

/// Core structure of the smart contract.
#[near_bindgen]
#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct Contract {
    /// Map of users' Vaults with NFT TokenIds as their keys.
    vaults: UnorderedMap<NftId, Vault>,
}

#[near_bindgen]
impl Contract {
    /// Trivial init function.
    /// Panics if the contract is already initialized.
    #[init]
    #[private]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            vaults: UnorderedMap::new(b"a"),
        }
    }
}

impl Contract {
    /// Adds provided amount of tokens to the vault specified by [`NftId`].
    /// If there is no vault for the provided [`NftId`] then it will create a new one.
    pub fn store(&mut self, nft_id: NftId, ft_contract_id: AccountId, amount: u128) {
        let mut vault = if let Some(vault) = self.vaults.get(&nft_id) {
            log!("current balance of {}: {}", ft_contract_id, amount);
            vault
        } else {
            let mut vault = Vault::new();
            vault.add_asset(ft_contract_id.clone());
            vault
        };
        vault.store(ft_contract_id, amount);

        log!("vault created for: {:?}", nft_id);
        self.vaults.insert(&nft_id, &vault);
    }

    /// Shortcut to get vault from internal storage.
    /// Panics if there is no vault associated with the given [`NftId`].
    pub fn get_vault(&self, nft_id: &NftId) -> Vault {
        self.vaults
            .get(nft_id)
            .expect("vault is not created for the given nft_id")
    }

    /// Returns existing or new vault.
    pub fn get_vault_or_create(&self, nft_id: &NftId) -> Vault {
        self.vaults.get(nft_id).unwrap_or_else(|| {
            log!("new vault created: {:?}", nft_id);
            Vault::new()
        })
    }

    pub fn transfer_to(ft_contract_id: AccountId, nft_owner: AccountId, amount: u128) -> Promise {
        log!(
            "add transfer ft: {}, receiver: {}, amount: {}",
            ft_contract_id,
            nft_owner,
            amount
        );
        let memo = Some("Nft Benefits transfer".to_string());

        log!("ft transfer: {}", ft_contract_id);
        ft::ext(ft_contract_id)
            .with_attached_deposit(1)
            .ft_transfer(nft_owner, U128(amount), memo)
    }
}

impl Default for Contract {
    fn default() -> Self {
        Self::new()
    }
}
