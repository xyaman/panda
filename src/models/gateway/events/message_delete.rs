use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageDelete {
    pub id: String,
    pub channel_id: String,
    pub guild_id: Option<String>,
}
