use async_trait::async_trait;

use crate::{cfg_runtime, Address, Context};

#[async_trait]
pub trait Actor: Unpin + Send + Sync + Sized + 'static {
    fn created(&self) {}
    async fn started(&self) {}
    async fn stopped(&self) {}

    async fn run(self) {
        Context::new().run(self).await;
    }

    cfg_runtime! {
        fn spawn(self) -> Address<Self> {
            Context::new().spawn(self)
        }
    }
}
