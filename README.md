# Messages. Convenient asynchronous communication

**Status:**
[![CI](https://github.com/popzxc/messages-rs/workflows/CI/badge.svg)](https://github.com/popzxc/messages-rs/actions)

**Project info:**
[![Docs.rs](https://docs.rs/messages/badge.svg)](https://docs.rs/messages)
[![Latest Version](https://img.shields.io/crates/v/messages.svg)](https://crates.io/crates/messages)
[![License](https://img.shields.io/github/license/popzxc/messages-rs.svg)](https://github.com/popzxc/messages-rs)
![Rust 1.44+ required](https://img.shields.io/badge/rust-1.44+-blue.svg?label=Rust)

## Description

`messages` is a very simplistic library, which provides a more declarative interface than raw channels, but yet
not overcomplicates things with too much functionality.

It is intended to be used when channels in your project start looking a bit messy, but you aren't sure that
migrating to the actor framework is a right choice.

To compare channels-based implementation and the implementation that uses `messages`, see:
- [channels example](examples/simple_channels.rs);
- [messages example](examples/simple.rs).

## Contributing

All kind of contributions is really appreciated!

## License

`messages` library is licensed under the MIT License. See [LICENSE](LICENSE) for details.
