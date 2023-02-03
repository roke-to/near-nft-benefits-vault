use std::time::Duration;

use anyhow::Result;
use log::info;
use tokio::time::sleep;

use crate::tests::{environment::Environment, NEAR};

#[tokio::test]
async fn test_withdraw_callback_get_nft_info_failed() -> Result<()> {
    let env = Environment::new(0).await?;

    let res = env
        .vault_withdraw(env.fungible_tokens[0].id(), 0)
        .await?
        .into_result()
        .expect_err("should fail");
    let failure = res.failures()[0];
    assert_eq!(
        &failure.executor_id,
        env.vault.id(),
        "vault should panic in callback"
    );

    let error = failure.clone().into_result().expect_err("should be error");

    let error = format!("{error:?}");

    assert!(error.contains("NFT info query returned nothing"));

    Ok(())
}

#[tokio::test]
async fn test_withdraw_callback_no_replenishers_zero_assets() -> Result<()> {
    let env = Environment::new(0).await?;
    env.nft_mint_all().await?;
    env.nft_transfer().await?;

    let res = env
        .vault_withdraw(env.fungible_tokens[0].id(), 0)
        .await?
        .into_result()
        .expect_err("should fail");
    let failure = res.failures()[0];
    assert_eq!(
        &failure.executor_id,
        env.vault.id(),
        "vault should panic in callback"
    );

    let error = failure.clone().into_result().expect_err("should be error");

    let error = format!("{error:?}");

    assert!(
        error.contains("vault is not created"),
        "expected a specific log message"
    );

    Ok(())
}

async fn withdraw_callback_with_assets_impl(custom_ft_count: usize) -> Result<()> {
    let env = Environment::new(custom_ft_count).await?;
    env.nft_mint_all().await?;
    env.nft_transfer().await?;
    for token in env.fungible_tokens.iter().map(|t| t.id()) {
        env.vault_deposit(token, 0).await?;
    }

    let res = env
        .vault_withdraw(env.fungible_tokens[custom_ft_count].id(), 0)
        .await?
        .into_result()
        .expect("should succeed");
    assert!(res.failures().is_empty(), "should be no failures");
    Ok(())
}

#[tokio::test]
async fn test_withdraw_callback_no_replenishers_single_asset() -> Result<()> {
    withdraw_callback_with_assets_impl(0).await
}

#[tokio::test]
async fn test_withdraw_callback_no_replenishers_two_assets() -> Result<()> {
    withdraw_callback_with_assets_impl(1).await
}

#[tokio::test]
async fn test_withdraw_callback_no_replenishers_ten_assets() -> Result<()> {
    withdraw_callback_with_assets_impl(9).await
}

async fn withdraw_callback_single_asset_impl(replenishers_count: usize) -> Result<()> {
    let mut env = Environment::new(0).await?;
    env.nft_mint_all().await?;
    env.nft_transfer().await?;

    env.deploy_replenishers(replenishers_count).await?;
    env.top_up_replenishers(env.fungible_tokens[0].id(), NEAR, 0)
        .await?;

    env.vault_deposit(env.fungible_tokens[0].id(), 0).await?;

    let res = env
        .vault_withdraw(env.fungible_tokens[0].id(), 0)
        .await?
        .into_result()
        .expect("should succeed");
    assert!(res.failures().is_empty(), "should be no failures");

    Ok(())
}

#[tokio::test]
async fn test_withdraw_callback_single_replenisher_with_expiration() -> Result<()> {
    let mut env = Environment::new(0).await?;
    env.nft_mint_all().await?;
    env.nft_transfer().await?;

    env.deploy_replenishers(1).await?;

    let token = env.fungible_tokens[0].id();
    env.top_up_replenishers(token, NEAR, 0).await?;

    env.vault_deposit(token, 0).await?;

    info!("wait for 1 sec");
    sleep(Duration::from_secs(1)).await;

    let res = env
        .vault_withdraw(env.fungible_tokens[0].id(), 0)
        .await?
        .into_result()
        .expect("should succeed");
    assert!(res.failures().is_empty(), "should be no failures");

    let replenishers_count = env.vault_view_replenishers(0).await?.unwrap().len();
    assert!(replenishers_count == 0, "replenisher should expire");

    Ok(())
}

#[tokio::test]
async fn test_withdraw_callback_single_replenisher_single_asset() -> Result<()> {
    withdraw_callback_single_asset_impl(1).await
}

// #[tokio::test]
// async fn test_withdraw_callback_two_replenishers_single_asset() -> Result<()> {
//     withdraw_callback_single_asset_impl(2).await
// }

// #[tokio::test]
// async fn test_withdraw_callback_ten_replenishers_single_asset() -> Result<()> {
//     withdraw_callback_single_asset_impl(10).await
// }
