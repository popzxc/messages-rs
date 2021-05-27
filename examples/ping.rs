use messages::*;

/// Define `Ping` message
struct Ping(usize);

/// Actor
struct MyActor {
    count: usize,
}

/// Declare actor and its context
impl Actor for MyActor {}

/// Handler for `Ping` message
#[async_trait]
impl Handler<Ping> for MyActor {
    type Result = usize;

    async fn handle(&mut self, msg: Ping) -> Self::Result {
        self.count += msg.0;
        self.count
    }
}

#[tokio::main]
async fn main() {
    // start new actor
    let mut address = MyActor { count: 10 }.spawn();

    // send message and get future for result
    let res = address.send(Ping(10)).await;

    // handle() returns tokio handle
    println!("RESULT: {}", res.unwrap() == 20);

    // stop system and exit
    address.stop().await;
}
