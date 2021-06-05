//! An address of an actor.
//!
//! See [`Actor`] documentation for details.

use std::sync::Arc;

use crate::{
    actor::Actor,
    cfg_runtime,
    context::{InputHandle, Signal},
    envelope::{EnvelopeProxy, MessageEnvelope, NotificationEnvelope},
    errors::SendError,
    handler::{Handler, Notifiable},
};
use futures::{
    channel::{mpsc, oneshot},
    lock::Mutex,
    SinkExt, Stream, StreamExt,
};

/// `Address` is an object used to communicate with [`Actor`]s.
///
/// Assuming that [`Actor`] is capable of processing messages of a certain
/// type, the [`Address`] can be used to interact with [`Actor`] by using
/// either [`Address::send`] (for messages) or [`Address::notify`] (for notifications).
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

    /// Sends a message to the [`Actor`] and receives the response.
    ///
    /// ## Examples
    ///
    /// This example assumes that `messages` is used with `rt-tokio` feature enabled.
    ///
    /// ```rust
    /// # use messages::prelude::*;
    ///
    /// struct Sum;
    ///
    /// #[async_trait]
    /// impl Actor for Sum {}
    ///
    /// #[async_trait]
    /// impl Handler<(u8, u8)> for Sum {
    ///     type Result = u16;
    ///     // Implementation omitted.
    ///     # async fn handle(&mut self, (a, b): (u8, u8), context: &Context<Self>) -> u16 {
    ///     #    (a as u16) + (b as u16)
    ///     # }
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///    let mut addr = Sum.spawn();
    ///    let result = addr.send((22, 20)).await.unwrap();
    ///    assert_eq!(result, 42);
    ///    # addr.stop().await;
    ///    # addr.wait_for_stop().await;  
    /// }
    /// ```
    pub async fn send<IN>(&mut self, message: IN) -> Result<A::Result, SendError>
    where
        A: Actor + Send + Handler<IN> + 'static,
        IN: Send + 'static,
        A::Result: Send + 'static,
    {
        let (sender, receiver) = oneshot::channel();
        let envelope: MessageEnvelope<A, IN> = MessageEnvelope::new(message, sender);

        let message = Box::new(envelope) as Box<dyn EnvelopeProxy<A> + Send + 'static>;

        self.sender
            .send(message)
            .await
            .map_err(|_| SendError::ReceiverDisconnected)?;

        receiver.await.map_err(|_| SendError::ReceiverDisconnected)
    }

    /// Sends a notification to the [`Actor`] without receiving any kind of response.
    ///
    /// ## Examples
    ///
    /// This example assumes that `messages` is used with `rt-tokio` feature enabled.
    ///
    /// ```rust
    /// # use messages::prelude::*;
    ///
    /// struct Ping;
    ///
    /// #[async_trait]
    /// impl Actor for Ping {}
    ///
    /// #[async_trait]
    /// impl Notifiable<u8> for Ping {
    ///     async fn notify(&mut self, input: u8, context: &Context<Self>) {
    ///         println!("Received number {}", input);
    ///     }
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///    let mut addr = Ping.spawn();
    ///    addr.notify(42).await.unwrap();
    ///    # addr.stop().await;
    ///    # addr.wait_for_stop().await;  
    /// }
    /// ```
    pub async fn notify<IN>(&mut self, message: IN) -> Result<(), SendError>
    where
        A: Actor + Send + Notifiable<IN> + 'static,
        IN: Send + 'static,
    {
        let envelope: NotificationEnvelope<A, IN> = NotificationEnvelope::new(message);

        let message = Box::new(envelope) as Box<dyn EnvelopeProxy<A> + Send + 'static>;

        self.sender
            .send(message)
            .await
            .map_err(|_| SendError::ReceiverDisconnected)?;

        Ok(())
    }

    /// Combines provided stream and this `Address` object, returning a future
    /// that will run while stream yields messages and send them to the server.
    ///
    /// [`Actor`] associated with this `Address` must implmenet [`Notifiable`] trait
    /// to process messages from the stream.
    ///
    /// Future returned by this method should not normally be directly `await`ed,
    /// but rather is expected to be used in some kind of `spawn` function of
    /// the used runtime (e.g. `tokio::spawn` or `async_std::task::spawn`).
    pub async fn into_stream_forwarder<IN, S>(mut self, mut stream: S) -> Result<(), SendError>
    where
        A: Actor + Send + Notifiable<IN> + 'static,
        S: Send + Stream<Item = IN> + Unpin,
        IN: Send + 'static,
    {
        while let Some(message) = stream.next().await {
            self.notify(message).await?;
        }
        Ok(())
    }

    /// Returns `true` if `Address` is still connected to the [`Actor`].
    pub fn connected(&self) -> bool {
        !self.sender.is_closed()
    }

    /// Sends a stop request to the corresponding [`Actor`].
    ///
    /// Sending this message does not mean that actor will be stopped immediately.
    /// In order to make sure that the actor is stopped, [`Address::wait_for_stop`]
    /// should be used.
    ///
    /// Does nothing if address is disconnected from the actor or actor already has
    /// been stopped.
    pub async fn stop(&mut self) {
        // If actor is already stopped, we're fine with it.
        let _ = self.signal_sender.send(Signal::Stop).await;
    }

    /// Creates a future that waits for actor to be fully stopped.
    ///
    /// Note that this method does not request an actor to stop, it only waits for it
    /// in order to stop actor, [`Address::stop`] should be used.
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

