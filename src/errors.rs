//! Errors that can occur during the actor interaction workflow.

use thiserror::Error;

/// Errors that can occur while sending the message.
#[derive(Debug, Error)]
pub enum SendError {
    /// Error emitted when it was attempted to send a message to the stopped actor.
    #[error("Actor does not accepting messages")]
    ReceiverDisconnected,
}
