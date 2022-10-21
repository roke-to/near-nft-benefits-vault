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
        let ft_account_id = env::predecessor_account_id();
        let amount = amount.0;

        let request = Request::from_json(&msg).expect("request deserialization failed");
        match request {
            Request::TopUp {
                nft_id,
                nft_contract_id,
            } => {
                log!(
                    "{} transferred {} of {} to vault {}",
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
