use super::*;

pub mod block_admin;
pub mod counter_agent;
pub mod price_changer;
pub mod token_admin;

use std::marker::{Send, Sync};

use crate::settings::parameters::Fixed;

/// Universal agent methods for interacting with the simulation environment or
/// loop.
#[async_trait::async_trait]
pub trait Agent: Sync + Send {
    /// Executed outside the main simulation loop.
    async fn startup(&mut self) -> Result<()> {
        Ok(())
    }

    /// Executed by each agent inside the main simulation loop.
    /// Ordering is determined by placement in the simulation loop.
    async fn step(&mut self) -> Result<()> {
        Ok(())
    }

    /// Executed by each agent in a separate loop before the main loop.
    async fn priority_step(&mut self) -> Result<()> {
        Ok(())
    }
}

pub struct Agents(pub Vec<Box<dyn Agent>>);

impl Agents {
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Box<dyn Agent>> {
        self.0.iter_mut()
    }
}

impl Agents {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn add(mut self, agent: impl Agent + 'static) -> Self {
        self.0.push(Box::new(agent));
        self
    }
}

#[async_trait::async_trait]
impl Agent for Agents {
    async fn step(&mut self) -> Result<()> {
        Ok(())
    }

    async fn priority_step(&mut self) -> Result<()> {
        Ok(())
    }
}
