use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageDeleteBulk {
    pub ids: Vec<String>,
    pub channel_id: String,
    pub guild_id: Option<String>,
}
