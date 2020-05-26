use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TypingStart {
    pub channel_id: String,
    pub guild_id: Option<String>,
    pub user_id: String,
    pub timestamp: u64, // Unix time in seconds
}
