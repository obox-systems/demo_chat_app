use serde::{Deserialize, Serialize};

/// Message model for sending to the client.
#[derive(Debug, Deserialize, Serialize)]
pub struct MessageWithId {
  /// Message id.
  pub id: i64,
  /// Message owner's name.
  pub username: String,
  /// Message contents.
  pub message: String,
}

/// Message model for receiving.
#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
  /// Message owner's name.
  pub username: String,
  /// Message contents.
  pub message: String,
}
