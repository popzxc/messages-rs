use messages::{async_trait, handler::Handler, Actor, Context};

struct PingActor {}

impl Actor for PingActor {}

#[async_trait]
impl Handler<u8> for PingActor {
    type Result = u8;

    async fn handle(&mut self, input: u8) -> u8 {
        input
    }
}

#[tokio::test]
async fn basic_workflow() {
    let mailbox: Context<PingActor> = Context::new(PingActor {});

    let mut address = mailbox.address();
    let future = tokio::spawn(mailbox.run());

    let response = address.send(10).await.unwrap();
    assert_eq!(response, 10);

    address.stop().await;

    assert!(future.await.is_ok());
}
