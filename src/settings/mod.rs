pub mod parameters;

use parameters::*;

use crate::simulations::SimulationType;
use config::{Config, ConfigError};
use serde::{Deserialize, Serialize};

/// Defines the configuration for a simulation.
///
/// This struct holds all the necessary parameters and configurations needed to run a simulation.
/// It encompasses several sub-configurations such as `TrajectoryParameters` and `GBMParameters`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimulationConfig<P: Parameterized<f64>> {
    /// The type of simulation to run, defined by an enum `SimulationType`.
    pub simulation: SimulationType,

    /// Maximum number of parallel simulations to run.
    pub max_parallel: Option<usize>,

    /// Directory where the simulation output will be stored.
    pub output_directory: String,

    /// Name of the file where the simulation results will be written.
    pub output_file_name: Option<String>,

    /// Parameters specific to the trajectory of the simulation.
    pub trajectory: TrajectoryParameters<P>,

    /// Parameters specific to the Geometric Brownian Motion (GBM) if applicable.
    pub gbm: Option<GBMParameters<P>>,

    /// Parameters related to block configurations.
    pub block: BlockParameters,
}

impl SimulationConfig<Meta> {
    /// Creates a new `SimulationConfig` instance from a configuration file.
    ///
    /// Reads the specified configuration file and deserializes it into a `SimulationConfig` object.
    /// The `config_path` is the path to the configuration file in question.
    pub fn new(config_path: &str) -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(config::File::with_name(config_path))
            .build()?;
        s.try_deserialize()
    }
}

impl Parameterized<SimulationConfig<Fixed>> for SimulationConfig<Meta> {
    /// Generates a list of `SimulationConfig` instances with fixed parameters.
    ///
    /// This method is responsible for taking the meta parameters defined in the configuration,
    /// generating the actual fixed parameters, and creating a list of complete `SimulationConfig` instances.
    fn generate(&self) -> Vec<SimulationConfig<Fixed>> {
        let mut result = vec![];
        let trajectories = self.trajectory.generate();

        let gbms = self
            .gbm
            .as_ref()
            .map(|gbm| gbm.generate())
            .unwrap_or_default();

        if gbms.is_empty() {
            panic!("You must supply either a gbm  configuration.");
        }

        for trajectory in &trajectories {
            for gbm in &gbms {
                let output_directory = self.output_directory.clone()
                    + "/gbm_drift="
                    + &gbm.drift.0.to_string()
                    + "_vol="
                    + &gbm.volatility.0.to_string();
                let output_file_name =
                    format!("trajectory={}", trajectory.output_tag.clone().unwrap());
                result.push(SimulationConfig {
                    simulation: self.simulation,
                    max_parallel: None,
                    output_directory,
                    output_file_name: Some(output_file_name),
                    trajectory: trajectory.clone(),
                    gbm: Some(*gbm),
                    block: self.block,
                });
            }
        }

        result
    }
}
