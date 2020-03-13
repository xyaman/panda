use crate::models::guild::*;
use serde::{Deserialize, Serialize};

use std::ops::Deref;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuildCreate(pub Guild);

impl Deref for GuildCreate {
    type Target = Guild;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
