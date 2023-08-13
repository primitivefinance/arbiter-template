use arbiter_core::manager::Manager;
use arbiter_core::middleware::RevmMiddleware;
use ethers::providers::Middleware;
use std::{error::Error, sync::Arc};

use crate::bindings::counter::Counter;

mod bindings;

const TEST_ENV_LABEL: &str = "test";

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let mut manager = Manager::new();
    let _ = manager.add_environment(TEST_ENV_LABEL, 1.0, 1);

    let client_with_signer = Arc::new(RevmMiddleware::new(
        manager.environments.get(TEST_ENV_LABEL).unwrap(),
    ));
    println!("created client with address {}", client_with_signer.default_sender().unwrap());
    manager.start_environment(TEST_ENV_LABEL)?;

    let counter = Counter::deploy(client_with_signer.clone(), ())
        .unwrap()
        .send()
        .await
        .unwrap();
    println!("Counter contract deployed at {}", counter.address());

    for index in 0..10 {
        counter.increment().call().await.unwrap();
        println!("Counter incremented to {}", index);
    }

    Ok(())
}
