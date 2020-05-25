//! Channel related models

mod embed;
mod message;
mod overwrite;

// Re-exports
pub use embed::{Embed, EmbedAuthor, EmbedField, EmbedFooter, EmbedImage, EmbedProvider, EmbedThumbnail, EmbedVideo};
pub use message::Message;
pub use overwrite::Overwrite;

use crate::models::user::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Channel {
    pub d: String,
    #[serde(rename = "type")]
    pub kind: u64,
    pub guild_id: Option<String>,
    pub position: Option<u64>,
    //permission_overwrites: Option<Overwrite>,
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
