#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod prelude {
    /// Convenience re-export of [`async_trait`](https://docs.rs/async-trait/) proc-macro.
    pub use async_trait::async_trait;

    pub use crate::{
        actor::Actor,
        address::Address,
        context::Context,
        errors::SendError,
        handler::{Handler, Notifiable},
    };

    super::cfg_runtime! {
        pub use crate::registry::{Service, Registry};
    }
}

pub mod actor;
pub mod address;
pub mod context;
pub mod errors;
pub mod handler;

cfg_runtime! {
    pub mod registry;
}

mod envelope;
mod runtime;
