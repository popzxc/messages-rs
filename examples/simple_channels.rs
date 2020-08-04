//! This example implements a simple service that responds to the incoming messages.
//! Unlike `simple.rs`, this example is build atop of the raw channels.

use anyhow::Result;
use futures::{channel::{mpsc, oneshot}, SinkExt, StreamExt};
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug)]
pub enum ServiceMessage {
    Notification(u64),
    Request(u64, oneshot::Sender<u64>),
    Stop,
}

#[derive(Debug)]
pub struct Service {
    value: AtomicU64,
    mailbox: mpsc::Receiver<ServiceMessage>,
}

impl Service {
    pub fn new(mailbox: mpsc::Receiver<ServiceMessage>) -> Self {
        Self {
            value: 0.into(),
            mailbox,
        }
    }

    pub async fn run(mut self) -> Result<()> {
        let value = self.value;

        while let Some(message) = self.mailbox.next().await {
            match message {
                ServiceMessage::Notification(new_value) => {
                    value.store(new_value, Ordering::SeqCst);
                }
                ServiceMessage::Request(request, response) => {
                    let response_value = request + value.load(Ordering::SeqCst);

                    response.send(response_value).expect("Sending response failed");
                }
                ServiceMessage::Stop => {
                    break;
                }
            }
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let (mut message_sender, message_receiver) = mpsc::channel(128);
    let (request_sender, request_receiver) = oneshot::channel();

    // Start a service.
    let service = Service::new(message_receiver);
    let task_handle = tokio::spawn(service.run());

    // Send a notification.
    message_sender.send(ServiceMessage::Notification(10)).await.unwrap();

    // Send a request and receive a response.
    message_sender.send(ServiceMessage::Request(1, request_sender)).await.unwrap();
    let response = request_receiver.await.unwrap();
    assert_eq!(response, 11);

    // Stop service.
    message_sender.send(ServiceMessage::Stop).await.unwrap();
    assert!(task_handle.await.is_ok());

    Ok(())
}