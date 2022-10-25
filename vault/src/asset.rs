use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    Balance,
};

/// Stores only `Balance` for now.
#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct Asset {
    pub balance: Balance,
}

impl Asset {
    /// Creates new `Asset` instance.
    pub fn new(balance: Balance) -> Self {
        Self { balance }
    }
}
