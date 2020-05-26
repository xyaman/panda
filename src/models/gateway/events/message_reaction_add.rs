use crate::models::{emoji::Emoji, guild::GuildMember};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageReactionAdd {
    pub user_id: String,    //
    pub channel_id: String, //
    pub message_id: String, //
    pub guild_id: Option<String>,
    pub member: Option<GuildMember>,
    pub emoji: Emoji, //
}
