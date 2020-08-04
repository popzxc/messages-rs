//! This example implements a simple service that responds to the incoming messages.
//! Unlike `simple.rs`, this example is build atop of the `messages` crate.

use anyhow::Result;
use messages::{Address, Mailbox, Request};
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug)]
pub enum ServiceMessage {
    Notification(u64),
    Request(Request<u64, u64>),
}

#[derive(Debug, Default)]
pub struct Service {
    value: AtomicU64,
    mailbox: Mailbox<ServiceMessage>,
}

impl Service {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn address(&self) -> Address<ServiceMessage> {
        self.mailbox.address()
    }

    pub async fn run(self) -> Result<()> {
        let value = self.value;
        self.mailbox
            .run_with(|msg| async {
                match msg {
                    ServiceMessage::Notification(new_value) => {
                        value.store(new_value, Ordering::SeqCst);
                    }
                    ServiceMessage::Request(request) => {
                        let response_value = *request.message() + value.load(Ordering::SeqCst);

                        request
                            .respond(response_value)
                            .expect("Sending response failed");
                    }
                }
            })
            .await?;

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Start a service.
    let service = Service::new();
    let mut address = service.address();
    let task_handle = tokio::spawn(service.run());

    // Send a notification.
    address
        .send(ServiceMessage::Notification(10))
        .await
        .unwrap();

    // Send a request and receive a response.
    let (request, response) = Request::new(1);
    address
        .send(ServiceMessage::Request(request))
        .await
        .unwrap();
    let response = response.await.unwrap();
    assert_eq!(response, 11);

    // Stop service.
    address.stop().await.unwrap();
    assert!(task_handle.await.is_ok());

    Ok(())
}
