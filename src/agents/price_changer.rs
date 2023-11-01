use crate::agents::*;
use crate::settings::{parameters::GBMParameters, SimulationConfig};
use arbiter_core::bindings::liquid_exchange::LiquidExchange;
use arbiter_core::environment::Environment;
use arbiter_core::math::{float_to_wad, GeometricBrownianMotion, StochasticProcess, Trajectories};
use arbiter_core::middleware::RevmMiddleware;
use ethers::utils::parse_ether;

/// The [`PriceChanger`] holds the data and has methods that allow it to update
/// the price of the [`LiquidExchange`].
pub struct PriceChanger {
    /// The path the price process takes.
    pub trajectory: Trajectories,

    /// The [`LiquidExchange`] contract with the admin `Client`.
    pub liquid_exchange: LiquidExchange<RevmMiddleware>,

    /// The index of the current price in the trajectory.
    pub index: usize,
}

impl PriceChanger {
    /// Create a new [`PriceChanger`] with the given [`LiquidExchange`] contract
    /// bound to the admin `Client`. The [`PriceChanger`] will use the
    /// `OrnsteinUhlenbeck` process to generate a price trajectory with the
    /// constants defined in `config.rs`.
    /// Ornstein-Uhlenbeck processes are useful for modeling the price of stable
    /// tokens.
    pub async fn new(
        environment: &Environment,
        token_admin: &token_admin::TokenAdmin,
        config: &SimulationConfig<Fixed>,
    ) -> Result<Self> {
        let client = RevmMiddleware::new(environment, "price_changer".into())?;
        let liquid_exchange = LiquidExchange::deploy(
            client,
            (
                token_admin.arbx.address(),
                token_admin.arby.address(),
                float_to_wad(config.trajectory.initial_price.0),
            ),
        )?
        .send()
        .await?;

        token_admin
            .mint(
                liquid_exchange.address(),
                parse_ether(100_000_000_000_u64).unwrap(),
                parse_ether(100_000_000_000_u64).unwrap(),
            )
            .await?;

        let trajectory_params = &config.trajectory;
        let trajectory = match trajectory_params.process.as_str() {
            "gbm" => {
                let GBMParameters { drift, volatility } = config.gbm.unwrap();
                GeometricBrownianMotion::new(drift.0, volatility.0).seedable_euler_maruyama(
                    trajectory_params.initial_price.0,
                    trajectory_params.t_0.0,
                    trajectory_params.t_n.0,
                    trajectory_params.num_steps,
                    1,
                    false,
                    trajectory_params.seed,
                )
            }
            _ => panic!("Invalid process type"),
        };

        Ok(Self {
            trajectory,
            liquid_exchange,
            index: 1, /* start after the initial price since it is already set on contract
                       * deployment */
        })
    }

    /// Update the price of the [`LiquidExchange`] contract to the next price in
    /// the trajectory and increment the index.
    pub async fn update_price(&mut self) -> Result<()> {
        let price = self.trajectory.paths[0][self.index];
        self.liquid_exchange
            .set_price(arbiter_core::math::float_to_wad(price))
            .send()
            .await?
            .await?;
        self.index += 1;
        Ok(())
    }
}

#[async_trait::async_trait]
impl Agent for PriceChanger {
    async fn step(&mut self) -> Result<()> {
        self.update_price().await?;
        Ok(())
    }
}
