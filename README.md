# Simple chat API backend demo

This is a backend for a chat.

The essence of the work is that the client sends a notification to the backend
and these notifications are shared with other connected clients.

Messages are stored in memory on an SQLite DB for the sake of simplicity.

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
