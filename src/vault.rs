use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::{UnorderedMap, UnorderedSet},
    env::{self, sha256},
    require,
    serde::{Deserialize, Serialize},
    AccountId,
};

use crate::{asset::Asset, nft_id::NftId};

/// Stores map with different FT assets.
/// FT contracts' account ids used as keys.
#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct Vault {
    assets: UnorderedMap<AccountId, Asset>,
    replenishers: UnorderedSet<Replenisher>,
}

#[derive(Debug, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Replenisher {
    contract_id: AccountId,
    callback: String,
    args: String,
    expiration_timestamp_ms: Option<u64>,
}

impl Vault {
    /// Creates new vault.
    pub fn new(nft_id: &NftId) -> Self {
        let nft_id_borsh = borsh::to_vec(&nft_id).expect("can't serialize NftId");

        let mut prefix = sha256(&nft_id_borsh);

        let mut prefix_assets = prefix.clone();
        prefix_assets.extend_from_slice(b"assets");
        let assets = UnorderedMap::new(prefix_assets);

        prefix.extend_from_slice(b"replenishers");
        let replenishers = UnorderedSet::new(prefix);

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
        asset.inc_balance(amount);
        self.assets.insert(fungible_token, &asset);
    }

    /// Reduces balance of the given asset.
    /// Panics if there is no asset associated with the `fungible_token`.
    pub fn reduce_balance(&mut self, fungible_token: &AccountId, amount: u128) {
        let mut asset = self.assets.get(fungible_token).expect("unknown asset");
        asset.reduce_balance(amount);
        self.assets.insert(fungible_token, &asset);
    }

    /// Adds replenisher to the Vault.
    ///
    /// # Panics
    ///
    /// Panics if replenisher is already registered.
    pub fn add_replenisher(
        &mut self,
        contract_id: AccountId,
        callback: String,
        args: String,
        duration_secs: Option<u64>,
    ) {
        let expiration_timestamp_ms = duration_secs.map(|d| {
            let now_ms = env::block_timestamp_ms();
            let duration_ms = d * 1000;
            now_ms + duration_ms
        });
        let replenisher = Replenisher {
            contract_id,
            callback,
            args,
            expiration_timestamp_ms,
        };
        require!(
            !self.replenishers.contains(&replenisher),
            "replenisher is already registered"
        );
        self.replenishers.insert(&replenisher);
    }

    pub fn insert_replenisher(&mut self, replenisher: &Replenisher) {
        require!(
            self.replenishers.insert(replenisher),
            "replenisher already exists"
        );
    }

    pub fn remove_replenishers(&mut self) -> Vec<Replenisher> {
        let repl = self.replenishers.to_vec();
        self.replenishers.clear();
        repl
    }

    /// Returns a reference to the replenishers of this [`Vault`].
    pub fn replenishers(&self) -> &UnorderedSet<Replenisher> {
        &self.replenishers
    }

    /// Returns a reference to the assets of this [`Vault`].
    pub fn assets(&self) -> &UnorderedMap<AccountId, Asset> {
        &self.assets
    }

    /// Returns asset for the given fungible token contract Id.
    pub fn get_asset(&self, fungible_token: &AccountId) -> Option<Asset> {
        self.assets.get(fungible_token)
    }

    /// Returns the assets count of this [`Vault`].
    pub fn assets_count(&self) -> u64 {
        self.assets.len()
    }
}

impl Replenisher {
    /// Returns a reference to the contract id of this [`Replenisher`].
    pub fn contract_id(&self) -> &AccountId {
        &self.contract_id
    }

    /// Returns a reference to the callback of this [`Replenisher`].
    pub fn callback(&self) -> &str {
        self.callback.as_ref()
    }

    /// Returns a reference to the args of this [`Replenisher`].
    pub fn args(&self) -> &str {
        self.args.as_ref()
    }

    pub fn is_expired(&self) -> bool {
        let now = env::block_timestamp_ms();
        if let Some(ts) = self.expiration_timestamp_ms {
            ts <= now
        } else {
            false
        }
    }
}
