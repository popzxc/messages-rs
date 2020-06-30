use crate::errors::SendError;
use anyhow::Result;
use futures::channel::oneshot;

#[derive(Debug)]
pub struct Request<Input, Output> {
    message: Input,
    respond_channel: oneshot::Sender<Output>,
}

impl<Input, Output> Request<Input, Output> {
    pub fn new(message: Input) -> (Self, oneshot::Receiver<Output>) {
        let (respond_channel, receiver_channel) = oneshot::channel();

        let message = Self {
            message,
            respond_channel,
        };

        (message, receiver_channel)
    }

    pub fn inner(&self) -> &Input {
        &self.message
    }

    pub fn into_inner(self) -> (Input, oneshot::Sender<Output>) {
        (self.message, self.respond_channel)
    }

    pub async fn respond(self, output: Output) -> Result<()> {
        self.respond_channel
            .send(output)
            .map_err(|_| SendError::ReceiverDisconnected)?;

        Ok(())
    }
}
