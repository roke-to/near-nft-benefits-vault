use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_sdk::{
    assert_one_yocto, assert_self, env, json_types::U128, log, near_bindgen, require, AccountId,
    Gas, Promise, PromiseError,
};

use crate::{interface::nft::nft, nft_id::NftId, Contract, ContractExt};

#[near_bindgen]
impl Contract {
    /// Public call to withdraw all FTs of every type from the Vault.
    /// The contract will check caller ownership of the NFT specified by the NFT contract Id and NFT Id.
    /// Then it will try to find the corresponding vault with access via provided contract/id pair.
    /// Exactly 1 yoctoNEAR must be attached.
    ///
    /// # Gas consumption
    /// Without internal XCC: 0.5 TGas - doesn't depend on amount of tokens in the vault.
    #[payable]
    pub fn withdraw_all(&mut self, nft_contract_id: AccountId, nft_id: TokenId) -> Promise {
        let caller = env::predecessor_account_id();
        log!("withdraw_all called by {}", caller);

        // 1 yoctoNEAR should attached to this call to prevent abuse.
        assert_one_yocto();

        let nft_id = NftId::new(nft_contract_id, nft_id);

        let get_nft_info =
            nft::ext(nft_id.contract_id().clone()).nft_token(nft_id.token_id().to_owned());
        let withdraw_all = Self::ext(env::current_account_id()).withdraw_all_callback(nft_id);

        get_nft_info.then(withdraw_all)
    }

    /// Public call to withdraw all tokens of a single type of FT from the Vault.
    /// The contract will check ownership of the NFT spectified by the arguments.
    /// Than it will try to find the vault with access via provided contract/id pair.
    /// And it makes `ft_transfer` with all available tokens on the provided `fungible_token`.
    /// Exactly 1 yoctoNEAR must be attached.
    ///
    /// # Gas consumption
    /// Without internal XCC: 11 TGas - doesn't depend on amount of tokens in the vault.
    #[payable]
    pub fn withdraw(
        &mut self,
        nft_contract_id: AccountId,
        nft_id: TokenId,
        fungible_token: AccountId,
    ) -> Promise {
        let caller = env::predecessor_account_id();
        log!("withdraw called by {}", caller);

        // 1 yoctoNEAR should attached to this call to prevent abuse.
        assert_one_yocto();

        let nft_id = NftId::new(nft_contract_id, nft_id);

        let get_nft_info = nft::ext(nft_id.contract_id().clone())
            // .with_static_gas(Gas::ONE_TERA * 4)
            .nft_token(nft_id.token_id().to_owned());

        let withdraw_and_replenish = Self::ext(env::current_account_id())
            // .with_static_gas(Gas::ONE_TERA * 280)
            .withdraw_callback(nft_id, fungible_token, None, true);

        get_nft_info.then(withdraw_and_replenish)
    }