cfg_runtime! {

use crate::{
    handler::Coroutine,
    envelope::CoroutineEnvelope
};

impl<A> Address<A> {
    /// Version of [`Address::into_stream_forwarder`] that automatically spawns the future.
    ///
    /// Returned future is the join handle of the spawned task, e.g. it can be awaited
    /// if the user is interested in the moment when the stream stopped sending messages.
    pub fn spawn_stream_forwarder<IN, S>(self, stream: S) -> crate::runtime::JoinHandle<Result<(), SendError>>
    where
        A: Actor + Send + Notifiable<IN> + 'static,
        S: Send + Stream<Item = IN> + Unpin + 'static,
        IN: Send + 'static,
    {
        crate::runtime::spawn(self.into_stream_forwarder(stream))
    }


    /// Sends a message to the [`Actor`] and receives the response.
    /// Unlike in [`Address::send`], `calculate` supports parallel execution.
    ///
    /// ## Examples
    ///
    /// This example assumes that `messages` is used with `rt-tokio` feature enabled.
    ///
    /// ```rust
    /// # use messages::prelude::*;
    /// #[derive(Clone)]
    /// struct Sum;
    ///
    /// #[async_trait]
    /// impl Actor for Sum {}
    ///
    /// #[async_trait]
    /// impl Coroutine<(u8, u8)> for Sum {
    ///     type Result = u16;
    ///     async fn calculate(self, (a, b): (u8, u8)) -> u16 {
    ///         (a as u16) + (b as u16)
    ///     }
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///    let mut addr = Sum.spawn();
    ///    let result = addr.calculate((22, 20)).await.unwrap();
    ///    assert_eq!(result, 42);
    ///    # addr.stop().await;
    ///    # addr.wait_for_stop().await;
    /// }
    /// ```
    pub async fn calculate<IN>(&self, message: IN) -> Result<A::Result, SendError>
    where
        A: Actor + Send + Coroutine<IN> + 'static,
        IN: Send + 'static,
        A::Result: Send + 'static,
    {
        let mut addr = self.sender.clone();
        let (sender, receiver) = oneshot::channel();
        let envelope: CoroutineEnvelope<A, IN> = CoroutineEnvelope::new(message, sender);

        let message = Box::new(envelope) as Box<dyn EnvelopeProxy<A> + Send + 'static>;

        addr
            .send(message)
            .await
            .map_err(|_| SendError::ReceiverDisconnected)?;

        receiver.await.map_err(|_| SendError::ReceiverDisconnected)
    }
}
}
