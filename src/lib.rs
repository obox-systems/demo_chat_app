#![warn(rust_2018_idioms)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

//! This is a backend for a chat.

/// Describes models for objects.
pub mod data;

/// Handles DB connection.
pub mod db;

/// Responsible for the chat API.
pub mod server;

/// Helper functions.
pub mod utils;

/// Responsible for WebSocket connections and message exchange.
pub mod ws;
