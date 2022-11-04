pub mod args;
pub mod format_helpers;
pub mod setup;

use anyhow::{Context, Result};
use futures::{stream::FuturesUnordered, TryStreamExt};
use log::{debug, info};
use near_sdk::{
    json_types::U128,
    serde_json::{json, to_vec},
};
use std::{collections::HashMap, str::FromStr};
use tokio::fs::read;
use workspaces::{
    network::Sandbox, result::ExecutionFinalResult, sandbox, Account, AccountId, Contract, Worker,
};

use crate::{
    interface::request::Request,
    tests::{
        environment::{
            args::{
                ft_transfer_call_json, nft_metadata_json, nft_mint_json, nft_transfer_json,
                vault_view_bytes, vault_withdraw_all_json, vault_withdraw_json,
            },
            format_helpers::format_execution_result,
            setup::{
                init_logger, prepare_fungible_tokens, prepare_issuer_account, prepare_nft_contract,
                prepare_nft_owner_account, prepare_vault_contract, register_account,
            },
        },
        {
            FT_BALANCE_OF_CALL, FT_TRANSFER_WITH_CALLBACK_CALL, NFT_MINT_CALL,
            NFT_MINT_STORAGE_DEPOSIT, NFT_TOKEN_ID, NFT_TRANSFER_CALL,
            VAULT_ADD_REPLENISHMENT_CALLBACK_CALL, VAULT_BALANCE_OF_CALL, VAULT_REPLENISH_ARGS,
            VAULT_REPLENISH_CALLBACK, VAULT_TEST_DEPOSIT, VAULT_TEST_REPLENISHER_WASM,
            VAULT_VIEW_CALL, VAULT_VIEW_REPLENISHERS_CALL, VAULT_WITHDRAW_ALL_CALL,
            VAULT_WITHDRAW_CALL, WASMS_LOCATION,
        },
    },
    vault::Replenisher,
    views::{BalanceView, VaultView},
};

use self::args::{ft_balance_of_bytes, vault_balance_of_bytes};

/// Struct contains a bunch of useful contracts and accounts, frequently used in test cases.
pub struct Environment {
    /// Sandboxed network worker.
    pub sandbox: Worker<Sandbox>,
    /// Various fungible tokens contracts. The #0 contract is w-near.
    pub fungible_tokens: Vec<Contract>,
    /// The account that issues NFT and pays benefits.
    pub issuer: Account,
    /// The account that owns NFT and receives benefits.
    pub nft_owner: Account,
    /// The Vault contract.
    pub vault: Contract,
    /// A simple NFT contract.
    pub nft: Contract,
    pub replenisher: Option<Contract>,
}

impl Environment {
    pub async fn new() -> Result<Self> {
        init_logger();
        let sandbox = sandbox().await?;
        info!("sandbox initialized");

        let fungible_tokens = prepare_fungible_tokens(sandbox.clone()).await?;

        let issuer = tokio::spawn(prepare_issuer_account(
            sandbox.clone(),
            fungible_tokens.clone(),
        ));
        let nft_owner = tokio::spawn(prepare_nft_owner_account(
            sandbox.clone(),
            fungible_tokens.clone(),
        ));
        let vault = tokio::spawn(prepare_vault_contract(
            sandbox.clone(),
            fungible_tokens.clone(),
        ));
        let nft = tokio::spawn(prepare_nft_contract(sandbox.clone()));

        let issuer = issuer.await??;
        let nft_owner = nft_owner.await??;
        let vault = vault.await??;
        let nft = nft.await??;

        info!("issuer account: {}", issuer.id());
        info!("NFT owner account: {}", nft_owner.id());
        info!("NFT benefits vault account ready on: {}\n", vault.id());

        Ok(Environment {
            sandbox,
            fungible_tokens,
            issuer,
            nft_owner,
            vault,
            nft,
            replenisher: None,
        })
    }

    pub async fn deploy_replenisher(&mut self) -> Result<()> {
        let path = format!("{WASMS_LOCATION}/{VAULT_TEST_REPLENISHER_WASM}");
        let wasm = read(path).await?;
        let contract = self.sandbox.dev_deploy(&wasm).await?;
        info!("replenisher deployed at: {}", contract.id());

        let res = contract.call("new").transact().await?;
        debug!(
            "\nVault test replenisher contract initialization outcome: {}\n",
            format_execution_result(&res)
        );

        let tokens = self.fungible_tokens.iter().map(|ft| ft.id());
        register_account(contract.as_account(), tokens).await?;
        debug!("replenisher account registered in all tokens");

        self.replenisher = Some(contract);
        Ok(())
    }

    pub async fn nft_mint(&self) -> Result<()> {
        let token_metadata = nft_metadata_json();
        let args = nft_mint_json(self.issuer.id(), &token_metadata);
        let res = self
            .nft
            .call(NFT_MINT_CALL)
            .args_json(args)
            .deposit(NFT_MINT_STORAGE_DEPOSIT)
            .transact()
            .await?;
        debug!("NFT mint: {}", format_execution_result(&res));
        Ok(())
    }

