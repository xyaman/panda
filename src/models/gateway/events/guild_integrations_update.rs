use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GuildIntegrationsUpdate {
    pub guild_id: String,
}
