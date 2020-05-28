use crate::{
    error::Result,
    http::HttpClient,
    models::{guild::GuildMember, user::User},
};

use super::{Embed, MentionChannel, MessageReference, Attachment, Reaction, MessageApplication};

use serde::{Deserialize, Serialize};
use serde_repr::*;

#[derive(Debug, Deserialize, Serialize)]
/// Represents a message sent in a channel within Discord.
pub struct Message {
    /// ID of the message
    pub id: String,
    
    /// ID of the channel where the message was sent
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
    #[serde(default)]
    pub mentions_channels: Vec<MentionChannel>,
    pub attachments: Vec<Attachment>,
    #[serde(default)]
    pub embed: Vec<Embed>,
    #[serde(default)]
    pub reactions: Vec<Reaction>,
    pub nonce: Option<String>,
    pub pinned: bool,
    pub webhook_id: Option<String>,
    #[serde(rename = "type")]
    pub kind: Option<Kind>,
    // activity: MessageActivity,
    pub application: Option<MessageApplication>,
    pub message_reference: Option<MessageReference>,
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

    pub async fn remove(&self, http: &HttpClient) -> Result<()> {
        http.delete_message(&self.channel_id, &self.id).await
    }

    pub async fn send_message(&self, http: &HttpClient, content: impl AsRef<str>) -> Result<Message> {
        http.send_message(&self.channel_id, content).await
    }

    pub async fn send_embed(&self, http: &HttpClient, embed: super::Embed) -> Result<Message> {
        http.send_embed(&self.channel_id, embed).await
    }

    pub async fn add_reaction(&self, http: &HttpClient, emoji: impl AsRef<str>) -> Result<()> {
        http.add_reaction(&self.channel_id, &self.id, emoji).await
    }
}
