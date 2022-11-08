use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::{
    log, near_bindgen,
    serde::{Deserialize, Serialize},
    AccountId,
};

use crate::{nft_id::NftId, vault::Replenisher, Contract, ContractExt};

/// Complete list of tokens in the Vault.
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct BalanceView {
    /// Unique identifier of the NFT.
    pub nft_id: NftId,
    /// List of FTs.
    pub tokens: Vec<Token>,
}

/// Info about FT.
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Token {
    /// Account id of the FT contract.
    pub contract_id: AccountId,
    /// Amount of this tokens in the Vault.
    pub amount: u128,
}

/// Info about vault.
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct VaultView {
    /// Key NFT with access to the Vault.
    pub nft_id: NftId,
    /// The number of FT types in the Vault.
    pub assets_count: u64,
}

#[near_bindgen]
impl Contract {
    /// Function to view balances of all assets in the vault.
    pub fn balance_of(&self, nft_contract_id: AccountId, nft_id: TokenId) -> Option<BalanceView> {
        let nft_id = NftId::new(nft_contract_id, nft_id);
        let vault = self.vaults.get(&nft_id)?;

        let tokens: Vec<_> = vault
            .assets()
            .iter()
            .map(|(contract_id, asset)| Token {
                contract_id,
                amount: asset.balance(),
            })
            .collect();

        Some(BalanceView { nft_id, tokens })
    }

    /// Function to view the amount of assets in the Vault.
    pub fn vault_assets_count(
        &self,
        nft_contract_id: AccountId,
        nft_id: TokenId,
    ) -> Option<VaultView> {
        let nft_id = NftId::new(nft_contract_id, nft_id);
        let vault = self.get_vault(&nft_id);
        Some(VaultView {
            nft_id,
            assets_count: vault.assets_count(),
        })
    }

    /// Function to view the replenishers of the vault.
    pub fn replenishers(
        &self,
        nft_contract_id: AccountId,
        nft_id: TokenId,
    ) -> Option<Vec<Replenisher>> {
        log!("view replenishers: {} {}", nft_contract_id, nft_id);
        let nft_id = NftId::new(nft_contract_id, nft_id);
        let vault = self.vaults.get(&nft_id)?;
        log!("vault assets count: {}", vault.assets_count());
        let replenishers = vault.replenishers().as_vector().to_vec();
        log!("replenishers: {:?}", replenishers);
        Some(replenishers)
    }
}
