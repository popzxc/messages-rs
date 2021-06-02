use messages::prelude::{async_trait, Actor, Context, Handler};

mod registry;

#[derive(Debug)]
struct PingActor;

impl Actor for PingActor {}

#[async_trait]
impl Handler<u8> for PingActor {
    type Result = u8;

    async fn handle(&mut self, input: u8, _: &mut Context<Self>) -> u8 {
        input
    }
}

#[tokio::test]
async fn basic_workflow() {
    let actor = PingActor;
    let mailbox: Context<PingActor> = Context::new();

    let mut address = mailbox.address();
    let future = tokio::spawn(mailbox.run(actor));

    let response = address.send(10).await.unwrap();
    assert_eq!(response, 10);

    address.stop().await;

    assert!(future.await.is_ok());
}

#[tokio::test]
async fn runtime_based() {
    let mut address = PingActor.spawn();
    let response = address.send(10).await.unwrap();
    assert_eq!(response, 10);
    address.stop().await;
}
