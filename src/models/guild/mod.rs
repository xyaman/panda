mod member;
mod role;

use crate::models::{channel::Channel, emoji::Emoji};
use serde::{Deserialize, Serialize};

pub use member::Member as GuildMember;
pub use role::Role;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Guild {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) icon: Option<String>,
    pub(crate) splash: Option<String>,
    pub(crate) owner: Option<bool>,
    pub(crate) owner_id: String,
    pub(crate) permissions: Option<u64>,
    pub(crate) region: String,
    pub(crate) afk_channel_id: Option<String>,
    pub(crate) afk_timeout: u64,
    pub(crate) embed_enabled: Option<bool>,
    pub(crate) embed_channel_id: Option<String>,
    pub(crate) verification_level: u64, // maybe use enum
    pub(crate) default_message_notifications: u64,
    pub(crate) explicit_content_filter: u64,
    pub(crate) roles: Vec<Role>,
    pub(crate) emojis: Vec<Emoji>,
    //pub(crate) features: Vec<Feature>,
    pub(crate) mfa_level: u64,
    pub(crate) application_id: Option<String>,
    pub(crate) widget_enabled: Option<bool>,
    pub(crate) widget_channel_id: Option<String>,
    pub(crate) system_channel_id: Option<String>,
    // pub(crate) max_presences:
    pub(crate) max_members: Option<u64>,
    pub(crate) vanity_url_code: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) banner: Option<String>,
    pub(crate) premium_tier: u64,
    pub(crate) premium_subscription_count: Option<u64>,
    pub(crate) preferred_locale: String,

    // This fields are only sent within GUILD_CREATE event
    pub(crate) joined_at: String,
    pub(crate) large: Option<bool>,
    pub(crate) unavailable: bool,
    pub(crate) member_count: Option<u64>,
    // pub voice_states: Vec<Voice>,
    pub(crate) members: Vec<GuildMember>,
    pub(crate) channels: Vec<Channel>,
    // pub presences: Vec<Presence>,
}

impl Guild {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn icon(&self) -> Option<&str> {
        self.icon.as_ref().map(|s| s.as_str())
    }

    pub fn splash(&self) -> Option<&str> {
        self.splash.as_ref().map(|s| s.as_str())
    }

    pub fn owner(&self) -> Option<bool> {
        self.owner
    }

    pub fn owner_id(&self) -> &str {
        &self.owner_id
    }

    pub fn permissions(&self) -> Option<u64> {
        self.permissions
    }

    pub fn region(&self) -> &str {
        &self.region
    }

    pub fn afk_channel_id(&self) -> Option<&str> {
        self.afk_channel_id.as_ref().map(|s| s.as_str())
    }

    pub fn afk_timeout(&self) -> u64 {
        self.afk_timeout
    }

    pub fn embed_enabled(&self) -> Option<bool> {
        self.embed_enabled
    }

    pub fn embed_channel_id(&self) -> Option<&str> {
        self.embed_channel_id.as_ref().map(|s| s.as_str())
    }

    pub fn verification_level(&self) -> u64 {
        self.verification_level
    }

    pub fn default_message_notifications(&self) -> u64 {
        self.default_message_notifications
    }

    pub fn explicit_content_filter(&self) -> u64 {
        self.explicit_content_filter
    }

    pub fn roles(&self) -> &[Role] {
        self.roles.as_ref()
    }

    pub fn emojis(&self) -> &[Emoji] {
        self.emojis.as_ref()
    }

    //features: Vec<Feature>,

    pub fn mfa_level(&self) -> u64 {
        self.mfa_level
    }

    pub fn application_id(&self) -> Option<&str> {
        self.application_id.as_ref().map(|s| s.as_str())
    }

    pub fn widget_enabled(&self) -> Option<bool> {
        self.widget_enabled
    }

    pub fn widget_channel_id(&self) -> Option<&str> {
        self.widget_channel_id.as_ref().map(|s| s.as_str())
    }

    pub fn system_channel_id(&self) -> Option<&str> {
        self.system_channel_id.as_ref().map(|s| s.as_str())
    }

    // pub max_presences:

    pub fn max_members(&self) -> Option<u64> {
        self.max_members
    }

    pub fn vanity_url_code(&self) -> Option<&str> {
        self.vanity_url_code.as_ref().map(|s| s.as_str())
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_ref().map(|s| s.as_str())
    }

    pub fn banner(&self) -> Option<&str> {
        self.banner.as_ref().map(|s| s.as_str())
    }

    pub fn premium_tier(&self) -> u64 {
        self.premium_tier
    }

    pub fn premium_subscription_count(&self) -> Option<u64> {
        self.premium_subscription_count
    }

    pub fn preferred_locale(&self) -> &str {
        &self.preferred_locale
    }

    //This fields are only sent within GUILD_CREATE event

    pub fn joined_at(&self) -> &str {
        &self.joined_at
    }

    pub fn is_large(&self) -> Option<bool> {
        self.large
    }

    pub fn unavailable(&self) -> bool {
        self.unavailable
    }

    pub fn member_count(&self) -> Option<u64> {
        self.member_count
    }

    // pub fn voice_states(&self) -> Vec<Voice>,

    pub fn members(&self) -> &[GuildMember] {
        self.members.as_ref()
    }

    pub fn channels(&self) -> &[Channel] {
        self.channels.as_ref()
    }

    // pub fn presences(&self) -> Vec<Presence>,
}
