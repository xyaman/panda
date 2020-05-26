use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageReference {
    pub id: Option<String>,
    pub channel_id: Option<String>,
    pub guild_id: Option<String>,
}
