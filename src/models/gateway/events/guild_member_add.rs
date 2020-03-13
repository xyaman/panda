use crate::models::guild::GuildMember;
use serde::{Deserialize, Serialize};

use std::ops::Deref;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuildMemberAdd(pub GuildMember);

impl Deref for GuildMemberAdd {
    type Target = GuildMember;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
