use near_sdk::near_bindgen;

use crate::Contract;

#[near_bindgen]
impl Contract {
    /// Function to view the content of the vault.
    pub fn balance_of(&self, nft_id: TokenId) -> Option<Balance> {
        let vault = self.vaults.get(&nft_id)?;

        let tokens: Vec<_> = vault
            .assets
            .iter()
            .map(|(account_id, asset)| Token {
                account_id,
                amount: asset.balance,
            })
            .collect();

        Some(Balance { nft_id, tokens })
    }
}
