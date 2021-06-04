//! Example of using `messages` without runtime features.
//! In this example we use `smol` to demonstrate some runtime
//! not supported out of the box.

use messages::prelude::*;

struct Ping(usize);

struct MyActor {
    count: usize,
}

impl Actor for MyActor {}

#[async_trait]
impl Handler<Ping> for MyActor {
    type Result = usize;

    async fn handle(&mut self, msg: Ping, _: &mut Context<Self>) -> Self::Result {
        self.count += msg.0;
        self.count
    }
}

fn main() {
    smol::block_on(async {
        // Without runtime, we have to manually create the context in order
        // to obtain the actor's address.
        let context = Context::new();
        let mut address = context.address();
        let actor_handle = smol::spawn(context.run(MyActor { count: 10 }));

        // Send message and get the result.
        let res = address.send(Ping(10)).await;

        // Check whether actor returned the expected message.
        println!("RESULT: {}", res.unwrap() == 20);

        // Stop the actor.
        address.stop().await;
        address.wait_for_stop().await;
        actor_handle.await;
    });
}
