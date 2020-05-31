use crate::models::user::*;

use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PresenceUpdate {
    #[serde(rename = "user")]
    #[serde(deserialize_with = "deserialize_user_id")]
    pub user_id: String,
    pub roles: Vec<String>,
    pub game: Option<Activity>,
    pub guild_id: String,
    pub status: String, // use enum
    pub activities: Vec<Activity>,
    //client_status: ClientStatus
}

// We use this function because discord not always send a full user,
// but always send the user id.
fn deserialize_user_id<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct User {
        id: String,
    }

    Deserialize::deserialize(deserializer).map(|user: User| user.id)
}
