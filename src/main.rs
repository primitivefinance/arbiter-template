
use std::error::Error;
use arbiter_core::{bindings::arbiter_token::ArbiterToken, manager::SimulationManager};

use crate::bindings::counter::Counter;

mod bindings;

const TEST_ENV_LABEL: &str = "test";
const TEST_AGENT_NAME: &str = "test_agent";
const TEST_ARG_NAME: &str = "test_arg";
const TEST_ARG_SYMBOL: &str = "TEST";
const TEST_ARG_DECIMALS: u8 = 18;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    // Create a new simulation manager
    let mut manager = SimulationManager::new();
    // Add a new environment
    manager.add_environment(TEST_ENV_LABEL.clone().to_string())?;
    manager.run_environment(TEST_ENV_LABEL.clone().to_string())?;
    // Add a new agent to the environment
    let environment = manager.environments.get_mut(TEST_ENV_LABEL).unwrap();

    environment.add_agent(TEST_AGENT_NAME.to_string());
    let agent = environment.clients.get(TEST_AGENT_NAME).unwrap();
    // Deploy a new ArbiterToken contract
    let token = ArbiterToken::deploy(agent.client.clone(), (
            TEST_ARG_NAME.to_string(),
            TEST_ARG_SYMBOL.to_string(),
            TEST_ARG_DECIMALS,
        ),
    )?
    .send().await?;
    println!("Deployed ArbiterToken to address: {}", token.address());

    // deploy counter contract
    let counter = Counter::deploy(agent.client.clone(), ())?.send().await?;
    println!("Deployed Counter to address: {}", counter.address());



    Ok(())
}