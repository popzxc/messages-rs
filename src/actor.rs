use async_trait::async_trait;

use crate::ActorRunner;

#[async_trait]
pub trait Actor: Send + Sync + Sized + 'static {
    fn created(&self) {}
    async fn started(&self) {}
    async fn stopped(&self) {}

    fn into_runner(self) -> ActorRunner<Self> {
        ActorRunner::new(self)
    }

    fn into_runner_with_capacity(self, capacity: usize) -> ActorRunner<Self> {
        ActorRunner::with_capacity(self, capacity)
    }
}
