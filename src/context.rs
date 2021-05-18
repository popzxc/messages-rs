use std::pin::Pin;

use crate::{actor::Actor, address::Address, envelope::EnvelopeProxy, errors::ReceiveError};
use futures::{channel::mpsc, StreamExt};

#[derive(Debug, Clone)]
pub(crate) enum Signal {
    Stop,
}

/// Default capacity for the mailbox.
pub const DEFAULT_CAPACITY: usize = 128;

pub(crate) type InputHandle<A> = Box<dyn EnvelopeProxy<A> + Send + 'static>;

pub struct Context<ACTOR> {
    actor: ACTOR,
    receiver: mpsc::Receiver<InputHandle<ACTOR>>,
    signal_receiver: mpsc::Receiver<Signal>,
    address: Address<ACTOR>,
}

impl<ACTOR> Context<ACTOR>
where
    ACTOR: 'static + Send + Actor + Unpin,
{
    pub fn new(actor: ACTOR) -> Self {
        Self::with_capacity(actor, DEFAULT_CAPACITY)
    }

    pub fn with_capacity(actor: ACTOR, capacity: usize) -> Self {
        let (sender, receiver) = mpsc::channel(capacity);
        let (signal_sender, signal_receiver) = mpsc::channel(capacity);

        let address = Address::new(sender, signal_sender);

        actor.created();

        Self {
            actor,
            receiver,
            signal_receiver,
            address,
        }
    }

    pub fn address(&self) -> Address<ACTOR> {
        self.address.clone()
    }

    pub async fn run(mut self) -> Result<(), ReceiveError> {
        self.actor.started().await;

        let mut running = true;
        while running {
            futures::select! {
                result = self.receiver.next() => {
                    match result {
                        Some(mut envelope) => {
                            let actor_pin = Pin::new(&mut self.actor);
                            envelope.handle(actor_pin).await;
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

        self.actor.stopped().await;
        Ok(())
    }
}
