use anyhow::Result;
use near_sdk::{
    json_types::U128,
    serde_json::{json, to_string, to_vec, Value},
};
use workspaces::AccountId;

use crate::tests::NFT_TOKEN_ID;

pub fn nft_metadata_json() -> Value {
    json!({
        "title": "Olympus Mons",
        "description": "Tallest mountain in charted solar system",
        "media": "https://upload.wikimedia.org/wikipedia/commons/thumb/0/00/Olympus_Mons_alt.jpg/1024px-Olympus_Mons_alt.jpg",
        "copies": 1
    })
}

pub fn nft_mint_json(receiver_id: &AccountId, token_metadata: &Value) -> Value {
    json!({
        "token_id": NFT_TOKEN_ID,
        "receiver_id": receiver_id,
        "token_metadata": token_metadata
    })
}

pub fn nft_transfer_json(receiver_id: &AccountId) -> Value {
    json!({
        "receiver_id": receiver_id,
        "token_id": NFT_TOKEN_ID,
    })
}

pub fn ft_transfer_call_json(receiver_id: &AccountId, amount: U128, msg: &str) -> Value {
    json!({
        "receiver_id": receiver_id,
        "amount": amount,
        "msg": msg,
    })
}

pub fn replenisher_withdraw_str(msg: &str) -> Result<String> {
    let args = to_string(&json!({
        "msg": msg,
    }))?;

    Ok(args)
}

pub fn add_replenishment_callback_str(nft_contract_id: &AccountId, args: &str) -> Result<String> {
    let args = to_string(&json!({
        "nft_contract_id": nft_contract_id,
        "nft_id": NFT_TOKEN_ID,
        "callback": "withdraw_call",
        "args": args,
    }))?;

    Ok(args)
}

pub fn replenisher_ft_on_transfer_request_str(vault: &AccountId, args: &str) -> Result<String> {
    let args = to_string(&json!({
        "vault": vault,
        "args": args,
    }))?;

    Ok(args)
}

pub fn vault_view_bytes(nft_contract_id: &AccountId) -> Result<Vec<u8>> {
    let args = to_vec(&json!({
        "nft_contract_id": nft_contract_id,
        "nft_id": NFT_TOKEN_ID,
    }))?;

    Ok(args)
}

pub fn vault_balance_of_bytes(nft_contract_id: &AccountId) -> Result<Vec<u8>> {
    let args = to_vec(&json!({
        "nft_contract_id": nft_contract_id,
        "nft_id": NFT_TOKEN_ID,
    }))?;

    Ok(args)
}

pub fn vault_withdraw_all_json(nft_contract_id: &AccountId) -> Value {
    json!({
        "nft_contract_id": nft_contract_id,
        "nft_id": NFT_TOKEN_ID,
    })
}

pub fn vault_withdraw_json(nft_contract_id: &AccountId, fungible_token: &AccountId) -> Value {
    json!({
        "nft_contract_id": nft_contract_id,
        "nft_id": NFT_TOKEN_ID,
        "fungible_token": fungible_token,
    })
}

pub fn ft_balance_of_bytes(account_id: &AccountId) -> Result<Vec<u8>> {
    let args = to_vec(&json!({
        "account_id": account_id,
    }))?;

    Ok(args)
}
