pub mod args;
pub mod format_helpers;
pub mod setup;

use anyhow::{Context, Error, Result};
use futures::{
    stream::{FuturesOrdered, FuturesUnordered},
    TryStreamExt,
};
use log::{debug, info};
use near_sdk::{
    json_types::U128,
    serde_json::{json, to_string, to_vec},
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
                add_replenishment_callback_str, ft_balance_of_bytes, ft_transfer_call_json,
                nft_metadata_json, nft_mint_json, nft_transfer_json,
                replenisher_ft_on_transfer_request_str, replenisher_withdraw_str,
                vault_balance_of_bytes, vault_view_bytes, vault_withdraw_all_json,
                vault_withdraw_amount_json, vault_withdraw_json,
            },
            format_helpers::format_execution_result,
            setup::{
                init_logger, prepare_fungible_tokens, prepare_issuer_account, prepare_nft_contract,
                prepare_nft_owner_account, prepare_vault_contract, register_account,
            },
        },
        FT_BALANCE_OF_CALL, FT_TRANSFER_WITH_CALLBACK_CALL, NFT_MINT_CALL,
        NFT_MINT_STORAGE_DEPOSIT, NFT_TOKEN_ID_BASE, NFT_TRANSFER_CALL,
        VAULT_ADD_REPLENISHMENT_CALLBACK_CALL, VAULT_ASSETS_COUNT_VIEW, VAULT_BALANCE_OF_VIEW,
        VAULT_REPLENISH_ARGS, VAULT_REPLENISH_CALLBACK, VAULT_TEST_DEPOSIT,
        VAULT_TEST_REPLENISHER_WASM, VAULT_VIEW_REPLENISHERS_CALL, VAULT_WITHDRAW_ALL_CALL,
        VAULT_WITHDRAW_AMOUNT_CALL, VAULT_WITHDRAW_CALL, WASMS_LOCATION,
    },
    vault::Replenisher,
    views::{BalanceView, VaultView},
};

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
    pub non_fungible_tokens: Vec<Contract>,
    pub replenishers: Vec<Contract>,
}

impl Environment {
    pub async fn new(custom_ft_count: usize) -> Result<Self> {
        init_logger();
        let sandbox = sandbox().await?;
        info!("sandbox initialized");

        let fungible_tokens = prepare_fungible_tokens(sandbox.clone(), custom_ft_count).await?;

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
        let non_fungible_tokens = vec![nft.await??];

        info!("issuer account: {}", issuer.id());
        info!("NFT owner account: {}", nft_owner.id());
        info!("NFT benefits vault account ready on: {}\n", vault.id());

        Ok(Environment {
            sandbox,
            fungible_tokens,
            issuer,
            nft_owner,
            vault,
            non_fungible_tokens,
            replenishers: Vec::new(),
        })
    }

    pub async fn add_nft_contract(&mut self) -> Result<()> {
        let contract = prepare_nft_contract(self.sandbox.clone()).await?;
        self.non_fungible_tokens.push(contract);
        Ok(())
    }

    pub fn nft_first(&self) -> &Contract {
        self.nft_nth(0)
    }

    pub fn nft_nth(&self, index: usize) -> &Contract {
        self.non_fungible_tokens
            .get(index)
            .expect("NFT with given index wasn't deployed")
    }

    pub async fn deploy_replenishers(&mut self, count: usize) -> Result<()> {
        let path = format!("{WASMS_LOCATION}/{VAULT_TEST_REPLENISHER_WASM}");
        let wasm = read(path).await?;

        let mut contracts = FuturesOrdered::new();

        for i in 0..count {
            let sandbox = self.sandbox.clone();
            let tokens: Vec<_> = self
                .fungible_tokens
                .iter()
                .map(|ft| ft.id().clone())
                .collect();
            let code = wasm.clone();
            contracts.push_back(async move {
                let contract = sandbox.dev_deploy(&code).await?;
                info!("replenisher #{i} deployed at: {}", contract.id());
                let res = contract.call("new").transact().await?;
                debug!(
                    "\nVault test replenisher contract initialization outcome: {}\n",
                    format_execution_result(&res)
                );
                register_account(contract.as_account(), tokens.iter()).await?;
                debug!("replenisher account registered in all tokens");

                Ok::<Contract, Error>(contract)
            });
        }
        self.replenishers = contracts.try_collect().await?;

        Ok(())
    }

    pub async fn top_up_replenishers(&self, token: &AccountId, amount: u128) -> Result<()> {
        let req = Request::transfer(
            NFT_TOKEN_ID_BASE.to_owned(),
            self.nft_first().id().as_str().parse()?,
        );
        let transfer_req = to_string(&req)?;

        let args = replenisher_withdraw_str(&transfer_req)?;

        let args = add_replenishment_callback_str(self.nft_first().id(), &args)?;

        let msg = replenisher_ft_on_transfer_request_str(self.vault.id(), &args)?;

        for replenisher in self.replenishers.iter().map(|r| r.id()) {
            self.ft_transfer_call(&self.issuer, replenisher, token, amount, &msg)
                .await?
                .into_result()?;
        }

        Ok(())
    }

