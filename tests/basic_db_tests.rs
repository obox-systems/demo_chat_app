use std::{net::TcpListener, fmt::format};

use chat_app::create_temp_db;

#[tokio::test]
async fn post_and_read_messages(){
  let addr = spawn_app();
  let client = reqwest::Client::new();

  let response = client
    .post(&format!("{addr}/message"))
    .header("Content-Type", "application/json")
    .body("{ \"username\": \"hunter\", \"message\": \"hello\" }")
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
  assert_eq!(
    "[{\"username\":\"hunter\",\"message\":\"hello\"}]",
    response.text().await.expect("Failed to read response body")
  );
}

fn spawn_app() -> String {
  let listener = TcpListener::bind("[::1]:0")
    .expect("Failed to bind to random port");
  let port = listener.local_addr().unwrap().port();
  let conn = create_temp_db().expect("Failed to create temp db");
  let server = chat_app::run(listener, conn).expect("Failed to start server");
  _ = tokio::spawn(server);
  format!("http://[::1]:{port}")
}