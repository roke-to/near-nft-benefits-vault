use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::{env, json_types::U128, log, near_bindgen, AccountId, PromiseOrValue};

use crate::{interface::request::Request, Contract, ContractExt};

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

        let request = Request::from_json(&msg).expect("request deserialization failed");
        match request {
            Request::TopUp {
                nft_id,
                nft_contract_id,
            } => {
                log!(
                    "{} transferred {} of {} to vault #{}",
                    sender_id,
                    amount,
                    ft_account_id,
                    nft_id
                );

                self.store(nft_id, nft_contract_id, ft_account_id, amount);
            }
        }

        PromiseOrValue::Value(U128(0))
    }
}
