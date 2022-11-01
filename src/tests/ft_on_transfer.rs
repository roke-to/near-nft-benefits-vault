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
        environment::{
            format_helpers::format_execution_result, setup::replenish_account_wrap_near,
        },
        FT_BALANCE_OF_CALL, NEAR, NFT_TOKEN_ID,
    },
};

use super::{environment::Environment, VAULT_TEST_DEPOSIT};

#[tokio::test]
async fn test_ft_on_transfer_invalid_msg() -> Result<()> {
    let env = Environment::new().await?;

    let args = json!({
        "receiver_id": env.vault.id(),
        "amount": U128(VAULT_TEST_DEPOSIT),
        "msg": "invalid msg",
    });

    let token_contract_id = env.fungible_tokens[0].id();

    let res = env
        .issuer
        .call(token_contract_id, "ft_transfer_call")
        .args_json(args)
        .deposit(1)
        .max_gas()
        .transact()
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
    let req = Request::top_up(NFT_TOKEN_ID.to_owned(), nft_contract_id);
    let req = to_string(&req)?;

    let args = json!({
        "receiver_id": env.vault.id(),
        "amount": U128(VAULT_TEST_DEPOSIT),
        "msg": req,
    });

    let token_contract_id = env.fungible_tokens[0].id();

    let res = env
        .issuer
        .call(token_contract_id, "ft_transfer_call")
        .args_json(args)
        .deposit(1)
        .max_gas()
        .transact()
        .await?;
    println!("top up res: {}", format_execution_result(&res));

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

    env.issue_nft().await?;
    env.transfer_nft().await?;
    replenish_account_wrap_near(&env.nft_owner, env.fungible_tokens[0].id()).await?;

    let nft_contract_id = near_sdk::AccountId::from_str(env.nft.id().as_str())?;
    let req = Request::transfer(NFT_TOKEN_ID.to_owned(), nft_contract_id);
    let req = to_string(&req)?;

    let args = json!({
        "receiver_id": env.vault.id(),
        "amount": U128(VAULT_TEST_DEPOSIT),
        "msg": req,
    });

    let token_contract_id = env.fungible_tokens[0].id();

    // In real world this method will be called by an issuer.
    // But for this test it is enough to call just part of the whole XCC chain.
    // So owner signs the transaction because access to the vault needed.
    let res = env
        .nft_owner
        .call(token_contract_id, "ft_transfer_call")
        .args_json(args)
        .deposit(1)
        .max_gas()
        .transact()
        .await?;
    println!("transfer res: {res:#?}");

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
