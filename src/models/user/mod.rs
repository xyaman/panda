//! User related models

mod activity;
pub use activity::Activity;

use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct User {
    pub(crate) id: String,
    pub(crate) username: String,
    pub(crate) discriminator: String,
    pub(crate) avatar: Option<String>,
    pub(crate) bot: Option<bool>,
    pub(crate) locale: Option<String>,
    pub(crate) verified: Option<bool>,
    pub(crate) email: Option<String>,
    pub(crate) flags: Option<u64>,
    pub(crate) premium_type: Option<u64>,
}

impl User {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn discriminator(&self) -> &str {
        &self.discriminator
    }

    pub fn avatar(&self) -> Option<&str> {
        self.avatar.as_ref().map(|s| s.as_str())
    }

    pub fn is_bot(&self) -> Option<bool> {
        self.bot
    }

    pub fn locale(&self) -> Option<&str> {
        self.locale.as_ref().map(|s| s.as_str())
    }

    pub fn is_verified(&self) -> Option<bool> {
        self.verified
    }

    pub fn email(&self) -> Option<&str> {
        self.email.as_ref().map(|s| s.as_str())
    }

    pub fn flags(&self) -> Option<u64> {
        self.flags
    }

    pub fn premium_type(&self) -> Option<u64> {
        self.premium_type
    }
}
