use std::pin::Pin;

use async_trait::async_trait;
use futures::channel::oneshot;

use crate::{Actor, Handler};

#[async_trait]
pub(crate) trait EnvelopeProxy<A: Actor + Unpin>: Send + 'static {
    async fn handle(&mut self, actor: Pin<&mut A>);
}

pub(crate) struct Envelope<IN, OUT> {
    message: Option<IN>,
    response: Option<oneshot::Sender<OUT>>,
}

impl<IN, OUT> Envelope<IN, OUT> {
    pub(crate) fn new(message: IN, response: oneshot::Sender<OUT>) -> Self {
        Self {
            message: Some(message),
            response: Some(response),
        }
    }
}

#[async_trait]
impl<A, IN, OUT> EnvelopeProxy<A> for Envelope<IN, OUT>
where
    A: Handler<IN, OUT> + Actor + Send + Unpin,
    IN: Send + 'static,
    OUT: Send + 'static,
{
    async fn handle(&mut self, actor: Pin<&mut A>) {
        let message = self
            .message
            .take()
            .expect("`Envelope::handle` called twice");
        let response = self.response.take().unwrap();

        let result = actor.get_mut().handle(message).await;
        let _ = response.send(result);
    }
}
