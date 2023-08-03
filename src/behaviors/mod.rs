


// struct DeployBehavior {
//     name: String,
// }

// #[async_trait::async_trait]
// impl Behavior for DeployBehavior {
//     async fn process_event(&mut self) -> bool {
//         self.deploy();
//         true
//     }
//     fn sync_state(&mut self) {
//         todo!()
//     }
// }

// impl DeployBehavior {
//     pub fn new(name: String) -> Self {
//         Self { name }
//     }

//     fn deploy(&self) {
//         // deploy counter contract
//         let counter = Counter::deploy(self.client.clone(), ())?.send().await?;
//         println!("Deployed Counter to address: {}", counter.address());
//     }
// }

