use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::{env, json_types::U128, near_bindgen, AccountId, PromiseOrValue};

use crate::{Contract, ContractExt};

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
        let token_id = &msg; // @TODO token id validation.

        self.store(token_id.clone(), ft_account_id, amount);

        PromiseOrValue::Value(U128(0))
    }
}
