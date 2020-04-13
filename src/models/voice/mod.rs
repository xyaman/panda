use crate::models::guild::GuildMember;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct VoiceState {
    guild_id: Option<String>,
    channel_id: String,
    user_id: String,
    member: Option<GuildMember>,
    session_id: String,
    deaf: bool,
    mute: bool,
    self_deaf: bool,
    self_mute: bool,
    self_stream: Option<bool>,
    supress: bool,
}
