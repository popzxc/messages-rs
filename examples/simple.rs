//! This example implements a simple service that responds to the incoming messages.
//! Unlike `simple.rs`, this example is build atop of the `messages` crate.

use anyhow::Result;
use messages::{async_trait, handler::Handler, Actor};
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Default)]
pub struct Service {
    value: AtomicU64,
}

impl Actor for Service {}

#[async_trait]
impl Handler<u64> for Service {
    async fn handle(&self, input: u64) {
        self.value.store(input, Ordering::SeqCst);
    }
}

#[async_trait]
impl Handler<u64, u64> for Service {
    async fn handle(&self, input: u64) -> u64 {
        input + self.value.load(Ordering::SeqCst)
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
    let service = Service::new().into_runner();
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
