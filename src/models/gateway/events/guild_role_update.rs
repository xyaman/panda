use crate::models::guild::Role;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GuildRoleUpdate {
    guild_id: String,
    role: Role,
}
