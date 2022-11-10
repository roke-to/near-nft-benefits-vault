use anyhow::Result;

use crate::tests::environment::Environment;

#[tokio::test]
async fn test_withdraw_callback_get_nft_info_failed() -> Result<()> {
    let env = Environment::new(0).await?;

    let res = env
        .vault_withdraw(env.fungible_tokens[0].id())
        .await?
        .into_result()
        .expect_err("should fail");
    let failure = res.failures()[0];
    assert_eq!(
        &failure.executor_id,
        env.vault.id(),
        "vault should panic in callback"
    );

    assert!(failure
        .clone()
        .into_result()
        .expect_err("should be error")
        .to_string()
        .contains("NFT info query returned nothing"));

    Ok(())
}

#[tokio::test]
async fn test_withdraw_callback_no_replenishers_zero_assets() -> Result<()> {
    let env = Environment::new(0).await?;
    env.nft_mint().await?;
    env.nft_transfer().await?;

    let res = env
        .vault_withdraw(env.fungible_tokens[0].id())
        .await?
        .into_result()
        .expect_err("should fail");
    let failure = res.failures()[0];
    assert_eq!(
        &failure.executor_id,
        env.vault.id(),
        "vault should panic in callback"
    );

    assert!(
        failure
            .clone()
            .into_result()
            .expect_err("should be error")
            .to_string()
            .contains("vault is not created"),
        "expected a specific log message"
    );

    Ok(())
}

async fn withdraw_callback_with_assets_impl(custom_ft_count: usize) -> Result<()> {
    let env = Environment::new(custom_ft_count).await?;
    env.nft_mint().await?;
    env.nft_transfer().await?;
    for token in env.fungible_tokens.iter().map(|t| t.id()) {
        env.vault_deposit(token).await?;
    }

    let res = env
        .vault_withdraw(env.fungible_tokens[custom_ft_count].id())
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

#[tokio::test]
async fn test_withdraw_callback_single_replenisher_single_asset() -> Result<()> {
    let mut env = Environment::new(0).await?;
    env.nft_mint().await?;
    env.nft_transfer().await?;

    env.deploy_replenisher().await?;
    env.vault_add_replenisher().await?;

    env.vault_deposit(env.fungible_tokens[0].id()).await?;

    let res = env
        .vault_withdraw(env.fungible_tokens[0].id())
        .await?
        .into_result()
        .expect("should succeed");
    assert!(res.failures().is_empty(), "should be no failures");

    Ok(())
}
