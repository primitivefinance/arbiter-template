use arbiter_core::{
    data_collection::EventLogger,
    environment::builder::{BlockSettings, EnvironmentBuilder},
};

use super::*;
use crate::{
    agents::{block_admin::BlockAdmin, counter_agent::CounterAgent, Agents},
    settings::SimulationConfig,
};

/// Asynchronously sets up a `Counter` simulation using the provided configuration.
///
/// This function prepares the environment, initializes the `BlockAdmin` and `CounterAgent`,
/// logs events, and returns a `Simulation` with the configured agents, steps, and environment.
///
/// # Arguments
///
/// * `config` - The configuration for the simulation based on `SimulationConfig<Fixed>`.
///
/// # Returns
///
/// * A `Result` containing the fully initialized `Simulation` or an error if any step fails.
pub async fn setup(config: SimulationConfig<Fixed>) -> Result<Simulation> {
    let environment = EnvironmentBuilder::new()
        .block_settings(BlockSettings::UserControlled)
        .build();

    let block_admin = BlockAdmin::new(&environment, &config).await?;
    let counter_agent = CounterAgent::new(&environment).await?;

    EventLogger::builder()
        .directory(config.output_directory)
        .file_name(config.output_file_name.unwrap())
        .add(counter_agent.counter.events(), "counter")
        .run()?;

    Ok(Simulation {
        agents: Agents::new().add(block_admin).add(counter_agent),
        steps: config.trajectory.num_steps,
        environment,
    })
}