    pub async fn nft_transfer(&self) -> Result<()> {
        let args = nft_transfer_json(self.nft_owner.id());
        let res = self
            .issuer
            .call(self.nft.id(), NFT_TRANSFER_CALL)
            .args_json(args)
            .deposit(1)
            .transact()
            .await?;
        debug!("NFT transfer: {}", format_execution_result(&res));
        Ok(())
    }

    pub async fn ft_transfer_call(
        &self,
        sender: &Account,
        receiver: &AccountId,
        token: &AccountId,
        amount: u128,
        msg: &str,
    ) -> Result<ExecutionFinalResult> {
        info!(
            "ft_transfer_call: \n\tsender: {sender:?} \n\treceiver: {receiver} \n\ttoken: {token:?}"
        );

        let args = ft_transfer_call_json(receiver, U128(amount), msg);

        let res = sender
            .call(token, FT_TRANSFER_WITH_CALLBACK_CALL)
            .args_json(args)
            .deposit(1)
            .max_gas()
            .transact()
            .await?;

        debug!("ft_transfer_call res: {}", format_execution_result(&res));

        Ok(res)
    }

    pub async fn vault_deposit(&self, token_contract_id: &AccountId) -> Result<()> {
        let nft_contract_id = near_sdk::AccountId::from_str(self.nft.id().as_str()).unwrap();
        let nft_id = NFT_TOKEN_ID.to_owned();
        let request = Request::top_up(nft_id, nft_contract_id);
        let request = near_sdk::serde_json::to_string(&request).unwrap();

        self.ft_transfer_call(
            &self.issuer,
            self.vault.id(),
            token_contract_id,
            VAULT_TEST_DEPOSIT,
            &request,
        )
        .await?
        .into_result()?;

        self.vault_view_print().await
    }

    pub async fn vault_view_print(&self) -> Result<()> {
        let args = vault_view_bytes(self.nft.id())?;
        let res = self.vault.view(VAULT_VIEW_CALL, args).await?;
        let vault_view: Option<VaultView> = res.json()?;
        println!("vault view: {vault_view:#?}");
        Ok(())
    }

    pub async fn vault_balance_of(&self) -> Result<Option<BalanceView>> {
        let args = vault_balance_of_bytes(self.nft.id())?;
        let res = self
            .sandbox
            .view(self.vault.id(), VAULT_BALANCE_OF_CALL, args)
            .await
            .with_context(|| "failed to call method on contract")?;
        let balance = res.json()?;
        Ok(balance)
    }

    pub async fn vault_withdraw_all(&self) -> Result<()> {
        let args = vault_withdraw_all_json(self.nft.id());
        let res = self
            .nft_owner
            .call(self.vault.id(), VAULT_WITHDRAW_ALL_CALL)
            .args_json(args)
            .deposit(1)
            .max_gas()
            .transact()
            .await?;
        println!("withdraw all: {}", format_execution_result(&res));

        Ok(())
    }

    pub async fn vault_withdraw(&self, fungible_token: &AccountId) -> Result<()> {
        let args = vault_withdraw_json(self.nft.id(), fungible_token);
        let res = self
            .nft_owner
            .call(self.vault.id(), VAULT_WITHDRAW_CALL)
            .args_json(args)
            .deposit(1)
            .max_gas()
            .transact()
            .await?;
        // debug!("withdraw: {}", format_execution_result(&res));
        debug!("withdraw: {res:#?}");

        Ok(())
    }

    pub async fn ft_balance_of(
        account_id: AccountId,
        token: Contract,
    ) -> Result<(AccountId, u128)> {
        let args = ft_balance_of_bytes(&account_id)?;
        let res = token.view(FT_BALANCE_OF_CALL, args).await?;
        let balance: U128 = res.json()?;
        Ok((token.id().clone(), balance.0))
    }

    pub async fn all_ft_balances_of(
        &self,
        account_id: &AccountId,
    ) -> Result<HashMap<AccountId, u128>> {
        let calls: FuturesUnordered<_> = self
            .fungible_tokens
            .iter()
            .map(|t| Self::ft_balance_of(account_id.clone(), t.clone()))
            .collect();
        calls.try_collect().await
    }

    pub async fn vault_add_replenisher(&self) -> Result<()> {
        let args = json!({
            "nft_contract_id": self.nft.id(),
            "nft_id": NFT_TOKEN_ID,
            "callback": VAULT_REPLENISH_CALLBACK,
            "args": VAULT_REPLENISH_ARGS,
        });
        let res = self
            .issuer
            .call(self.vault.id(), VAULT_ADD_REPLENISHMENT_CALLBACK_CALL)
            .args_json(args)
            .deposit(1)
            .transact()
            .await?;

        debug!("add replenisher: {}", format_execution_result(&res));

        Ok(())
    }

    pub async fn vault_view_replenishers(&self) -> Result<Option<Vec<Replenisher>>> {
        let args = to_vec(&json!({
            "nft_contract_id": self.nft.id(),
            "nft_id": NFT_TOKEN_ID,
        }))?;
        let res = self
            .issuer
            .view(self.vault.id(), VAULT_VIEW_REPLENISHERS_CALL, args)
            .await?;

        debug!("view replenishers: logs: {:#?}", res.logs);

        let replenishers = res.json()?;

        Ok(replenishers)
    }
}
