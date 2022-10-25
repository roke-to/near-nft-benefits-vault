use anyhow::{Context, Result};
use tests_integration::{Environment, NEAR};

#[tokio::main]
pub async fn main() -> Result<()> {
    let env = Environment::new().await?;

    env.issue_nft().await?;

    env.deposit_to_vault().await?;

    let balance = env
        .balance_of()
        .await
        .with_context(|| "failed to check balance of")?;

    println!("balance: {balance:#?}");

    assert_eq!(
        balance
            .tokens
            .iter()
            .find(|token| token.account_id.as_str() == env.wrap_near.id().as_str())
            .expect("wrap near is not registered in vault")
            .amount,
        2 * NEAR
    );

    Ok(())
}
