use near_primitives::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{AccountId, Balance};

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct Asset {
    pub ft_account_id: AccountId,
    pub balance: Balance,
}

impl Asset {
    pub fn new(ft_account_id: AccountId, balance: Balance) -> Self {
        Self {
            ft_account_id,
            balance,
        }
    }
}
