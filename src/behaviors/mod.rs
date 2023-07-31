use arbiter_core::agent::Behavior;

struct MyBehavior {
    name: String,
}

#[async_trait::async_trait]
impl Behavior for MyBehavior {
    async fn process_event(&mut self) -> bool {
        todo!()
    }
    fn sync_state(&mut self) {
        todo!()
    }
}