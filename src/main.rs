use std::time::Instant;

use anyhow::Result;
use clap::{ArgAction, CommandFactory, Parser, Subcommand};

pub mod agents;
pub mod bindings;
pub mod settings;
pub mod simulations;

/// Represents command-line arguments passed to this binary.
#[derive(Parser)]
#[clap(name = "Excalibur")]
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
    Simulate {
        #[clap(index = 1, default_value = "src/config/gbm.toml")]
        config_path: String,
    },
}

/// The entry point for the simulation tool.
///
/// This binary provides a command-line interface for the simulation-driven development.
/// It allows users to run simulations by specifying configuration paths, with detailed command-line
/// feedback provided through the `clap` crate.
///
/// # Usage
/// Run the binary without arguments to see available commands and options.
/// Example usage for running simulations:
/// ```
/// $ cargo run simulate [path_to_config]
/// ```
///
/// By default, if no configuration path is provided, it will read from "src/config/gbm.toml".
///
/// These simulations are performed in Arbiter's in memory revm instance and with the exposed RevmMiddleware.
fn main() -> Result<()> {
    let args = Args::parse();

    match &args.command {
        Some(Commands::Simulate { config_path }) => {
            println!("Reading from config path: {}", config_path);
            let start = Instant::now();
            // This is the entry point for the simulation
            simulations::batch(config_path)?;
            let duration = start.elapsed();
            println!("Total duration of simulations: {:?}", duration);
        }
        None => Args::command().print_long_help()?,
    }
    Ok(())
}
