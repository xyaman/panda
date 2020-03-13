use crate::models::guild::Role;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GuildRoleCreate {
    guild_id: String,
    role: Role,
}
