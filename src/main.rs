
use std::{error::Error, env};
use arbiter_core::{bindings::arbiter_token::ArbiterToken, manager::SimulationManager, agent::Agent};

use crate::bindings::counter::Counter;

mod bindings;
mod behaviors;

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
    manager.configure_lambda(100.00, TEST_ENV_LABEL.clone().to_string())?;
    manager.run_environment(TEST_ENV_LABEL.clone().to_string())?;
    // Add a new agent to the environment
    manager.add_agent(TEST_AGENT_NAME.to_string(), TEST_ENV_LABEL.clone().to_string())?;

    // TODO Need to create a new agent and add it to the environment
    // maybe something like this: 
    // let bob = Agent::new("name".to_owned(), manager.environments.get(&TEST_ENV_LABEL.clone().to_string()));
    // bob.add_behavior("test".to_owned(), behaviors::TestBehavior::new());
    // Probably read up on artemis


    // Deploy a new ArbiterToken contract
    // let token = ArbiterToken::deploy(agent.client.clone(), (
    //         TEST_ARG_NAME.to_string(),
    //         TEST_ARG_SYMBOL.to_string(),
    //         TEST_ARG_DECIMALS,
    //     ),
    // )?
    // .send().await?;
    // println!("Deployed ArbiterToken to address: {}", token.address());

    // // deploy counter contract
    // let counter = Counter::deploy(agent.client.clone(), ())?.send().await?;
    // println!("Deployed Counter to address: {}", counter.address());
    Ok(())
}