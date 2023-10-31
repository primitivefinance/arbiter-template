use anyhow::{Ok, Result};
use clap::{ArgAction, CommandFactory, Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "arbiter_examplse")]
#[clap(version = env!("CARGO_PKG_VERSION"))]
#[clap(about = "Simulation driven development.", long_about = None)]
#[clap(author)]
struct Args {
    /// Defines the subcommand to execute.
    #[command(subcommand)]
    command: Option<Commands>,
}

/// Defines available subcommands for the `Arbiter` tool.
#[derive(Subcommand)]
enum Commands {
    Counter,
    ForkContract,
    ForkEOA,
    Uniswap, 
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    match &args.command {
        Some(Commands::Counter {}) => simulation::simulations::counter_example().await?,
        Some(Commands::ForkContract {}) => simulation::simulations::load_contract_from_fork().await?,
        Some(Commands::ForkEOA {}) => simulation::simulations::load_eoa_from_disk().await?,
        Some(Commands::Uniswap) => simulation::simulations::uniswap_example().await?,
        None => Args::command().print_long_help()?,
    }
    Ok(())
}


