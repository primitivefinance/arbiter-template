use std::sync::Arc;

use arbiter_core::{middleware::RevmMiddleware, environment::Environment, bindings::arbiter_token::ArbiterToken};
use ethers::types::{Address, U256};
use bindings::counter::Counter;

use super::*;

#[derive(Clone)]
pub struct CounterAgent {
    pub client: Arc<RevmMiddleware>,
    pub counter: Counter<RevmMiddleware>,
}

impl CounterAgent {
    pub async fn new(environment: &Environment) -> Result<Self> {
        let client = RevmMiddleware::new(environment, "counter_agent".into())?;
        let counter = Counter::deploy(client.clone(), ())?.send().await?;

        Ok(Self { client, counter})
    }

    pub async fn increment (&self) -> Result<()> {
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
