use criterion::*;
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
    c.bench_function("Actor spawn", move |b| {
        b.to_async(runtime()).iter(|| async {
            let _ = black_box(Ping.spawn());
        })
    });

    c.bench_function("Actor send message", move |b| {
        b.to_async(runtime()).iter_with_setup(
            || Ping.spawn(),
            |mut addr| async move {
                let _x = black_box(addr.send(20u8).await.unwrap());
            },
        )
    });

    c.bench_function("Actor notify", move |b| {
        b.to_async(runtime()).iter_with_setup(
            || Ping.spawn(),
            |mut addr| async move {
                addr.notify(20u8).await.unwrap();
            },
        )
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
