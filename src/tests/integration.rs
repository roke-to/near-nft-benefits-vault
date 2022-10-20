use anyhow::Result;

use crate::tests::{environment::Environment, NEAR};

#[tokio::test]
async fn test_contract() -> Result<()> {
    let env = Environment::new().await?;

    env.deposit_to_vault().await?;
    println!("deposit to vault: OK");

    check_vault_state(&env).await?;
    println!("check vault state: OK");

    Ok(())
}

async fn check_vault_state(env: &Environment) -> Result<()> {
    let balance = env.balance_of().await?;
    assert_eq!(
        balance
            .tokens
            .iter()
            .find(|t| t.account_id.as_str() == env.wrap_near.id().as_str())
            .expect("wrap near is not registered in the vault")
            .amount,
        2 * NEAR
    );
    Ok(())
}
