use std::sync::Arc;

use crate::{
    errors::SendError,
    handler::Handler,
    mailbox::{InputHandle, Signal},
};
use futures::{
    channel::{mpsc, oneshot},
    FutureExt, SinkExt,
};

/// Address is an entity capable of sending messages.
/// It represents a sender side of communication, and the receiver side is represented using [Mailbox](../mailbox/struct.Mailbox.html).
pub struct Address<A> {
    sender: mpsc::Sender<InputHandle<A>>,
    signal_sender: mpsc::Sender<Signal>,
}

impl<A> Clone for Address<A> {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
            signal_sender: self.signal_sender.clone(),
        }
    }
}

impl<A> Address<A> {
    /// Internal constructor for the `Address` object.
    pub(crate) fn new(
        sender: mpsc::Sender<InputHandle<A>>,
        signal_sender: mpsc::Sender<Signal>,
    ) -> Self {
        Self {
            sender,
            signal_sender,
        }
    }

    /// Sends a message to the corresponding `Mailbox`.
    pub async fn send<IN, OUT>(&mut self, message: IN) -> Result<OUT, SendError>
    where
        A: Send + Handler<IN, OUT> + 'static,
        Arc<A>: Send,
        IN: Send + 'static,
        OUT: Send + 'static,
    {
        let (sender, receiver) = oneshot::channel();
        let handler = move |actor: Arc<A>| {
            let future = async move {
                let response_future = actor.handle(message);
                let output = response_future.await;
                let _ = sender.send(output);
            };

            future.boxed()
        };

        let message = Box::new(handler);

        self.sender
            .send(message)
            .await
            .map_err(|_| SendError::ReceiverDisconnected)?;

        receiver.await.map_err(|_| SendError::ReceiverDisconnected)
    }

    /// Sends a stop request to the corresponding `Mailbox`.
    pub async fn stop(&mut self) -> Result<(), SendError> {
        self.signal_sender
            .send(Signal::Stop)
            .await
            .map_err(|_| SendError::ReceiverDisconnected)
    }
}
