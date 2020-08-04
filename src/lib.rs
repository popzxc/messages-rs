//! Messages. Convenient asynchronous communication
//! 
//! `messages` is a very simplistic library, which provides a more declarative interface than raw channels, but yet
//! not overcomplicates things with too much functionality.
//! 
//! It is intended to be used when channels in your project start looking a bit messy, but you aren't sure that
//! migrating to the actor framework is a right choice.

pub use crate::{
    address::Address,
    errors::{ReceiveError, SendError},
    mailbox::Mailbox,
    request::Request,
};

pub mod address;
pub mod errors;
pub mod mailbox;
pub mod request;
