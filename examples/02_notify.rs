//! More comprehensive example that aims to show more of `message`
//! crate functionality.
//!
//! In this example, both `Handler` and `Notifiable` traits are implemented,
//! as well as additional methods of an `Actor` trait.

use messages::prelude::*;

#[derive(Debug, Default)]
pub struct Service {
    value: u64,
}

#[async_trait]
impl Actor for Service {
    // `started` method will be invoked *before* the first message
    // will be received.
    async fn started(&mut self) {
        println!("Service was started");
    }

    // `stopping` method will be invoked when the actor will be requested
    // to stop its execution.
    async fn stopping(&mut self) {
        println!("Service is stopping");
    }

    // `stopped` method will be invoked once actor is actually stopped.
    fn stopped(&mut self) {
        println!("Service has stopped");
    }
}

// Type that we will use to send messages.
#[derive(Debug)]
pub struct Request(pub u64);

#[async_trait]
impl Handler<Request> for Service {
    type Result = u64;

    async fn handle(&mut self, input: Request, _: &mut Context<Self>) -> u64 {
        self.value + input.0
    }
}

// Type that we will use for notifications.
#[derive(Debug)]
pub struct Notification(pub u64);

// Unlike `Handler`, `Notifiable` trait doesn't have output.
// It only serves one purpose: deliver a message to an actor.
// No response is expected.
#[async_trait]
impl Notifiable<Notification> for Service {
    async fn notify(&mut self, input: Notification, _: &mut Context<Self>) {
        self.value = input.0;
    }
}

impl Service {
    pub fn new() -> Self {
        Self::default()
    }
}

#[tokio::main]
async fn main() {
    // Start a service.
    let mut address = Service::new().spawn();

    // Send a notification.
    address.notify(Notification(10)).await.unwrap();

    // Send a request and receive a response.
    let response: u64 = address.send(Request(1)).await.unwrap();
    assert_eq!(response, 11);

    // Stop service.
    address.stop().await;
    // Wait for service to stop.
    address.wait_for_stop().await;
    // Ensure that actor is not running anymore.
    assert!(!address.connected());
}
