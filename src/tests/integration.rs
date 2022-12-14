use anyhow::Result;
use log::{debug, info};
use near_sdk::serde_json::to_string;
use workspaces::Contract;

use crate::{
    interface::request::Request,
    tests::{
        environment::{
            args::{
                add_replenishment_callback_str, replenisher_ft_on_transfer_request_str,
                replenisher_withdraw_str,
            },
            Environment,
        },
        NEAR, NFT_TOKEN_ID_BASE,
    },
};

#[tokio::test]
pub async fn test_contract() -> Result<()> {
    let env = Environment::new(0).await?;

    for contract_id in env.fungible_tokens.iter().map(Contract::id) {
        env.vault_deposit(contract_id, 0).await?;
        println!("deposit to vault {contract_id}: OK",);
    }

    check_vault_state(&env).await?;
    println!("check vault state: OK");

    Ok(())
}

async fn check_vault_state(env: &Environment) -> Result<()> {
    println!("nft_contract_id: {}", env.nft_first().id());
    let balance = env.vault_balance_of(0).await?.unwrap();
    assert_eq!(
        balance
            .tokens
            .iter()
            .find(|t| t.contract_id.as_str() == env.fungible_tokens[0].id().as_str())
            .expect("wrap near is not registered in the vault")
            .amount,
        NEAR
    );
    Ok(())
}

#[tokio::test]
pub async fn test_interaction_with_contract_replenisher() -> Result<()> {
    let mut env = Environment::new(0).await?;
    env.deploy_replenishers(1).await?;
    env.nft_mint_all().await?;
    env.nft_transfer().await?;

    let (token_account, balance) =
        Environment::ft_balance_of(env.issuer.id().clone(), env.fungible_tokens[0].clone()).await?;
    debug!("ft_balance: \n\ttoken: {token_account},\n\tbalance: {balance}");

    let amount = NEAR;
    let nft_id = format!("{NFT_TOKEN_ID_BASE}0");
    let req = Request::transfer(nft_id, env.nft_first().id().as_str().parse()?);

    let transfer_req = to_string(&req)?;

    let args = replenisher_withdraw_str(&transfer_req)?;

    let args = add_replenishment_callback_str(env.nft_first().id(), &args, 0)?;

    let msg = replenisher_ft_on_transfer_request_str(env.vault.id(), &args)?;

    env.ft_transfer_call(
        &env.issuer,
        env.replenishers[0].id(),
        env.fungible_tokens[0].id(),
        amount,
        &msg,
    )
    .await?
    .into_result()?;

    let replenishers = env.vault_view_replenishers(0).await?.unwrap();
    assert!(replenishers.len() == 1);
    assert!(replenishers[0].contract_id().as_str() == env.replenishers[0].id().as_str());

    let (token_before, balance_before) =
        Environment::ft_balance_of(env.nft_owner.id().clone(), env.fungible_tokens[0].clone())
            .await?;
    info!("balance of NFT owner BEFORE withdrawal: {balance_before} of tokens {token_before}");

    let vault_balance = env.vault_balance_of(0).await?.unwrap();
    assert_eq!(
        vault_balance.nft_id.contract_id().as_str(),
        env.nft_first().id().as_str(),
        "NFT contracts don't match"
    );
    assert_eq!(
        vault_balance.nft_id.token_id(),
        format!("{NFT_TOKEN_ID_BASE}0"),
        "NFT TokenIds don't match"
    );
    assert!(vault_balance.tokens.is_empty(), "vault should be empty");

    info!("calling vault.withdraw()");
    env.vault_withdraw(env.fungible_tokens[0].id(), 0)
        .await?
        .into_result()?;

    let (token_after, balance_after) =
        Environment::ft_balance_of(env.nft_owner.id().clone(), env.fungible_tokens[0].clone())
            .await?;
    info!("balance of NFT owner AFTER withdrawal: {balance_after} of tokens {token_after}");

    assert_eq!(token_after, token_before, "tokens mismatch");
    assert_eq!(
        balance_after,
        balance_before + NEAR,
        "balance should increase by 1 NEAR"
    );
    Ok(())
}
