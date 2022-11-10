use anyhow::Result;
use workspaces::Contract;

use crate::tests::{environment::Environment, VAULT_TEST_DEPOSIT};

// Tests withdrawal of all available tokens in the vault.
#[tokio::test]
async fn test_withdraw_all() -> Result<()> {
    let env = Environment::new(2).await?;
    println!("\n<--- test environment initialized --->\n");

    env.nft_mint().await?;
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

    env.vault_withdraw_all()
        .await?
        .into_result()
        .expect("withdraw failed");
    println!("\n<--- gathered all benefits --->\n");

    let nft_owner_final_balances = env.all_ft_balances_of(env.nft_owner.id()).await?;
    println!("\n<--- nft owner final balances: {nft_owner_final_balances:?} --->\n");

    for (token, initial_balance) in &nft_owner_initial_balances {
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
