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
    /// Request [`Kind`].
    kind: Kind,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Kind {
    /// On ft transfer with this variant as a msg the Contract will increase balance of the corresponding Vault.
    TopUp,
    /// If the contract have received FT with this request kind, it will immediately transfer it to the NFT owner.
    Transfer,
}

impl Request {
    /// Tryies to deserialize self from the provided json.
    pub fn from_json(s: &str) -> Result<Self, Error> {
        from_str(s)
    }

    /// Creates new instance of [`Request`].
    #[cfg(test)]
    pub fn new(nft_id: TokenId, nft_contract_id: AccountId, kind: Kind) -> Self {
        Self {
            nft_id,
            nft_contract_id,
            kind,
        }
    }

    /// Creates new instance of TopUp [`Request`].
    #[cfg(test)]
    pub fn top_up(nft_id: TokenId, nft_contract_id: AccountId) -> Self {
        Self::new(nft_id, nft_contract_id, Kind::TopUp)
    }

    /// Creates new instance of Transfer [`Request`].
    #[cfg(test)]
    pub fn transfer(nft_id: TokenId, nft_contract_id: AccountId) -> Self {
        Self::new(nft_id, nft_contract_id, Kind::Transfer)
    }

    /// Returns request [`Kind`].
    pub fn kind(&self) -> Kind {
        self.kind
    }

    /// Returns reference to the NFT id.
    pub fn nft_id(&self) -> &TokenId {
        &self.nft_id
    }

    /// Returns reference to the NFT contract id.
    pub fn nft_contract_id(&self) -> &AccountId {
        &self.nft_contract_id
    }
}
