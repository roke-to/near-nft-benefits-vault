use anyhow::Result;
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::{collections::UnorderedMap, json_types::U128, serde_json::json};
use tokio::fs::read;
use workspaces::{
    network::Sandbox, result::ExecutionFinalResult, sandbox, testnet, Account, AccountId, Contract,
    Worker,
};

use crate::{vault::Vault, Contract as VaultsContract};

const WASMS_LOCATION: &str = "dist";
const WRAP_NEAR_WASM: &str = "wrap.wasm";

const NEAR: u128 = 10u128.pow(24);

const WRAP_NEAR_TESTNET_ACCOUNT_ID: &str = "wrap.testnet";
const WRAP_NEAR_STORAGE_DEPOSIT_CALL: &str = "storage_deposit";
const WRAP_NEAR_STORAGE_DEPOSIT: u128 = 12_500_000_000_000_000_000_000;
const WRAP_NEAR_DEPOSIT_CALL: &str = "near_deposit";
const WRAP_NEAR_DEPOSIT: u128 = 10 * NEAR;

#[tokio::test]
async fn test_contract() -> Result<()> {
    let sandbox = sandbox().await?;
    println!("sandbox initialized");

    let wrap_near = prepare_wrap_near_contract(&sandbox).await?;
    println!("wrap NEAR token account ready on: {}\n", wrap_near.id());

    let issuer = prepare_issuer_account(&sandbox, wrap_near.id()).await?;
    println!("issuer account: {}", issuer.id());

    let nft_owner = prepare_nft_owner_account(&sandbox, wrap_near.id()).await?;
    println!("NFT owner account: {}", nft_owner.id());

    let vault = prepare_vault_contract(&sandbox, wrap_near.id()).await?;
    println!("NFT benefits vault account ready on: {}\n", wrap_near.id());

    deposit_to_vault(&issuer, &wrap_near, &vault).await?;
    println!("deposit to vault: OK");

    check_vault_state(&nft_owner, &vault).await?;
    println!("check vault state: OK");

    Ok(())
}

async fn prepare_issuer_account(
    sandbox: &Worker<Sandbox>,
    wrap_near: &AccountId,
) -> Result<Account> {
    let issuer = sandbox.dev_create_account().await?;

    register_account(&issuer, wrap_near).await?;

    Ok(issuer)
}

async fn register_account(account: &Account, wrap_near: &AccountId) -> Result<()> {
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

async fn prepare_nft_owner_account(
    sandbox: &Worker<Sandbox>,
    wrap_near: &AccountId,
) -> Result<Account> {
    let owner = sandbox.dev_create_account().await?;

    register_account(&owner, wrap_near).await?;

    Ok(owner)
}

async fn prepare_wrap_near_contract(sandbox: &Worker<Sandbox>) -> Result<Contract> {
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
        "wrapNEAR contract initialization outcome: {}\n",
        format_execution_result(&res)
    );

    Ok(contract)
}

async fn prepare_vault_contract(
    sandbox: &Worker<Sandbox>,
    wrap_near: &AccountId,
) -> Result<Contract> {
    let name = env!("CARGO_PKG_NAME").replace('-', "_");

    let path = format!("{WASMS_LOCATION}/{name}.wasm");
    println!("read WASM contract code from: {path}");

    let wasm = read(path).await?;
    println!("vault WASM code imported");

    let contract = sandbox.dev_deploy(&wasm).await?;
    println!("vault WASM code deployed");

    register_account(contract.as_account(), wrap_near).await?;

    Ok(contract)
}

async fn deposit_to_vault(
    issuer: &Account,
    token_contract_id: &Contract,
    vault: &Contract,
) -> Result<()> {
    let args = json!(
        {
            "receiver_id": vault.id(),
            "amount": U128(NEAR),
            "msg": ""
        }
    );

    let res = issuer
        .call(token_contract_id.id(), "ft_transfer_call")
        .args_json(args)
        .deposit(1)
        .max_gas()
        .transact()
        .await?;

    println!("deposit to vault: {}", format_execution_result(&res));

    Ok(())
}

async fn check_vault_state(nft_owner: &Account, vault: &Contract) -> Result<()> {
    let state = nft_owner.view(vault.id(), "balance", vec![]).await?;
    println!("balance view call logs: {:#?}", state.logs);

    // println!("serialized response len: {}", state.result.len());

    // let vaults: UnorderedMap<TokenId, Vault> = state.borsh()?;

    // println!("vault state: {:#?}", vaults);
    todo!()
}

fn format_execution_result(res: &ExecutionFinalResult) -> String {
    format!(
        "\ntotal gas burnt: {}
        transaction: {:#?}
        receipts: {:#?}
        is success: {:#?}",
        res.total_gas_burnt,
        res.outcome(),
        res.receipt_outcomes(),
        res.is_success()
    )
}
