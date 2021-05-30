use messages::prelude::*;

struct Fibonacci(pub u32);

struct SyncActor;

impl Actor for SyncActor {}

#[async_trait]
impl Handler<Fibonacci> for SyncActor {
    type Result = Result<u64, ()>;

    async fn handle(&mut self, msg: Fibonacci, _: &mut Context<Self>) -> Self::Result {
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
    let mut address = SyncActor.spawn();

    // send 5 messages
    for n in 5..10 {
        println!("{:?}", address.send(Fibonacci(n)).await.unwrap());
    }

    address.stop().await;
}
