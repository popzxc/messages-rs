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
        // Artificially added big sleep to easily see whether requests are executed in parallel.
        // If requests will be processed sequentially, time to execute it will be `10 * N` seconds.
        // Otherwise, it will take `10 * (N / num_cpus)` seconds.
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
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
    // We send one message per CPU.
    let n_messages = num_cpus::get() as u32;

    let mut address = FibonacciActor.spawn();

    let mut futures = Vec::with_capacity(n_messages as usize);
    for n in 5..(5 + n_messages) {
        let fut = address.calculate(FibonacciRequest(n));
        futures.push(async move { (n, fut.await) });
    }

    let results = futures::future::join_all(futures).await;
    for (n, res) in results {
        println!("Result for {} is {:?}", n, res);
    }

    address.stop().await;
}
