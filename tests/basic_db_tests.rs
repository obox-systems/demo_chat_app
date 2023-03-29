use std::net::TcpListener;

use chat_app::db::create_temp_db;
use serde_json::Value;

#[tokio::test]
async fn post_and_read_messages() {
  let addr = spawn_app().await;
  let client = reqwest::Client::new();

  let response = client
    .post(&format!("{addr}/message"))
    .header("Content-Type", "application/json")
    .body(r#"{"username":"hunter","message":"hello"}"#)
    .send()
    .await
    .expect("Failed to post message");
  assert!(response.status().is_success());

  let response = client
    .get(&format!("{addr}/messages"))
    .send()
    .await
    .expect("Failed to get messages");
  assert!(response.status().is_success());
  let response: Value = {
    let text = response.text().await.expect("Failed to read response body");
    serde_json::from_str(&text).expect("Invalid json returned")
  };
  assert_eq!(&response[0]["id"], 1);
  assert_eq!(&response[0]["username"], "hunter");
  assert_eq!(&response[0]["message"], "hello");
}

async fn spawn_app() -> String {
  let listener = TcpListener::bind("[::1]:0").expect("Failed to bind http to random port");
  let port = listener.local_addr().unwrap().port();
  let conn = create_temp_db().await.expect("Failed to create temp db");
  let server = chat_app::server::run(listener, "[::1]:0", conn)
    .await
    .expect("Failed to start server");
  _ = tokio::spawn(server);
  format!("http://[::1]:{port}")
}
