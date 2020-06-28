use crate::{errors::SendError, message::Message};
use futures::{channel::mpsc, SinkExt};

#[derive(Debug)]
pub struct Address<Input, Output> {
    sender: mpsc::Sender<Message<Input, Output>>,
}

impl<Input, Output> Clone for Address<Input, Output> {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
        }
    }
}

impl<Input, Output> Address<Input, Output> {
    pub(crate) fn new(sender: mpsc::Sender<Message<Input, Output>>) -> Self {
        Self { sender }
    }

    pub async fn request(&mut self, message: Input) -> Result<Output, SendError> {
        let (wrapped, receiver) = Message::request(message);
        self.sender
            .send(wrapped)
            .await
            .map_err(|_| SendError::ReceiverDisconnected)?;

        let response = receiver
            .await
            .map_err(|_| SendError::ReceiverDisconnected)?;
        Ok(response)
    }

    pub async fn notify(&mut self, message: Input) -> Result<(), SendError> {
        let wrapped = Message::notification(message);
        self.sender
            .send(wrapped)
            .await
            .map_err(|_| SendError::ReceiverDisconnected)?;

        Ok(())
    }
}
