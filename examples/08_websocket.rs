use std::env;

mod server {
    pub(super) async fn run(_addr: String) {
        todo!()
    }
}

mod client {
    pub(super) async fn run(_addr: String) {
        todo!()
    }
}

#[tokio::main]
async fn main() {
    let command = match env::args()
        .nth(1)
        .map(|s| s.to_ascii_lowercase())
        .filter(|c| c == "server" || c == "client")
    {
        Some(command) => command,
        None => {
            println!(
                "Usage: `cargo run --example 08_websocket -- [client|server] <arg>`, \
                where `arg` is either server address or bind address."
            );
            return;
        }
    };

    let arg = env::args()
        .nth(2)
        .unwrap_or_else(|| "127.0.0.1:8080".to_owned());

    match command.as_ref() {
        "server" => server::run(arg).await,
        "client" => client::run(arg).await,
        _ => unreachable!(),
    }
}
