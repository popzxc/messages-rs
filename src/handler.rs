use async_trait::async_trait;

#[async_trait]
pub trait Handler<IN, OUT = ()> {
    async fn handle(&self, input: IN) -> OUT;
}
