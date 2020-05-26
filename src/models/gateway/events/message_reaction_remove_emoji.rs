use crate::models::emoji::Emoji;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageReactionRemoveEmoji {
    pub channel_id: String,
    pub guild_id: Option<String>,
    pub message_id: String,
    pub emoji: Emoji,
}
