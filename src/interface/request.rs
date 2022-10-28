use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::{
    serde::{Deserialize, Serialize},
    serde_json::{from_str, Error},
    AccountId,
};

/// This enum is used to expand the functionality of the Contract when receiving fungible tokens.
#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    /// On ft transfer with this variant as a msg the Contract will increase balance of the corresponding Vault.
    TopUp {
        /// Id of the NFT unique only within one NFT contract.
        nft_id: TokenId,
        /// NFT contract account id.
        nft_contract_id: AccountId,
    },
}

impl Request {
    /// Tryies to deserialize self from the provided json.
    pub fn from_json(s: &str) -> Result<Self, Error> {
        from_str(s)
    }
}
