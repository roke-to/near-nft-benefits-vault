/*!
NFT Benefits Vault.
*/

mod asset;
mod interface;

#[cfg(test)]
mod tests;
mod vault;
mod views;

use interface::nft::nft;
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_sdk::{
    assert_one_yocto,
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::UnorderedMap,
    env,
    json_types::U128,
    near_bindgen, require, AccountId, Promise, PromiseResult,
};
use vault::Vault;

use crate::interface::ft::ft;

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

    /// Public function to withdraw all FTs in the Vault.
    #[payable]
    pub fn withdraw_all(&mut self, nft_id: TokenId) -> Promise {
        // 1 yoctoNEAR should attached to this call to prevent abuse.
        assert_one_yocto();

        let vault = self.get_vault(&nft_id);

        let nft_info_promise = nft::ext(vault.nft_contract_id).nft_token(vault.nft_id.clone());

        let transfer_promise = nft_info_promise
            .then(Self::ext(env::current_account_id()).withdraw_all_callback(vault.nft_id));

        transfer_promise.then(Self::ext(env::current_account_id()).check_transfers())
    }

    #[private]
    pub fn withdraw_all_callback(
        &mut self,
        #[callback_result] nft_info: Option<Token>,
        nft_id: TokenId,
    ) -> Option<Promise> {
        let nft_info = nft_info.expect("NFT info query returned nothing");
        let nft_owner = nft_info.owner_id;
        require!(
            nft_owner == env::predecessor_account_id(),
            "vault access denied"
        );

        let vault = self.get_vault(&nft_id);

        let mut transfer_promise: Option<Promise> = None;
        for (ft_account_id, asset) in vault.assets.iter() {
            let memo = Some("Nft Benefits transfer".to_string());
            let ft_transfer_promise =
                ft::ext(ft_account_id).ft_transfer(nft_owner.clone(), U128(asset.balance), memo);

            if let Some(promise) = transfer_promise {
                transfer_promise = Some(promise.then(ft_transfer_promise)); // @TODO can be joined with `.and()`
            } else {
                transfer_promise = Some(ft_transfer_promise);
            }
        }
        transfer_promise
    }

    #[private]
    pub fn check_transfers(&mut self, #[callback_result] res: PromiseResult) {
        if let PromiseResult::Successful(_) = res {
            todo!("reduce balances");
        }
    }
}

impl Contract {
    /// Adds provided amount of tokens to the vault specified by NFT `token_id`.
    pub fn store(
        &mut self,
        nft_id: TokenId,
        nft_contract_id: AccountId,
        ft_account_id: AccountId,
        amount: u128,
    ) {
        let mut vault = if let Some(vault) = self.vaults.get(&nft_id) {
            vault
        } else {
            let mut vault = Vault::new(nft_id.clone(), nft_contract_id);
            vault.add_asset(ft_account_id.clone(), amount);
            vault
        };
        vault.store(ft_account_id, amount);

        self.vaults.insert(&nft_id, &vault);
    }

    pub fn get_vault(&self, nft_id: &TokenId) -> Vault {
        self.vaults
            .get(nft_id)
            .expect("vault is not created for the given nft_id")
    }
}

impl Default for Contract {
    fn default() -> Self {
        Self::new()
    }
}
