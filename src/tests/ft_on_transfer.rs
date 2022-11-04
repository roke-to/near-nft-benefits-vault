use std::str::FromStr;

use anyhow::Result;
use near_sdk::{
    json_types::U128,
    serde_json::{json, to_string, to_vec},
};
use workspaces::error::ErrorKind;

use crate::{
    interface::request::Request,
    tests::{
        environment::{setup::replenish_account_wrap_near, Environment},
        FT_BALANCE_OF_CALL, NEAR, NFT_TOKEN_ID, VAULT_TEST_DEPOSIT,
    },
};

#[tokio::test]
async fn test_ft_on_transfer_invalid_msg() -> Result<()> {
    let env = Environment::new().await?;

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
async fn test_ft_on_transfer_request_top_up() -> Result<()> {
    let env = Environment::new().await?;

    let nft_contract_id = near_sdk::AccountId::from_str(env.nft.id().as_str())?;
    let request = Request::top_up(NFT_TOKEN_ID.to_owned(), nft_contract_id);
    let request = to_string(&request)?;

    let token_contract_id = env.fungible_tokens[0].id();

    env.ft_transfer_call(
        &env.issuer,
        env.vault.id(),
        token_contract_id,
        VAULT_TEST_DEPOSIT,
        &request,
    )
    .await?
    .into_result()?;

    let balance = env
        .vault_balance_of()
        .await?
        .expect("should be some: ft_transfer_call failed");
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
async fn test_ft_on_transfer_request_transfer() -> Result<()> {
    let env = Environment::new().await?;

    env.nft_mint().await?;
    env.nft_transfer().await?;
    replenish_account_wrap_near(&env.nft_owner, env.fungible_tokens[0].id()).await?;

    let nft_contract_id = near_sdk::AccountId::from_str(env.nft.id().as_str())?;
    let request = Request::transfer(NFT_TOKEN_ID.to_owned(), nft_contract_id);
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

    let args = to_vec(&json!({
        "account_id": env.nft_owner.id(),
    }))?;
    let token_balance_of: U128 = env.fungible_tokens[0]
        .view(FT_BALANCE_OF_CALL, args)
        .await?
        .json()?;

    assert_eq!(
        token_balance_of.0,
        10 * NEAR,
        "all tokens should be transferred back to NFT owner"
    );

    let balance = env
        .vault_balance_of()
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
