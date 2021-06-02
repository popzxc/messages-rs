use messages::prelude::*;

#[derive(Debug, Default)]
struct PingActor;

impl Actor for PingActor {}

#[async_trait]
impl Handler<u8> for PingActor {
    type Result = u8;

    async fn handle(&mut self, input: u8, _: &mut Context<Self>) -> u8 {
        input
    }
}

impl Service for PingActor {
    const NAME: &'static str = "Ping";
}

#[tokio::test]
async fn get_from_registry() {
    let mut address: Address<PingActor> = Registry::service().await;
    let response = address.send(10).await.unwrap();
    assert_eq!(response, 10);
    address.stop().await;

    // Service must be restarted after stopping.
    let mut address: Address<PingActor> = Registry::service().await;
    let response = address.send(10).await.unwrap();
    assert_eq!(response, 10);
    address.stop().await;
}
