mod format_helpers;
mod setup;

use format_helpers::format_execution_result;
use setup::{
    prepare_issuer_account, prepare_nft_contract, prepare_nft_owner_account,
    prepare_vault_contract, prepare_wrap_near_contract,
};

use anyhow::{Context, Result};
use near_sdk::{
    json_types::U128,
    serde_json::{json, to_vec},
};
use std::str::FromStr;
use workspaces::{network::Sandbox, sandbox, Account, AccountId, Contract, Worker};

use crate::{
    interface::request::Request,
    views::{BalanceView, VaultView},
};

use super::{
    NFT_MINT_CALL, NFT_MINT_STORAGE_DEPOSIT, NFT_TOKEN_ID, NFT_TRANSFER_CALL,
    VAULT_BALANCE_OF_CALL, VAULT_TEST_DEPOSIT, VAULT_VIEW_CALL, VAULT_WITHDRAW_ALL_CALL,
    WRAP_NEAR_FT_BALANCE_OF,
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

    pub async fn transfer_nft(&self) -> Result<()> {
        let args = json!({
            "receiver_id": self.nft_owner.id(),
            "token_id": NFT_TOKEN_ID,
        });
        let res = self
            .issuer
            .call(self.nft.id(), NFT_TRANSFER_CALL)
            .args_json(args)
            .deposit(1)
            .transact()
            .await?;
        println!("NFT transfer: {}", format_execution_result(&res));
        Ok(())
    }

    pub async fn deposit_to_vault(&self) -> Result<()> {
        let nft_contract_id = near_sdk::AccountId::from_str(self.nft.id().as_str()).unwrap();
        let nft_id = NFT_TOKEN_ID.to_owned();
        let req = Request::TopUp {
            nft_id,
            nft_contract_id,
        };
        let req = near_sdk::serde_json::to_string(&req).unwrap();
        let args = json!({
            "receiver_id": self.vault.id(),
            "amount": U128(VAULT_TEST_DEPOSIT),
            "msg": req
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

        self.check_deposit_to_vault().await
    }

    pub async fn check_deposit_to_vault(&self) -> Result<()> {
        let args = to_vec(&json!({
            "nft_id": NFT_TOKEN_ID,
        }))?;
        let res = self.vault.view(VAULT_VIEW_CALL, args).await?;
        let vault_view: Option<VaultView> = res.json()?;
        println!("vault view: {:#?}", vault_view);
        Ok(())
    }

    pub async fn vault_balance_of(&self) -> Result<Option<BalanceView>> {
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

    pub async fn withdraw_all(&self) -> Result<()> {
        let args = json!({
            "nft_id": NFT_TOKEN_ID,
        });
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

    pub async fn wrap_near_ft_balance_of(&self, account_id: &AccountId) -> Result<u128> {
        let args = to_vec(&json!({
            "account_id": account_id,
        }))?;
        let res = self.wrap_near.view(WRAP_NEAR_FT_BALANCE_OF, args).await?;
        let balance: U128 = res.json()?;
        Ok(balance.0)
    }
}
