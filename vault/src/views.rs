use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::{
    near_bindgen,
    serde::{Deserialize, Serialize},
    AccountId,
};

use crate::{Contract, ContractExt, VaultId};

/// Complete list of tokens in the Vault.
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Balance {
    pub nft_id: TokenId,
    pub tokens: Vec<Token>,
}

/// Info about FT.
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Token {
    pub account_id: AccountId,
    pub amount: u128,
}

#[near_bindgen]
impl Contract {
    pub fn get_vault_address(&self, nft_id: TokenId, nft_contract: AccountId) -> Option<VaultId> {
        todo!("return vault contract id associated with the given pair");
    }
}
