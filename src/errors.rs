//! Errors that can occur during the actor interaction workflow.

/// Errors that can occur while sending the message.
#[derive(Debug)]
pub enum SendError {
    /// Error emitted when it was attempted to send a message to the stopped actor.
    ReceiverDisconnected,
}

impl std::fmt::Display for SendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Actor does not accepting messages")
    }
}

impl std::error::Error for SendError {}
