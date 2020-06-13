use super::{Guild, Channel, User};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Invite {
    pub code: String,
    pub guild: Option<Guild>,
    pub channel: Channel,
    pub inviter: Option<User>,
    pub target_user: Option<User>,
    // target_user_type always 1
    pub approximate_presence_count: Option<u64>,
    pub approximate_member_count: u64
}