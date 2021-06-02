//! [`Registry`] provides a way to get addresses of singleton-like addresses
//! by automatically managing their lifetime under the hood.
//!
//! This module is awailable only when `messages` is build with one of the supported
//! runtime features enabled, as it needs to spawn actors.

use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use futures::lock::Mutex;
use once_cell::sync::Lazy;

use crate::{prelude::Address, service::Service};

static REGISTRY: Lazy<Registry> = Lazy::new(Registry::new);

/// `Registry` is an manager object providing access to the addresses
/// of [`Actor`]s that implement [`Service`] trait.
///
/// `Registry` maintains a list of spawned services and when an address
/// of a service is requested, it checks whether the corresponding actor is
/// already running. If so, address of this actor is returned. Otherwise,
/// actor is spawned first.
///
/// ## Examples
///
/// # use messages::prelude::*;
///
/// struct Ping;
///
/// #[async_trait]
/// impl Actor for Ping {}
///
/// #[async_trait]
/// impl Service for Ping {
///     const NAME: &'static str = "PingService";   
/// }
///
/// #[tokio::main]
/// async fn main() {
///    let addr: Address<Ping> = Registry::service().await;
///    # addr.stop().await;
///    # addr.wait_for_stop().await;
/// }
/// ```
#[derive(Debug, Default)]
pub struct Registry {
    services: Mutex<HashMap<&'static str, Box<dyn Any + Send>>>,
}

impl Registry {
    fn new() -> Self {
        Self::default()
    }

    /// Returns an address of an actor that implements [`Service`] trait.
    ///
    /// This function checks whether the corresponding actor is
    /// already running. If so, address of this actor is returned. Otherwise,
    /// actor is spawned first.
    ///
    /// ## Panics
    ///
    /// This method panics if two services having the same name will be attempted
    /// to be instantiated. All the names of services are expected to be unique.
    pub async fn service<S: Service + Sized + 'static>() -> Address<S> {
        let mut lock = REGISTRY.services.lock().await;

        // Check whether address is already in registry.
        if let Some(maybe_addr) = lock.get(S::NAME) {
            // Check whether we can downcast the stored address to a desired type.
            if let Some(addr) = maybe_addr.downcast_ref::<Address<S>>() {
                // Check whether actor is running. It for some reason is was stopped,
                // we will have to re-launch it again.
                if addr.connected() {
                    return addr.clone();
                }
            } else {
                // Two or more services have a not unique name.
                panic!(
                    "Two or more services have a not unique name. \
                    Name is {}, attempt to retrieve the type {:?}, but stored type is {:?}",
                    S::NAME,
                    TypeId::of::<Address<S>>(),
                    (&*maybe_addr).type_id()
                );
            }
        }

        // Address is either not in the registry or has been stopped.
        // We now have to spawn and store it in the registry.
        let addr = S::start_service();
        lock.insert(S::NAME, Box::new(addr.clone()));

        addr
    }
}
