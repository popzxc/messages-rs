use criterion::*;
use futures::{
    channel::{mpsc, oneshot},
    SinkExt, StreamExt,
};
use messages::prelude::*;

struct Ping;

fn runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
}

#[async_trait]
impl Actor for Ping {}

#[async_trait]
impl Handler<u8> for Ping {
    type Result = u8;

    async fn handle(&mut self, input: u8, _context: &mut Context<Self>) -> Self::Result {
        input
    }
}

#[async_trait]
impl Notifiable<u8> for Ping {
    async fn notify(&mut self, _input: u8, _context: &mut Context<Self>) {
        // Do nothing
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    // Actors benchmarks.

    let mut g = c.benchmark_group("Actor workflow");
    g.throughput(Throughput::Elements(1));

    g.bench_function("Actor spawn", move |b| {
        b.to_async(runtime()).iter(|| async {
            let _ = black_box(Ping.spawn());
        })
    });

    g.bench_function("Actor send message", move |b| {
        b.to_async(runtime()).iter_with_setup(
            || Ping.spawn(),
            |mut addr| async move {
                let _x = black_box(addr.send(20u8).await.unwrap());
            },
        )
    });

    g.bench_function("Actor notify", move |b| {
        b.to_async(runtime()).iter_with_setup(
            || Ping.spawn(),
            |mut addr| async move {
                addr.notify(20u8).await.unwrap();
            },
        )
    });

    g.finish();

    // Raw channel benchmarks.

    let mut g = c.benchmark_group("Raw channel throughput");
    g.throughput(Throughput::Elements(1));

    g.bench_function("Actor send message", move |b| {
        b.to_async(runtime()).iter_with_setup(
            || {
                let (oneshot_send, oneshot_recv) = oneshot::channel::<u8>();
                let (send, mut recv) = mpsc::channel::<oneshot::Sender<u8>>(16);
                tokio::spawn(async move {
                    let send = recv.next().await.unwrap();
                    send.send(42u8).unwrap();
                });

                (send, oneshot_send, oneshot_recv)
            },
            |(mut send, back_send, recv)| async move {
                let _x = send.send(back_send).await.unwrap();
                let _y = black_box(recv.await.unwrap());
            },
        )
    });

    g.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
