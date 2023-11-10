use crate::simulations::SimulationType;
use config::{Config, ConfigError};
use serde::{Deserialize, Serialize};

/// Defines the configuration for a simulation.
///
/// This struct holds all the necessary parameters and configurations needed to run a simulation.
/// It encompasses several sub-configurations such as `TrajectoryParameters` and `GBMParameters`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimulationConfig {
    /// The type of simulation to run, defined by an enum `SimulationType`.
    pub simulation: SimulationType,

    /// Directory where the simulation output will be stored.
    pub output_directory: String,

    /// Name of the file where the simulation results will be written.
    pub output_file_name: String,

    /// Parameters specific to the trajectory of the simulation.
    pub trajectory: TrajectoryParameters,

    /// Parameters specific to the Geometric Brownian Motion (GBM) if applicable.
    pub gbm: GBMParameters,

    /// Parameters related to block configurations.
    pub block: BlockParameters,
}

impl SimulationConfig {
    /// Creates a new `SimulationConfig` instance from a configuration file.
    ///
    /// Reads the specified configuration file and deserializes it into a `SimulationConfig` object.
    /// The `config_path` is the path to the configuration file in question.
    pub fn new(config_path: String) -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(config::File::with_name(&config_path))
            .build()?;
        s.try_deserialize()
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct BlockParameters {
    pub timestep_size: u64,
}

/// Defines parameters for a trajectory in the simulation.
///
/// Contains information like initial price, start and end times,
/// and number of steps and paths in the simulation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrajectoryParameters {
    /// The name.
    pub process: String,

    /// The initial price of the asset.
    pub initial_price: f64,

    /// The start time of the process.
    pub t_0: f64,

    /// The end time of the process.
    pub t_n: f64,

    /// The number of steps in the process.
    pub num_steps: usize,

    /// The number of paths in the process.
    pub num_paths: usize,

    /// The seed for the process.
    pub seed: u64,

    /// The tag for the output file.
    pub output_tag: Option<String>,
}

/// Contains the parameters for the Geometric Brownian Motion (GBM) process.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct GBMParameters {
    // The drift of the process.
    pub drift: f64,

    // The volatility of the process.
    pub volatility: f64,
}
