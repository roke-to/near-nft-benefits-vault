use anyhow::Result;
use futures::{
    stream::{FuturesOrdered, FuturesUnordered},
    StreamExt, TryStreamExt,
};
use log::{debug, info, warn};
use near_sdk::{json_types::U128, serde_json::json};
use tokio::fs::read;
use workspaces::{network::Sandbox, testnet, Account, AccountId, Contract, Worker};

use crate::tests::{
    environment::{args::ft_new, format_helpers::format_execution_result},
    FT_STORAGE_DEPOSIT, FT_STORAGE_DEPOSIT_CALL, FT_TRANSFER_CALL, FUNGIBLE_TOKEN_WASM, NEAR,
    NFT_NEW_DEFAULT_META_CALL, NFT_WASM, WASMS_LOCATION, WRAP_NEAR_DEPOSIT, WRAP_NEAR_DEPOSIT_CALL,
    WRAP_NEAR_TESTNET_ACCOUNT_ID, WRAP_NEAR_WASM,
};

pub fn init_logger() {
    if let Err(e) = env_logger::Builder::new()
        .parse_env("RUST_LOG")
        .format_timestamp(None)
        .format_module_path(false)
        .format_target(false)
        .try_init()
    {
        info!("logger already initialized: {}", e);
    }
}

pub async fn prepare_fungible_tokens(
    sandbox: Worker<Sandbox>,
    custom_ft_count: usize,
) -> Result<Vec<Contract>> {
    let wrap_near = tokio::spawn(prepare_wrap_near_contract(sandbox.clone()));
    let mut handles = FuturesOrdered::new();
    handles.push_back(wrap_near);
    for i in 0..custom_ft_count {
        let custom_ft = tokio::spawn(prepare_custom_ft(sandbox.clone(), i));
        handles.push_back(custom_ft);
    }

    let res: Vec<_> = handles.map(|h| h.unwrap()).try_collect().await?;
    info!("wrap NEAR token account ready on: {}\n", res[0].id());
    for contract in res.iter().skip(1) {
        info!("custom fungible token ready on: {}\n", contract.id());
    }
    Ok(res)
}

/// Prepares w-near contract for the Sandbox. Either imports it from testnet or uses local wasm binary.
async fn prepare_wrap_near_contract(sandbox: Worker<Sandbox>) -> Result<Contract> {
    let id = WRAP_NEAR_TESTNET_ACCOUNT_ID.parse()?;
    let contract = match testnet().await {
        Ok(testnet) => {
            let contract = sandbox.import_contract(&id, &testnet).transact().await?;
            info!("wrap NEAR contract imported from testnet");
            contract
        }
        Err(e) => {
            warn!("failed to connect to the testnet: {e}");
            info!("deploying local contract");
            let path = format!("{WASMS_LOCATION}/{WRAP_NEAR_WASM}");
            let wasm = read(path).await?;
            sandbox.dev_deploy(&wasm).await?
        }
    };

    let res = contract.call("new").transact().await?;
    debug!(
        "\nwrapNEAR contract initialization outcome: {}\n",
        format_execution_result(&res)
    );

    Ok(contract)
}

/// Prepares custom fungible token contract from NEAR examples.
async fn prepare_custom_ft(sandbox: Worker<Sandbox>, index: usize) -> Result<Contract> {
    let path = format!("{WASMS_LOCATION}/{FUNGIBLE_TOKEN_WASM}");
    let wasm = read(path).await?;
    let contract = sandbox.dev_deploy(&wasm).await?;

    let args = ft_new(contract.id(), index);

    let res = contract.call("new").args_json(args).transact().await?;
    debug!(
        "\ncustom fungible token initialization: {}\n",
        format_execution_result(&res)
    );

    Ok(contract)
}

async fn register_account_impl(account: &Account, token: &AccountId) -> Result<()> {
    let args = json!(
        {
            "account_id": account.id(),
        }
    );

    let res = account
        .call(token, FT_STORAGE_DEPOSIT_CALL)
        .args_json(args)
        .deposit(FT_STORAGE_DEPOSIT)
        .transact()
        .await?;
    debug!(
        "account storage deposit on {token} contract outcome: {}",
        format_execution_result(&res)
    );
    Ok(())
}

/// Registeres an account in NEP-141 compatible FT contract.
pub async fn register_account(
    account: &Account,
    tokens: impl Iterator<Item = &AccountId>,
) -> Result<()> {
    let tasks: FuturesUnordered<_> = tokens.map(|t| register_account_impl(account, t)).collect();
    tasks.try_collect().await?;
    Ok(())
}

/// Deposits [`WRAP_NEAR_DEPOSIT`] amount of NEAR tokens to the w-NEAR contract.
pub async fn replenish_account_wrap_near(account: &Account, wrap_near: &AccountId) -> Result<()> {
    let res = account
        .call(wrap_near, WRAP_NEAR_DEPOSIT_CALL)
        .deposit(WRAP_NEAR_DEPOSIT)
        .transact()
        .await?;
    info!(
        "deposit {} NEAR of {wrap_near} to {}: {}",
        WRAP_NEAR_DEPOSIT / NEAR,
        account.id(),
        format_execution_result(&res)
    );
    Ok(())
}

/// Transfers `1*10^precision` tokens to the `account`.
pub async fn replenish_account_custom_ft(account: &Account, token: &Contract) -> Result<()> {
    let amount = NEAR;
    let args = json!({
        "receiver_id": account.id(),
        "amount": U128(amount),
    });
    let res = token
        .call(FT_TRANSFER_CALL)
        .args_json(args)
        .deposit(1)
        .transact()
        .await?;

    info!(
        "deposit {amount} of {} to {}: {}",
        token.id(),
        account.id(),
        format_execution_result(&res)
    );

    Ok(())
}

pub async fn prepare_issuer_account(
    sandbox: Worker<Sandbox>,
    tokens: Vec<Contract>,
) -> Result<Account> {
    let issuer = sandbox.dev_create_account().await?;

    register_account(&issuer, tokens.iter().map(Contract::id)).await?;

    replenish_account_wrap_near(&issuer, tokens[0].id()).await?;

    let tasks: FuturesUnordered<_> = tokens
        .iter()
        .skip(1)
        .map(|t| replenish_account_custom_ft(&issuer, t))
        .collect();

    tasks.try_collect().await?;

    Ok(issuer)
}

pub async fn prepare_nft_owner_account(
    sandbox: Worker<Sandbox>,
    tokens: Vec<Contract>,
) -> Result<Account> {
    let owner = sandbox.dev_create_account().await?;

    register_account(&owner, tokens.iter().map(Contract::id)).await?;

    Ok(owner)
}

pub async fn prepare_vault_contract(
    sandbox: Worker<Sandbox>,
    tokens: Vec<Contract>,
) -> Result<Contract> {
    let name = env!("CARGO_PKG_NAME").replace('-', "_");

    let path = format!("{WASMS_LOCATION}/{name}.wasm");
    debug!("read WASM contract code from: {path}");

    let wasm = read(path).await?;
    debug!("vault WASM code imported");

    let contract = sandbox.dev_deploy(&wasm).await?;
    info!("vault WASM code deployed at: {}", contract.id());

    register_account(contract.as_account(), tokens.iter().map(Contract::id)).await?;

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
    debug!(
        "NFT contract initialization: {}",
        format_execution_result(&res)
    );

    Ok(contract)
}
