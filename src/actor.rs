//! Actor is an entity capable of receiving and processing messages.
//!
//! For details, see the [`Actor`] documentation.

use async_trait::async_trait;

use crate::{address::Address, cfg_runtime, context::Context};

/// Actor is an entity capable of receiving and processing messages.
///
/// Each actor runs in an associated [`Context`] object that is capable
/// of managing the message delivery and maintaing the actor state.
///
/// Minimal implementation of an actor doesn't require any methods on it
/// and can be used on (almost) any type as a marker.
///
/// However, optional methods for better control of the actor lifespan
/// are provided:
///
/// - `started` method is called once [`Context`] was launched and is about
///   to start processing incoming messages.
/// - `stopping` method is called after either [`Address::stop`] method was
///   executed, or all the [`Address`] objects connected to an actor were
///   dropped.
/// - `stopped` method is a notification signaling that the [`Context`] future
///   is finishing its execution and after this call `Actor` object will be
///   dropped.
///
/// ## Stopping
///
/// Actor can stop in the following scenarios:
///
/// - All the [`Address`] objects connected to the actor were dropped.
/// - [`Address::stop`] method was invoked.
/// - Runtime in which [`Context`] is spawned was shutdown.
///
/// Note that if actor's address was obtained from the [`Registry`](crate::prelude::Registry), it will
/// never stop because of the first reason, as the address object will be
/// stored inside of the registry.
///
/// ## Prerequisites
///
/// As actors must be suitable for using in the multithreaded runtimes,
/// each type implementing `Actor` must be [`Send`], [`Sync`] and [`'static`][static_lt].
///
/// Additionally, it must implement [`Unpin`](std::marker::Unpin)
///
/// [static_lt]: https://doc.rust-lang.org/reference/types/trait-object.html#trait-object-lifetime-bounds
///
/// ## Extensions
///
/// When one of the runtime features is enabled in this crate, there is also an extension trait available:
/// [`RuntimeActorExt`], which provides more convenient interface for spawning actors, e.g. [`RuntimeActorExt::spawn`].
///
/// ## Examples
///
/// This example assumes that `messages` is used with `rt-tokio` feature enabled.
///
/// ```rust
/// # use messages::prelude::*;
///
/// struct Ping;
///
/// #[async_trait]
/// impl Actor for Ping {
///     async fn started(&mut self) {
///         println!("Actor was started");
///     }
///
///     async fn stopping(&mut self) {
///         println!("Actor is stopping");
///     }
///
///     fn stopped(&mut self) {
///         println!("Actor is stopped");
///     }
/// }
///
/// #[tokio::main]
/// async fn main() {
///    let mut addr = Ping.spawn();
///    addr.stop().await;
///    addr.wait_for_stop().await;  
/// }
/// ```
#[async_trait]
pub trait Actor: Unpin + Send + Sync + Sized + 'static {
    /// Method called after [`Context::run`] method was invoked.
    ///
    /// It is guaranteed to be called *before* any message will be
    /// passed to an actor.
    async fn started(&mut self) {}

    /// Method called once actor finished processing messages. It can
    /// happen either after [`Address::stop`] was called or
    /// all the [`Address`] objects will be dropped.
    ///
    /// It is guaranteed that after invocation of this method there will
    /// be no messages passed to the actor.
    async fn stopping(&mut self) {}

    /// Final notification about actor life end. Invoking this method
    /// will only be followed by the destruction of a [`Context`] object.
    fn stopped(&mut self) {}

    /// Creates [`Context`] object and starts the message processing loop.
    ///
    /// Future returned by this method should not normally be directly `await`ed,
    /// but rather is expected to be used in some kind of `spawn` function of
    /// the used runtime (e.g. `tokio::spawn` or `async_std::task::spawn`).
    async fn run(self) {
        Context::new().run(self).await;
    }

    /// Alternative to `run` function that should be used if an actor
    /// needs access to the [`Context`] object to be created (e.g. to
    /// know its own address).
    async fn create_and_run<F>(f: F)
    where
        F: FnOnce(&mut Context<Self>) -> Self + Send,
    {
        let mut context = Context::new();
        let this = f(&mut context);
        context.run(this).await;
    }
}

cfg_runtime! {
/// Extension trait for `Actor` providing more convenient interface when
/// one of the runtime features is enabled.
pub trait RuntimeActorExt: Actor {
    /// Spawns an actor using supported runtime.
    ///
    /// Returns an address of this actor.
    fn spawn(self) -> Address<Self> {
        Context::new().spawn(self)
    }

    /// Same as [`Actor::create_and_run`], but spawns
    /// the future instead of returning it.
    ///
    /// Returns an address of this actor.
    fn create_and_spawn<F>(f: F) -> Address<Self>
    where
        F: FnOnce(&mut Context<Self>) -> Self + Send,
    {
        let mut context = Context::new();
        let this = f(&mut context);
        context.spawn(this)
    }
}

impl<T> RuntimeActorExt for T
where
    T: Actor
{}
}
