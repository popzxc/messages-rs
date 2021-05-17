//! Example of sync actor. It can be used for cpu bound tasks. Only one sync
//! actor runs within arbiter's thread. Sync actor processes one message at a
//! time. Sync arbiter can start multiple threads with separate instance of
//! actor in each.

use messages::*;

struct Fibonacci(pub u32);

struct SyncActor;

impl Actor for SyncActor {}

#[async_trait]
impl Handler<Fibonacci> for SyncActor {
    type Result = Result<u64, ()>;

    async fn handle(&mut self, msg: Fibonacci) -> Self::Result {
        if msg.0 == 0 {
            Err(())
        } else if msg.0 == 1 {
            Ok(1)
        } else {
            let mut i = 0;
            let mut sum = 0;
            let mut last = 0;
            let mut curr = 1;
            while i < msg.0 - 1 {
                sum = last + curr;
                last = curr;
                curr = sum;
                i += 1;
            }
            Ok(sum)
        }
    }
}

#[tokio::main]
async fn main() {
    // start sync arbiter with 3 threads
    let service = SyncActor.into_runner();
    let mut address = service.address();
    let task_handle = tokio::spawn(service.run());

    // send 5 messages
    for n in 5..10 {
        println!("{:?}", address.send(Fibonacci(n)).await.unwrap());
    }

    address.stop().await;
    assert!(task_handle.await.is_ok());
}
