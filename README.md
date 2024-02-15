# Arbiter Template

Minimal template for simulating contracts with arbiter. This template provides an example of how to build Agent-Based Models (ABM) with evm parity. In this model, you can think of anything that happens as a *behavior* of an agent. Agents can have externally owned accounts (EOAs), interact with each other, and interact with smart contracts. 

This repository has an example behavior [`Incrementer`](src/bahaviors/incrementer.rs). The current design philosophy is that the user should only ever have to build agent behaviors implementing the [`Behavior`](https://github.com/primitivefinance/arbiter/blob/fe6b556d715d641aa9378ae20560629ec6ba5b43/arbiter-engine/src/machine.rs#L73) trait. In this example, the `Incrementer` behavior is configured with a [config file](https://github.com/primitivefinance/arbiter-template/blob/main/configs/example.toml). Configuring behaviors with a config file is a design choice we made to enable versatile parameterization at runtime as opposed to compile time.

### Prerequisites

- Rust programming language and Cargo package manager (latest stable version recommended)
- [Foundry](https://book.getfoundry.sh/getting-started/installation) is used behind the scenes to generate rust contract bindings. Make sure you have forge installed and up to date.

## Usage

> `cargo generate` is a tool to create new Rust projects from pre-existing templates. It is used to create a new project from a template.

``` bash 
cargo install cargo-generate
```
Use the template
```
cargo generate --git https://github.com/primitivefinance/arbiter-template
```


> Run the template project
```bash
cargo run simulate configs/example.toml -vvv
```

## Log Verbosity
The `-vvv` flag is used to increase the verbosity of the logs. The more `v`'s, the more verbose the logs.