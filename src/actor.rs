use async_trait::async_trait;

use crate::Context;

#[async_trait]
pub trait Actor: Unpin + Send + Sync + Sized + 'static {
    fn created(&self) {}
    async fn started(&self) {}
    async fn stopped(&self) {}

    fn into_runner(self) -> Context<Self> {
        Context::new(self)
    }

    fn into_runner_with_capacity(self, capacity: usize) -> Context<Self> {
        Context::with_capacity(self, capacity)
    }
}
