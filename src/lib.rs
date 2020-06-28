use futures::{
    channel::{mpsc, oneshot},
    SinkExt, StreamExt,
};

#[derive(Debug)]
pub enum SendError {
    ReceiverDisconnected,
    NoResponseExpected,
}

#[derive(Debug)]
pub enum ReceiveError {
    AllSendersDisconnected,
}

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

    pub fn message(message: Input) -> Self {
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
    fn new(sender: mpsc::Sender<Message<Input, Output>>) -> Self {
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
        let wrapped = Message::message(message);
        self.sender
            .send(wrapped)
            .await
            .map_err(|_| SendError::ReceiverDisconnected)?;

        Ok(())
    }
}
