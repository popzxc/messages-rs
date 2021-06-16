# Benchmarking

This document contains the benchmarking results and comparison with other actor libraries.

## Used machine

All the results are presented for 2020 MacBook Pro (M1 / 8gb ram).

## Ring benchmark

Actix provides a [ring benchmark](https://github.com/actix/actix/blob/master/actix/examples/ring.rs) to show
its performance.

In order to competition to be more or less fair, this section will include not only results
for the benchmark as-is, but also for an async version of `actix` benchmark.

In order to make `actix` example async, the `Handler` structure was modified in a following way:

```rust
impl Handler<Payload> for Node {
    // Result is not a unit type anymore, it's the future.
    type Result = ResponseFuture<()>;

    fn handle(&mut self, msg: Payload, _: &mut Context<Self>) -> Self::Result {
        if msg.0 >= self.limit {
            // ..left as-is
            return Box::pin(async {});
        }
        // ...left as-is
        Box::pin(async {})
    }
}
```

### `actix`, sync version

`cargo run --release --example ring -- 2000 2000`
Time taken: 0.510156 seconds (7840738 msg/second)

### `actix`, async version

`cargo run --release --example ring -- 2000 2000`
Time taken: 1.108587 seconds (3608196 msg/second)

### `messages`

`cargo run --release --example 04_ring -- 2000 2000`
Time taken: 0.940551 seconds (4252826 msg/second)

## Operations benchmark

Below you can find results for common operations of `messages`.

### `message` operations

**Spawn**

    time:   [1.3477 us **1.3814 us** 1.4149 us]
    thrpt:  [706.78 Kelem/s **723.92 Kelem/s** 741.99 Kelem/s]

**Send message**
              
    time:   [19.788 us **19.992 us** 20.255 us]
    thrpt:  [49.371 Kelem/s **50.019 Kelem/s** 50.535 Kelem/s]

**Notify**

    time:   [75.169 ns **76.816 ns** 78.467 ns]
    thrpt:  [12.744 Melem/s **13.018 Melem/s** 13.303 Melem/s]


### Raw channels

**Send message (Raw channel)**

    time:   [19.540 us **19.632 us** 19.738 us]
    thrpt:  [50.663 Kelem/s **50.936 Kelem/s** 51.176 Kelem/s]
