use std::{collections::hash_map::DefaultHasher, hash::Hasher};

use super::*;
pub trait Parameterized<T> {
    fn generate(&self) -> Vec<T>;
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Fixed(pub f64);
impl Parameterized<f64> for Fixed {
    fn generate(&self) -> Vec<f64> {
        vec![self.0]
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Meta(LinspaceParameters);
impl Parameterized<f64> for Meta {
    fn generate(&self) -> Vec<f64> {
        self.0.generate()
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct LinspaceParameters {
    pub start: Option<f64>,
    pub end: Option<f64>,
    pub steps: Option<usize>,
    pub fixed: Option<f64>,
}

impl LinspaceParameters {
    fn generate(&self) -> Vec<f64> {
        // Check if start, end, steps are all Some
        match (self.start, self.end, self.steps) {
            (Some(start), Some(end), Some(steps)) => {
                if self.fixed.is_some() {
                    panic!("Both linspace and fixed parameters are set");
                }
                let step_size = (end - start) / (steps as f64 - 1.0);
                (0..steps).map(|i| start + step_size * i as f64).collect()
            }
            // If only fixed is Some, return a vec with that fixed value
            (_, _, _) if self.fixed.is_some() => vec![self.fixed.unwrap()],
            // Otherwise, configuration is invalid
            _ => panic!("Invalid configuration for LinspaceParameters. Please provide a `start`, `end`, and `steps` or alternatively just provide a `fixed` value."),
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct BlockParameters {
    pub timestep_size: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrajectoryParameters<P: Parameterized<f64>> {
    pub process: String,
    /// The initial price of the asset.
    pub initial_price: P,
    /// The start time of the process.
    pub t_0: P,
    /// The end time of the process.
    pub t_n: P,
    /// The number of steps in the process.
    pub num_steps: usize,
    pub num_paths: usize,
    pub seed: u64,
    pub output_tag: Option<String>,
}

impl Parameterized<TrajectoryParameters<Fixed>> for TrajectoryParameters<Meta> {
    fn generate(&self) -> Vec<TrajectoryParameters<Fixed>> {
        let initial_price = self.initial_price.generate();
        let t_0 = self.t_0.generate();
        let t_n = self.t_n.generate();
        let mut result = vec![];
        let mut hasher = DefaultHasher::new();
        let mut seed = self.seed;
        for p in initial_price {
            for t0 in t_0.clone() {
                for tn in t_n.clone() {
                    for index in 0..self.num_paths {
                        result.push(TrajectoryParameters {
                            process: self.process.clone(),
                            initial_price: Fixed(p),
                            t_0: Fixed(t0),
                            t_n: Fixed(tn),
                            num_steps: self.num_steps,
                            num_paths: 1,
                            seed,
                            output_tag: Some(index.to_string()),
                        });
                        hasher.write_u64(seed);
                        seed = hasher.finish();
                    }
                }
            }
        }
        result
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct GBMParameters<P: Parameterized<f64>> {
    // The drift of the process.
    pub drift: P,
    // The volatility of the process.
    pub volatility: P,
}

impl Parameterized<GBMParameters<Fixed>> for GBMParameters<Meta> {
    fn generate(&self) -> Vec<GBMParameters<Fixed>> {
        let drift = self.drift.generate();
        let volatility = self.volatility.generate();
        let mut result = vec![];
        for d in drift {
            for v in volatility.clone() {
                result.push(GBMParameters {
                    drift: Fixed(d),
                    volatility: Fixed(v),
                });
            }
        }
        result
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct OUParameters<P: Parameterized<f64>> {
    /// The mean (price) of the process.
    pub mean: P,
    /// The standard deviation of the process.
    pub std_dev: P,
    /// The theta parameter of the process.
    /// This describes how strongly the process will revert to the mean.
    pub theta: P,
}

impl Parameterized<OUParameters<Fixed>> for OUParameters<Meta> {
    fn generate(&self) -> Vec<OUParameters<Fixed>> {
        let mean = self.mean.generate();
        let std_dev = self.std_dev.generate();
        let theta = self.theta.generate();
        let mut result = vec![];
        for m in mean {
            for s in std_dev.clone() {
                for t in theta.clone() {
                    result.push(OUParameters {
                        mean: Fixed(m),
                        std_dev: Fixed(s),
                        theta: Fixed(t),
                    });
                }
            }
        }
        result
    }
}
