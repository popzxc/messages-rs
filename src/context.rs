use std::{pin::Pin, sync::Arc};

use crate::{actor::Actor, address::Address, cfg_runtime, envelope::EnvelopeProxy};
use futures::{channel::mpsc, lock::Mutex, StreamExt};

#[derive(Debug, Clone)]
pub(crate) enum Signal {
    Stop,
}

/// Default capacity for the mailbox.
pub const DEFAULT_CAPACITY: usize = 128;

pub(crate) type InputHandle<A> = Box<dyn EnvelopeProxy<A> + Send + 'static>;

pub struct Context<ACTOR> {
    receiver: mpsc::Receiver<InputHandle<ACTOR>>,
    signal_receiver: mpsc::Receiver<Signal>,
    address: Address<ACTOR>,
    stop_handle: Arc<Mutex<()>>,
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
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_CAPACITY)
    }

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

    pub fn address(&self) -> Address<ACTOR> {
        self.address.clone()
    }

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
                            let self_pin = Pin::new(&mut self);
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
                            running = false;
                        }
                    }
                }
            }
        }

        actor.stopped().await;
    }

    cfg_runtime! {
        pub fn spawn(self, actor: ACTOR) -> Address<ACTOR> {
            let address = self.address();
            crate::runtime::spawn(self.run(actor));
            address
        }
    }
}
