use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ChannelPinsUpdate {
    pub guild_id: Option<String>,
    pub channel_id: String,
    //last_pin_timestamp:
}
