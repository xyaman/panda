//! Channel related models

mod attachment;
mod embed;
mod message;
mod message_application;
mod message_reference;
mod overwrite;
mod reaction;

// Re-exports
pub use attachment::Attachment;
pub use embed::{Embed, EmbedAuthor, EmbedField, EmbedFooter, EmbedImage, EmbedProvider, EmbedThumbnail, EmbedVideo};
pub use message::Message;
pub use message_application::MessageApplication;
pub use message_reference::MessageReference;
pub use overwrite::Overwrite;
pub use reaction::Reaction;

use crate::models::user::*;

use serde::{Deserialize, Serialize};
use serde_repr::*;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Channel {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: ChannelKind,
    pub guild_id: Option<String>,
    pub position: Option<u64>,
    #[serde(default)]
    pub permission_overwrites: Vec<Overwrite>,
    pub name: Option<String>,
    pub topic: Option<String>,
    pub nsfw: Option<bool>,
    pub last_message_id: Option<String>,

    // For voice channels
    pub bitrate: Option<u64>,
    pub user_limit: Option<u64>,
    pub rate_limit_per_user: Option<u64>,

    // For DM
    pub recipients: Option<Vec<User>>,
    pub icon: Option<String>,
    pub owner_id: Option<String>,
    pub application_id: Option<String>,

    pub parent_id: Option<String>,
    pub last_pin_timestamp: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MentionChannel {
    id: String,
    guild_id: String,
    #[serde(rename = "type")]
    kind: u64, // TODO
    name: String,
}

#[derive(Clone, Debug, Deserialize_repr, Serialize_repr, PartialEq)]
#[repr(u8)]
pub enum ChannelKind {
    GuildText = 0,
    DM = 1,
    GuildVoice = 2,
    GroupDM = 3,
    GuildCategory = 4,
    GuildNews = 5,
    GuildStore = 6,
}
