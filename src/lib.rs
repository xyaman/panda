//! # Async Discord Library
//!
//! Discord it's a very simple and friendly discord api library
//!
//! #Features
//! - __Fast__
//! - __Simple__

//#![deny(missing_docs)]
#![recursion_limit = "1024"]
// Modules
#[doc(inline)]
pub mod client;
#[doc(inline)]
pub mod error;
pub mod models;
pub mod utils;

mod gateway;
mod http;

// Re-exports
pub use models::gateway::events;

// Shortcut functions

/// Creates a new panda Client
pub async fn new(token: impl Into<String>) -> error::Result<client::Client> {
    client::Client::new(token).await
}
