use crate::models::guild::GuildMember;
use crate::models::user::User;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
/// Represents a message sent in a channel within Discord.
pub struct Message {
    id: String,
    channel_id: String,
    guild_id: Option<String>,
    author: User,
    member: Option<GuildMember>,
    content: String,
    timestamp: String,
    edited_timestamp: Option<String>,
    tts: bool,
    mention_everyone: bool,
    mentions: Vec<User>,
    mention_roles: Vec<String>,
    // mentions_channels: Vec<ChannelMention>,
    // attatchments: Vec<Attachtment>,
    // embed: Vec<Embed>,
    // reactions: Vec<Reactions>
    nonce: Option<String>,
    pinned: bool,
    webhook_id: Option<String>,
    #[serde(rename = "type")]
    kind: Option<u64>, // TODO: Use const enum
    // activity: MessageActivity,
    // application: MessageApplication,
    // message_reference: MessageReference
    flags: Option<u64>,
}

impl Message {
    /// Returns the message id
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the id of the channel the message was sent
    pub fn channel_id(&self) -> &str {
        &self.channel_id
    }

    /// Returns the id of the guild the message was sent in,
    pub fn guild_id(&self) -> Option<&str> {
        self.guild_id.as_ref().map(|s| s.as_str())
    }

    /// Returns the author of this message
    pub fn author(&self) -> &User {
        &self.author
    }

    /// Returns the guild member properties for this message's author
    pub fn member(&self) -> Option<&GuildMember> {
        self.member.as_ref()
    }

    /// Returns the contents of the message
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Returns when this message was sent in ISO8601 timestamp
    pub fn timestamp(&self) -> &str {
        &self.timestamp
    }

    /// Returns when this message was edited (None if never) in ISO8601 timestamp
    pub fn edited_timestamp(&self) -> Option<&str> {
        self.edited_timestamp.as_ref().map(|v| v.as_str())
    }

    /// Returns whether this was a TTS message
    pub fn tts(&self) -> bool {
        self.tts
    }

    /// Returns whether this message mentions everyone
    pub fn mention_everyone(&self) -> bool {
        self.mention_everyone
    }

    /// Returns the users specifically mentioned in the message
    pub fn mentions(&self) -> &[User] {
        &self.mentions
    }

    /// Returns the roles specifically mentioned in this message
    pub fn mention_roles(&self) -> &[String] {
        &self.mention_roles
    }

    // pub fn mentions_channels(&self) -> &[ChannelMention] {
    //  &self.mentions_channels
    // }

    // pub fn attatchments(&self) -> &[Attachtment] {
    //  &self.attatchments
    // }

    // pub fn embed -> &[Embed] {
    //  &self.embed
    // }

    // pub fn reactions(&self) -> &[Reactions] {
    //  &self.reactions
    // }

    pub fn nonce(&self) -> Option<&str> {
        self.nonce.as_ref().map(|s| s.as_str())
    }

    pub fn pinned(&self) -> bool {
        self.pinned
    }

    pub fn webhook_id(&self) -> Option<&str> {
        self.webhook_id.as_ref().map(|v| v.as_str())
    }

    pub fn kind(&self) -> Option<u64> {
        self.kind
    }

    // activity: MessageActivity,
    // application: MessageApplication,
    // message_reference: MessageReference
    pub fn flags(&self) -> Option<u64> {
        self.flags
    }
}
