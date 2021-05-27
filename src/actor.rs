use async_trait::async_trait;

use crate::{cfg_runtime, Address, Context};

#[async_trait]
pub trait Actor: Unpin + Send + Sync + Sized + 'static {
    async fn started(&self) {}
    async fn stopped(&self) {}

    async fn run(self) {
        Context::new().run(self).await;
    }

    async fn create_and_run<F>(f: F)
    where
        F: FnOnce(&mut Context<Self>) -> Self + Send,
    {
        let mut context = Context::new();
        let this = f(&mut context);
        context.run(this).await;
    }

    cfg_runtime! {
        fn spawn(self) -> Address<Self> {
            Context::new().spawn(self)
        }

        fn create_and_spawn<F>(f: F) -> Address<Self>
        where
            F: FnOnce(&mut Context<Self>) -> Self + Send,
        {
            let mut context = Context::new();
            let this = f(&mut context);
            context.spawn(this)
        }
    }
}
