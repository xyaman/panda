use crate::models::guild::*;
use crate::models::user::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Ready {
    v: u8,
    user: User,
    pub private_channels: Option<Vec<()>>,

    #[serde(skip_deserializing)]
    pub guilds: Vec<Guild>,

    pub session: String,
    pub shard: Option<[u64; 2]>,
}

impl Ready {
    pub fn version(&self) -> u8 {
        self.v
    }

    pub fn user(&self) -> &User {
        &self.user
    }
}
