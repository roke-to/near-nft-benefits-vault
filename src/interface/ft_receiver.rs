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
        let ft_account_id = env::predecessor_account_id();

        log!("received {} tokens from {}", amount, sender_id);

        let request = match Request::from_json(&msg) {
            Ok(req) => req,
            Err(e) => panic_str(&format!("request deserialization failed due to error: {e}")),
        };
        match request.kind() {
            Kind::TopUp => {
                let token_id = request.nft_id().clone();
                let nft_contract_id = request.nft_contract_id().clone();
                let nft_id = NftId::new(nft_contract_id, token_id);
                log!(
                    "{} transferred {} of {} to vault {:?}",
                    sender_id,
                    amount,
                    ft_account_id,
                    nft_id
                );

                self.store(nft_id, ft_account_id, amount);
            }
            Kind::Transfer => {
                let transfer_to_promise = self.redirect_to();
                let check_transfer =
                    Contract::ext(env::current_account_id()).check_redirected_transfer();
                transfer_to_promise.then(check_transfer);
            }
        }

        PromiseOrValue::Value(U128(0))
    }
}
