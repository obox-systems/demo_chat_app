use std::error::Error;

use chat_app::run;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let http = std::net::TcpListener::bind("127.0.0.1:8080")
    .expect("Failed to bind http on port 8080");
  _ = run(http)?.await;
  Ok(())
}
