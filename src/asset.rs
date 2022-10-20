use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    Balance,
};

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct Asset {
    pub balance: Balance,
}

impl Asset {
    pub fn new(balance: Balance) -> Self {
        Self { balance }
    }
}
