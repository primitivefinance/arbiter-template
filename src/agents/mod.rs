use super::*;

pub mod block_admin;
pub mod counter_agent;
pub mod price_changer;
pub mod token_admin;

use std::marker::{Send, Sync};

use crate::settings::parameters::Fixed;

/// Universal agent methods for interacting with the simulation environment or
/// loop.
/// Agents are expected to be both [`Send`] and [`Sync`].
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
/// A collection of agents that can be operated on collectively.
pub struct Agents(pub Vec<Box<dyn Agent>>);

impl Agents {
    /// Returns a mutable iterator over the agents.
    /// This can be used to invoke methods on each agent individually.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Box<dyn Agent>> {
        self.0.iter_mut()
    }
}

impl Agents {
    /// Constructs a new [`Agents`] collection.
    /// This static method provides a way to create a new collection of agents.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self(vec![])
    }

    /// Adds a new agent to the collection.
    /// This method takes ownership of the agent and adds it to the collection.
    #[allow(clippy::should_implement_trait)]
    pub fn add(mut self, agent: impl Agent + 'static) -> Self {
        self.0.push(Box::new(agent));
        self
    }
}

/// [`Agent`] trait implementation for a collection of agents.
/// This allows collective operations on the group of agents.
#[async_trait::async_trait]
impl Agent for Agents {
    /// Implementation of the `step` method for the collection.
    /// This allows the collection to forward the step action to each agent.
    async fn step(&mut self) -> Result<()> {
        Ok(())
    }

    /// Implementation of the `priority_step` method for the collection.
    /// This allows the collection to forward the priority step action to each agent.
    async fn priority_step(&mut self) -> Result<()> {
        Ok(())
    }
}
