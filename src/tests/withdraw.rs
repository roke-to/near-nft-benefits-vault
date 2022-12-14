use anyhow::Result;
use workspaces::Contract;

use crate::tests::{environment::Environment, VAULT_TEST_DEPOSIT};

#[tokio::test]
async fn test_withdraw_single_ft() -> Result<()> {
    let env = Environment::new(1).await?;

    env.nft_mint_all().await?;
    println!("\n<--- nft issued --->\n");

    env.nft_transfer().await?;
    println!("\n<--- nft transferred --->\n");

    let nft_owner_initial_balances = env.all_ft_balances_of(env.nft_owner.id()).await?;
    println!("\n<--- nft owner initial balances: {nft_owner_initial_balances:?} --->\n");

    for contract_id in env.fungible_tokens.iter().map(Contract::id) {
        env.vault_deposit(contract_id, 0).await?;
        println!("\ndeposit to vault of {contract_id}");
    }
    println!("\n<--- deposited to vault --->\n");

    env.vault_withdraw(env.fungible_tokens[1].id(), 0)
        .await?
        .into_result()?;
    println!("\n<--- gathered all benefits --->\n");

    let nft_owner_final_balances = env.all_ft_balances_of(env.nft_owner.id()).await?;
    println!("\n<--- nft owner final balances: {nft_owner_final_balances:?} --->\n");

    assert_eq!(
        nft_owner_initial_balances
            .get(env.fungible_tokens[0].id())
            .unwrap(),
        nft_owner_final_balances
            .get(env.fungible_tokens[0].id())
            .unwrap(),
        "wrap near balances should remain the same"
    );

    assert_eq!(
        *nft_owner_initial_balances
            .get(env.fungible_tokens[1].id())
            .unwrap()
            + VAULT_TEST_DEPOSIT,
        *nft_owner_final_balances
            .get(env.fungible_tokens[1].id())
            .unwrap(),
        "NFT owner balance of custom FT should increase by standard test deposit amount"
    );
    println!("\n<--- nft owner balance checked, test passed --->\n");

    Ok(())
}
