use async_trait::async_trait;

#[async_trait]
pub trait Handler<IN> {
    type Result;

    async fn handle(&mut self, input: IN) -> Self::Result;
}
