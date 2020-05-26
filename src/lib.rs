#![doc(
    html_logo_url = "https://i.postimg.cc/3rGyjPqQ/logo.png",
    html_favicon_url = "https://i.postimg.cc/3rGyjPqQ/logo.png"
)]
//! # Async Discord Library
//!
//! Panda it's a very simple and friendly discord api library
//!
//! # Features
//! - __Fast__
//! - __Simple__
//!# Configuring async runtime
//! panda supports `tokio` and `async-std` runtimes, by default it uses `tokio`,
//! to use `async-std` change the feature flags in `Cargo.toml`
//!
//! ```toml
//! [dependencies.panda]
//! version = "0.5.1"
//! default-features = false
//! features = ["async-std-runtime"]
//! ```
//!
//!
//! # Example usage
//! It will print the bot name when the bot is ready.
//!
//! ```rust
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!
//!     let mut client = panda::new("your token here").await?;
//!
//!     client.on_ready(|s, ready| async move {
//!         println!("Bot {} is ready", ready.user.username);
//!
//!         Ok(())
//!     });
//!
//!     client.start().await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! All events are in the [Discord Documentation](https://discord.com/developers/docs/topics/gateway#commands-and-events), and to use it in client, you have to use `client.on_` plus
//! the event in snake case.

#![recursion_limit = "1024"]
macro_rules! define_cfg {
    ($($def: item)+) => {
        $(
            #[cfg(any(
                all(feature="tokio-runtime", not(feature="async-std-runtime")),
                all(feature="async-std-runtime", not(feature="tokio-runtime"))
            ))]
            $def
        )+
    }
}

//#![deny(missing_docs)]

define_cfg! {
    // Modules
    #[doc(inline)]
    pub mod client;
    #[doc(inline)]
    pub mod models;
    #[doc(inline)]
    pub mod utils;

    mod error;
    mod gateway;
    mod http;
    mod runtime;

    pub use error::PandaError;
    pub use http::HttpClient;

    // Re-exports
    pub use models::gateway::events;

    // Types

    /// Alias for Result<(), Box<dyn std::error::Error>>
    pub type HandlerResult = Result<(), Box<dyn std::error::Error>>;

    /// Alias for Arc<SessionData<S>>
    pub type Session<S> = std::sync::Arc<client::SessionData<S>>;

    /// Create a new panda Client without state
    pub async fn new(token: impl Into<String>) -> error::Result<client::Client<()>> {
        client::Client::<()>::new(token).await
    }

    /// Create a new panda Client with state
    pub async fn new_with_state<S: Sync + Send>(token: impl Into<String>, state: S) -> error::Result<client::Client<S>> {
        client::Client::<S>::new_with_state(token, state).await
    }
}

#[cfg(all(feature = "async-std-runtime", feature = "tokio-runtime"))]
compile_error!(
    "`tokio-runtime` and `async-std-runtime` can't be enable at the same time \
    please use just one of them, to use `async-std-runtime`, use default-features = false"
);

#[cfg(all(not(feature = "async-std-runtime"), not(feature = "tokio-runtime")))]
compile_error!(
    "You don't have a selected runtime as feature, please select `tokio-runtime` or \
    `async-std-runtime` in Cargo.toml"
);
