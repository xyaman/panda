//! Discord Models
//!
//! Here you can find all models from the Discord API

#[doc(inline)]
pub mod channel;
#[doc(inline)]
pub mod emoji;
#[doc(inline)]
pub mod gateway;
#[doc(inline)]
pub mod guild;
#[doc(inline)]
pub mod user;
#[doc(inline)]
pub mod voice;
#[doc(inline)]
pub mod invite;
#[doc(inline)]
pub mod webhook;

// Re-export all models
pub use channel::*;
pub use emoji::*;
pub use gateway::*;
pub use guild::*;
pub use user::*;
pub use voice::*;
pub use invite::*;
pub use webhook::*;
