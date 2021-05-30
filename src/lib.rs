pub mod prelude {
    pub use async_trait::async_trait;

    pub use crate::{
        actor::Actor,
        address::Address,
        context::Context,
        errors::{ReceiveError, SendError},
        handler::{Handler, Notifiable},
    };
}

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
