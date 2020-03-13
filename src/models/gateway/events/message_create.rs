use crate::models::channel::Message;
use serde::{Deserialize, Serialize};

use std::ops::Deref;

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageCreate(pub Message);

impl Deref for MessageCreate {
    type Target = Message;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
