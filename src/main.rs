use arbiter_core::{environment::builder::EnvironmentBuilder, middleware::RevmMiddleware};
use std::error::Error;

use crate::bindings::counter::Counter;

mod bindings;

const TEST_ENV_LABEL: &str = "test";

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let environment = EnvironmentBuilder::new().label(TEST_ENV_LABEL).build();

    let client_with_signer = RevmMiddleware::new(&environment, None)?;

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
