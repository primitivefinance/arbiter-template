# Arbiter Template

Minimal template for simulating contracts with arbiter. This template is used by the `arbiter init` command when starting new simulations. This template provides a framework for performantly simulating Agent Based Models (ABM) with evm parity. In this model you can think of any things that happens as an action of an agent. Agents can own keys and externally owned accounts, they can interact with each other and they can interact with smart contracts. 

This repository has some example agents including a [`TokenAdmin`](src/agents/token_admin.rs), [`BlockAdmin`](src/agents/block_admin.rs), and [`CounterAgent`](src/agents/counter_agent.rs) which their own functionality and responsibilities. We also give an example of [how to parametarize your simulations](src/settings/mod.rs) with a configuration file containing different price paths and price path parameters. These can be played with to see how the simulation changes. Furthermore we provide an [api to batch simulations](src/simulations/mod.rs) and run them in parallel. This is useful for running many simulations with different parameters.

### Prerequisites

- Rust programming language and Cargo package manager (latest stable version recommended)
- [Foundry](https://book.getfoundry.sh/getting-started/installation) is used behind the scenes to generate rust contract bindings. Make sure you have forge installed and up to date.

## Usage
1. Install arbiter

``` bash
cargo install arbiter
```

2. Create arbiter project from this template

``` bash 
arbiter init <name_of_project>
```

3. Run the project
```bash
cargo run simulate src/config/counter.toml
```

## Documentation

The documentation for the repository is primarily inline with the code. Cargo automatically can compile these into a browsable format. To view the documentation run the following command:

```bash
cargo doc --open
```