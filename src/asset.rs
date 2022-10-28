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
    pub fn new() -> Self {
        Self { balance: 0 }
    }

    /// Subtracts provided amount from inner balance with overflow check.
    /// Panics if overflow occured.
    pub fn reduce_balance(&mut self, amount: Balance) {
        self.balance = self
            .balance
            .checked_sub(amount)
            .expect("subtraction with overflow");
    }
}

impl Default for Asset {
    fn default() -> Self {
        Self::new()
    }
}
