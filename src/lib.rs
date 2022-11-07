#![warn(clippy::all)]
#![doc = include_str!("../README.md")]

pub mod calls;

mod asset;
mod interface;
mod nft_id;
mod vault;
mod views;

#[cfg(test)]
mod tests;

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
    pub fn store(&mut self, nft_id: &NftId, fungible_token: &AccountId, amount: u128) {
        let mut vault = self.get_vault_or_create(nft_id);
        vault.store(fungible_token, amount);

        self.vaults.insert(nft_id, &vault);
        log!("{} of {} stored in {:?}", amount, fungible_token, nft_id);
    }

    /// Shortcut to get vault from internal storage.
    /// Panics if there is no vault associated with the given [`NftId`].
    #[must_use]
    pub fn get_vault(&self, nft_id: &NftId) -> Vault {
        self.vaults
            .get(nft_id)
            .expect("vault is not created for the given nft_id")
    }

    /// Returns existing or new vault.
    #[must_use]
    pub fn get_vault_or_create(&self, nft_id: &NftId) -> Vault {
        self.vaults.get(nft_id).unwrap_or_else(|| {
            log!("new vault created: {:?}", nft_id);
            Vault::new()
        })
    }

    pub fn transfer_to(fungible_token: AccountId, nft_owner: AccountId, amount: u128) -> Promise {
        log!(
            "add transfer ft: {}, receiver: {}, amount: {}",
            fungible_token,
            nft_owner,
            amount
        );
        let memo = Some("Nft Benefits transfer".to_string());

        log!("ft transfer: {}", fungible_token);
        ft::ext(fungible_token)
            .with_attached_deposit(1)
            .ft_transfer(nft_owner, U128(amount), memo)
    }
}

impl Default for Contract {
    fn default() -> Self {
        Self::new()
    }
}
