use crate::models::guild::GuildMember;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuildMembersChunk {
    pub guild_id: String,
    pub members: Vec<GuildMember>,
    not_found: Option<Vec<String>>,
    //presences: Vec<Presence>
}
