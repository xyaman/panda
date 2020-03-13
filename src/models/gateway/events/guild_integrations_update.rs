use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GuildIntegrationsUpdate {
    guild_id: String,
}
