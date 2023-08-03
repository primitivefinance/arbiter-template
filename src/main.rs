
use std::{error::Error};
use arbiter_core::{manager::SimulationManager, agent::{Agent}};

use crate::bindings::counter::Counter;

mod bindings;
mod behaviors;

const TEST_ENV_LABEL: &str = "test";
const TEST_AGENT_NAME: &str = "bob";
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
    // manager.add_agent(TEST_AGENT_NAME.to_string(), TEST_ENV_LABEL.clone().to_string())?;
    let bob = Agent::new(TEST_AGENT_NAME.to_string());
    // let deploy_behavior = behaviors::DeployBehavior::new("deploy".to_string());
    // bob.add_behavior(deploy_behavior);

    manager.add_agent(bob, TEST_ENV_LABEL.clone().to_string())?;

    let bob = manager.environments.get_mut(&TEST_ENV_LABEL.clone().to_string()).unwrap().agents.get(0).unwrap();

    let counter = Counter::deploy(bob.client.clone(), ())?.send().await?;

    println!("Deployed Counter to address: {}", counter.address());

    manager.environments.get_mut(&TEST_ENV_LABEL.clone().to_string()).unwrap();

    // TODO Need to create a new agent and add it to the environment
    // maybe something like this: 
    // let bob = Agent::new("name".to_owned(), manager.environments.get(&TEST_ENV_LABEL.clone().to_string()));
    // bob.add_behavior(behaviors::TestBehavior::new());
    // Probably read up on artemis 
    // https://github.com/paradigmxyz/artemis/blob/c8ab223a363a875f685ab177839eacfffc9d8de0/crates/artemis-core/src/types.rs#L25



    // Deploy a new ArbiterToken contract
    // let token = ArbiterToken::deploy(agent.client.clone(), (
    //         TEST_ARG_NAME.to_string(),
    //         TEST_ARG_SYMBOL.to_string(),
    //         TEST_ARG_DECIMALS,
    //     ),
    // )?
    // .send().await?;
    // println!("Deployed ArbiterToken to address: {}", token.address());


    Ok(())
}