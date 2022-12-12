use anyhow::Result;
use workspaces::Contract;

use crate::tests::{environment::Environment, VAULT_TEST_DEPOSIT};

#[tokio::test]
async fn test_withdraw_amount_single_ft() -> Result<()> {
    let env = Environment::new(0).await?;
    println!("\n<--- test environment initialized --->\n");

    env.nft_mint_all().await?;
    println!("\n<--- nft issued --->\n");

    env.nft_transfer().await?;
    println!("\n<--- nft transferred --->\n");

    let nft_owner_initial_balances = env.all_ft_balances_of(env.nft_owner.id()).await?;
    println!("\n<--- nft owner initial balances: {nft_owner_initial_balances:?} --->\n");

    for contract_id in env.fungible_tokens.iter().map(Contract::id) {
        env.vault_deposit(contract_id).await?;
        println!("\ndeposit to vault of {contract_id}");
    }
    println!("\n<--- deposited to vault --->\n");

    env.vault_withdraw_amount(env.fungible_tokens[1].id(), VAULT_TEST_DEPOSIT / 2)
        .await?;
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
            + VAULT_TEST_DEPOSIT / 2,
        *nft_owner_final_balances
            .get(env.fungible_tokens[1].id())
            .unwrap(),
        "NFT owner balance of custom FT should increase by the half of standard test deposit amount"
    );

    let vault_balance = env.vault_balance_of().await?.expect("vault should exist");

    let token = vault_balance
        .tokens
        .iter()
        .find(|t| t.contract_id.as_str() == env.fungible_tokens[1].id().as_str())
        .expect("custom ft should exist");

    assert_eq!(
        token.amount,
        VAULT_TEST_DEPOSIT / 2,
        "half of the standard test deposit should remain in the vault"
    );

    println!("\n<--- nft owner balance checked, test passed --->\n");

    Ok(())
}
