use arbiter_core::middleware::RevmMiddleware;
use arbiter_core::{
    manager::Manager,
    strategies::{SimulationCollector, SimulationExecutor},
};
use artemis_core::engine::Engine;
use crossbeam_channel::unbounded;
use std::{error::Error, sync::Arc};

use crate::bindings::counter::Counter;

mod bindings;
mod strategies;

const TEST_ENV_LABEL: &str = "test";
const TEST_AGENT_NAME: &str = "bob";

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let mut manager = Manager::new();
    let _ = manager.add_environment(TEST_ENV_LABEL, 1.0, 1, Engine::new());

    let client = Arc::new(RevmMiddleware::new(
        manager.environments.get(TEST_ENV_LABEL).unwrap(),
    ));

    let (sender, receiver) = crossbeam_channel::unbounded();

    let strategy =
        strategies::CounterStrategy::new(TEST_AGENT_NAME.to_string(), client.clone(), sender);
    let collector = SimulationCollector::new(receiver);
    let executor = SimulationExecutor::new(client.clone());

    manager
        .environments
        .get_mut(TEST_ENV_LABEL)
        .unwrap()
        .engine()
        .add_collector(Box::new(collector));
    manager
        .environments
        .get_mut(TEST_ENV_LABEL)
        .unwrap()
        .engine()
        .add_strategy(Box::new(strategy));
    manager
        .environments
        .get_mut(TEST_ENV_LABEL)
        .unwrap()
        .engine()
        .add_executor(Box::new(executor));

    manager.start_environment(TEST_ENV_LABEL).await?;

    Ok(())
}
