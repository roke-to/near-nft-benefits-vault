#![doc = include_str!("../README.md")]

mod asset;
mod interface;

mod nft_id;
#[cfg(test)]
mod tests;
mod vault;
mod views;

use interface::{ft::ft, nft::nft};
use nft_id::NftId;
use vault::Vault;

use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_sdk::{
    assert_one_yocto, assert_self,
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::UnorderedMap,
    env,
    json_types::U128,
    log, near_bindgen, require, AccountId, Promise, PromiseError,
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

    /// Public function to withdraw all FTs from the Vault.
    /// The contract will check ownership of the NFT spectified by the arguments.
    /// After that it will try to find the vault with access via provided contract/id pair.
    /// Exactly 1 yoctoNEAR must be attached.
    #[payable]
    pub fn withdraw_all(&mut self, nft_contract_id: AccountId, nft_id: TokenId) -> Promise {
        let caller = env::predecessor_account_id();
        log!("withdraw_all call by {}", caller);
        log!("check attached deposit: 1 yocto");
        // 1 yoctoNEAR should attached to this call to prevent abuse.
        assert_one_yocto();

        let nft_id = NftId::new(nft_contract_id, nft_id);

        let nft_info_promise =
            nft::ext(nft_id.contract_id().to_owned()).nft_token(nft_id.token_id().to_owned());

        nft_info_promise.then(Self::ext(env::current_account_id()).withdraw_all_callback(nft_id))
    }

    /// Public function to withdraw a single type of FTs from the Vault.
    /// The contract will check ownership of the NFT spectified by the arguments.
    /// After that it will try to find the vault with access via provided contract/id pair.
    /// And it makes `ft_transfer` with all available tokens on the provided `ft_contract_id`.
    /// Exactly 1 yoctoNEAR must be attached.
    #[payable]
    pub fn withdraw(
        &mut self,
        nft_contract_id: AccountId,
        nft_id: TokenId,
        ft_contract_id: AccountId,
    ) -> Promise {
        let caller = env::predecessor_account_id();
        log!("withdraw call by {}", caller);
        log!("check attached deposit: 1 yocto");
        // 1 yoctoNEAR should attached to this call to prevent abuse.
        assert_one_yocto();

        let nft_id = NftId::new(nft_contract_id, nft_id);

        let nft_info_promise =
            nft::ext(nft_id.contract_id().to_owned()).nft_token(nft_id.token_id().to_owned());

        nft_info_promise
            .then(Self::ext(env::current_account_id()).withdraw_callback(nft_id, ft_contract_id))
    }

    // @TODO think about other variants to name FT sources.
    /// This method is used to add replenishers for the Vault.
    /// When NFT owner calls withdraw methods the Contract will transfer available tokens from its own balance
    /// and will request available benefits from registered replenishers.
    /// This method MUST be called and signed by replenisher himself
    /// because future replenishment requests will be made to the signer account.
    #[payable]
    pub fn add_replenishment_callback(
        &mut self,
        nft_contract_id: AccountId,
        nft_id: TokenId,
        callback: String,
        args: String,
    ) {
        assert_one_yocto();

        let nft_id = NftId::new(nft_contract_id, nft_id);

        let mut vault = self.get_vault_or_create(&nft_id);

        let contract_id = env::signer_account_id();

        vault.add_replenisher(contract_id, callback, args);

        self.vaults.insert(&nft_id, &vault);
    }

    /// Callback invokes after request to the NFT contract to check ownership and grant access to the vault.
    /// Private: can be called only by this contract.
    ///
    /// Panics if:
    /// - called NOT by itself,
    /// - previous Promise failed,
    /// - there is no NFT with provided id on the NFT contract,
    /// - signer is NOT an NFT owner,
    /// - there is no Vault with access via given [`NftId`].
    #[private]
    pub fn withdraw_all_callback(
        &mut self,
        #[callback_result] nft_info: Result<Option<Token>, PromiseError>,
        nft_id: NftId,
    ) {
        assert_self();
        let signer = env::signer_account_id();
        log!("withdraw_all_callback called by signer: {}", signer);

        let nft_info = nft_info
            .expect("failed to get nft info")
            .expect("NFT info query returned nothing");
        let nft_owner = nft_info.owner_id;
        require!(nft_owner == signer, "vault access denied");

        let vault = self.get_vault(&nft_id);

        for (ft_contract_id, asset) in vault.assets.iter() {
            Self::transfer_and_adjust_balance(
                nft_id.clone(),
                ft_contract_id,
                nft_owner.clone(),
                asset.balance,
            );
        }
    }

    /// Callback makes transfer of a single FT type to the signer if all requirements are met.
    #[private]
    pub fn withdraw_callback(
        &mut self,
        #[callback_result] nft_info: Result<Option<Token>, PromiseError>,
        nft_id: NftId,
        ft_contract_id: AccountId,
    ) -> Promise {
        assert_self();
        let signer = env::signer_account_id();
        log!("withdraw_callback called by signer: {}", signer);

        let nft_info = nft_info
            .expect("failed to get nft info")
            .expect("NFT info query returned nothing");
        let nft_owner = nft_info.owner_id;
        require!(nft_owner == signer, "vault access denied");

        let vault = self.get_vault(&nft_id);

        let asset = vault.assets.get(&ft_contract_id).expect("unknown asset");
        Self::transfer_and_adjust_balance(nft_id, ft_contract_id, nft_owner, asset.balance)
    }

    /// Callback invokes after each FT transfer call from this contract in withdrawal process.
    /// If transfer was success, internal balance will be reduced by the amount transferred.
    /// Private: can be called only by this contract.
    #[private]
    pub fn adjust_balance(
        &mut self,
        #[callback_result] res: Result<(), PromiseError>,
        nft_id: NftId,
        ft_contract_id: AccountId,
        amount: u128,
    ) {
        assert_self();
        log!(
            "check transfers called by signer: {}",
            env::signer_account_id()
        );
        if res.is_ok() {
            let mut vault = self.get_vault(&nft_id);
            vault.reduce_balance(ft_contract_id.clone(), amount);
            self.vaults.insert(&nft_id, &vault);
            log!(
                "withdrawal success, nft: {:?}, ft: {}, amount: {}",
                nft_id,
                ft_contract_id,
                amount
            );
        } else {
            todo!("process promise failed");
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

    /// Returns Promise chained pair with FT transfer and balance adjustment.
    pub fn transfer_and_adjust_balance(
        nft_id: NftId,
        ft_contract_id: AccountId,
        nft_owner: AccountId,
        amount: u128,
    ) -> Promise {
        log!(
            "add transfer ft: {}, receiver: {}, amount: {}",
            ft_contract_id,
            nft_owner,
            amount
        );
        let memo = Some("Nft Benefits transfer".to_string());

        log!("ft transfer: {}", ft_contract_id);
        let ft_transfer_promise = ft::ext(ft_contract_id.clone())
            .with_attached_deposit(1)
            .ft_transfer(nft_owner, U128(amount), memo);

        log!("adjust balance: {}", ft_contract_id);
        let adjust_balance =
            Self::ext(env::current_account_id()).adjust_balance(nft_id, ft_contract_id, amount);
        ft_transfer_promise.then(adjust_balance)
    }
}

impl Default for Contract {
    fn default() -> Self {
        Self::new()
    }
}
