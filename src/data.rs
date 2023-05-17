use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageWithId {
  pub id: i64,
  pub username: String,
  pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
  pub username: String,
  pub message: String,
}
