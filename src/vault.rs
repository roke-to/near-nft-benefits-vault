use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::{UnorderedMap, UnorderedSet},
    require,
    serde::{Deserialize, Serialize},
    AccountId,
};

use crate::asset::Asset;

/// Stores map with different FT assets.
/// FT contracts' account ids used as keys.
#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct Vault {
    pub assets: UnorderedMap<AccountId, Asset>,
    replenishers: UnorderedSet<Replenisher>,
}

#[derive(Debug, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Replenisher {
    contract_id: AccountId,
    callback: String,
    args: String,
}

impl Vault {
    /// Creates new vault.
    pub fn new() -> Self {
        let assets = UnorderedMap::new(b"b");
        let replenishers = UnorderedSet::new(b"c");
        Self {
            assets,
            replenishers,
        }
    }

    /// Increases balance of the FT by provided amount.
    /// If there is no Asset with `fungible_token` it will create a new one.
    /// `fungible_token`: account id of the contract with FT asset.
    /// `amount`: amount of tokens to be stored.
    pub fn store(&mut self, fungible_token: &AccountId, amount: u128) {
        let mut asset = self.assets.get(fungible_token).unwrap_or_else(Asset::new);
        asset.balance += amount;
        self.assets.insert(fungible_token, &asset);
    }

    /// Reduces balance of the given asset.
    /// Panics if there is no asset associated with the `fungible_token`.
    pub fn reduce_balance(&mut self, fungible_token: &AccountId, amount: u128) {
        let mut asset = self.assets.get(fungible_token).expect("unknown asset");
        asset.reduce_balance(amount);
        self.assets.insert(fungible_token, &asset);
    }

    pub fn add_replenisher(&mut self, contract_id: AccountId, callback: String, args: String) {
        let replenisher = Replenisher {
            contract_id,
            callback,
            args,
        };
        require!(
            !self.replenishers.contains(&replenisher),
            "replenisher is already registered"
        );
        self.replenishers.insert(&replenisher);
    }

    pub fn replenishers(&self) -> &UnorderedSet<Replenisher> {
        &self.replenishers
    }
}

impl Default for Vault {
    fn default() -> Self {
        Self::new()
    }
}

impl Replenisher {
    pub fn contract_id(&self) -> &AccountId {
        &self.contract_id
    }

    pub fn callback(&self) -> &str {
        self.callback.as_ref()
    }

    pub fn args(&self) -> &str {
        self.args.as_ref()
    }
}
