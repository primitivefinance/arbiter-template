pub mod parameters;
use std::{env, path::Path};

use parameters::*;

use crate::simulations::SimulationType;
use serde::{Deserialize, Serialize};
use config::{Config, ConfigError};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimulationConfig<P: Parameterized<f64>> {
    pub simulation: SimulationType,
    pub max_parallel: Option<usize>,
    pub output_directory: String,
    pub output_file_name: Option<String>,
    pub trajectory: TrajectoryParameters<P>,
    pub gbm: Option<GBMParameters<P>>,
    pub block: BlockParameters,
}

impl SimulationConfig<Meta> {
    pub fn new(config_path: &str) -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(config::File::with_name(config_path))
            .build()?;
        s.try_deserialize()
    }
}

impl Parameterized<SimulationConfig<Fixed>> for SimulationConfig<Meta> {
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