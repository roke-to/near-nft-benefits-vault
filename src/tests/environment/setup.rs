use anyhow::Result;
use near_sdk::serde_json::json;
use tokio::fs::read;
use workspaces::{network::Sandbox, testnet, Account, AccountId, Contract, Worker};

use crate::tests::{
    NFT_NEW_DEFAULT_META_CALL, NFT_WASM, WASMS_LOCATION, WRAP_NEAR_DEPOSIT, WRAP_NEAR_DEPOSIT_CALL,
    WRAP_NEAR_STORAGE_DEPOSIT, WRAP_NEAR_STORAGE_DEPOSIT_CALL, WRAP_NEAR_TESTNET_ACCOUNT_ID,
    WRAP_NEAR_WASM,
};

use super::format_helpers::format_execution_result;

/// Prepares w-near contract for the Sandbox.
/// Either imports it from testnet or uses local wasm binary.
pub async fn prepare_wrap_near_contract(sandbox: &Worker<Sandbox>) -> Result<Contract> {
    let id = WRAP_NEAR_TESTNET_ACCOUNT_ID.parse()?;
    let contract = match testnet().await {
        Ok(testnet) => {
            let contract = sandbox.import_contract(&id, &testnet).transact().await?;
            println!("wrap NEAR contract imported from testnet");
            contract
        }
        Err(e) => {
            println!("failed to connect to the testnet: {e}");
            println!("deploying local contract");
            let path = format!("{WASMS_LOCATION}/{WRAP_NEAR_WASM}");
            let wasm = read(path).await?;
            sandbox.dev_deploy(&wasm).await?
        }
    };

    let res = contract.call("new").transact().await?;
    println!(
        "\nwrapNEAR contract initialization outcome: {}\n",
        format_execution_result(&res)
    );

    Ok(contract)
}

pub async fn register_account(account: &Account, wrap_near: &AccountId) -> Result<()> {
    let args = json!(
        {
            "account_id": wrap_near,
        }
    );

    let res = account
        .call(wrap_near, WRAP_NEAR_STORAGE_DEPOSIT_CALL)
        .args_json(args)
        .deposit(WRAP_NEAR_STORAGE_DEPOSIT)
        .transact()
        .await?;
    println!(
        "account storage deposit in wrap near contract outcome: {}",
        format_execution_result(&res)
    );

    let res = account
        .call(wrap_near, WRAP_NEAR_DEPOSIT_CALL)
        .deposit(WRAP_NEAR_DEPOSIT)
        .transact()
        .await?;
    println!(
        "account registration in wrap near contract outcome: {}",
        format_execution_result(&res)
    );
    Ok(())
}

pub async fn prepare_issuer_account(
    sandbox: Worker<Sandbox>,
    wrap_near: AccountId,
) -> Result<Account> {
    let issuer = sandbox.dev_create_account().await?;

    register_account(&issuer, &wrap_near).await?;

    Ok(issuer)
}

pub async fn prepare_nft_owner_account(
    sandbox: Worker<Sandbox>,
    wrap_near: AccountId,
) -> Result<Account> {
    let owner = sandbox.dev_create_account().await?;

    register_account(&owner, &wrap_near).await?;

    Ok(owner)
}

pub async fn prepare_vault_contract(
    sandbox: Worker<Sandbox>,
    wrap_near: AccountId,
) -> Result<Contract> {
    let name = env!("CARGO_PKG_NAME").replace('-', "_");

    let path = format!("{WASMS_LOCATION}/{name}.wasm");
    println!("read WASM contract code from: {path}");

    let wasm = read(path).await?;
    println!("vault WASM code imported");

    let contract = sandbox.dev_deploy(&wasm).await?;
    println!("vault WASM code deployed");

    register_account(contract.as_account(), &wrap_near).await?;

    Ok(contract)
}

pub async fn prepare_nft_contract(sandbox: Worker<Sandbox>) -> Result<Contract> {
    let path = format!("{WASMS_LOCATION}/{NFT_WASM}");

    let wasm = read(path).await?;

    let contract = sandbox.dev_deploy(&wasm).await?;

    let args = json!({
        "owner_id": contract.id(),
    });

    let res = contract
        .call(NFT_NEW_DEFAULT_META_CALL)
        .args_json(args)
        .transact()
        .await?;
    println!(
        "NFT contract initialization: {}",
        format_execution_result(&res)
    );

    Ok(contract)
}
