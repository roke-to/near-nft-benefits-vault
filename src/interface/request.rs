use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::{
    serde::{Deserialize, Serialize},
    serde_json::{from_str, Error},
    AccountId,
};

#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    TopUp {
        nft_id: TokenId,
        nft_contract_id: AccountId,
    },
}

impl Request {
    pub fn from_json(s: &str) -> Result<Self, Error> {
        from_str(s)
    }
}
