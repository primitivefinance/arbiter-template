use arbiter_core::{
    data_collection::EventLogger,
    environment::builder::{BlockSettings, EnvironmentBuilder},
};

use super::*;
use crate::{
    agents::{
        block_admin::BlockAdmin, price_changer::PriceChanger, token_admin::TokenAdmin, Agents,
    },
    settings::SimulationConfig,
};

/// Asynchronously sets up a `SimulatedPricePath` simulation using the provided configuration.
///
/// This function prepares the environment, initializes various agents including
/// `BlockAdmin`, `TokenAdmin`, and `PriceChanger`, logs events, and then returns a `Simulation`
/// object which houses the configured agents, steps, and the environment.
///
/// # Arguments
///
/// * `config` - The configuration for the simulation based on `SimulationConfig<Fixed>`.
///
/// # Returns
///
/// * A `Result` containing the fully initialized `Simulation` or an error if any step of the setup fails.
pub async fn setup(config: SimulationConfig<Fixed>) -> Result<Simulation> {
    let environment = EnvironmentBuilder::new()
        .block_settings(BlockSettings::UserControlled)
        .build();

    let block_admin = BlockAdmin::new(&environment, &config).await?;
    let token_admin = TokenAdmin::new(&environment).await?;
    let price_changer = PriceChanger::new(&environment, &token_admin, &config).await?;

    EventLogger::builder()
        .directory(config.output_directory)
        .file_name(config.output_file_name.unwrap())
        .add(price_changer.liquid_exchange.events(), "lex")
        .run()?;

    Ok(Simulation {
        agents: Agents::new().add(price_changer).add(block_admin),
        steps: config.trajectory.num_steps,
        environment,
    })
}
