//! This example implements a simple service that responds to the incoming messages.
//! Unlike `simple.rs`, this example is build atop of the `messages` crate.

use anyhow::Result;
use messages::prelude::*;

#[derive(Debug, Default)]
pub struct Service {
    value: u64,
}

impl Actor for Service {}
#[derive(Debug)]
pub struct Notification(pub u64);

#[derive(Debug)]
pub struct Request(pub u64);

#[async_trait]
impl Handler<Notification> for Service {
    type Result = ();

    async fn handle(&mut self, input: Notification, _: &mut Context<Self>) {
        self.value = input.0;
    }
}

#[async_trait]
impl Handler<Request> for Service {
    type Result = u64;

    async fn handle(&mut self, input: Request, _: &mut Context<Self>) -> u64 {
        self.value + input.0
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
    let mut address = Service::new().spawn();

    // Send a notification.
    address.send(Notification(10)).await.unwrap();

    // Send a request and receive a response.
    let response: u64 = address.send(Request(1)).await.unwrap();
    assert_eq!(response, 11);

    // Stop service.
    address.stop().await;

    Ok(())
}
