use arbiter_core::{
    environment::{BlockSettings, EnvironmentParameters, GasSettings},
    manager::Manager,
    middleware::RevmMiddleware,
};
use std::{error::Error, sync::Arc};

use crate::bindings::counter::Counter;

mod bindings;

const TEST_ENV_LABEL: &str = "test";

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let mut manager = Manager::new();
    let _ = manager.add_environment(EnvironmentParameters {
        label: TEST_ENV_LABEL.to_owned(),
        block_settings: BlockSettings::UserControlled,
        gas_settings: GasSettings::UserControlled,
    });
    manager.start_environment(TEST_ENV_LABEL)?;

    let client_with_signer = Arc::new(
        RevmMiddleware::new(manager.environments.get(TEST_ENV_LABEL).unwrap(), None).unwrap(),
    );
    println!(
        "created client with address {:?}",
        client_with_signer.address()
    );

    let counter = Counter::deploy(client_with_signer.clone(), ())?
        .send()
        .await?;
    println!("Counter contract deployed at {:?}", counter.address());

    for index in 0..10 {
        let _ = counter.increment().send().await?.await?;
        println!("Counter incremented to {}", index + 1);
    }
    // post state mutation call to show that the state has changed with send
    let count = counter.number().call().await?;
    println!("Counter count is {}", count);

    Ok(())
}
