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
    /// Gas consumption:
    /// - first TopUp, new vault created: 1.5 TGas
    /// - second TopUp, existing vault: 1.6 TGas
    /// - first Transfer, new vault created: 6.4 TGas
    /// - second Transfer, existing vault: 6.5 TGas
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        let amount = amount.0;

        // This callback is called by the FT contract,
        // so predecessor account IS the FT contract.
        let fungible_token = env::predecessor_account_id();

        log!(
            "received {} tokens from {}, msg: {}",
            amount,
            sender_id,
            msg
        );

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
                    "{} transferred {} of {} tokens to vault {:?}",
                    sender_id,
                    amount,
                    fungible_token,
                    nft_id
                );

                self.store(&nft_id, &fungible_token, amount);
            }
            Kind::Transfer => {
                log!(
                    "{} sended {} of {} tokens to be transferred to NFT({:?}) owner",
                    sender_id,
                    amount,
                    fungible_token,
                    nft_id
                );
                self.store(&nft_id, &fungible_token, amount);
                Self::ext(env::current_account_id()).withdraw_amount(
                    nft_contract_id,
                    token_id,
                    fungible_token,
                    U128(amount),
                );
            }
        }
        PromiseOrValue::Value(U128(0))
    }
}
