# Messages. Convenient asynchronous communication

**Status:**
[![CI](https://github.com/popzxc/messages-rs/workflows/CI/badge.svg)](https://github.com/popzxc/messages-rs/actions)

**Project info:**
[![Docs.rs](https://docs.rs/messages/badge.svg)](https://docs.rs/messages)
[![Latest Version](https://img.shields.io/crates/v/messages.svg)](https://crates.io/crates/messages)
[![License](https://img.shields.io/github/license/popzxc/messages-rs.svg)](https://github.com/popzxc/messages-rs)
![Rust 1.50+ required](https://img.shields.io/badge/rust-1.50+-blue.svg?label=Rust)

## Description

`messages` is a runtime-agnostic actor library.

It is heavily inspired by [`actix`][actix], a great actor framework.

This crate can be used with any runtime, whether it popular or not.
However, for the biggest one (`tokio` and `async-std`) there is an optional
built-in support enabling more convenient interface (such as an automatic
actor spawning).

[actix]: https://crates.io/crates/actix

## Key features

- Full runtime independence. Can be used with any possible runtime that can spawn futures.
- Low dependencies amount. 2 mandatory dependencies and up to 1 optional runtime dependency.
- Low amount of boilerplate without derive macros.
- Good performance (close to raw channels).
- Relevant (but sufficient) functionality only.

## Which library should I choose?

`actix` is a great, thoughtful, polished, and optimized library. If it is *possible*
for you, you should consider it as the main option.

However, if any of statements below apply to your use case, `messages` may be better:

- You can't or don't want to stick to the Actix runtime.
- Your tasks may not have the similar runtime expense (`actix-rt` does not have work stealing
  and thus some threads may be underloaded in that case).
- You are seeking for the simpler interface and don't want to implement asynchronous code atop
  of the initially sync interface.
  
**But what about [`xactor`](https://crates.io/crates/xactor)?**

`xactor` is another good library inspired by Actix. It initially was built for [`async-std`] but
then gained [`tokio`] support.

Nonetheless, this library is not runtime-agnostic. It supports `async-std` and `tokio` v1, but
is not (yet) compatible with another runtimes.

That being said, this library initially serves different purpose: provide a way to implement
actor workflow without having to think about supported runtimes.

## Asyncness

This library is async-first, meaning that everything is made with respect to asynchronous architecture.
While in *some* cases synchronous interfaces could've been more performant, it'd make the interface much
more bloated. If synchronous actor interface is preferred, consider using `actix`, as it provides one.

In order to provide convenient interface, this crate uses [`async_trait`](https://docs.rs/async-trait/)
to declare traits with `async` methods.
To make the experience more convenient, `async_trait::async_trait` macro is publicly re-exported
in the [`prelude`] module.

## Performance

TL;DR: This library provides performance slightly worse that either `actix` (for asynchronous message handling)
and raw channels, but not much.

More details are presented in the [BENCHES.md](./BENCHES.md).

*Note:* `messages` treats `async` and multi-threaded context as its main environment,
thus it may be less suitable (or, more precisely, less efficient) for the partially
sync context.

## Examples

### With runtime features

```rust
use messages::prelude::*;

struct Example; // Most of the types can be an actor.

// While `Actor` implementation can be customized, it is not required.
#[async_trait]
impl Actor for Example {}

// Message handler that calculated sum of two numbers.
#[async_trait]
impl Handler<(u8, u8)> for Example {
    type Result = u16;
    async fn handle(&mut self, (a, b): (u8, u8), context: &Context<Self>) -> u16 {
        (a as u16) + (b as u16)
    }
}

// Notification handler that calculated just writes received number to stdout.
#[async_trait]
impl Notifiable<u8> for Example {
    async fn notify(&mut self, input: u8, context: &Context<Self>) {
        println!("Received number {}", input);
    }
}

#[tokio::main]
async fn main() {
   let mut addr = Example.spawn();
   let result = addr.send((22, 20)).await.unwrap();
   assert_eq!(result, 42);
   addr.notify(42).await.unwrap();
   addr.stop().await;
   addr.wait_for_stop().await;  
}
```

### Without runtime features

```rust
use messages::prelude::*;

struct Ping;

#[async_trait]
impl Actor for Ping {}

#[async_trait]
impl Handler<u8> for Ping {
    type Result = u8;
    async fn handle(&mut self, input: u8, context: &Context<Self>) -> u8 {
        input
    }
}

#[tokio::main]
async fn main() {
   let context = Context::new();
   let mut addr = context.address();
   let actor = Ping;
   // Could've been any other runtime.
   let mut task_handle = tokio::spawn(context.run(actor));
   let result = addr.send(42).await.unwrap();
   assert_eq!(result, 42);
   addr.stop().await;
   addr.wait_for_stop().await;
   task_handle.await.unwrap();
}
```

### More

More examples can be found in the [examples](./examples) directory.

List of currently provided examples:

- [Ping](./examples/01_ping.rs): Simple ping actor without much of functionality.
- [Notify](./examples/02_notify.rs): More verbose example showing capabilities of the actor interface.
- [Fibonacci](./examples/03_fibonacci.rs): Example of a coroutine actor, i.e. one that can process messages in parallel.
- [Ring](./examples/04_ring.rs): Ring benchmark, mostly copied from the corresponding `actix` example.
- [Timed stream](./examples/05_timed_stream.rs): Example showing both how to attach stream to an actor and send timed notifications to it.
- [`async-std`](./examples/06_async_std.rs): Version of the `Notify` example adapted for `async-std` runtime.
- [`smol`](./examples/07_no_runtime.rs): Example of using a runtime not supported out of the box. In that case, `smol`.
- [WebSocket](./examples/08_websocket.rs): Simple actor-based echo websocket server (and a client to play with it).


## Contributing

All kind of contributions is really appreciated!

## License

`messages` library is licensed under the MIT License. See [LICENSE](LICENSE) for details.
