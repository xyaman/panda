use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageReactionRemoveAll {
    pub channel_id: String,
    pub message_id: String,
    pub guild_id: Option<String>,
}
