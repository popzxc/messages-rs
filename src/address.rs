use std::sync::Arc;

use crate::{
    context::{InputHandle, Signal},
    envelope::{Envelope, EnvelopeProxy},
    errors::SendError,
    handler::Handler,
    Actor,
};
use futures::{
    channel::{mpsc, oneshot},
    lock::Mutex,
    SinkExt, Stream, StreamExt,
};

pub struct Address<A> {
    sender: mpsc::Sender<InputHandle<A>>,
    signal_sender: mpsc::Sender<Signal>,
    stop_handle: Arc<Mutex<()>>,
}

impl<A> std::fmt::Debug for Address<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Address").finish()
    }
}

impl<A> Clone for Address<A> {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
            signal_sender: self.signal_sender.clone(),
            stop_handle: self.stop_handle.clone(),
        }
    }
}

impl<A> Address<A> {
    pub(crate) fn new(
        sender: mpsc::Sender<InputHandle<A>>,
        signal_sender: mpsc::Sender<Signal>,
        stop_handle: Arc<Mutex<()>>,
    ) -> Self {
        Self {
            sender,
            signal_sender,
            stop_handle,
        }
    }

    pub async fn send<IN>(&mut self, message: IN) -> Result<A::Result, SendError>
    where
        A: Actor + Send + Handler<IN> + 'static,
        IN: Send + 'static,
        A::Result: Send + 'static,
    {
        let (sender, receiver) = oneshot::channel();
        let envelope: Envelope<A, IN> = Envelope::new(message, sender);

        let message = Box::new(envelope) as Box<dyn EnvelopeProxy<A> + Send + 'static>;

        self.sender
            .send(message)
            .await
            .map_err(|_| SendError::ReceiverDisconnected)?;

        receiver.await.map_err(|_| SendError::ReceiverDisconnected)
    }

    pub async fn into_stream_forwarder<IN, S>(mut self, mut stream: S) -> Result<(), SendError>
    where
        A: Actor + Send + Handler<IN> + 'static,
        S: Send + Stream<Item = IN> + Unpin,
        IN: Send + 'static,
        A::Result: Send + 'static,
    {
        while let Some(message) = stream.next().await {
            self.send(message).await?;
        }
        Ok(())
    }

    pub fn connected(&self) -> bool {
        !self.sender.is_closed()
    }

    /// Sends a stop request to the corresponding `Mailbox`.
    pub async fn stop(&mut self) {
        // If actor is already stopped, we're fine with it.
        let _ = self.signal_sender.send(Signal::Stop).await;
    }

    /// Creates a future that waits for actor to be fully stopped.
    pub async fn wait_for_stop(&self) {
        // We will only able to obtain the lock when context will release it.
        // However, we don't want to exit early in case this method is called
        // before actor is actually started, so we do it in the loop until
        // the channel is disconnected.
        while self.connected() {
            self.stop_handle.lock().await;
        }
    }
}
