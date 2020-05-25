mod member;
mod role;

use crate::models::{channel::Channel, emoji::Emoji};
use serde::{Deserialize, Serialize};

pub use member::Member as GuildMember;
pub use role::Role;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Guild {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub splash: Option<String>,
    pub owner: Option<bool>,
    pub owner_id: String,
    pub permissions: Option<u64>,
    pub region: String,
    pub afk_channel_id: Option<String>,
    pub afk_timeout: u64,
    pub embed_enabled: Option<bool>,
    pub embed_channel_id: Option<String>,
    pub verification_level: u64, // maybe use enum
    pub default_message_notifications: u64,
    pub explicit_content_filter: u64,
    pub roles: Vec<Role>,
    pub emojis: Vec<Emoji>,
    //pe) features: Vec<Feature>,
    pub mfa_level: u64,
    pub application_id: Option<String>,
    pub widget_enabled: Option<bool>,
    pub widget_channel_id: Option<String>,
    pub system_channel_id: Option<String>,
    // te) max_presences:
    pub max_members: Option<u64>,
    pub vanity_url_code: Option<String>,
    pub description: Option<String>,
    pub banner: Option<String>,
    pub premium_tier: u64,
    pub premium_subscription_count: Option<u64>,
    pub preferred_locale: String,
    // elds are only sent within GUILD_CREATE event
    pub joined_at: String,
    pub large: Option<bool>,
    pub unavailable: bool,
    pub member_count: Option<u64>,
    // ce_states: Vec<Voice>,
    pub members: Vec<GuildMember>,
    pub channels: Vec<Channel>,
    // pub presences: Vec<Presence>,
}
