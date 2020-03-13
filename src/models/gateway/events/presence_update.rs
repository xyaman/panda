use crate::models::user::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PresenceUpdate {
    user: User,
    roles: Vec<String>,
    game: Option<Activity>,
    guild_id: String,
    status: String, // use enum
    activities: Vec<Activity>,
    //client_status: ClientStatus
}

impl PresenceUpdate {
    pub fn user(&self) -> &User {
        &self.user
    }
    
    pub fn roles(&self) -> &[String] {
        self.roles.as_ref()
    }
    
    pub fn game(&self) -> Option<&Activity> {
        self.game.as_ref()
    }
    
    pub fn guild_id(&self) -> &str {
        &self.guild_id
    }
    
    pub fn status(&self) -> &str { // use enum
        &self.status
    }

    pub fn activities(&self) -> &[Activity] {
        self.activities.as_ref()
    }
    //client_status: ClientStatus
}