use std::sync::Arc;

use arbiter_core::{middleware::RevmMiddleware, environment::Environment, bindings::arbiter_token::ArbiterToken};
use ethers::types::{Address, U256};

use super::*;

#[derive(Clone)]
pub struct TokenAdmin {
    pub client: Arc<RevmMiddleware>,
    pub arbx: ArbiterToken<RevmMiddleware>,
    pub arby: ArbiterToken<RevmMiddleware>,
}

impl TokenAdmin {
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
