use crate::{address::Address, errors::ReceiveError, message::Message};
use futures::{channel::mpsc, StreamExt};

#[derive(Debug)]
pub struct Mailbox<Input, Output> {
    receiver: mpsc::Receiver<Message<Input, Output>>,

    address: Address<Input, Output>,
}

impl<Input, Output> Mailbox<Input, Output> {
    pub fn new() -> Self {
        // TODO limit should be configurable
        let (sender, receiver) = mpsc::channel(128);

        let address = Address::new(sender);

        Self { receiver, address }
    }

    pub fn address(&self) -> Address<Input, Output> {
        self.address.clone()
    }

    pub async fn receive(&mut self) -> Result<Message<Input, Output>, ReceiveError> {
        if let Some(message) = self.receiver.next().await {
            Ok(message)
        } else {
            Err(ReceiveError::AllSendersDisconnected)
        }
    }
}
