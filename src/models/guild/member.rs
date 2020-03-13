use crate::models::user::User;
// use crate::models::guild::Role;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Member {
    user: Option<User>,
    nick: Option<String>,
    roles: Vec<String>,
    joined_at: Option<String>,
    premium_since: Option<String>,
    deaf: bool,
    mute: bool,

    // only for GUILD_MEMBER_ADD
    guild_id: Option<String>,
}

impl Member {
    pub fn user(&self) -> Option<&User> {
        self.user.as_ref()
    }

    pub fn nick(&self) -> Option<&str> {
        self.nick.as_ref().map(|s| s.as_str())
    }

    pub fn roles(&self) -> &[String] {
        self.roles.as_ref()
    }

    pub fn joined_at(&self) -> Option<&str> {
        self.joined_at.as_ref().map(|s| s.as_str())
    }

    pub fn premium_since(&self) -> Option<&str> {
        self.premium_since.as_ref().map(|s| s.as_str())
    }

    pub fn is_deaf(&self) -> bool {
        self.deaf
    }

    pub fn is_mute(&self) -> bool {
        self.mute
    }

    pub fn guild_id(&self) -> Option<&str> {
        self.guild_id.as_ref().map(|s| s.as_str())
    }
}
