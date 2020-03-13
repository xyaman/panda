use crate::models::user::User;
use serde::{Deserialize, Serialize};

use std::ops::Deref;

#[derive(Debug, Deserialize, Serialize)]
pub struct GuildBan {
    guild_id: String,
    user: User,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GuildBanAdd(GuildBan);

#[derive(Debug, Deserialize, Serialize)]
pub struct GuildBanRemove(GuildBan);

impl Deref for GuildBanAdd {
    type Target = GuildBan;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for GuildBanRemove {
    type Target = GuildBan;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
