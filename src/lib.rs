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
    context::Context,
    errors::{ReceiveError, SendError},
    handler::Handler,
};

pub mod actor;
pub mod address;
pub mod context;
pub mod errors;
pub mod handler;

cfg_runtime! {
    pub mod registry;
    pub mod service;
}

mod envelope;
mod runtime;
