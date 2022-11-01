use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::{
    env::{self, panic_str},
    json_types::U128,
    log, near_bindgen, AccountId, PromiseOrValue,
};

use crate::{
    interface::request::{Kind, Request},
    nft_id::NftId,
    Contract, ContractExt,
};

#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        let amount = amount.0;

        // This callback is called by the FT contract,
        // so predecessor account IS the FT contract.
        let ft_contract_id = env::predecessor_account_id();

        log!("received {} tokens from {}", amount, sender_id);

        let request = match Request::from_json(&msg) {
            Ok(req) => req,
            Err(e) => panic_str(&format!("request deserialization failed due to error: {e}")),
        };
        let token_id = request.nft_id().clone();
        let nft_contract_id = request.nft_contract_id().clone();
        let nft_id = NftId::new(nft_contract_id.clone(), token_id.clone());
        match request.kind() {
            Kind::TopUp => {
                log!(
                    "{} transferred {} of {} to vault {:?}",
                    sender_id,
                    amount,
                    ft_contract_id,
                    nft_id
                );

                self.store(nft_id, ft_contract_id, amount);
            }
            Kind::Transfer => {
                let mut vault = self.get_vault_or_create(&nft_id);
                vault.store(ft_contract_id.clone(), amount);
                self.vaults.insert(&nft_id, &vault);
                Self::ext(env::current_account_id()).withdraw_amount(
                    nft_contract_id,
                    token_id,
                    ft_contract_id,
                    amount,
                );
            }
        }

        PromiseOrValue::Value(U128(0))
    }
}
