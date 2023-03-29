use chat_app::{db::open_db, server::run};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let conn = open_db("./db").await?;
  let http =
    std::net::TcpListener::bind("127.0.0.1:8080").expect("Failed to bind http on port 8080");
  let ws_addr = "127.0.0.1:9090";
  _ = run(http, ws_addr, conn).await?.await;
  Ok(())
}
