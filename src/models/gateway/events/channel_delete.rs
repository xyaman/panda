use crate::models::channel::Channel;
use serde::{Deserialize, Serialize};

use std::ops::Deref;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChannelDelete(pub Channel);

impl Deref for ChannelDelete {
    type Target = Channel;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
