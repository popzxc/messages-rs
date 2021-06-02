use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

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

#[derive(Debug, Default)]
pub struct ActorState {
    started: AtomicBool,
    stopping: AtomicBool,
    stopped: AtomicBool,
}

#[derive(Debug, Clone)]
struct WorkflowActor {
    state: Arc<ActorState>,
}

impl WorkflowActor {
    pub fn new(state: Arc<ActorState>) -> Self {
        Self { state }
    }
}

#[async_trait]
impl Actor for WorkflowActor {
    async fn started(&mut self) {
        self.state.started.store(true, Ordering::SeqCst);
    }

    async fn stopping(&mut self) {
        self.state.stopping.store(true, Ordering::SeqCst);
    }

    fn stopped(&mut self) {
        self.state.stopped.store(true, Ordering::SeqCst);
    }
}

#[async_trait]
impl Handler<()> for WorkflowActor {
    type Result = ();

    async fn handle(&mut self, _input: (), _context: &mut Context<Self>) -> Self::Result {}
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

#[tokio::test]
async fn lifespan_methods() {
    let state = Arc::new(ActorState::default());

    let mut address = WorkflowActor::new(state.clone()).spawn();
    // Wait for actor to actually start.
    address.send(()).await.unwrap();

    assert!(state.started.load(Ordering::SeqCst));
    assert!(!state.stopping.load(Ordering::SeqCst));
    assert!(!state.stopped.load(Ordering::SeqCst));

    address.stop().await;
    address.wait_for_stop().await;
    assert!(state.started.load(Ordering::SeqCst));
    assert!(state.stopping.load(Ordering::SeqCst));
    assert!(state.stopped.load(Ordering::SeqCst));
}
