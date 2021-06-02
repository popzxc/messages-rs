use thiserror::Error;

/// Errors that can occur while sending the message.
#[derive(Debug, Error)]
pub enum SendError {
    #[error("Actor stopped accepting messages")]
    ReceiverDisconnected,
}
