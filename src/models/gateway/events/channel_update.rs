use crate::models::channel::Channel;
use serde::{Deserialize, Serialize};

use std::ops::Deref;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChannelUpdate(pub Channel);

impl Deref for ChannelUpdate {
    type Target = Channel;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
