use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TypingStart {
    channel_id: String,
    guild_id: Option<String>,
    user_id: String,
    timestamp: u64, // Unix time in seconds
}

impl TypingStart {
    pub fn channel_id(&self) -> &str {
        &self.channel_id
    }

    pub fn guild_id(&self) -> Option<&str> {
        self.guild_id.as_ref().map(|s| s.as_str())
    }

    pub fn user_id(&self) -> &str {
        &self.user_id
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }
}
