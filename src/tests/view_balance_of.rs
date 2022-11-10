use anyhow::{Context, Result};
use workspaces::Contract;

use crate::tests::NEAR;

use super::environment::Environment;

// Tests `balance_of` veiw method of the Contract.
#[tokio::test]
pub async fn test_view_balance_of_method() -> Result<()> {
    // Initialize test environment.
    let env = Environment::new(0).await?;

    // Issue NFT that will be used as the key to the vault.
    env.nft_mint().await?;

    // Deposit all kinds of existing FTs to the vault.
    for contract_id in env.fungible_tokens.iter().map(Contract::id) {
        env.vault_deposit(contract_id).await?;
        println!("successful deposit to vault of {contract_id}");
    }

    // Call view method to get balances in the vault.
    let balance = env
        .vault_balance_of()
        .await
        .with_context(|| "failed to check balance of")?
        .unwrap();

    println!("balance: {balance:#?}");

    // Check wrap near balance.
    let wrap_near_amount = balance
        .tokens
        .iter()
        .find(|token| token.contract_id.as_str() == env.fungible_tokens[0].id().as_str())
        .expect("wrap near is not registered in vault")
        .amount;

    assert_eq!(wrap_near_amount, NEAR);

    Ok(())
}
