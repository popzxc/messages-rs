use std::pin::Pin;

use async_trait::async_trait;
use futures::channel::oneshot;

use crate::{handler::Notifiable, Actor, Context, Handler};

#[async_trait]
pub(crate) trait EnvelopeProxy<A: Actor + Unpin>: Send + 'static {
    async fn handle(&mut self, actor: Pin<&mut A>, context: Pin<&mut Context<A>>);
}

pub(crate) struct Envelope<A: Handler<IN>, IN> {
    message: Option<IN>,
    response: Option<oneshot::Sender<A::Result>>,
    _marker: std::marker::PhantomData<A>,
}

impl<A, IN> Envelope<A, IN>
where
    A: Handler<IN>,
{
    pub(crate) fn new(message: IN, response: oneshot::Sender<A::Result>) -> Self {
        Self {
            message: Some(message),
            response: Some(response),
            _marker: std::marker::PhantomData,
        }
    }
}

#[async_trait]
impl<A, IN> EnvelopeProxy<A> for Envelope<A, IN>
where
    A: Handler<IN> + Actor + Send + Unpin,
    IN: Send + 'static,
    A::Result: Send + 'static,
{
    async fn handle(&mut self, actor: Pin<&mut A>, context: Pin<&mut Context<A>>) {
        let message = self
            .message
            .take()
            .expect("`Envelope::handle` called twice");
        let response = self.response.take().unwrap();

        let result = actor.get_mut().handle(message, context.get_mut()).await;
        let _ = response.send(result);
    }
}

pub(crate) struct NotifyEnvelope<A: Notifiable<IN>, IN> {
    message: Option<IN>,
    _marker: std::marker::PhantomData<A>,
}

impl<A, IN> NotifyEnvelope<A, IN>
where
    A: Notifiable<IN>,
{
    pub(crate) fn new(message: IN) -> Self {
        Self {
            message: Some(message),
            _marker: std::marker::PhantomData,
        }
    }
}

#[async_trait]
impl<A, IN> EnvelopeProxy<A> for NotifyEnvelope<A, IN>
where
    A: Notifiable<IN> + Actor + Send + Unpin,
    IN: Send + 'static,
{
    async fn handle(&mut self, actor: Pin<&mut A>, context: Pin<&mut Context<A>>) {
        let message = self
            .message
            .take()
            .expect("`Envelope::handle` called twice");

        actor.get_mut().notify(message, context.get_mut()).await;
    }
}
