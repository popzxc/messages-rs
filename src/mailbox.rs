use crate::{address::Address, errors::ReceiveError};
use futures::{channel::mpsc, StreamExt};
use std::future::Future;

// TODO: `stop` request should be supported
#[derive(Debug)]
pub struct Mailbox<Input> {
    receiver: mpsc::Receiver<Input>,
    address: Address<Input>,
}

impl<Input> Mailbox<Input> {
    pub fn new() -> Self {
        // TODO limit should be configurable
        let (sender, receiver) = mpsc::channel(128);

        let address = Address::new(sender);

        Self { receiver, address }
    }

    pub fn address(&self) -> Address<Input> {
        self.address.clone()
    }

    pub async fn receive(&mut self) -> Result<Input, ReceiveError> {
        if let Some(message) = self.receiver.next().await {
            Ok(message)
        } else {
            Err(ReceiveError::AllSendersDisconnected)
        }
    }

    // TODO: `self` is not required as an argument, maybe make it an external function?
    pub async fn run_with<F, Fut>(mut self, mut handler: F) -> Result<(), ReceiveError>
    where
        F: FnMut(&mut Self, Input) -> Fut,
        Fut: Future<Output = ()>,
    {
        // TODO: There should be a possibility to stop mailbox.
        while let Some(message) = self.receiver.next().await {
            handler(&mut self, message).await;
        }

        Err(ReceiveError::AllSendersDisconnected)
    }
}
