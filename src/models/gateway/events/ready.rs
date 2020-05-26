use crate::models::guild::*;
use crate::models::user::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Ready {
    pub v: u8,
    pub user: User,
    // pub private_channels: Option<Vec<()>>,
    #[serde(skip_deserializing)]
    pub guilds: Vec<Guild>,

    pub session_id: String,
    pub shard: Option<[u64; 2]>,
}
