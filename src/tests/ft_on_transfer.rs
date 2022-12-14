use std::str::FromStr;

use anyhow::Result;
use log::info;
use near_sdk::serde_json::to_string;
use workspaces::error::ErrorKind;

use crate::{
    interface::request::Request,
    tests::{
        environment::{setup::replenish_account_wrap_near, Environment},
        NEAR, NFT_TOKEN_ID_BASE, VAULT_TEST_DEPOSIT,
    },
};

#[tokio::test]
async fn test_ft_on_transfer_invalid_msg() -> Result<()> {
    let env = Environment::new(0).await?;

    let token_contract_id = env.fungible_tokens[0].id();

    let res = env
        .ft_transfer_call(
            &env.issuer,
            env.vault.id(),
            token_contract_id,
            VAULT_TEST_DEPOSIT,
            "invalid msg",
        )
        .await?;

    let receipts = res.receipt_failures();
    let failure = receipts
        .iter()
        .find(|r| &r.executor_id == env.vault.id())
        .expect("at least one receipt should fail");

    let error = (**failure).clone().into_result().expect_err("unreachable");

    println!("execution error: {error}");

    assert!(matches!(error.kind(), ErrorKind::Execution));
    Ok(())
}

#[tokio::test]
async fn test_ft_on_transfer_request_top_up_new_vault() -> Result<()> {
    let env = Environment::new(0).await?;

    let token_contract_id = env.fungible_tokens[0].id();
    env.vault_deposit(token_contract_id, 0).await?;

    let balance = env
        .vault_balance_of(0)
        .await?
        .expect("should be some: vault deposit failed");
    let token = balance
        .tokens
        .iter()
        .find(|t| t.contract_id.as_str() == token_contract_id.as_str())
        .expect("should be some");

    assert_eq!(
        token.amount, VAULT_TEST_DEPOSIT,
        "vault token balance should be equal to amount in Request"
    );

    Ok(())
}

#[tokio::test]
async fn test_ft_on_transfer_request_top_up_existing_vault() -> Result<()> {
    let env = Environment::new(0).await?;

    let token_contract_id = env.fungible_tokens[0].id();
    env.vault_deposit(token_contract_id, 0).await?;

    env.vault_deposit(token_contract_id, 0).await?;

    let balance = env
        .vault_balance_of(0)
        .await?
        .expect("should be some: vault deposit failed");
    let token = balance
        .tokens
        .iter()
        .find(|t| t.contract_id.as_str() == token_contract_id.as_str())
        .expect("should be some");

    assert_eq!(
        token.amount,
        2 * VAULT_TEST_DEPOSIT,
        "vault token balance should be equal to amount in Request"
    );

    Ok(())
}

#[tokio::test]
async fn test_ft_on_transfer_request_transfer_new_vault() -> Result<()> {
    let env = Environment::new(0).await?;

    env.nft_mint_all().await?;
    env.nft_transfer().await?;
    replenish_account_wrap_near(&env.nft_owner, env.fungible_tokens[0].id()).await?;

    let nft_contract_id = near_sdk::AccountId::from_str(env.nft_first().id().as_str())?;
    let request = Request::transfer(NFT_TOKEN_ID_BASE.to_owned(), nft_contract_id);
    let request = to_string(&request)?;

    let token_contract_id = env.fungible_tokens[0].id();

    // In real world this method will be called by an issuer.
    // But for this test it is enough to call just part of the whole XCC chain.
    // So owner signs the transaction because access to the vault needed.
    env.ft_transfer_call(
        &env.nft_owner,
        env.vault.id(),
        token_contract_id,
        VAULT_TEST_DEPOSIT,
        &request,
    )
    .await?
    .into_result()?;

    let (_, balance) =
        Environment::ft_balance_of(env.nft_owner.id().clone(), env.fungible_tokens[0].clone())
            .await?;
    assert_eq!(
        balance,
        10 * NEAR,
        "all tokens should be transferred back to NFT owner"
    );

    let balance = env
        .vault_balance_of(0)
        .await?
        .expect("should be some: ft_transfer_call failed");
    let token = balance
        .tokens
        .iter()
        .find(|t| t.contract_id.as_str() == token_contract_id.as_str())
        .expect("should be some");

    assert_eq!(
        token.amount, 0,
        "vault token balance should be equal to amount in Request"
    );

    Ok(())
}

#[tokio::test]
async fn test_ft_on_transfer_request_transfer_existing_vault() -> Result<()> {
    let env = Environment::new(0).await?;

    env.nft_mint_all().await?;
    env.nft_transfer().await?;
    replenish_account_wrap_near(&env.nft_owner, env.fungible_tokens[0].id()).await?;

    let nft_contract_id = near_sdk::AccountId::from_str(env.nft_first().id().as_str())?;
    let request = Request::transfer(NFT_TOKEN_ID_BASE.to_owned(), nft_contract_id);
    let request = to_string(&request)?;

    let token_contract_id = env.fungible_tokens[0].id();

    // In real world this method will be called by an issuer.
    // But for this test it is enough to call just part of the whole XCC chain.
    // So owner signs the transaction because access to the vault needed.
    env.ft_transfer_call(
        &env.nft_owner,
        env.vault.id(),
        token_contract_id,
        VAULT_TEST_DEPOSIT,
        &request,
    )
    .await?
    .into_result()?;

    env.ft_transfer_call(
        &env.nft_owner,
        env.vault.id(),
        token_contract_id,
        VAULT_TEST_DEPOSIT,
        &request,
    )
    .await?
    .into_result()?;

    let (_, balance) =
        Environment::ft_balance_of(env.nft_owner.id().clone(), env.fungible_tokens[0].clone())
            .await?;
    assert_eq!(
        balance,
        10 * NEAR,
        "all tokens should be transferred back to NFT owner"
    );

    let balance = env
        .vault_balance_of(0)
        .await?
        .expect("should be some: ft_transfer_call failed");
    let token = balance
        .tokens
        .iter()
        .find(|t| t.contract_id.as_str() == token_contract_id.as_str())
        .expect("should be some");

    assert_eq!(
        token.amount, 0,
        "vault token balance should be equal to amount in Request"
    );

    Ok(())
}

#[tokio::test]
async fn test_ft_on_transfer_request_top_up_multiple_nft_new_vaults() -> Result<()> {
    let mut env = Environment::new(0).await?;
    env.add_nft_contract().await?;
    info!("second NFT contract added");

    env.nft_mint_all().await?;
    info!("all NFT minted");

    env.nft_transfer_all().await?;
    info!("all NFTs transferred to issuer");

    let token_contract_id = env.fungible_tokens[0].id();

    for i in 0..2 {
        info!("test case NFT index: {i}");

        env.vault_deposit(token_contract_id, i).await?;
        info!("#{i} vault deposit completed");

        let balance = env
            .vault_balance_of(i)
            .await?
            .expect("should be some: vault deposit failed");
        info!("#{i} received vault balance");

        let token = balance
            .tokens
            .iter()
            .find(|t| t.contract_id.as_str() == token_contract_id.as_str())
            .expect("should be some");

        assert_eq!(
            token.amount, VAULT_TEST_DEPOSIT,
            "vault token balance should be equal to amount in Request"
        );
    }

    Ok(())
}
