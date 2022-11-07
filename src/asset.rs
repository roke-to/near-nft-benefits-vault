use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    Balance,
};

/// Stores only `Balance` for now.
#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct Asset {
    balance: Balance,
}

impl Asset {
    /// Creates new [`Asset`] instance with zero balance.
    pub fn new() -> Self {
        Self { balance: 0 }
    }

    /// Increases asset balance by the provided `amount`.
    ///
    /// # Panics
    ///
    /// Panics if an overflow occured.
    pub fn inc_balance(&mut self, amount: Balance) {
        self.balance = self
            .balance
            .checked_add(amount)
            .expect("addition with overflow");
    }

    /// Reduces asset balance by the provided `amount`.
    ///
    /// # Panics
    ///
    /// Panics if an overflow occured.
    pub fn reduce_balance(&mut self, amount: Balance) {
        self.balance = self
            .balance
            .checked_sub(amount)
            .expect("subtraction with overflow");
    }

    /// Returns the balance of this [`Asset`].
    pub fn balance(&self) -> Balance {
        self.balance
    }
}

impl Default for Asset {
    fn default() -> Self {
        Self::new()
    }
}
