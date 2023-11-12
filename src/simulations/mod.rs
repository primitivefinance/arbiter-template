/// Arbiter Simulation module for handling different types of simulations.
///
/// This module provides structs and functions for executing and managing
/// various types of simulations, including counter simulations and price path simulations.
use arbiter_core::environment::Environment;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::agents::Agents;

pub mod counter;
pub mod price_path_simulation;

use crate::settings::SimulationConfig;
use anyhow::Result;
use tokio::runtime::Builder;

/// Represents the main Simulation structure.
///
/// This struct encapsulates agents, steps, and the environment needed
/// for a simulation.
pub struct Simulation {
    pub agents: Agents,
    pub steps: usize,
    environment: Environment,
}

/// Defines the types of simulations available.
///
/// The `SimulationType` enum provides an easy way to specify and differentiate
/// between different types of simulations, such as `SimulatedPricePath` and `Counter`.
/// If you wanted to add a simulation you would add it here to this enum
#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SimulationType {
    SimulatedPricePath,
    Counter,
}

impl SimulationType {
    /// Asynchronously runs the specified simulation type based on the provided configuration.
    ///
    /// This function matches on the `SimulationType` to determine which simulation setup to use,
    /// then executes the chosen simulation.
    async fn run(config: SimulationConfig) -> Result<()> {
        let simulation = match config.simulation {
            SimulationType::SimulatedPricePath => {
                price_path_simulation::setup(config.clone()).await?
            }
            SimulationType::Counter => counter::setup(config.clone()).await?,
        };
        match looper(simulation.agents, simulation.steps).await {
            Result::Ok(_) => {
                simulation.environment.stop()?;
                Ok(())
            }
            Err(e) => {
                simulation.environment.stop()?;
                Err(e)
            }
        }
    }
}

/// Executes a batch of simulations based on the provided configuration path.
///
/// This function sets up multiple simulations to run in parallel, manages available resources using a semaphore,
/// and handles any errors that arise during execution.
pub fn batch(config_paths: Vec<String>) -> Result<()> {
    // Create a multi-threaded runtime
    let rt = Builder::new_multi_thread().build()?;

    let mut configs = vec![];
    for path in config_paths {
        configs.push(SimulationConfig::new(path)?);
    }

    rt.block_on(async {
        let mut handles = vec![];
        let errors = Arc::new(tokio::sync::Mutex::new(vec![]));

        for config in configs {
            let errors_clone = errors.clone();
            handles.push(tokio::spawn(async move {
                let result = SimulationType::run(config).await;
                if let Err(e) = result {
                    let mut errors_clone_lock = errors_clone.lock().await;
                    errors_clone_lock.push(e);
                }
            }));
        }

        for handle in handles {
            handle.await?;
        }

        Ok(())
    })
}

/// Asynchronously loops through agents and performs the steps for each agent.
///
/// This function starts each agent, then performs priority steps and regular steps
/// for a given number of iterations.
pub async fn looper(mut agents: Agents, steps: usize) -> Result<()> {
    for agent in agents.iter_mut() {
        agent.startup().await?;
    }

    for _ in 0..steps {
        for agent in agents.iter_mut() {
            agent.priority_step().await?;
        }

        for agent in agents.iter_mut() {
            agent.step().await?;
        }
    }

    Ok(())
}
