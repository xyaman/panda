use crate::models::user::User;
// use crate::models::guild::Role;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Member {
    pub user: Option<User>,
    pub nick: Option<String>,
    pub roles: Vec<String>,
    pub joined_at: Option<String>,
    pub premium_since: Option<String>,
    pub deaf: bool,
    pub mute: bool,

    // only for GUILD_MEMBER_ADD
    pub guild_id: Option<String>,
}
