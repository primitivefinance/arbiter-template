use std::sync::Arc;

use arbiter_core::{
    bindings::arbiter_token::ArbiterToken, environment::Environment, middleware::RevmMiddleware,
};
use ethers::types::{Address, U256};

use super::*;

/// Manages the administrative operations for two types of tokens within the simulation environment.
/// The token admin is responsible for minting tokens to agents and other contracts.
#[derive(Clone)]
pub struct TokenAdmin {
    /// The client interface for interacting with the [RevmMiddleware].
    pub client: Arc<RevmMiddleware>,

    /// The arbiter token X contract.
    pub arbx: ArbiterToken<RevmMiddleware>,

    /// The arbiter token Y contract.
    pub arby: ArbiterToken<RevmMiddleware>,
}

impl TokenAdmin {
    /// Creates a new [`TokenAdmin`] instance, deploying two ArbiterToken contracts.
    ///
    /// # Arguments
    /// * [`Environment`] - The simulation environment containing blockchain network configurations.
    ///
    /// # Returns
    /// * [`Result<Self>`] - The result of the operation, yielding a new [`TokenAdmin`] if successful.
    pub async fn new(environment: &Environment) -> Result<Self> {
        let client = RevmMiddleware::new(environment, "token_admin".into())?;
        let decimals = 18_u8;

        let arbx = ArbiterToken::deploy(
            client.clone(),
            ("Arbiter Token X".to_string(), "arbx".to_string(), decimals),
        )?
        .send()
        .await?;
        let arby = ArbiterToken::deploy(
            client.clone(),
            ("Arbiter Token Y".to_string(), "arby".to_string(), decimals),
        )?
        .send()
        .await?;

        Ok(Self { client, arbx, arby })
    }

    pub async fn mint(&self, to: Address, amount_x: U256, amount_y: U256) -> Result<()> {
        self.arbx.mint(to, amount_x).send().await?.await?;
        self.arby.mint(to, amount_y).send().await?.await?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl Agent for TokenAdmin {}
