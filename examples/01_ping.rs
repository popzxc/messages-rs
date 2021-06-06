//! Simples possible example of a ping actor that stores
//! sum of received messages and responds with the message
//! being sent.

// Prelude contains all the required definitions to work
// with the messages crate.
use messages::prelude::*;

// Define `Ping` message.
// It will be sent to our actor.
// Note that no derives are required to make things work.
// Any type can represent a message as long as it's thread-safe.
struct Ping(usize);

// Our actor. Despite the fact that it will be used in the multi-threaded
// runtime, we don't have to bother ourselves with `RwLock`s and `Arc`s:
// synchronization guarantees are on the `messages` crate.
struct MyActor {
    count: usize,
}

// In order to turn our structure into an actor, we have to implement the
// corresponding trait for it.
// It is possible to customize actor behavior by implementing additional methods
// for it, but it's not required.
impl Actor for MyActor {}

// In order to be able to process messages, we must implement either `Handler` or
// `Notifiable` trait. Since we want to return values to the message sender,
// we are going to use `Handler` trait.
#[async_trait]
impl Handler<Ping> for MyActor {
    // Define the type of response that will be sent to the message author.
    type Result = usize;

    // Define the logic of the message processing.
    async fn handle(&mut self, msg: Ping, _: &Context<Self>) -> Self::Result {
        self.count += msg.0;
        self.count
    }
}

#[tokio::main]
async fn main() {
    // Start new actor
    let mut address = MyActor { count: 10 }.spawn();

    // Send message and get the result.
    let res = address.send(Ping(10)).await;

    // Check whether actor returned the expected message.
    println!("RESULT: {}", res.unwrap() == 20);
}
