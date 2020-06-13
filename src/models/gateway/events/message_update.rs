use crate::models::Embed;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageUpdate {
    pub id: String,
    pub channel_id: String,
    pub guild_id: Option<String>,

    #[serde(default)]
    pub embeds: Vec<Embed>,
}
