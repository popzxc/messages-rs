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
This library aims to solve two main issues with `actix`:
`actix` is bound to the `actix-rt`, a custom tokio-based runtime.

This crate can be used with any runtime, whether it popular or not.
However, for the biggest one (`tokio` and `async-std`) there is an optional
built-in support enabling more convenient interface (such as an automatic
actor spawning).

`messages` treats `async` and multi-threaded context as its main environment,
thus it may be less suitable (or, more precisely, less efficient) for the partially
sync context.

[actix]: https://crates.io/crates/actix

## Which library should I choose?

`actix` is a great, thoughtful, polished, and optimized library. If it is *possible*
for you, you should consider it as the main option.

However, if any of statements below apply to your use case, `messages` may be better:

- You can't or don't want to stick to the Actix runtime.
- Your tasks may not have the similar runtime expense (`actix-rt` does not have work stealing
  and thus some threads may be underloaded in that case).
- You are seeking for the simpler interface and don't want to implement asynchronous code atop
  of the initially sync interface.

## Asyncness

In order to provide convenient interface, this crate uses [`async_trait`](https://docs.rs/async-trait/)
to declare traits with `async` methods.
To make the experience more convenient, `async_trait::async_trait` macro is publicly re-exported
in the [`prelude`] module.

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

More examples (e.g. example of attaching a stream to an actor) can be found in the [examples](./examples) directory.

## Contributing

All kind of contributions is really appreciated!

## License

`messages` library is licensed under the MIT License. See [LICENSE](LICENSE) for details.
