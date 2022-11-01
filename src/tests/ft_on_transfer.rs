use anyhow::Result;
use near_sdk::{json_types::U128, serde_json::json};
use workspaces::error::ErrorKind;

use super::{
    environment::{format_helpers::format_execution_result, Environment},
    VAULT_TEST_DEPOSIT,
};

#[tokio::test]
async fn test_ft_on_transfer_invalid_msg() -> Result<()> {
    let env = Environment::new().await?;

    let args = json!({
        "receiver_id": env.vault.id(),
        "amount": U128(VAULT_TEST_DEPOSIT),
        "msg": "invalid msg",
    });

    let token_contract_id = env.fungible_tokens[0].id();

    let res = env
        .issuer
        .call(token_contract_id, "ft_transfer_call")
        .args_json(args)
        .deposit(1)
        .max_gas()
        .transact()
        .await?;

    let receipts = res.receipt_failures();
    let failure = receipts
        .iter()
        .find(|r| &r.executor_id == env.vault.id())
        .expect("at least one receipt should fail");

    let error = (**failure).clone().into_result().expect_err("unreachable");

    println!("execution error: {error}");

    assert!(matches!(error.kind(), ErrorKind::Execution));
    Ok(())
}
