use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ChannelPinsUpdate {
    guild_id: Option<String>,
    channel_id: String,
    //last_pin_timestamp:
}
