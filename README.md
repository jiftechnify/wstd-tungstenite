# wstd-tungstenite

Async WebSocket support for Wasm / WASI 0.2 applications on top of [wstd].

[![Crates.io](https://img.shields.io/crates/v/wstd-tungstenite)](https://crates.io/crates/wstd-tungstenite)
[![Docs.rs](https://img.shields.io/docsrs/wstd-tungstenite)](https://docs.rs/wstd-tungstenite)
[![MIT Licensed](https://img.shields.io/crates/l/wstd-tungstenite)](./LICENSE)

[wstd]: https://github.com/bytecodealliance/wstd

## Installation

```bash
cargo add wstd-tungstenite

# Provides extension methods for Stream/Sink
# You may want to add this for ergonomic WebSocketStream operations!
cargo add futures
```

## Example

Minimal WebSocket server:

```rust
// src/main.rs

use futures::{SinkExt, StreamExt};
use wstd::{io, iter::AsyncIterator, net::TcpListener};
use wstd_tungstenite::{accept_async, tungstenite::Message};

#[wstd::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Listening on {}", listener.local_addr()?);

    let mut incoming = listener.incoming();
    while let Some(stream) = incoming.next().await {
        let stream = stream?;
        println!("Connection from {}", stream.peer_addr()?);

        let mut ws = accept_async(stream)
            .await
            .expect("Error during the websocket handshake occurred");

        wstd::runtime::spawn(async move {
            while let Some(msg) = ws.next().await {
                if let Ok(msg) = msg {
                    println!("received: {msg}");
                    if ws.send(Message::Text("hello!".into())).await.is_err() {
                        break;
                    }
                } else {
                    break;
                }
            }
        })
        .detach();
    }
    Ok(())
}
```

Build it, then run on [wasmtime]:

```bash
cargo build --target wasm32-wasip2
wasmtime run -S inherit-network=y ./target/wasm32-wasip2/release/example.wasm
```

[wasmtime]: https://wasmtime.dev/

## Credits
This crate is heavily inspired by [tokio-tungstenite] and [async-tungstenite] crates.
Indeed, API is mostly same and much of the code is borrowed from them.

And, of course, big thanks to original [tungstenite] works!

[tokio-tungstenite]: https://github.com/snapview/tokio-tungstenite
[async-tungstenite]: https://github.com/sdroege/async-tungstenite
[tungstenite]: https://github.com/snapview/tungstenite-rs

## License

MIT
