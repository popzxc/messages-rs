#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(
    missing_debug_implementations,
    rust_2018_idioms,
    missing_docs,
    unreachable_pub
)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
// Seems to be broken right now: https://github.com/rust-lang/rust-clippy/issues/8300
#![allow(clippy::no_effect_underscore_binding)]

//! `messages` is a runtime-agnostic actor library.
//!
//! It is heavily inspired by [`actix`][actix], a great actor framework.
//!
//! This crate can be used with any runtime, whether it popular or not.
//! However, for the biggest one (`tokio` and `async-std`) there is an optional
//! built-in support enabling more convenient interface (such as an automatic
//! actor spawning).
//!
//! [actix]: https://crates.io/crates/actix
//!
//! ## Asyncness
//!
//! In order to provide convenient interface, this crate uses [`async_trait`](https://docs.rs/async-trait/)
//! to declare traits with `async` methods.
//! To make the experience more convenient, `async_trait::async_trait` macro is publicly re-exported
//! in the [`prelude`] module.
//!
//! ## Examples
//!
//! ### With runtime features
//!
//! ```rust
//! use messages::prelude::*;
//!
//! struct Example; // Most of the types can be an actor.
//!
//! // While `Actor` implementation can be customized, it is not required.
//! #[async_trait]
//! impl Actor for Example {}
//!
//! // Message handler that calculated sum of two numbers.
//! #[async_trait]
//! impl Handler<(u8, u8)> for Example {
//!     type Result = u16;
//!     async fn handle(&mut self, (a, b): (u8, u8), context: &Context<Self>) -> u16 {
//!         (a as u16) + (b as u16)
//!     }
//! }
//!
//! // Notification handler that calculated just writes received number to stdout.
//! #[async_trait]
//! impl Notifiable<u8> for Example {
//!     async fn notify(&mut self, input: u8, context: &Context<Self>) {
//!         println!("Received number {}", input);
//!     }
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!    let mut addr = Example.spawn();
//!    let result = addr.send((22, 20)).await.unwrap();
//!    assert_eq!(result, 42);
//!    addr.notify(42).await.unwrap();
//!    addr.stop().await;
//!    addr.wait_for_stop().await;  
//! }
//! ```
//!
//! ### Without runtime features
//!
//! ```rust
//! use messages::prelude::*;
//!
//! struct Ping;
//!
//! #[async_trait]
//! impl Actor for Ping {}
//!
//! #[async_trait]
//! impl Handler<u8> for Ping {
//!     type Result = u8;
//!     async fn handle(&mut self, input: u8, context: &Context<Self>) -> u8 {
//!         input
//!     }
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!    let context = Context::new();
//!    let mut addr = context.address();
//!    let actor = Ping;
//!    // Could've been any other runtime.
//!    let mut task_handle = tokio::spawn(context.run(actor));
//!    let result = addr.send(42).await.unwrap();
//!    assert_eq!(result, 42);
//!    addr.stop().await;
//!    addr.wait_for_stop().await;
//!    task_handle.await.unwrap();
//! }
//! ```
//!
//! ## Main entities
//!
//! Main entites of this crate:
//!
//! - [`Actor`](crate::prelude::Actor): definition of an actor.
//! - [`Context`](crate::prelude::Context): execution context for an actor.
//! - [`Address`](crate::prelude::Address): address of an actor that is used to communicate with it.
//! - Handler traits: [`Handler`](crate::prelude::Handler) and [`Notifiable`](crate::prelude::Notifiable).
//!
//! With runtime features enabled, there are also several more points of interest:
//!
//! - [`Registry`](crate::prelude::Registry): Collection of independent, unique and named actors.
//! - [`Service`](crate::prelude::Service): Actor that can be stored in the registry.
//! - [`Coroutine`](crate::prelude::Coroutine): Alternative to the `Handler` trait that allows
//!   parallel message processing.
//!

/// Collection of the main types required to work with `messages` crate.
pub mod prelude {
    /// Convenience re-export of [`async_trait`](https://docs.rs/async-trait/) proc-macro.
    pub use async_trait::async_trait;

    pub use crate::{
        actor::{Actor, ActorAction},
        address::Address,
        context::Context,
        errors::SendError,
        handler::{Handler, Notifiable},
    };

    super::cfg_runtime! {
        pub use crate::registry::{Service, Registry};
        pub use crate::actor::RuntimeActorExt;
        pub use crate::handler::Coroutine;

        /// Re-export of `JoinHandle` of chosen runtime.
        #[cfg_attr(not(docsrs), doc(hidden))]
        // ^ Kludge: `cargo deadlinks` finds a broken link in the tokio docs,
        // and currently it's not possible to ignore that error.
        // However, we don't want to completely hide this element.
        pub use crate::runtime::JoinHandle;
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
