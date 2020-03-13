use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GuildRoleDelete {
    guild_id: String,
    role_id: String,
}
