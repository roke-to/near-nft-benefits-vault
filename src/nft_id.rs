use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    serde::{Deserialize, Serialize},
    AccountId,
};

/// Unique identifier of the NFT to be used as the Key to the Vault.
/// Only [`TokenId`] is insufficient because there can be multiple NFT contracts containing the same [`TokenId`].
#[derive(Debug, BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct NftId {
    contract_id: AccountId,
    token_id: TokenId,
}

impl NftId {
    /// Creates new insance.
    /// `contract_id`: NFT contract account id where this NFT exists.
    /// `token_id`: identifier of the NFT.
    pub fn new(contract_id: AccountId, token_id: TokenId) -> Self {
        Self {
            contract_id,
            token_id,
        }
    }

    /// Returns reference to the NFT `contract_id`.
    #[inline]
    pub fn contract_id(&self) -> &AccountId {
        &self.contract_id
    }

    /// Returns reference to the NFT `token_id`.
    #[inline]
    pub fn token_id(&self) -> &str {
        &self.token_id
    }
}
