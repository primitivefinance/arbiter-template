#[allow(non_snake_case)]
pub mod bindings;
pub mod simulation;
pub mod v2_core_bindings;
use anyhow::{Ok, Result};
use arbiter_core::{
    bindings::weth,
    environment::{builder::EnvironmentBuilder, fork::Fork},
    middleware::RevmMiddleware,
};
use bindings::counter::Counter;
use ethers::{providers::Middleware, types::Address};
use std::str::FromStr;
