use crate::models::user::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PresenceUpdate {
    pub user: User,
    pub roles: Vec<String>,
    pub game: Option<Activity>,
    pub guild_id: String,
    pub status: String, // use enum
    pub activities: Vec<Activity>,
    //client_status: ClientStatus
}
