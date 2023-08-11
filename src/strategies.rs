use arbiter_core::strategies::{SimulationActions, SimulationEvents};
use arbiter_core::{environment::RevmResult, middleware::RevmMiddleware};
use artemis_core::types::Strategy;
use std::sync::Arc;

use crate::bindings::counter::Counter;
use anyhow::Result;
use async_trait::async_trait;

pub struct CounterStrategy {
    label: String,
    client: Arc<RevmMiddleware>,
    counter: Option<Counter<RevmMiddleware>>,
    sender: crossbeam_channel::Sender<SimulationEvents>,
}

impl CounterStrategy {
    pub fn new(
        label: String,
        client: Arc<RevmMiddleware>,
        sender: crossbeam_channel::Sender<SimulationEvents>,
    ) -> Self {
        Self {
            label,
            client,
            counter: None,
            sender,
        }
    }
}

#[async_trait]
impl Strategy<SimulationEvents, SimulationActions> for CounterStrategy {
    async fn sync_state(&mut self) -> Result<()> {
        let counter = Counter::deploy(self.client.clone(), ())
            .unwrap()
            .send()
            .await
            .unwrap();
        self.counter = Some(counter);
        self.sender
            .send(SimulationEvents::Message(
                "Counter contract deployed".to_string(),
            ))
            .unwrap();
        self.sender
            .send(SimulationEvents::Message("Second message".to_string()))
            .unwrap();
        println!("contract deployed");
        Ok(())
    }

    async fn process_event(&mut self, _event: SimulationEvents) -> Vec<SimulationActions> {
        self.sender
            .send(SimulationEvents::Message(
                "Counter contract call".to_string(),
            ))
            .unwrap();
        match _event {
            SimulationEvents::Message(_event) => {
                if let Some(counter) = &self.counter {
                    let contract_call = counter.increment();
                    println!("contract call");
                    vec![SimulationActions::ContractSend(contract_call)]
                } else {
                    vec![]
                }
            }
            _ => vec![],
        }
    }
}
