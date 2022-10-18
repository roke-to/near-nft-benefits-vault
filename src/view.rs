use near_sdk::{borsh, near_bindgen, serde_json};

use crate::{Contract, ContractExt};

#[near_bindgen]
impl Contract {
    pub fn balance(&self) -> Vec<u8> {
        borsh::to_vec(&self.vaults).expect("serialization failed")
    }
}
