use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuildDelete {
    pub id: String,
    pub unavailable: bool,
}
