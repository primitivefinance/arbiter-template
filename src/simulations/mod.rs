use arbiter_core::environment::Environment;
use std::sync::Arc;
use tokio::sync::Semaphore;
use serde::{Serialize, Deserialize};

use super::*;
use crate::{
    agents::{Agent, Agents},
    settings::parameters::Fixed,
};

pub mod price_path_simulation;
pub mod counter;

use settings::parameters::Parameterized;
use tokio::runtime::Builder;
use crate::settings::SimulationConfig;
use anyhow::Result;

pub struct Simulation {
    pub agents: Agents,
    pub steps: usize,
    environment: Environment,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SimulationType {
    SimulatedPricePath,
    Counter
}

impl SimulationType {
    async fn run(config: SimulationConfig<Fixed>) -> Result<()> {
        let simulation = match config.simulation {
            SimulationType::SimulatedPricePath => price_path_simulation::setup(config.clone()).await?,
            SimulationType::Counter => counter::setup(config.clone()).await?,
        };
        match looper(simulation.agents, simulation.steps).await {
            Result::Ok(_) => {
                simulation.environment.stop()?;
                Ok(())
            }
            Err(e) => {
                let metadata = format!(
                    "{}_{}",
                    config.output_directory,
                    config.output_file_name.unwrap()
                );
                let error_string = format!("Error in simulation `{:?}`: {:?}", metadata, e);
                simulation.environment.stop()?;
                Err(e)
            }
        }
    }
}


pub fn batch(config_path: &str) -> Result<()> {
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