    /// Public call to withdraw specified `amount` of tokens of a single type of FT from the Vault.
    /// The contract will check ownership of the NFT spectified by the arguments.
    /// Than it will try to find the vault with access via provided contract/id pair.
    /// And it makes `ft_transfer` with specified amount of tokens on the provided `fungible_token`.
    /// Exactly 1 yoctoNEAR must be attached.
    ///
    /// # Gas consumption
    /// Without internal XCC: 10.3 TGas - doesn't depend on amount of tokens in the vault.
    #[payable]
    pub fn withdraw_amount(
        &mut self,
        nft_contract_id: AccountId,
        nft_id: TokenId,
        fungible_token: AccountId,
        amount: U128,
    ) {
        let caller = env::predecessor_account_id();
        log!("withdraw amount called by {}", caller);

        if caller != env::current_account_id() {
            // 1 yoctoNEAR should attached to this call to prevent abuse.
            assert_one_yocto();
        }

        let nft_id = NftId::new(nft_contract_id, nft_id);
        log!("key: {:?}", nft_id);

        let get_nft_info = nft::ext(nft_id.contract_id().clone())
            // .with_static_gas(Gas::ONE_TERA * 4)
            .nft_token(nft_id.token_id().to_owned());

        let withdraw_without_replenish = Self::ext(env::current_account_id())
            // .with_static_gas(Gas::ONE_TERA * 210)
            .withdraw_callback(nft_id, fungible_token, Some(amount), false);

        get_nft_info.then(withdraw_without_replenish);
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
    ///
    /// # Gas consumption
    /// Without internal XCC:
    ///     - 0 assets:   3.0 TGas
    ///     - 1 asset:   11.0 TGas
    ///     - 2 assets:  14.1 TGas
    ///     - 10 assets: 56.6 TGas
    #[private]
    pub fn withdraw_all_callback(
        &mut self,
        #[callback_result] nft_info: Result<Option<Token>, PromiseError>,
        nft_id: NftId,
    ) {
        assert_self();
        let signer = env::signer_account_id();
        log!("withdraw all callback called by signer: {}", signer);

        let nft_info = nft_info
            .expect("failed to get nft info")
            .expect("NFT info query returned nothing");
        let nft_owner = nft_info.owner_id;
        require!(nft_owner == signer, "vault access denied");

        let vault = self.get_vault(&nft_id);

        for (fungible_token, asset) in vault.assets().iter() {
            let amount = asset.balance();
            let transfer = Self::transfer_to(fungible_token.clone(), nft_owner.clone(), amount);
            let adjust_balance = Self::ext(env::current_account_id()).adjust_balance(
                nft_id.clone(),
                fungible_token,
                U128(amount),
            );
            transfer.then(adjust_balance);
        }
    }

    /// Callback makes transfer of a single FT type to the signer if all requirements are met.
    ///
    /// # Gas consumption
    /// Without internal XCC:
    ///     - no replenishers:
    ///         - 0 assets:  3.0 TGas
    ///         - 1 asset:   8.7 TGas
    ///         - 2 assets:  8.8 TGas
    ///         - 10 assets: 8.8 TGas
    ///     - 1 replenisher:   11.5 TGas
    ///     - 2 replenishers:  14.2 TGas
    ///     - 10 replenishers: 36.1 TGas
    #[private]
    pub fn withdraw_callback(
        &mut self,
        #[callback_result] nft_info: Result<Option<Token>, PromiseError>,
        nft_id: NftId,
        fungible_token: AccountId,
        amount: Option<U128>,
        replenish: bool,
    ) {
        assert_self();
        let signer = env::signer_account_id();

        let nft_info = nft_info
            .expect("failed to get nft info")
            .expect("NFT info query returned nothing");

        let nft_owner = nft_info.owner_id;
        require!(nft_owner == signer, "vault access denied");

        log!("vault access granted");

        let mut vault = self.get_vault(&nft_id);

        let asset = vault.get_asset(&fungible_token);

        let mut promise = if let Some(asset) = asset {
            let amount = if let Some(amount) = amount {
                amount.0.min(asset.balance())
            } else {
                asset.balance()
            };
            log!(
                "transfer {} of {} tokens to {}",
                amount,
                fungible_token,
                nft_owner
            );

            let transfer = Self::transfer_to(fungible_token.clone(), nft_owner, amount);
            let adjust = Self::ext(env::current_account_id()).adjust_balance(
                nft_id.clone(),
                fungible_token,
                U128(amount),
            );
            Some(transfer.then(adjust))
        } else {
            log!("no {} tokens in the vault", fungible_token);
            None
        };

        if replenish {
            let replenishers_vec = vault.remove_replenishers();
            for replenisher in replenishers_vec.into_iter() {
                log!(
                    "calling replenisher: {}.{}({})",
                    replenisher.contract_id(),
                    replenisher.callback(),
                    replenisher.args()
                );
                let replenish = Promise::new(replenisher.contract_id().clone()).function_call(
                    replenisher.callback().to_owned(),
                    replenisher.args().as_bytes().to_vec(),
                    0,
                    Gas::ONE_TERA * 100,
                );
                promise = Some(if let Some(p) = promise {
                    p.then(replenish)
                } else {
                    replenish
                });
                if replenisher.is_expired() {
                    log!(
                        "replenisher {}.{}() is expired and removed",
                        replenisher.contract_id(),
                        replenisher.callback()
                    );
                } else {
                    vault.insert_replenisher(&replenisher);
                }
            }
            self.vaults.insert(&nft_id, &vault);
        }
    }

    /// This method is used to add replenishers for the Vault.
    /// When NFT owner calls withdraw methods the Contract will transfer available tokens from its own balance
    /// and will request available benefits from registered replenishers.
    /// This method MUST be called and signed by replenisher himself
    /// because future replenishment requests will be made to the signer account.
    ///
    /// # Gas consumption
    /// 3.6 TGas
    #[payable]
    pub fn add_replenishment_callback(
        &mut self,
        nft_contract_id: AccountId,
        nft_id: TokenId,
        callback: String,
        args: String,
        duration_secs: Option<u64>,
    ) {
        assert_one_yocto();

        let nft_id = NftId::new(nft_contract_id, nft_id);
        log!("NFT Id: {:?}", nft_id);

        let mut vault = self.get_vault_or_create(&nft_id);

        let contract_id = env::predecessor_account_id();

        log!("add replenisher: [{}].{}(args)", contract_id, callback);
        if let Some(d) = duration_secs {
            log!("replenisher will expire in {} seconds", d);
        }
        vault.add_replenisher(contract_id, callback, args, duration_secs);

        self.vaults.insert(&nft_id, &vault);
    }

    /// Callback invokes after each FT transfer call from this contract in withdrawal process.
    /// If transfer was success, internal balance will be reduced by the amount transferred.
    /// Private: can be called only by this contract.
    ///
    /// # Gas consumption
    /// 4.1 TGas
    #[private]
    pub fn adjust_balance(
        &mut self,
        #[callback_result] res: Result<(), PromiseError>,
        nft_id: NftId,
        fungible_token: AccountId,
        amount: U128,
    ) {
        assert_self();
        log!(
            "check transfers called by signer: {}",
            env::signer_account_id()
        );
        let amount = amount.0;
        if res.is_ok() {
            let mut vault = self.get_vault(&nft_id);
            vault.reduce_balance(&fungible_token, amount);
            self.vaults.insert(&nft_id, &vault);
            log!(
                "withdrawal success, nft: {:?}, ft: {}, amount: {}",
                nft_id,
                fungible_token,
                amount
            );
        } else {
            log!("transfer {} if {} tokens failed", amount, fungible_token);
        }
    }
}