    pub async fn nft_mint_all(&self) -> Result<()> {
        for (i, contract) in self.non_fungible_tokens.iter().enumerate() {
            let token_metadata = nft_metadata_json(i);
            let args = nft_mint_json(self.issuer.id(), &token_metadata, i);
            let res = contract
                .call(NFT_MINT_CALL)
                .args_json(args)
                .deposit(NFT_MINT_STORAGE_DEPOSIT)
                .transact()
                .await?;
            debug!("NFT #[{i}] mint: {}", format_execution_result(&res));
        }
        Ok(())
    }

    pub async fn nft_transfer(&self) -> Result<()> {
        let nft_contract_id = self.nft_first().id();
        Self::nft_transfer_impl(&self.issuer, self.nft_owner.id(), nft_contract_id, 0).await
    }

    pub async fn nft_transfer_all(&self) -> Result<()> {
        let from = &self.issuer;
        let to = self.nft_owner.id();
        for (i, contract) in self.non_fungible_tokens.iter().enumerate() {
            Self::nft_transfer_impl(from, to, contract.id(), i).await?;
        }
        Ok(())
    }

    pub async fn nft_transfer_impl(
        from: &Account,
        to: &AccountId,
        nft_contract_id: &AccountId,
        index: usize,
    ) -> Result<()> {
        let args = nft_transfer_json(to, index);
        let res = from
            .call(nft_contract_id, NFT_TRANSFER_CALL)
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

    pub async fn vault_deposit(
        &self,
        token_contract_id: &AccountId,
        nft_index: usize,
    ) -> Result<()> {
        let nft_contract_id =
            near_sdk::AccountId::from_str(self.nft_nth(nft_index).id().as_str()).unwrap();
        let nft_id = format!("{NFT_TOKEN_ID_BASE}{nft_index}");
        let request = Request::top_up(nft_id, nft_contract_id);
        let request = near_sdk::serde_json::to_string(&request).unwrap();

        let res = self
            .ft_transfer_call(
                &self.issuer,
                self.vault.id(),
                token_contract_id,
                VAULT_TEST_DEPOSIT,
                &request,
            )
            .await?
            .into_result()?;
        debug!("ft transfer call: {:#?}", res);

        info!("check vault view: token_contract_id: {token_contract_id}, nft_index: {nft_index}");
        self.vault_view_print(nft_index).await
    }

    pub async fn vault_view_print(&self, index: usize) -> Result<()> {
        let args = vault_view_bytes(self.nft_nth(index).id(), index)?;
        let res = self.vault.view(VAULT_ASSETS_COUNT_VIEW, args).await?;
        let vault_view: Option<VaultView> = res.json()?;
        println!("vault view: {vault_view:#?}");
        Ok(())
    }

    pub async fn vault_balance_of(&self, nft_contract_index: usize) -> Result<Option<BalanceView>> {
        let nft_contract_id = self.nft_nth(nft_contract_index).id();
        let args = vault_balance_of_bytes(nft_contract_id, nft_contract_index)?;

        let res = self
            .sandbox
            .view(self.vault.id(), VAULT_BALANCE_OF_VIEW, args)
            .await
            .with_context(|| "failed to call method on contract")?;
        let balance = res.json()?;
        Ok(balance)
    }

    pub async fn vault_withdraw_all(&self) -> Result<ExecutionFinalResult> {
        let args = vault_withdraw_all_json(self.nft_first().id());
        let res = self
            .nft_owner
            .call(self.vault.id(), VAULT_WITHDRAW_ALL_CALL)
            .args_json(args)
            .deposit(1)
            .max_gas()
            .transact()
            .await?;
        debug!("withdraw all: {}", format_execution_result(&res));

        Ok(res)
    }

    pub async fn vault_withdraw(&self, fungible_token: &AccountId) -> Result<ExecutionFinalResult> {
        let args = vault_withdraw_json(self.nft_first().id(), fungible_token);
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

        Ok(res)
    }

    pub async fn vault_withdraw_amount(
        &self,
        fungible_token: &AccountId,
        amount: u128,
    ) -> Result<()> {
        let args = vault_withdraw_amount_json(self.nft_first().id(), fungible_token, U128(amount));
        let res = self
            .nft_owner
            .call(self.vault.id(), VAULT_WITHDRAW_AMOUNT_CALL)
            .args_json(args)
            .deposit(1)
            .max_gas()
            .transact()
            .await?;
        debug!("withdraw amount: {res:#?}");
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

    pub async fn vault_add_test_replenisher(&self) -> Result<()> {
        let args = json!({
            "nft_contract_id": self.nft_first().id(),
            "nft_id": NFT_TOKEN_ID_BASE,
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
            "nft_contract_id": self.nft_first().id(),
            "nft_id": NFT_TOKEN_ID_BASE,
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
