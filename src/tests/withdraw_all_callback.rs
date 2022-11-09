use anyhow::Result;

use crate::tests::environment::Environment;

#[tokio::test]
async fn test_withdraw_all_callback_get_nft_info_failed() -> Result<()> {
    let env = Environment::new().await?;

    let res = env
        .vault_withdraw_all()
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
async fn test_withdraw_all_callback_zero_assets() -> Result<()> {
    let env = Environment::new().await?;
    env.nft_mint().await?;
    env.nft_transfer().await?;

    let res = env
        .vault_withdraw_all()
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
        .contains("vault is not created"));
    Ok(())
}

#[tokio::test]
async fn test_withdraw_all_callback_single_asset() -> Result<()> {
    let env = Environment::new().await?;
    env.nft_mint().await?;
    env.nft_transfer().await?;
    env.vault_deposit(env.fungible_tokens[0].id()).await?;

    let res = env
        .vault_withdraw_all()
        .await?
        .into_result()
        .expect("should succeed");
    assert!(res.failures().is_empty(), "should be not failures");
    Ok(())
}

#[tokio::test]
async fn test_withdraw_all_callback_two_assets() -> Result<()> {
    let env = Environment::new().await?;
    env.nft_mint().await?;
    env.nft_transfer().await?;
    env.vault_deposit(env.fungible_tokens[0].id()).await?;
    env.vault_deposit(env.fungible_tokens[1].id()).await?;

    let res = env
        .vault_withdraw_all()
        .await?
        .into_result()
        .expect("should succeed");
    assert!(res.failures().is_empty(), "should be not failures");
    Ok(())
}
