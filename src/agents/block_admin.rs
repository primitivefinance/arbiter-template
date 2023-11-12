use std::sync::Arc;

use arbiter_core::environment::Environment;
use arbiter_core::middleware::RevmMiddleware;
use ethers::providers::Middleware;
use settings::SimulationConfig;

use super::*;

/// A structure representing a block admin agent.
/// This agent is responsible for updating the block number and timestamp.
#[derive(Clone)]
pub struct BlockAdmin {
    /// A client to interface with arbiter's revm middleware.
    /// You can think of this as the agents wallet or EOA.
    pub client: Arc<RevmMiddleware>,

    /// The size of each timestep in the simulation, representing block time passage.
    pub timestep_size: u64,

    /// The current simulated block timestamp.
    pub block_timestamp: u64,

    /// The current simulated block number.
    pub block_number: u64,
}

impl BlockAdmin {
    /// Creates a new BlockAdmin using the provided environment and simulation configuration.
    ///
    /// # Arguments
    /// * [`Environment`] - The environment containing blockchain node information.
    /// * [`SimulationConfig`] - The simulation configuration providing block timestep size.
    ///
    /// # Returns
    /// * [`Result<Self>`] - A result containing the new BlockAdmin or an error.
    pub async fn new(environment: &Environment, config: &SimulationConfig) -> Result<Self> {
        let client = RevmMiddleware::new(environment, "block_admin".into())?;
        let timestep_size = config.block.timestep_size;
        let block_number = client.get_block_number().await?.as_u64();
        let block_timestamp = client.get_block_timestamp().await?.as_u64();

        Ok(Self {
            client,
            timestep_size,
            block_timestamp,
            block_number,
        })
    }
    /// Updates the simulated block information.
    ///
    /// Increments the block number and calculates the new block timestamp based on the timestep size.
    ///
    /// # Returns
    /// * [`Result<()>`] - A result indicating the success or failure of the operation.
    pub fn update_block(&mut self) -> Result<()> {
        self.block_number += 1;
        self.block_timestamp = self.block_number * self.timestep_size;
        self.client
            .update_block(self.block_number, self.block_timestamp)?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl Agent for BlockAdmin {
    async fn step(&mut self) -> Result<()> {
        self.update_block()?;
        Ok(())
    }
    async fn startup(&mut self) -> Result<()> {
        self.update_block()?;
        Ok(())
    }
}
