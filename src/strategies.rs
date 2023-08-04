use std::sync::Arc;
use artemis_core::types::Strategy;
use arbiter_core::{middleware::RevmMiddleware, environment::RevmResult};

use async_trait::async_trait;
use ethers::providers::Middleware;
use anyhow::Result;
use crate::bindings::counter::Counter;


pub struct DeployStrategy<M> {
    label: String,
    client: Arc<M>,
}

impl<M: Middleware + 'static> DeployStrategy<M> {
    pub fn new(label: String, client: Arc<M>) -> Self {
        Self {
            label,
            client,
        }
    }
}

#[async_trait]
impl<M: Middleware + 'static> Strategy<RevmResult, ()> for DeployStrategy<M> {
    async fn sync_state(&mut self) -> Result<()> {
        todo!()
    }

    async fn process_event(&mut self, _event: RevmResult) -> Option<()> {

        let counter = Counter::deploy(self.client.clone(), ()).unwrap().send().await.unwrap();
        println!("Deployed Counter to address: {}", counter.address());
        Some(())

    }
}