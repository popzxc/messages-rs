use crate::errors::SendError;
use futures::{channel::mpsc, SinkExt};

#[derive(Debug)]
pub struct Address<Input> {
    sender: mpsc::Sender<Input>,
}

impl<Input> Clone for Address<Input> {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
        }
    }
}

impl<Input> Address<Input> {
    pub(crate) fn new(sender: mpsc::Sender<Input>) -> Self {
        Self { sender }
    }

    pub async fn send(&mut self, message: Input) -> Result<(), SendError> {
        self.sender
            .send(message)
            .await
            .map_err(|_| SendError::ReceiverDisconnected)?;

        Ok(())
    }
}
