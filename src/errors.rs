use thiserror::Error;

/// Errors that can occur while sending the message.
#[derive(Debug, Error)]
pub enum SendError {
    #[error("Mailbox stopped accepting messages")]
    ReceiverDisconnected,
}

/// Errors that can occur while receiving the message.
#[derive(Debug, Error)]
pub enum ReceiveError {
    #[error("All the senders have disconnected")]
    AllSendersDisconnected,
    #[error("Mailbox received stop request and no longer accepts incoming messages")]
    Stopped,
}
