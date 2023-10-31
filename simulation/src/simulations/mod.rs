use super::*;

use v2_core_bindings::uniswap_v2_factory::UniswapV2Factory;
const FORK_PATH: &str = "../../fork_example/test.json";

// This is an example of deploying a contract and then mutating its state
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

// This is an example of loading an eoa from disk and then using it to deploy a contract
// Note: you need to have forked state written to disk with arbiter fork command
pub async fn load_eoa_from_disk() -> Result<()> {
    let fork = Fork::from_disk(FORK_PATH).unwrap();

    // Get the environment going
    let environment = EnvironmentBuilder::new().db(fork.db).build();

    // grab vitaliks address and create a client with it
    let vitalik_address = fork.eoa.get("vitalik").unwrap();
    let vitalik_as_a_client = RevmMiddleware::new_from_forked_eoa(&environment, *vitalik_address)?;

    // test a state mutating call from the forked eoa
    let weth = weth::WETH::deploy(vitalik_as_a_client.clone(), ())
        .unwrap()
        .send()
        .await?;
    println!("vitalik deployed the weth contract as {:?}", weth.address());

    // test a non mutating call from the forked eoa
    let eth_balance = vitalik_as_a_client
        .get_balance(*vitalik_address, None)
        .await
        .unwrap();
    println!("vitalik has {} eth", eth_balance);
    Ok(())
}

// This is an example of loading a contract from disk with it's state
// Note: you need to have forked state written to disk with arbiter fork command
pub async fn load_contract_from_fork() -> Result<()> {
    // Create fork from
    let fork = Fork::from_disk(FORK_PATH)?;

    // Get the environment going
    let environment = EnvironmentBuilder::new().db(fork.db).build();

    // Create a client
    let client = RevmMiddleware::new(&environment, Some("name"))?;

    // Deal with the weth contract
    let weth_meta = fork.contracts_meta.get("weth").unwrap();
    let weth = weth::WETH::new(weth_meta.address, client.clone());

    let address_to_check_balance =
        Address::from_str(&weth_meta.mappings.get("balanceOf").unwrap()[0])?;

    println!("checking address: {}", address_to_check_balance);
    let balance = weth.balance_of(address_to_check_balance).call().await?;
    println!("balance is {}", balance);
    Ok(())
}


pub async fn uniswap_example() -> Result<()> {
    let environment = EnvironmentBuilder::new().build();

    let client_with_signer = RevmMiddleware::new(&environment, None)?;

    println!(
        "created client with address {:?}",
        client_with_signer.address()
    );

    let uniswap_factory = UniswapV2Factory::deploy(client_with_signer.clone(), client_with_signer.address())?
        .send()
        .await?;
    println!("UniswapFactory contract deployed at {:?}", uniswap_factory.address());

    Ok(())
}