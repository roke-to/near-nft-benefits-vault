use anyhow::{Context, Result};
use near_sdk::{
    json_types::U128,
    serde_json::{json, to_vec},
};
use tokio::fs::read;
use workspaces::{
    network::Sandbox,
    result::{ExecutionFinalResult, ExecutionOutcome},
    sandbox, testnet, Account, AccountId, Contract, Worker,
};

use nft_benefits_vault::views::Balance;

use crate::{
    NEAR, NFT_MINT_CALL, NFT_MINT_STORAGE_DEPOSIT, NFT_NEW_DEFAULT_META_CALL, NFT_TOKEN_ID,
    NFT_WASM, VAULT_BALANCE_OF_CALL, WASMS_LOCATION, WRAP_NEAR_DEPOSIT, WRAP_NEAR_DEPOSIT_CALL,
    WRAP_NEAR_STORAGE_DEPOSIT, WRAP_NEAR_STORAGE_DEPOSIT_CALL, WRAP_NEAR_TESTNET_ACCOUNT_ID,
    WRAP_NEAR_WASM,
};

pub struct Environment {
    pub sandbox: Worker<Sandbox>,
    pub wrap_near: Contract,
    pub issuer: Account,
    pub nft_owner: Account,
    pub vault: Contract,
    pub nft: Contract,
}

impl Environment {
    pub async fn new() -> Result<Self> {
        let sandbox = sandbox().await?;
        println!("sandbox initialized");

        let wrap_near = prepare_wrap_near_contract(&sandbox).await?;
        println!("wrap NEAR token account ready on: {}\n", wrap_near.id());

        let issuer = tokio::spawn(prepare_issuer_account(
            sandbox.clone(),
            wrap_near.id().clone(),
        ));
        let nft_owner = tokio::spawn(prepare_nft_owner_account(
            sandbox.clone(),
            wrap_near.id().clone(),
        ));
        let vault = tokio::spawn(prepare_vault_contract(
            sandbox.clone(),
            wrap_near.id().clone(),
        ));
        let nft = tokio::spawn(prepare_nft_contract(sandbox.clone()));

        let issuer = issuer.await??;
        let nft_owner = nft_owner.await??;
        let vault = vault.await??;
        let nft = nft.await??;

        println!("issuer account: {}", issuer.id());
        println!("NFT owner account: {}", nft_owner.id());
        println!("NFT benefits vault account ready on: {}\n", vault.id());

        Ok(Environment {
            sandbox,
            wrap_near,
            issuer,
            nft_owner,
            vault,
            nft,
        })
    }

    pub async fn issue_nft(&self) -> Result<()> {
        let token_metadata = json!({
            "title": "Olympus Mons",
            "description": "Tallest mountain in charted solar system",
            "media": "https://upload.wikimedia.org/wikipedia/commons/thumb/0/00/Olympus_Mons_alt.jpg/1024px-Olympus_Mons_alt.jpg",
            "copies": 1
        });
        let args = json!({
            "token_id": NFT_TOKEN_ID,
            "receiver_id": self.issuer.id(),
            "token_metadata": token_metadata
        });
        let res = self
            .nft
            .call(NFT_MINT_CALL)
            .args_json(args)
            .deposit(NFT_MINT_STORAGE_DEPOSIT)
            .transact()
            .await?;
        println!("NFT mint: {}", format_execution_result(&res));
        Ok(())
    }

    pub async fn deposit_to_vault(&self) -> Result<()> {
        let args = json!({
            "receiver_id": self.vault.id(),
            "amount": U128(NEAR),
            "msg": NFT_TOKEN_ID
        });

        let res = self
            .issuer
            .call(self.wrap_near.id(), "ft_transfer_call")
            .args_json(args)
            .deposit(1)
            .max_gas()
            .transact()
            .await?;

        println!("deposit to vault: {}", format_execution_result(&res));

        Ok(())
    }

    pub async fn balance_of(&self) -> Result<Balance> {
        let args = to_vec(&json!({
            "nft_id": NFT_TOKEN_ID,
        }))?;
        let res = self
            .sandbox
            .view(self.vault.id(), VAULT_BALANCE_OF_CALL, args)
            .await
            .with_context(|| "failed to call method on contract")?;
        let balance = res.json()?;
        Ok(balance)
    }
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
        "\nwrapNEAR contract initialization outcome: {}\n",
        format_execution_result(&res)
    );

    Ok(contract)
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

async fn prepare_issuer_account(sandbox: Worker<Sandbox>, wrap_near: AccountId) -> Result<Account> {
    let issuer = sandbox.dev_create_account().await?;

    register_account(&issuer, &wrap_near).await?;

    Ok(issuer)
}

async fn prepare_nft_owner_account(
    sandbox: Worker<Sandbox>,
    wrap_near: AccountId,
) -> Result<Account> {
    let owner = sandbox.dev_create_account().await?;

    register_account(&owner, &wrap_near).await?;

    Ok(owner)
}

async fn prepare_vault_contract(
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

pub fn format_execution_result(res: &ExecutionFinalResult) -> String {
    format!(
        "\ntransaction: {}
receipts: {}
is success: {:#?}",
        format_execution_outcome(res.outcome()),
        format_receipt_outcomes(res.receipt_outcomes()),
        res.is_success()
    )
}

fn format_receipt_outcomes(outcomes: &[ExecutionOutcome]) -> String {
    outcomes
        .iter()
        .map(|outcome| format_execution_outcome(outcome) + "\n")
        .collect()
}

fn format_execution_outcome(outcome: &ExecutionOutcome) -> String {
    format!(
        "
    executor_id: {}
    gas_burnt: {},
    logs: {:#?},
    tokens_burnt: {}",
        outcome.executor_id, outcome.gas_burnt, outcome.logs, outcome.tokens_burnt
    )
}
