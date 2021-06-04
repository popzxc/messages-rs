//! This example shows how to notify actor at a certain interval.
//! It also demonstrates how to attach stream to an actor via its address.

use std::time::{Duration, Instant};

use futures::StreamExt;
use messages::prelude::*;
use tokio_stream::wrappers::IntervalStream;

#[derive(Debug)]
pub struct Service {
    last_notified: Instant,
}

impl Actor for Service {}

#[derive(Debug)]
pub struct Notification;

#[async_trait]
impl Notifiable<Notification> for Service {
    async fn notify(&mut self, _input: Notification, _: &mut Context<Self>) {
        println!(
            "Notified after {}ms",
            self.last_notified.elapsed().as_millis()
        );
        self.last_notified = Instant::now();
    }
}

impl Service {
    pub fn create() -> Self {
        Self {
            last_notified: Instant::now(),
        }
    }
}

#[tokio::main]
async fn main() {
    // Start a service.
    let address = Service::create().spawn();

    // Attach a stream that will ping the service every 100ms.
    // It will emit 10 values only.
    let interval_stream = IntervalStream::new(tokio::time::interval(Duration::from_millis(100)))
        .take(10)
        .map(|_| Notification);

    let join_handle = address.spawn_stream_forwarder(interval_stream);

    // Wait until stream yielded all its values.
    join_handle.await.unwrap().unwrap();
}
