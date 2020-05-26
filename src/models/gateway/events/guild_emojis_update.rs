use crate::models::emoji::Emoji;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GuildEmojisUpdate {
    pub guild_id: String,
    pub emojis: Vec<Emoji>,
}
