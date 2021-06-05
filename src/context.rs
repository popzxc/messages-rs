//! Context represents an environment in which actor is being executed.
//!
//! For details, see the [`Context`] documentation.

use std::{pin::Pin, sync::Arc};

use crate::{actor::Actor, address::Address, cfg_runtime, envelope::EnvelopeProxy};
use futures::{channel::mpsc, lock::Mutex, StreamExt};

#[derive(Debug)]
pub(crate) enum Signal {
    Stop,
}

/// Default capacity for the mailbox.
pub const DEFAULT_CAPACITY: usize = 128;

pub(crate) type InputHandle<A> = Box<dyn EnvelopeProxy<A> + Send + 'static>;

/// `Context` represents an environment in which actor is being executed.
///
/// It is capable of transferring incoming messages to the actor, providing
/// actor's address and managing it lifetime (e.g. stopping it).
pub struct Context<ACTOR> {
    receiver: mpsc::Receiver<InputHandle<ACTOR>>,
    signal_receiver: mpsc::Receiver<Signal>,
    address: Address<ACTOR>,
    stop_handle: Arc<Mutex<()>>,
}

impl<ACTOR> std::fmt::Debug for Context<ACTOR> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Context").finish()
    }
}

impl<ACTOR> Default for Context<ACTOR>
where
    ACTOR: 'static + Send + Actor + Unpin,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<ACTOR> Context<ACTOR>
where
    ACTOR: 'static + Send + Actor + Unpin,
{
    /// Creates a new `Context` object with default capacity (128 elements).
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_CAPACITY)
    }

    /// Creates a new `Context` object with custom capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        let (sender, receiver) = mpsc::channel(capacity);
        let (signal_sender, signal_receiver) = mpsc::channel(capacity);

        let stop_handle = Arc::new(Mutex::new(()));
        let address = Address::new(sender, signal_sender, stop_handle.clone());

        Self {
            receiver,
            signal_receiver,
            address,
            stop_handle,
        }
    }

    /// Returns an address of the actor.
    pub fn address(&self) -> Address<ACTOR> {
        self.address.clone()
    }

    /// Starts the message handling routine.
    ///
    /// Future returned by this method should not normally be directly `await`ed,
    /// but rather is expected to be used in some kind of `spawn` function of
    /// the used runtime (e.g. `tokio::spawn` or `async_std::task::spawn`).
    pub async fn run(mut self, mut actor: ACTOR) {
        // Acquire the lock on the mutex so addresses can be used to `await` until
        // the actor is stopped.
        let stop_handle = self.stop_handle.clone();
        let _mutex_handle = stop_handle.lock().await;

        actor.started().await;

        let mut running = true;
        while running {
            futures::select! {
                result = self.receiver.next() => {
                    match result {
                        Some(mut envelope) => {
                            let actor_pin = Pin::new(&mut actor);
                            let self_pin = Pin::new(&self);
                            envelope.handle(actor_pin, self_pin).await;
                        }
                        None => {
                            // ALl senders disconnected, stopping.
                            running = false;
                        }
                    }
                },
                signal = self.signal_receiver.next() => {
                    match signal {
                        Some(Signal::Stop) | None => {
                            // Notify actor about being stopped.
                            actor.stopping().await;
                            running = false;
                        }
                    }
                }
            }
        }

        // Notify actor that it was fully stopped.
        actor.stopped();
    }
}

cfg_runtime! {
use futures::FutureExt;

impl<ACTOR> Context<ACTOR>
where
    ACTOR: 'static + Send + Actor + Unpin,
{
    /// Spawns an actor and returns its address.
    pub fn spawn(self, actor: ACTOR) -> Address<ACTOR> {
        let address = self.address();
        let _handle = crate::runtime::spawn(self.run(actor)).boxed();
        address
    }
}
}
