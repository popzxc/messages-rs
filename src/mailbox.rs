use std::sync::Arc;

use crate::{address::Address, errors::ReceiveError};
use futures::{channel::mpsc, future::BoxFuture, StreamExt};

#[derive(Debug, Clone)]
pub(crate) enum Signal {
    Stop,
}

/// Default capacity for the mailbox.
pub const DEFAULT_CAPACITY: usize = 128;

pub(crate) type InputHandle<A> = Box<dyn FnOnce(Arc<A>) -> BoxFuture<'static, ()> + Send + 'static>;

/// Mailbox is an entity capable of receiving messages.
/// It represents a receiver side of communication, and the sender side is represented using [Address](../address/struct.Address.html).
pub struct Mailbox<ACTOR> {
    actor: Arc<ACTOR>,
    receiver: mpsc::Receiver<InputHandle<ACTOR>>,
    signal_receiver: mpsc::Receiver<Signal>,
    address: Address<ACTOR>,
}

impl<ACTOR> Mailbox<ACTOR>
where
    ACTOR: 'static + Send,
{
    /// Creates a new `Mailbox` with a [DEFAULT_CAPACITY].
    pub fn new(actor: ACTOR) -> Self {
        Self::with_capacity(actor, DEFAULT_CAPACITY)
    }

    /// Creates a new `Mailbox` with a provided capacity.
    pub fn with_capacity(actor: ACTOR, capacity: usize) -> Self {
        let actor = Arc::new(actor);

        let (sender, receiver) = mpsc::channel(capacity);
        let (signal_sender, signal_receiver) = mpsc::channel(capacity);

        let address = Address::new(sender, signal_sender);

        Self {
            actor,
            receiver,
            signal_receiver,
            address,
        }
    }

    /// Creates an [Address] object to communicate with this `Mailbox`.
    pub fn address(&self) -> Address<ACTOR> {
        self.address.clone()
    }

    pub async fn run(&mut self) -> Result<(), ReceiveError> {
        loop {
            futures::select! {
                result = self.receiver.next() => {
                    let handler = result.ok_or(ReceiveError::AllSendersDisconnected)?;
                    handler(self.actor.clone());
                },
                signal = self.signal_receiver.next() => {
                    let signal = signal.ok_or(ReceiveError::AllSendersDisconnected)?;
                    match signal {
                        Signal::Stop => {
                            return Ok(())
                        }
                    }
                }
            }
        }
    }
}
