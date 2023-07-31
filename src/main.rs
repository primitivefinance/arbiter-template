
use std::{error::Error};
use core::{bindings::arbiter_token::ArbiterToken, manager::SimulationManager};

mod bindings;

const TEST_ENV_LABEL: &str = "test";
const TEST_AGENT_NAME: &str = "test_agent";
const TEST_ARG_NAME: &str = "test_arg";
const TEST_ARG_SYMBOL: &str = "TEST";
const TEST_ARG_DECIMALS: u8 = 18;

#[tokio::main]
pub async fn run() -> Result<(), Box<dyn Error>> {
    // let mut environment = Environment::new(TEST_ENV_LABEL.to_string());
    let mut manager = SimulationManager::new();
    manager.add_environment(TEST_ENV_LABEL.clone().to_string())?;
    let mut environment = manager.environments.get_mut(TEST_ENV_LABEL).unwrap();
    environment.add_agent(TEST_AGENT_NAME.to_string(), environment.connection.clone());
    let agent = environment.clients[0].clone();
    let address = ArbiterToken::deploy(agent.client, (
            TEST_ARG_NAME.to_string(),
            TEST_ARG_SYMBOL.to_string(),
            TEST_ARG_DECIMALS,
        ),
    )?
    .send().await?;
}
