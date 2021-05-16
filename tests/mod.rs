use messages::{async_trait, handler::Handler, Actor, ActorRunner};
use tokio::runtime::Builder;

struct PingActor {}

impl Actor for PingActor {}

#[async_trait]
impl Handler<u8, u8> for PingActor {
    async fn handle(&self, input: u8) -> u8 {
        input
    }
}

#[test]
fn message_box() {
    let mut basic_rt = Builder::new().basic_scheduler().build().unwrap();

    basic_rt.block_on(async {
        let mailbox: ActorRunner<PingActor> = ActorRunner::new(PingActor {});

        let mut address = mailbox.address();
        let future = tokio::spawn(mailbox.run());

        let response = address.send(10).await.unwrap();
        assert_eq!(response, 10);

        address.stop().await.unwrap();

        assert!(future.await.is_ok());
    });
}
