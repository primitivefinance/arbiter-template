/// Arbiter Simulation module for handling different types of simulations.
///
/// This module provides structs and functions for executing and managing
/// various types of simulations, including counter simulations and price path simulations.
use arbiter_core::environment::Environment;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Semaphore;

use super::*;
use crate::{agents::Agents, settings::parameters::Fixed};

pub mod counter;
pub mod price_path_simulation;

use crate::settings::SimulationConfig;
use anyhow::Result;
use settings::parameters::Parameterized;
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
    async fn run(config: SimulationConfig<Fixed>) -> Result<()> {
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
pub fn batch(config_path: &str) -> Result<()> {
    //
    let config = SimulationConfig::new(config_path)?;

    let direct_configs: Vec<SimulationConfig<Fixed>> = config.generate();

    // Create a multi-threaded runtime
    let rt = Builder::new_multi_thread().build()?;

    // Create a semaphore with a given number of permits
    let semaphore = config
        .max_parallel
        .map(|max_parallel| Arc::new(Semaphore::new(max_parallel)));

    rt.block_on(async {
        let mut handles = vec![];
        let errors = Arc::new(tokio::sync::Mutex::new(vec![]));

        for config in direct_configs {
            let errors_clone = errors.clone();
            let semaphore_clone = semaphore.clone();
            handles.push(tokio::spawn(async move {
                // Acquire a permit inside the spawned task
                let permit = if let Some(ref semaphore_clone) = semaphore_clone {
                    // Acquire a permit outside the spawned task
                    let permit = semaphore_clone.acquire().await.unwrap();
                    Some(permit)
                } else {
                    None
                };

                let result = SimulationType::run(config).await;
                match result {
                    Err(e) => {
                        let mut errors_clone_lock = errors_clone.lock().await;
                        errors_clone_lock.push(e);
                        // Drop the permit when the simulation is done.
                        drop(permit);
                    }
                    Result::Ok(_) => {
                        drop(permit);
                    }
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
