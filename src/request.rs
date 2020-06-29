use crate::errors::SendError;
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

    pub async fn respond(self, output: Output) -> Result<(), SendError> {
        self.respond_channel
            .send(output)
            .map_err(|_| SendError::ReceiverDisconnected)?;

        Ok(())
    }
}
