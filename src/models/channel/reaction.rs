use crate::models::Emoji;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Reaction {
    pub count: u64,
    pub me: bool,
    pub emoji: Emoji,
}
