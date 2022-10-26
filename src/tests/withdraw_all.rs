use anyhow::Result;

use crate::tests::{environment::Environment, VAULT_TEST_DEPOSIT};

#[tokio::test]
async fn test_withdraw_all() -> Result<()> {
    let env = Environment::new().await?;
    println!("\n<--- test environment initialized --->\n");

    let nft_owner_initial_balance = env.wrap_near_ft_balance_of(env.nft_owner.id()).await?;
    println!("\n<--- nft owner initial balance: {nft_owner_initial_balance} --->\n");

    env.issue_nft().await?;
    println!("\n<--- nft issued --->\n");

    env.transfer_nft().await?;
    println!("\n<--- nft transferred --->\n");

    env.deposit_to_vault().await?;
    println!("\n<--- deposited to vault --->\n");

    env.withdraw_all().await?;
    println!("\n<--- gathered all benefits --->\n");

    let nft_owner_final_balance = env.wrap_near_ft_balance_of(env.nft_owner.id()).await?;
    println!("\n<--- nft owner final balance: {nft_owner_final_balance} --->\n");

    assert_eq!(
        nft_owner_initial_balance + VAULT_TEST_DEPOSIT,
        nft_owner_final_balance,
        "NFT owner balance should increase by standard test deposit amount"
    );
    println!("\n<--- nft owner balance checked, test passed --->\n");

    Ok(())
}
