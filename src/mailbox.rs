use crate::{
    address::{Address, Message},
    errors::ReceiveError,
};
use futures::{channel::mpsc, StreamExt};
use std::future::Future;

/// Default capacity for the mailbox.
pub const DEFAULT_CAPACITY: usize = 128;

/// Mailbox is an entity capable of receiving messages.
/// It represents a receiver side of communication, and the sender side is represented using [Address](../address/struct.Address.html).
#[derive(Debug)]
pub struct Mailbox<Input> {
    stopped: bool,
    receiver: mpsc::Receiver<Message<Input>>,
    address: Address<Input>,
}

impl<Input> Default for Mailbox<Input> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Input> Mailbox<Input> {
    /// Creates a new `Mailbox` with a [DEFAULT_CAPACITY].
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_CAPACITY)
    }

    /// Creates a new `Mailbox` with a provided capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        let (sender, receiver) = mpsc::channel(capacity);

        let address = Address::new(sender);
        let stopped = false;

        Self {
            stopped,
            receiver,
            address,
        }
    }

    /// Creates an [Address] object to communicate with this `Mailbox`.
    pub fn address(&self) -> Address<Input> {
        self.address.clone()
    }

    /// Awaits for the any message to come and returns it.
    /// Returns an error if either this `Mailbox` has received a stop request, but was not resumed,
    /// or if all the senders have disconnected already.
    pub async fn receive(&mut self) -> Result<Input, ReceiveError> {
        if self.stopped {
            return Err(ReceiveError::Stopped);
        }

        if let Some(message) = self.receiver.next().await {
            match message {
                Message::Message(input) => Ok(input),
                Message::StopRequest => Err(ReceiveError::Stopped),
            }
        } else {
            Err(ReceiveError::AllSendersDisconnected)
        }
    }

    /// Runs an infinite loop which will handle all the incoming requests.
    /// Loop may exit with an `Ok(())` value if this `Mailbox` will receive a stop request,
    /// or with an `Err` value if all the senders will disconnect without providing a stop request.
    pub async fn run_with<F, Fut>(mut self, mut handler: F) -> Result<(), ReceiveError>
    where
        F: FnMut(Input) -> Fut,
        Fut: Future<Output = ()>,
    {
        self.stopped = false;
        while let Some(message) = self.receiver.next().await {
            match message {
                Message::Message(data) => {
                    handler(data).await;
                }
                Message::StopRequest => {
                    return Ok(());
                }
            }
        }

        Err(ReceiveError::AllSendersDisconnected)
    }

    /// Enables a mailbox again after it received a stop request.
    pub fn resume(&mut self) {
        self.stopped = false;
    }
}
