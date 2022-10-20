use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::{
    near_bindgen,
    serde::{Deserialize, Serialize},
    AccountId,
};

use crate::{Contract, ContractExt};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Balance {
    pub nft_id: TokenId,
    pub tokens: Vec<Token>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Token {
    pub account_id: AccountId,
    pub amount: u128,
}

#[near_bindgen]
impl Contract {
    pub fn balance_of(&self, nft_id: TokenId) -> Option<Balance> {
        let vault = self.vaults.get(&nft_id)?;

        let tokens: Vec<_> = vault
            .assets
            .iter()
            .map(|(account_id, asset)| Token {
                account_id,
                amount: asset.balance,
            })
            .collect();

        Some(Balance { nft_id, tokens })
    }
}
