use crate::errors::SendError;
use futures::channel::oneshot;

#[derive(Debug)]

pub struct Message<Input, Output> {
    message: Input,
    respond_channel: Option<oneshot::Sender<Output>>,
}

impl<Input, Output> Message<Input, Output> {
    pub fn request(message: Input) -> (Self, oneshot::Receiver<Output>) {
        let (respond_channel, receiver_channel) = oneshot::channel();

        let message = Self {
            message,
            respond_channel: Some(respond_channel),
        };

        (message, receiver_channel)
    }

    pub fn notification(message: Input) -> Self {
        Self {
            message,
            respond_channel: None,
        }
    }

    pub fn is_request(&self) -> bool {
        self.respond_channel.is_some()
    }

    pub async fn respond(self, output: Output) -> Result<(), SendError> {
        if let Some(channel) = self.respond_channel {
            channel
                .send(output)
                .map_err(|_| SendError::ReceiverDisconnected)?;

            Ok(())
        } else {
            Err(SendError::NoResponseExpected)
        }
    }
}
