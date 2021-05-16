//! This example implements a simple service that responds to the incoming messages.
//! Unlike `simple.rs`, this example is build atop of the `messages` crate.

use anyhow::Result;
use futures::{future::ready, FutureExt};
use messages::{handler::Handler, Actor, ActorRunner};
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Default)]
pub struct Service {
    value: AtomicU64,
}

impl Actor for Service {}

impl Handler<u64, ()> for Service {
    fn handle(&self, input: u64) -> futures::future::BoxFuture<()> {
        self.value.store(input, Ordering::SeqCst);
        ready(()).boxed()
    }
}

impl Handler<u64, u64> for Service {
    fn handle(&self, input: u64) -> futures::future::BoxFuture<u64> {
        let response_value = input + self.value.load(Ordering::SeqCst);

        ready(response_value).boxed()
    }
}

impl Service {
    pub fn new() -> Self {
        Self::default()
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Start a service.
    let service = ActorRunner::new(Service::new());
    let mut address = service.address();
    let task_handle = tokio::spawn(service.run());

    // Send a notification.
    let _: () = address.send(10).await.unwrap();

    // Send a request and receive a response.
    let response: u64 = address.send(1).await.unwrap();
    assert_eq!(response, 11);

    // Stop service.
    address.stop().await.unwrap();
    assert!(task_handle.await.is_ok());

    Ok(())
}
