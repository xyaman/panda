use crate::models::guild::GuildMember;
use serde::{Deserialize, Serialize};

// TODO: implement presences
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuildMembersChunk {
    pub guild_id: String,
    pub members: Vec<GuildMember>,
    #[serde(default)]
    pub not_found: Vec<String>,
    //presences: Vec<Presence>
}
