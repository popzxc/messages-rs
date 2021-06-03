//! In order for an [`Actor`] to be able to process messages,
//! it should have logic associated with them.
//!
//! For that matter, `messages` provides two traits:
//!
//! - [`Notifiable`]: handler for notifications, e.g. messages that do not require response.
//! - [`Handler`]: handler that produces some data as a response to the sent message.
//!
//! Note that [`Actor`] can implement both [`Notifiable`] and [`Handler`] traits in case
//! the calculated data is important for some modules, but not so much for others.
//!
//! [`Notifiable`] crate is generally more performant than [`Handler`] since it does not
//! include overhead to return result back to the original message sender.

use async_trait::async_trait;

use crate::prelude::{Actor, Context};

/// `Notifiable` is an extension trait for [`Actor`] that enables it
/// to process notifications.
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
/// impl Actor for Ping {}
///
/// #[async_trait]
/// impl Notifiable<u8> for Ping {
///     async fn notify(&mut self, input: u8, context: &mut Context<Self>) {
///         println!("Received number {}", input);
///     }
/// }
///
/// #[tokio::main]
/// async fn main() {
///    let mut addr = Ping.spawn();
///    addr.notify(42).await.unwrap();
///    # addr.stop().await;
///    # addr.wait_for_stop().await;  
/// }
/// ```
#[async_trait]
pub trait Notifiable<IN>: Sized + Actor {
    /// Processes notification.
    async fn notify(&mut self, input: IN, context: &mut Context<Self>);
}

/// `Notifiable` is an extension trait for [`Actor`] that enables it
/// to process messages and return results of the message processing.
///
/// ## Examples
///
/// This example assumes that `messages` is used with `rt-tokio` feature enabled.
///
/// ```rust
/// # use messages::prelude::*;
///
/// struct Sum;
///
/// #[async_trait]
/// impl Actor for Sum {}
///
/// #[async_trait]
/// impl Handler<(u8, u8)> for Sum {
///     type Result = u16;
///
///     async fn handle(&mut self, (a, b): (u8, u8), context: &mut Context<Self>) -> u16 {
///         (a as u16) + (b as u16)
///     }
/// }
///
/// #[tokio::main]
/// async fn main() {
///    let mut addr = Sum.spawn();
///    let result = addr.send((22, 20)).await.unwrap();
///    assert_eq!(result, 42);
///    # addr.stop().await;
///    # addr.wait_for_stop().await;  
/// }
/// ```
#[async_trait]
pub trait Handler<IN>: Sized + Actor {
    /// Result of the message processing.
    type Result;

    /// Processes a message.
    async fn handle(&mut self, input: IN, context: &mut Context<Self>) -> Self::Result;
}
