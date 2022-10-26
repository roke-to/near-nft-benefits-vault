use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    serde::{Deserialize, Serialize},
    AccountId,
};

#[derive(Debug, BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct NftId {
    contract_id: AccountId,
    token_id: TokenId,
}

impl NftId {
    pub fn new(contract_id: AccountId, token_id: TokenId) -> Self {
        Self {
            contract_id,
            token_id,
        }
    }

    #[inline]
    pub fn contract_id(&self) -> &AccountId {
        &self.contract_id
    }

    #[inline]
    pub fn token_id(&self) -> &str {
        &self.token_id
    }
}
