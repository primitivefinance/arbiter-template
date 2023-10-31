use std::{str::FromStr, time::Instant};

use anyhow::{Ok, Result};
use arbiter_core::{
    bindings::weth,
    environment::{builder::EnvironmentBuilder, fork::Fork},
    middleware::RevmMiddleware,
};
use ethers::{providers::Middleware, types::Address};

use crate::bindings::counter::Counter;

#[allow(unused_imports)]
mod bindings;


use clap::{ArgAction, CommandFactory, Parser, Subcommand};
use simulation::simulation::counter_example;



#[derive(Parser)]
#[clap(name = "arbiter_examplse")]
#[clap(version = env!("CARGO_PKG_VERSION"))]
#[clap(about = "Simulation driven development.", long_about = None)]
#[clap(author)]
struct Args {
    /// Defines the subcommand to execute.
    #[command(subcommand)]
    command: Option<Commands>,

    #[clap(short, long, global = true, required = false, action = ArgAction::Count, value_parser(
        clap::value_parser!(u8)))]
    verbose: Option<u8>,
}

/// Defines available subcommands for the `Arbiter` tool.
#[derive(Subcommand)]
enum Commands {
    /// Represents the `Bind` subcommand.
    Counter {},
    ForkContract {},
    ForkEOA {},
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    match &args.command {
        Some(Commands::Counter {}) => simulation::simulation::counter_example().await?,
        Some(Commands::ForkContract {}) => simulation::simulation::load_contract_from_fork().await?,
        Some(Commands::ForkEOA {}) => simulation::simulation::load_eoa_from_disk().await?,
        None => Args::command().print_long_help()?,
    }
    Ok(())
}


