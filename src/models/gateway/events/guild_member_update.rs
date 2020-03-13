use crate::models::user::User;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GuildMemberUpdate {
    guild_id: String,
    roles: Vec<String>,
    user: User,
    nick: String,
}
