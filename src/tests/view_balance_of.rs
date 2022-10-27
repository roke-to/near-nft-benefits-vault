use anyhow::{Context, Result};

use crate::tests::NEAR;

use super::environment::Environment;

// Tests `balance_of` veiw method of the Contract.
#[tokio::test]
pub async fn test_view_balance_of() -> Result<()> {
    let env = Environment::new().await?;

    env.issue_nft().await?;

    for contract_id in env.fungible_tokens.iter().map(|c| c.id()) {
        env.deposit_to_vault(contract_id).await?;
        println!("successful deposit to vault of {}", contract_id);
    }

    let balance = env
        .vault_balance_of()
        .await
        .with_context(|| "failed to check balance of")?
        .unwrap();

    println!("balance: {balance:#?}");

    let wrap_near_amount = balance
        .tokens
        .iter()
        .find(|token| token.account_id.as_str() == env.fungible_tokens[0].id().as_str())
        .expect("wrap near is not registered in vault")
        .amount;

    assert_eq!(wrap_near_amount, NEAR);

    Ok(())
}
