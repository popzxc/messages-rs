use std::env;

mod server {
    use futures::{Sink, SinkExt, StreamExt};
    use messages::prelude::*;
    use tokio::net::TcpListener;
    use tokio_tungstenite::tungstenite::{Error as WsError, Message};

    pub struct WebSocketConnection<T: Sink<Message>> {
        response: T,
    }

    impl<T> WebSocketConnection<T>
    where
        T: Sink<Message>,
    {
        pub fn new(response: T) -> Self {
            Self { response }
        }
    }

    impl<T> Actor for WebSocketConnection<T> where T: Sink<Message> + Send + Sync + Unpin + 'static {}

    #[async_trait]
    impl<T> Notifiable<Result<Message, WsError>> for WebSocketConnection<T>
    where
        T: Sink<Message> + Send + Sync + Unpin + 'static,
    {
        async fn notify(&mut self, input: Result<Message, WsError>, context: &Context<Self>) {
            let msg = match input {
                Ok(msg) => msg,
                Err(_err) => return,
            };

            match msg {
                Message::Text(input) => {
                    let _ = self.response.send(Message::Text(input)).await;
                }
                Message::Binary(input) => {
                    let _ = self.response.send(Message::Binary(input)).await;
                }
                Message::Ping(ping) => {
                    let _ = self.response.send(Message::Pong(ping)).await;
                }
                Message::Pong(_) => {
                    // We don't send ping messages, do nothing.
                }
                Message::Close(_) => {
                    context.address().stop().await;
                }
            }
        }
    }

    pub(super) async fn run(addr: String) {
        // Create the event loop and TCP listener we'll accept connections on.
        let try_socket = TcpListener::bind(&addr).await;
        let listener = try_socket.expect("Failed to bind");
        println!("Listening on: {}", addr);

        while let Ok((stream, _)) = listener.accept().await {
            let (ws_sink, ws_stream) = tokio_tungstenite::accept_async(stream)
                .await
                .expect("Error during the websocket handshake occurred")
                .split();
            let addr = WebSocketConnection::new(ws_sink).spawn();
            addr.spawn_stream_forwarder(ws_stream);
        }
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
