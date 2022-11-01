use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::{
    serde::{Deserialize, Serialize},
    serde_json::{from_str, Error},
    AccountId,
};

/// This struct is used to expand the functionality of the Contract when receiving fungible tokens.
#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    /// Id of the NFT unique only within one NFT contract.
    nft_id: TokenId,
    /// NFT contract account id.
    nft_contract_id: AccountId,
    kind: Kind,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Kind {
    /// On ft transfer with this variant as a msg the Contract will increase balance of the corresponding Vault.
    TopUp,
    Transfer,
}

impl Request {
    /// Tryies to deserialize self from the provided json.
    pub fn from_json(s: &str) -> Result<Self, Error> {
        from_str(s)
    }

    #[cfg(test)]
    pub fn top_up(nft_id: TokenId, nft_contract_id: AccountId) -> Self {
        Self {
            nft_id,
            nft_contract_id,
            kind: Kind::TopUp,
        }
    }

    pub fn kind(&self) -> Kind {
        self.kind
    }

    pub fn nft_id(&self) -> &TokenId {
        &self.nft_id
    }

    pub fn nft_contract_id(&self) -> &AccountId {
        &self.nft_contract_id
    }
}
