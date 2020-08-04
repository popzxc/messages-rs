use crate::errors::SendError;
use futures::{channel::mpsc, SinkExt};

/// Internal wrapper over sent messages with additional types
/// of requests.
#[derive(Debug)]
pub(crate) enum Message<Input> {
    /// User-defined message.
    Message(Input),
    /// Request to stop the Mailbox.
    StopRequest,
}

impl<Input> From<Input> for Message<Input> {
    fn from(input: Input) -> Self {
        Self::Message(input)
    }
}

/// Address is an entity capable of sending messages.
/// It represents a sender side of communication, and the receiver side is represented using [Mailbox](../mailbox/struct.Mailbox.html).
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
    /// Internal constructor for the `Address` object.
    pub(crate) fn new(sender: mpsc::Sender<Message<Input>>) -> Self {
        Self { sender }
    }

    /// Sends a message to the corresponding `Mailbox`.
    pub async fn send(&mut self, message: Input) -> Result<(), SendError> {
        self.sender
            .send(message.into())
            .await
            .map_err(|_| SendError::ReceiverDisconnected)
    }

    /// Sends a stop request to the corresponding `Mailbox`.
    pub async fn stop(&mut self) -> Result<(), SendError> {
        self.sender
            .send(Message::StopRequest)
            .await
            .map_err(|_| SendError::ReceiverDisconnected)
    }
}
