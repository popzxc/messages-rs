use crate::errors::SendError;
use anyhow::Result;
use futures::channel::oneshot;

/// `Request` structure represents a message which expects a response.
/// Initializing a `Request` will return two objects:
/// - A request itself, which has to be sent to the `Mailbox`;
/// - A oneshot receiver channel to await for the response.
#[derive(Debug)]
pub struct Request<Input, Output> {
    message: Input,
    respond_channel: oneshot::Sender<Output>,
}

impl<Input, Output> Request<Input, Output> {
    /// Initializes a new request, returning a `Request` object and a receiver channel to await for the response.
    pub fn new(message: Input) -> (Self, oneshot::Receiver<Output>) {
        let (respond_channel, receiver_channel) = oneshot::channel();

        let message = Self {
            message,
            respond_channel,
        };

        (message, receiver_channel)
    }

    /// Returns a reference to the message held in the request.
    pub fn message(&self) -> &Input {
        &self.message
    }

    /// Sends a response to the request initiator.
    pub fn respond(self, output: Output) -> Result<()> {
        self.respond_channel
            .send(output)
            .map_err(|_| SendError::ReceiverDisconnected)?;

        Ok(())
    }

    /// Destructs the request object, returning the request message and the sender channel.
    pub fn into_inner(self) -> (Input, oneshot::Sender<Output>) {
        (self.message, self.respond_channel)
    }
}
