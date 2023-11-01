use arbiter_core::{environment::Environment, middleware::RevmMiddleware};
use bindings::counter::Counter;
use std::sync::Arc;

use super::*;

/// A structure representing a counter agent.
/// This agent is responsible for incrementing the count on a counter contract.
#[derive(Clone)]
pub struct CounterAgent {
    // A client to interface with arbiter's revm middleware
    pub client: Arc<RevmMiddleware>,

    // An instance of a deployed counter contract
    pub counter: Counter<RevmMiddleware>,
}

impl CounterAgent {
    /// Creates a new instance of a [`CounterAgent`].
    ///
    /// # Arguments
    /// * [`Environment`] - A reference to the environment that holds blockchain configuration.
    ///
    /// # Returns
    /// * [`Result<Self>`] - Result of CounterAgent creation, containing the agent or an error.
    pub async fn new(environment: &Environment) -> Result<Self> {
        let client = RevmMiddleware::new(environment, "counter_agent".into())?;
        let counter = Counter::deploy(client.clone(), ())?.send().await?;

        Ok(Self { client, counter })
    }

    /// Increments the counter in the smart contract.
    ///
    /// # Returns
    /// * [`Result<()>`] - Result of the increment operation, indicating success or error.
    pub async fn increment(&self) -> Result<()> {
        self.counter.increment().send().await?.await?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl Agent for CounterAgent {
    async fn step(&mut self) -> Result<()> {
        self.increment().await?;
        Ok(())
    }
}
