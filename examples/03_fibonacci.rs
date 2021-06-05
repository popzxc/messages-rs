//! Example of actor used to calculate some stuff.
//! In that case, fibonacci numbers.
//!
//! This example uses `Coroutine` trait that allows message handling to be executed
//! in parallel.

use messages::prelude::*;

struct FibonacciRequest(pub u32);

#[derive(Debug, Clone)]
struct FibonacciActor;

impl Actor for FibonacciActor {}

#[async_trait]
impl Coroutine<FibonacciRequest> for FibonacciActor {
    type Result = Result<u64, ()>;

    async fn calculate(self, msg: FibonacciRequest) -> Self::Result {
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
    let mut address = FibonacciActor.spawn();

    // send 5 messages
    for n in 5..10 {
        println!(
            "{:?}",
            address.calculate(FibonacciRequest(n)).await.unwrap()
        );
    }

    address.stop().await;
}
