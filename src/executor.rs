use arbiter_core::Executor;

pub struct RevmExecutor {
    pub client: Arc<RevmMiddleware>,
}

impl Executor for RevmExecutor {
    async fn execute(&self, action: ()) -> Result<()> {
        todo!()
    }

}