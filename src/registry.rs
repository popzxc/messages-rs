use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use futures::lock::Mutex;
use once_cell::sync::Lazy;

use crate::{prelude::Address, service::Service};

static REGISTRY: Lazy<Registry> = Lazy::new(Registry::new);

#[derive(Debug, Default)]
pub struct Registry {
    services: Mutex<HashMap<&'static str, Box<dyn Any + Send>>>,
}

impl Registry {
    fn new() -> Self {
        Self::default()
    }

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
                    "Two or more services have a not unique name. Name is {}, attempt to retrieve the type {:?}, but stored type is {:?}",
                    S::NAME,
                    TypeId::of::<Address<S>>(),
                    maybe_addr.type_id()
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
