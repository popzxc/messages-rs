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
    //! Client implementation is respectfull borrowed from [`tokio-tungstenite`] [example].
    //! It does not use actors, and is put here just so you can play with the server.
    //!
    //! [`tokio-tungstenite`]: https://github.com/snapview/tokio-tungstenite
    //! [example]: https://github.com/snapview/tokio-tungstenite/blob/master/examples/client.rs

    use futures::{future, pin_mut, StreamExt};
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

    pub(super) async fn run(connect_addr: String) {
        if !connect_addr.starts_with("ws://") {
            eprintln!("Server URL must start with `ws://`");
            return;
        }

        let (stdin_tx, stdin_rx) = futures::channel::mpsc::unbounded();
        tokio::spawn(read_stdin(stdin_tx));

        let (ws_stream, _) = connect_async(connect_addr)
            .await
            .expect("Failed to connect");
        println!("WebSocket handshake has been successfully completed");

        let (write, read) = ws_stream.split();

        let stdin_to_ws = stdin_rx.map(Ok).forward(write);
        let ws_to_stdout = {
            read.for_each(|message| async {
                let data = message.unwrap().into_data();
                tokio::io::stdout().write_all(&data).await.unwrap();
            })
        };

        pin_mut!(stdin_to_ws, ws_to_stdout);
        future::select(stdin_to_ws, ws_to_stdout).await;
    }

    // Our helper method which will read data from stdin and send it along the
    // sender provided.
    async fn read_stdin(tx: futures::channel::mpsc::UnboundedSender<Message>) {
        let mut stdin = tokio::io::stdin();
        loop {
            let mut buf = vec![0; 1024];
            let n = match stdin.read(&mut buf).await {
                Err(_) | Ok(0) => break,
                Ok(n) => n,
            };
            buf.truncate(n);
            tx.unbounded_send(Message::binary(buf)).unwrap();
        }
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
