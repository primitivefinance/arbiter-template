use std::time::Instant;

use anyhow::Result;
use clap::{ArgAction, CommandFactory, Parser, Subcommand};

pub mod simulations;
pub mod settings;
pub mod agents;
pub mod bindings;

/// Represents command-line arguments passed to the `Arbiter` tool.
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

fn main() -> Result<()> {
    let args = Args::parse();

    match &args.command {
        Some(Commands::Simulate { config_path }) => {
            println!("Reading from config path: {}", config_path);
            let start = Instant::now();
            simulations::batch(config_path)?;
            let duration = start.elapsed();
            println!("Total duration of simulations: {:?}", duration);
        }
        None => Args::command().print_long_help()?,
    }
    Ok(())
}

