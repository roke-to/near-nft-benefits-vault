use anyhow::Result;

use super::{environment::Environment, VAULT_REPLENISH_ARGS, VAULT_REPLENISH_CALLBACK};

#[tokio::test]
async fn test_add_replenisher() -> Result<()> {
    let env = Environment::new(0).await?;

    env.vault_add_test_replenisher().await?;

    let replenishers = env
        .vault_view_replenishers(0)
        .await?
        .expect("must be some, because vault is created");

    assert!(!replenishers.is_empty(), "replenisher wasn't added");
    println!("replenishers are not empty");

    assert_eq!(
        replenishers[0].contract_id().as_str(),
        env.issuer.id().as_str(),
        "issuer must be registered as replenisher"
    );
    println!("replenisher contract id is correct");

    assert_eq!(
        replenishers[0].callback(),
        VAULT_REPLENISH_CALLBACK,
        "wrong callback"
    );
    println!("replenisher callback is correct");

    assert_eq!(replenishers[0].args(), VAULT_REPLENISH_ARGS, "wrong args");
    println!("replenisher args are correct");

    Ok(())
}
