use std::sync::Arc;

use arbiter_core::environment::Environment;
use arbiter_core::middleware::RevmMiddleware;
use ethers::providers::Middleware;
use settings::SimulationConfig;

use super::*;

#[derive(Clone)]
pub struct BlockAdmin {
    pub client: Arc<RevmMiddleware>,
    pub timestep_size: u64,
    pub block_timestamp: u64,
    pub block_number: u64,
}

impl BlockAdmin {
    pub async fn new(environment: &Environment, config: &SimulationConfig<Fixed>) -> Result<Self> {
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
