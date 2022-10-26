use anyhow::{Context, Result};

use crate::tests::NEAR;

use super::environment::Environment;

// Tests `balance_of` veiw method of the Contract.
#[tokio::test]
pub async fn test_view_balance_of() -> Result<()> {
    let env = Environment::new().await?;

    env.issue_nft().await?;

    env.deposit_to_vault().await?;

    let balance = env
        .vault_balance_of()
        .await
        .with_context(|| "failed to check balance of")?
        .unwrap();

    println!("balance: {balance:#?}");

    let wrap_near_amount = balance
        .tokens
        .iter()
        .find(|token| token.account_id.as_str() == env.wrap_near.id().as_str())
        .expect("wrap near is not registered in vault")
        .amount;

    assert_eq!(wrap_near_amount, NEAR);

    Ok(())
}
