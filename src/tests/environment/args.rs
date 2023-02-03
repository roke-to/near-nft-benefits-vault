use anyhow::Result;
use near_sdk::{
    json_types::U128,
    serde_json::{json, to_string, Value},
};
use workspaces::AccountId;

use crate::tests::{NEAR, NFT_TOKEN_ID_BASE};

pub fn nft_metadata(index: usize) -> Value {
    let title = format!("Olympus Mons #[{index}]");
    json!({
        "title": title,
        "description": "Tallest mountain in charted solar system",
        "media": "https://upload.wikimedia.org/wikipedia/commons/thumb/0/00/Olympus_Mons_alt.jpg/1024px-Olympus_Mons_alt.jpg",
        "copies": 1
    })
}

pub fn nft_mint(receiver_id: &AccountId, token_metadata: &Value, index: usize) -> Value {
    let token_id = format!("{NFT_TOKEN_ID_BASE}{index}");
    json!({
        "token_id": token_id,
        "receiver_id": receiver_id,
        "token_metadata": token_metadata
    })
}

pub fn nft_transfer(receiver_id: &AccountId, index: usize) -> Value {
    let token_id = format!("{NFT_TOKEN_ID_BASE}{index}");
    json!({
        "receiver_id": receiver_id,
        "token_id": token_id,
    })
}

pub fn ft_transfer_call(receiver_id: &AccountId, amount: U128, msg: &str) -> Value {
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

pub fn add_replenishment_callback_str(
    nft_contract_id: &AccountId,
    args: &str,
    index: usize,
    duration_secs: u64,
) -> Result<String> {
    let args_json = add_replenishment_callback(nft_contract_id, args, index, duration_secs);
    let args = to_string(&args_json)?;
    Ok(args)
}

pub fn add_replenishment_callback(
    nft_contract_id: &AccountId,
    args: &str,
    index: usize,
    duration_secs: u64,
) -> Value {
    let token_id = format!("{NFT_TOKEN_ID_BASE}{index}");
    json!({
        "nft_contract_id": nft_contract_id,
        "nft_id": token_id,
        "callback": "withdraw_call",
        "args": args,
        "duration_secs": duration_secs,
    })
}

pub fn replenisher_ft_on_transfer_request_str(vault: &AccountId, args: &str) -> Result<String> {
    let args = to_string(&json!({
        "vault": vault,
        "args": args,
    }))?;

    Ok(args)
}

pub fn vault_view(nft_contract_id: &AccountId, index: usize) -> Result<Value> {
    let token_id = format!("{NFT_TOKEN_ID_BASE}{index}");
    let args = json!({
        "nft_contract_id": nft_contract_id,
        "nft_id": token_id,
    });

    Ok(args)
}

pub fn vault_balance_of(nft_contract_id: &AccountId, index: usize) -> Result<Value> {
    let token_id = format!("{NFT_TOKEN_ID_BASE}{index}");
    let args = json!({
        "nft_contract_id": nft_contract_id,
        "nft_id": token_id,
    });

    Ok(args)
}

pub fn vault_withdraw_all(nft_contract_id: &AccountId, index: usize) -> Value {
    let token_id = format!("{NFT_TOKEN_ID_BASE}{index}");
    json!({
        "nft_contract_id": nft_contract_id,
        "nft_id": token_id,
    })
}

pub fn vault_withdraw(
    nft_contract_id: &AccountId,
    fungible_token: &AccountId,
    index: usize,
) -> Value {
    let token_id = format!("{NFT_TOKEN_ID_BASE}{index}");
    json!({
        "nft_contract_id": nft_contract_id,
        "nft_id": token_id,
        "fungible_token": fungible_token,
    })
}

pub fn vault_withdraw_amount(
    nft_contract_id: &AccountId,
    fungible_token: &AccountId,
    amount: U128,
    index: usize,
) -> Value {
    let token_id = format!("{NFT_TOKEN_ID_BASE}{index}");
    json!({
        "nft_contract_id": nft_contract_id,
        "nft_id": token_id,
        "fungible_token": fungible_token,
        "amount": amount,
    })
}

pub fn ft_balance_of(account_id: &AccountId) -> Result<Value> {
    let args = json!({
        "account_id": account_id,
    });

    Ok(args)
}

pub fn ft_new(owner_id: &AccountId, index: usize) -> Value {
    let name = format!("Token Name #{index}");
    let symbol = format!("EXLT{index}");
    json!({
        "owner_id": owner_id,
        "total_supply": U128(100 * NEAR),
        "metadata": {
            "spec": "ft-1.0.0",
            "name": name,
            "symbol": symbol,
            "decimals": 24
        }
    })
}
