//! Voice related models

use crate::models::guild::GuildMember;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct VoiceState {
    pub guild_id: Option<String>,
    pub channel_id: String,
    pub user_id: String,
    pub member: Option<GuildMember>,
    pub session_id: String,
    pub deaf: bool,
    pub mute: bool,
    pub self_deaf: bool,
    pub self_mute: bool,
    pub self_stream: Option<bool>,
    pub supress: bool,
}
