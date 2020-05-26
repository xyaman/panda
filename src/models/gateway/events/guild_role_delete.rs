use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GuildRoleDelete {
    pub guild_id: String,
    pub role_id: String,
}
