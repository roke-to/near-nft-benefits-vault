use anyhow::Result;

use crate::tests::{environment::Environment, NEAR};

#[tokio::test]
async fn test_contract() -> Result<()> {
    let env = Environment::new().await?;

    for contract_id in env.fungible_tokens.iter().map(|c| c.id()) {
        env.deposit_to_vault(contract_id).await?;
        println!("deposit to vault {}: OK", contract_id);
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
