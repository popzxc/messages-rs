//! Envelope is an entity capable of encapsulating the sent message
//! together with a way to report the result back to the sender (if needed).
//! It consists of two parts:
//!
//! - `EnvelopeProxy` trait that is being used by the `Context` to
//!   pass the message to the actor (which is only accessable by
//!   the `Context` itself).
//! - `MessageEnvelope` and `NotificationEnvelope` structures that
//!   actually have the message inside of them and implement `EnvelopeProxy`.
//!
//! The way it works is as follows:
//!
//! - User calls `Address::send` / `Address::notify` with a message that
//!   can be handled by the corresponding `Actor` type.
//! - `Address` creates an `*Envelope` object and converts it to the
//!   `Box<dyn EnvelopeProxy>`. Information about the message type is now
//!   elided and we can consider different messages to be of the same type.
//! - This "envelope" is sent to the `Context` through a channel.
//! - Once `Context` processes envelope, it creates `Pin`s to both itself
//!   and `Actor` and calls `EnvelopeProxy::handle` to process the message.

use std::pin::Pin;

use async_trait::async_trait;

use crate::{
    cfg_runtime,
    prelude::{Actor, Context, Handler, Notifiable},
};

#[async_trait]
pub(crate) trait EnvelopeProxy<A: Actor + Unpin>: Send + 'static {
    async fn handle(&mut self, actor: Pin<&mut A>, context: Pin<&Context<A>>);
}

pub(crate) struct MessageEnvelope<A: Handler<IN>, IN> {
    data: Option<(IN, async_oneshot::Sender<A::Result>)>,
}

impl<A, IN> MessageEnvelope<A, IN>
where
    A: Handler<IN>,
{
    pub(crate) fn new(message: IN, response: async_oneshot::Sender<A::Result>) -> Self {
        Self {
            data: Some((message, response)),
        }
    }
}

#[async_trait]
impl<A, IN> EnvelopeProxy<A> for MessageEnvelope<A, IN>
where
    A: Handler<IN> + Actor + Send + Unpin,
    IN: Send + 'static,
    A::Result: Send + Sync + 'static,
{
    async fn handle(&mut self, actor: Pin<&mut A>, context: Pin<&Context<A>>) {
        let (message, mut response) = self.data.take().expect("`Envelope::handle` called twice");

        let result = actor
            .get_mut()
            .handle(message, Pin::into_inner(context))
            .await;
        let _ = response.send(result);
    }
}

pub(crate) struct NotificationEnvelope<A: Notifiable<IN>, IN> {
    message: Option<IN>,
    _marker: std::marker::PhantomData<A>,
}

impl<A, IN> NotificationEnvelope<A, IN>
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
impl<A, IN> EnvelopeProxy<A> for NotificationEnvelope<A, IN>
where
    A: Notifiable<IN> + Actor + Send + Unpin,
    IN: Send + 'static,
{
    async fn handle(&mut self, actor: Pin<&mut A>, context: Pin<&Context<A>>) {
        let message = self
            .message
            .take()
            .expect("`Envelope::handle` called twice");

        actor
            .get_mut()
            .notify(message, Pin::into_inner(context))
            .await;
    }
}

cfg_runtime! {
    use crate::handler::Coroutine;

    pub(crate) struct CoroutineEnvelope<A: Coroutine<IN>, IN> {
        data: Option<(IN, async_oneshot::Sender<A::Result>)>,
    }

    impl<A, IN> CoroutineEnvelope<A, IN>
    where
        A: Coroutine<IN>,
    {
        pub(crate) fn new(message: IN, response: async_oneshot::Sender<A::Result>) -> Self {
            Self {
                data: Some((message, response)),
            }
        }
    }

    #[async_trait]
    impl<A, IN> EnvelopeProxy<A> for CoroutineEnvelope<A, IN>
    where
        A: Coroutine<IN> + Actor + Send + Unpin,
        IN: Send + 'static,
        A::Result: Send + Sync + 'static,
    {
        async fn handle(&mut self, actor: Pin<&mut A>, _context: Pin<&Context<A>>) {
            let actor = Pin::into_inner(actor).clone();
            let (message, mut response) = self
                .data
                .take()
                .expect("`Envelope::handle` called twice");

            crate::runtime::spawn(async move {
                let result = actor.calculate(message).await;
                let _ = response.send(result);
            });
        }
    }
}
