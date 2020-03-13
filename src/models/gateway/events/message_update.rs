use crate::models::channel::Message;
use serde::{Deserialize, Serialize};

use std::ops::Deref;

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageUpdate(pub Message);

impl Deref for MessageUpdate {
    type Target = Message;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
