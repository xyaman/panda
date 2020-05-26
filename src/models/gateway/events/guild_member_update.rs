use crate::models::user::User;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GuildMemberUpdate {
    pub guild_id: String,
    pub roles: Vec<String>,
    pub user: User,
    pub nick: String,
}
