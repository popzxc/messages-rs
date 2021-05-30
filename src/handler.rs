use async_trait::async_trait;

use crate::{Actor, Context};

#[async_trait]
pub trait Notifiable<IN>: Sized + Actor {
    async fn notify(&mut self, input: IN, context: &mut Context<Self>);
}

#[async_trait]
pub trait Handler<IN>: Sized + Actor {
    type Result;

    async fn handle(&mut self, input: IN, context: &mut Context<Self>) -> Self::Result;
}
