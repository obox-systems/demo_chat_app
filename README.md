# Simple chat API backend demo

This is a backend for a chat.

The essence of the work is that the client sends a message to the backend
and these messages are shared with other connected clients via websockets.

# Try it out!

1. Install [Rust](https://rustup.rs/)
2. Run the server:
```bash
$ cargo run --release
```
3. Use an API client of your choice to send and receive messages:
```
POST 127.0.0.1:8080/message
GET 127.0.0.1:8080/messages
```
4. As for continuous message updates you can use [websocat](https://github.com/vi/websocat):
```
$ cargo install websocat

$ websocat ws://127.0.0.1:9090
```
