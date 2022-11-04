use anyhow::Result;
use log::info;
use workspaces::Contract;

use crate::tests::{environment::Environment, NEAR, NFT_TOKEN_ID};

#[tokio::test]
pub async fn test_contract() -> Result<()> {
    let env = Environment::new().await?;

    for contract_id in env.fungible_tokens.iter().map(Contract::id) {
        env.deposit_to_vault(contract_id).await?;
        println!("deposit to vault {contract_id}: OK",);
    }

    check_vault_state(&env).await?;
    println!("check vault state: OK");

    Ok(())
}

async fn check_vault_state(env: &Environment) -> Result<()> {
    println!("nft_contract_id: {}", env.nft.id());
    let balance = env.vault_balance_of().await?.unwrap();
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
    let mut env = Environment::new().await?;
    env.deploy_replenisher().await?;
    env.issue_nft().await?;
    env.transfer_nft().await?;

    env.ft_transfer_call(
        &env.issuer,
        env.replenisher.as_ref().unwrap().id(),
        &env.fungible_tokens[0],
    )
    .await?;

    let replenishers = env.view_replenishers().await?.unwrap();
    assert!(replenishers.len() == 1);
    assert!(
        replenishers[0].contract_id().as_str() == env.replenisher.as_ref().unwrap().id().as_str()
    );

    let (token_before, balance_before) =
        Environment::ft_balance_of(env.nft_owner.id().clone(), env.fungible_tokens[0].clone())
            .await?;
    info!("balance of NFT owner BEFORE withdrawal: {balance_before} of tokens {token_before}");

    let vault_balance = env.vault_balance_of().await?.unwrap();
    assert_eq!(
        vault_balance.nft_id.contract_id().as_str(),
        env.nft.id().as_str(),
        "NFT contracts don't match"
    );
    assert_eq!(
        vault_balance.nft_id.token_id(),
        NFT_TOKEN_ID,
        "NFT TokenIds don't match"
    );
    assert!(vault_balance.tokens.is_empty(), "vault should be empty");

    // @TODO withdraw process
    env.withdraw(env.fungible_tokens[0].id()).await?;

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
