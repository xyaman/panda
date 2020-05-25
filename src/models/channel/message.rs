use crate::{
    error::Result,
    http::HttpClient,
    models::{guild::GuildMember, user::User},
};

use super::Embed;

use serde::{Deserialize, Serialize};
use serde_repr::*;

#[derive(Debug, Deserialize, Serialize)]
/// Represents a message sent in a channel within Discord.
pub struct Message {
    pub id: String,
    pub channel_id: String,
    pub guild_id: Option<String>,
    pub author: User,
    pub member: Option<GuildMember>,
    pub content: String,
    pub timestamp: String,
    pub edited_timestamp: Option<String>,
    pub tts: bool,
    pub mention_everyone: bool,
    pub mentions: Vec<User>,
    pub mention_roles: Vec<String>,
     // mentions_channels: Vec<ChannelMention>,
     // attatchments: Vec<Attachtment>,
    #[serde(default)]
    pub embed: Vec<Embed>,
    // reactions: Vec<Reactions>
    pub nonce: Option<String>,
    pub pinned: bool,
    pub webhook_id: Option<String>,
    #[serde(rename = "type")]
    pub kind: Option<Kind>,
    // activity: MessageActivity,
     // application: MessageApplication,
     // message_reference: MessageReference
    pub flags: Option<u64>,
}
#[derive(Debug, Deserialize_repr, Serialize_repr, PartialEq)]
#[repr(u8)]
pub enum Kind {
    GuildText = 0,
    DM = 1,
    GuildVoice = 2,
    GroupDM = 3,
    GuildCategory = 4,
    GuildNews = 5,
    GuildStore = 6,
}

impl Message {
    pub async fn send_message(&self, http: &HttpClient, content: impl AsRef<str>) -> Result<Message> {
        http.send_message(&self.channel_id, content).await
    }

    pub async fn send_embed(&self, http: &HttpClient, embed: super::Embed) -> Result<Message> {
        http.send_embed(&self.channel_id, embed).await
    }
}
