pub mod behaviors;
pub mod bindings;

use behaviors::Behaviors;

/// If you forked `arbiter`, then to run this example, you can do the following from the `arbiter/` directory:
/// ```sh
/// cargo run --example template simulate examples/template/configs/example.toml
/// ```
/// If you would like to see more detailed logs, you can run the following:
/// ```sh
/// cargo run --example template simulate examples/template/configs/example.toml -vvv
/// ```
/// to get `debug` level logs.
///
/// By running
/// ```sh
/// cargo run --example template
/// ```
/// you will get the `--help` message for the project.
///
/// If instead you are working with a template directly, you can run the following from the `arbiter-template/` directory:
/// ```sh
/// cargo run simulate configs/example.toml -vvv
/// ```
#[arbiter_macros::main(
    name = "ExampleArbiterProject",
    about = "Our example to get you started.",
    behaviors = Behaviors
)]
pub async fn main() {}
