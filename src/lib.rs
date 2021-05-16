//! Messages. Convenient asynchronous communication
//!
//! `messages` is a very simplistic library, which provides a more declarative interface than raw channels, but yet
//! not overcomplicates things with too much functionality.
//!
//! It is intended to be used when channels in your project start looking a bit messy, but you aren't sure that
//! migrating to the actor framework is a right choice.

pub use async_trait::async_trait;

pub use crate::{
    actor::Actor,
    address::Address,
    errors::{ReceiveError, SendError},
    handler::Handler,
    runner::ActorRunner,
};

pub mod actor;
pub mod address;
mod envelope;
pub mod errors;
pub mod handler;
pub mod runner;
