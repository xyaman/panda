mod message;
mod overwrite;

// Re-exports
pub use message::Message;
pub use overwrite::Overwrite;

use crate::models::user::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Channel {
    id: String,
    #[serde(rename = "type")]
    kind: u64,
    guild_id: Option<String>,
    position: Option<u64>,
    //permission_overwrites: Option<Overwrite>,
    name: Option<String>,
    topic: Option<String>,
    nsfw: Option<bool>,
    last_message_id: Option<String>,

    // For voice channels
    bitrate: Option<u64>,
    user_limit: Option<u64>,
    rate_limit_per_user: Option<u64>,

    // For DM
    recipients: Option<Vec<User>>,
    icon: Option<String>,
    owner_id: Option<String>,
    application_id: Option<String>,

    parent_id: Option<String>,
    last_pin_timestamp: Option<String>,
}

impl Channel {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn kind(&self) -> u64 {
        self.kind
    }

    pub fn guild_id(&self) -> Option<&str> {
        self.guild_id.as_ref().map(|s| s.as_str())
    }

    pub fn position(&self) -> Option<u64> {
        self.position
    }

    // //pub permission_overwrites: Option<Overwrite>,
    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|s| s.as_str())
    }

    pub fn topic(&self) -> Option<&str> {
        self.topic.as_ref().map(|s| s.as_str())
    }

    pub fn is_nsfw(&self) -> Option<bool> {
        self.nsfw
    }

    pub fn last_message_id(&self) -> Option<&str> {
        self.last_message_id.as_ref().map(|s| s.as_str())
    }

    // For voice channels
    // pub bitrate: Option<u64>,
    // pub user_limit: Option<u64>,
    // pub rate_limit_per_user: Option<u64>,

    // For DM
    pub fn recipients(&self) -> Option<&[User]> {
        self.recipients.as_ref().map(|s| s.as_slice())
    }

    pub fn icon(&self) -> Option<&str> {
        self.icon.as_ref().map(|s| s.as_str())
    }

    pub fn owner_id(&self) -> Option<&str> {
        self.owner_id.as_ref().map(|s| s.as_str())
    }

    pub fn application_id(&self) -> Option<&str> {
        self.application_id.as_ref().map(|s| s.as_str())
    }

    pub fn parent_id(&self) -> Option<&str> {
        self.parent_id.as_ref().map(|s| s.as_str())
    }

    pub fn last_pin_timestamp(&self) -> Option<&str> {
        self.last_pin_timestamp.as_ref().map(|s| s.as_str())
    }
}
