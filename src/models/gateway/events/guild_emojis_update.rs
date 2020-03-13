use crate::models::emoji::Emoji;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GuildEmojisUpdate {
    guild_id: String,
    emojis: Vec<Emoji>,
}
