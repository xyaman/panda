use crate::models::guild::Guild;
use serde::{Deserialize, Serialize};

use std::ops::Deref;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuildUpdate(pub Guild);

impl Deref for GuildUpdate {
    type Target = Guild;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
