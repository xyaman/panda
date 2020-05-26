use crate::models::emoji::Emoji;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageReactionRemove {
    pub user_id: String,
    pub channel_id: String,
    pub message_id: String,
    pub guild_id: Option<String>,
    pub emoji: Emoji,
}
