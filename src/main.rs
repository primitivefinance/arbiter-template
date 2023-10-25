use anyhow::{Ok, Result};
use arbiter_core::{
    bindings::weth,
    environment::{builder::EnvironmentBuilder, fork::Fork},
    middleware::RevmMiddleware,
};
use ethers::{providers::Middleware, types::U256};

use crate::bindings::counter::Counter;

#[allow(unused_imports)]
mod bindings;

const FORK_PATH: &str = "fork_example/test.json";

#[tokio::main]
pub async fn main() -> Result<()> {
    counter_example().await?;
    load_fork_from_disk().await?;
    Ok(())
}

pub async fn counter_example() -> Result<()> {
    let environment = EnvironmentBuilder::new().build();

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

pub async fn load_fork_from_disk() -> Result<()> {
    let fork = Fork::from_disk(FORK_PATH).unwrap();

    // Get the environment going
    let environment = EnvironmentBuilder::new().db(fork.db).build();

    let vitalik_address = fork.eoa.get("vitalik").unwrap();
    let vitalik_as_a_client = RevmMiddleware::new_from_forked_eoa(&environment, *vitalik_address);
    assert!(vitalik_as_a_client.is_ok());
    let vitalik_as_a_client = vitalik_as_a_client.unwrap();

    // test a state mutating call from the forked eoa
    let weth = weth::WETH::deploy(vitalik_as_a_client.clone(), ())
        .unwrap()
        .send()
        .await;
    assert!(weth.is_ok()); // vitalik deployed the weth contract

    // test a non mutating call from the forked eoa
    let eth_balance = vitalik_as_a_client
        .get_balance(*vitalik_address, None)
        .await
        .unwrap();
    assert_eq!(eth_balance, U256::from(934034962177715175765_u128));
    Ok(())
}

// Create a client
// let client = RevmMiddleware::new(&environment, Some("name")).unwrap();

// // Deal with the weth contract
// let weth_meta = fork.contracts_meta.get("weth").unwrap();
// let weth = weth::WETH::new(weth_meta.address, client.clone());

// let address_to_check_balance =
//     Address::from_str(&weth_meta.mappings.get("balanceOf").unwrap()[0]).unwrap();

// println!("checking address: {}", address_to_check_balance);
// let balance = weth
//     .balance_of(address_to_check_balance)
//     .call()
//     .await
//     .unwrap();
// assert_eq!(balance, U256::from(34890707020710109111_u128));

// // eoa check
// let eoa = fork.eoa.get("vitalik").unwrap();
// let eth_balance = client.get_balance(*eoa, None).await.unwrap();
// // Check the balance of the eoa with the load cheatcode
// assert_eq!(eth_balance, U256::from(934034962177715175765_u128));
