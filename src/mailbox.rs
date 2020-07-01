use crate::{
    address::{Address, Message},
    errors::ReceiveError,
};
use anyhow::Result;
use futures::{channel::mpsc, StreamExt};
use std::future::Future;

pub const DEFAULT_CAPACITY: usize = 128;

#[derive(Debug)]
pub struct Mailbox<Input> {
    receiver: mpsc::Receiver<Message<Input>>,
    address: Address<Input>,
}

impl<Input> Mailbox<Input> {
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_CAPACITY)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let (sender, receiver) = mpsc::channel(capacity);

        let address = Address::new(sender);

        Self { receiver, address }
    }

    pub fn address(&self) -> Address<Input> {
        self.address.clone()
    }

    pub async fn receive(&mut self) -> Result<Input> {
        if let Some(message) = self.receiver.next().await {
            match message {
                Message::Message(input) => Ok(input),
                Message::StopRequest => Err(ReceiveError::Stopped)?,
            }
        } else {
            Err(ReceiveError::AllSendersDisconnected)?
        }
    }

    pub async fn run_with<F, Fut>(mut self, mut handler: F) -> Result<()>
    where
        F: FnMut(Input) -> Fut,
        Fut: Future<Output = ()>,
    {
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

        Err(ReceiveError::AllSendersDisconnected)?
    }
}
