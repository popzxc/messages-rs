use crate::errors::SendError;
use futures::{channel::mpsc, SinkExt};

use anyhow::Result;

#[derive(Debug)]
pub(crate) enum Message<Input> {
    Message(Input),
    StopRequest,
}

impl<Input> From<Input> for Message<Input> {
    fn from(input: Input) -> Self {
        Self::Message(input)
    }
}

#[derive(Debug)]
pub struct Address<Input> {
    sender: mpsc::Sender<Message<Input>>,
}

impl<Input> Clone for Address<Input> {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
        }
    }
}

impl<Input> Address<Input> {
    pub(crate) fn new(sender: mpsc::Sender<Message<Input>>) -> Self {
        Self { sender }
    }

    pub async fn send(&mut self, message: Input) -> Result<()> {
        self.sender
            .send(message.into())
            .await
            .map_err(|_| SendError::ReceiverDisconnected)?;

        Ok(())
    }

    pub async fn stop(&mut self) -> Result<()> {
        self.sender
            .send(Message::StopRequest)
            .await
            .map_err(|_| SendError::ReceiverDisconnected)?;

        Ok(())
    }
}
