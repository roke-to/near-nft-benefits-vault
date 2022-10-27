use anyhow::Result;

use crate::tests::{environment::Environment, VAULT_TEST_DEPOSIT};

// Tests withdrawal of all available tokens in the vault.
#[tokio::test]
async fn test_withdraw_all() -> Result<()> {
    let env = Environment::new().await?;
    println!("\n<--- test environment initialized --->\n");

    let nft_owner_initial_balances = env.all_ft_balances_of(env.nft_owner.id()).await?;
    println!("\n<--- nft owner initial balances: {nft_owner_initial_balances:?} --->\n");

    env.issue_nft().await?;
    println!("\n<--- nft issued --->\n");

    env.transfer_nft().await?;
    println!("\n<--- nft transferred --->\n");

    for contract_id in env.fungible_tokens.iter().map(|c| c.id()) {
        env.deposit_to_vault(contract_id).await?;
        println!("\ndeposit to vault of {}", contract_id);
    }
    println!("\n<--- deposited to vault --->\n");

    env.withdraw_all().await?;
    println!("\n<--- gathered all benefits --->\n");

    let nft_owner_final_balances = env.all_ft_balances_of(env.nft_owner.id()).await?;
    println!("\n<--- nft owner final balances: {nft_owner_final_balances:?} --->\n");

    for (token, initial_balance) in nft_owner_initial_balances.iter() {
        let final_balance = nft_owner_final_balances
            .get(token)
            .expect("token not found");
        assert_eq!(
            *initial_balance + VAULT_TEST_DEPOSIT,
            *final_balance,
            "NFT owner balance should increase by standard test deposit amount"
        );
    }
    println!("\n<--- nft owner balance checked, test passed --->\n");

    Ok(())
}
