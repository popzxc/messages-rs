use futures::future::BoxFuture;

pub trait Handler<IN, OUT> {
    fn handle(&self, input: IN) -> BoxFuture<OUT>;
}
