// Channel
mod channel_create;
mod channel_delete;
mod channel_pins_update;
mod channel_update;

// Guild
mod guild_ban;
mod guild_create;
mod guild_delete;
mod guild_emojis_update;
mod guild_integrations_update;
mod guild_member_add;
mod guild_member_remove;
mod guild_member_update;
mod guild_members_chunk;
mod guild_role_create;
mod guild_role_delete;
mod guild_role_update;
mod guild_update;

// Message
mod message_create;
mod message_delete;
mod message_delete_bulk;
mod message_reaction_add;
mod message_reaction_remove;
mod message_reaction_remove_all;
mod message_reaction_remove_emoji;
mod message_update;

mod presence_update;
mod typing_start;
mod user_update;

mod voice_server_update;
mod voice_state_update;

mod ready;

// Re-exports
// CHANNEL
pub use channel_create::ChannelCreate;
pub use channel_delete::ChannelDelete;
pub use channel_pins_update::ChannelPinsUpdate;
pub use channel_update::ChannelUpdate;

// GUILDS
pub use guild_ban::GuildBanAdd;
pub use guild_ban::GuildBanRemove;
pub use guild_create::GuildCreate;
pub use guild_delete::GuildDelete;
pub use guild_emojis_update::GuildEmojisUpdate;
pub use guild_integrations_update::GuildIntegrationsUpdate;
pub use guild_member_add::GuildMemberAdd;
pub use guild_member_remove::GuildMemberRemove;
pub use guild_member_update::GuildMemberUpdate;
pub use guild_members_chunk::GuildMembersChunk;
pub use guild_role_create::GuildRoleCreate;
pub use guild_role_delete::GuildRoleDelete;
pub use guild_role_update::GuildRoleUpdate;
pub use guild_update::GuildUpdate;

// MESSAGE
pub use message_create::MessageCreate;
pub use message_delete::MessageDelete;
pub use message_delete_bulk::MessageDeleteBulk;
pub use message_reaction_add::MessageReactionAdd;
pub use message_reaction_remove::MessageReactionRemove;
pub use message_reaction_remove_all::MessageReactionRemoveAll;
pub use message_reaction_remove_emoji::MessageReactionRemoveEmoji;
pub use message_update::MessageUpdate;

// PRECENCE
pub use presence_update::PresenceUpdate;
pub use typing_start::TypingStart;
pub use user_update::UserUpdate;

// VOICE
pub use voice_server_update::VoiceServerUpdate;
pub use voice_state_update::VoiceStateUpdate;

// READY
pub use ready::Ready;

// crate
use super::payload::{Opcode, Payload};
use crate::error::{PandaError, Result};

use std::convert::TryFrom;

use serde_json::Value;

#[derive(Debug)]
pub(crate) enum Event {
    // Discord events
    Dispatch(DispatchEvent), // op: 0
    Reconnect,               // op: 7
    InvalidSession(bool),    // op: 9
    Hello(u64),              // op: 10
    HeartbeatACK,            // op: 11
    Close(PandaError),
}

#[derive(Debug)]
pub(crate) enum DispatchEvent {
    Ready(Ready),
    Resumed,
    Reconnect,
    ChannelCreate(ChannelCreate),
    ChannelUpdate(ChannelUpdate),
    ChannelDelete(ChannelDelete),
    ChannelPinsUpdate(ChannelPinsUpdate),

    // guild
    GuildCreate(GuildCreate),
    GuildUpdate(GuildUpdate),
    GuildDelete(GuildDelete),
    GuildBanAdd(GuildBanAdd),
    GuildBanRemove(GuildBanRemove),
    GuildEmojisUpdate(GuildEmojisUpdate),
    GuildIntegrationsUpdate(GuildIntegrationsUpdate),
    GuildMemberAdd(GuildMemberAdd),
    GuildMemberRemove(GuildMemberRemove),
    GuildMemberUpdate(GuildMemberUpdate),
    GuildMembersChunk(GuildMembersChunk),
    GuildRoleCreate(GuildRoleCreate),
    GuildRoleUpdate(GuildRoleUpdate),
    GuildRoleDelete(GuildRoleDelete),

    // message
    MessageCreate(MessageCreate),
    MessageUpdate(MessageUpdate),
    MessageDelete(MessageDelete),
    MessageDeleteBulk(MessageDeleteBulk),
    MessageReactionAdd(MessageReactionAdd),
    MessageReactionRemove(MessageReactionRemove),
    MessageReactionRemoveAll(MessageReactionRemoveAll),
    MessageReactionRemoveEmoji(MessageReactionRemoveEmoji),

    // presence
    PresenceUpdate(PresenceUpdate),
    TypingStart(TypingStart),
    UserUpdate(UserUpdate),

    // voice
    VoiceStateUpdate(VoiceStateUpdate),
    VoiceServerUpdate(VoiceServerUpdate),
}

impl TryFrom<Payload> for Event {
    type Error = PandaError;

    fn try_from(p: Payload) -> Result<Event> {
        match p.op {
            Opcode::Dispatch => Ok(Event::Dispatch(handle_dispatch(p)?)),
            Opcode::Reconnect => Ok(Event::Reconnect),
            Opcode::InvalidSession => {
                let d = p.d.ok_or_else(|| PandaError::InvalidPayloadFormat)?;
                let resumable = match d {
                    Value::Bool(v) => v,
                    _ => return Err(PandaError::InvalidPayloadFormat),
                };

                Ok(Event::InvalidSession(resumable))
            }
            Opcode::Hello => {
                #[derive(serde::Deserialize)]
                struct Hello {
                    heartbeat_interval: u64,
                }
                let d = p.d.ok_or_else(|| PandaError::InvalidPayloadFormat)?;
                let hello: Hello = serde_json::from_value(d).unwrap();

                Ok(Event::Hello(hello.heartbeat_interval))
            }
            Opcode::HeartbeatACK => Ok(Event::HeartbeatACK),
            _ => Err(PandaError::UnexpectedPayloadReceived),
        }
    }
}

///
fn handle_dispatch(p: Payload) -> Result<DispatchEvent> {
    let d = p.d.ok_or_else(|| PandaError::InvalidPayloadFormat)?;
    let t = p.t.ok_or_else(|| PandaError::InvalidPayloadFormat)?;

    match t.as_str() {
        "READY" => {
            let event = serde_json::from_value(d)?;
            Ok(DispatchEvent::Ready(event))
        }
        "RESUMED" => Ok(DispatchEvent::Resumed),
        "RECONNECT" => Ok(DispatchEvent::Reconnect),
        // Channel
        "CHANNEL_CREATE" => {
            let event = serde_json::from_value(d)?;
            Ok(DispatchEvent::ChannelCreate(event))
        }
        "CHANNEL_UPDATE" => {
            let event = serde_json::from_value(d)?;
            Ok(DispatchEvent::ChannelUpdate(event))
        }
        "CHANNEL_DELETE" => {
            let event = serde_json::from_value(d)?;
            Ok(DispatchEvent::ChannelDelete(event))
        }
        "CHANNEL_PINS_UPDATE" => {
            let event = serde_json::from_value(d)?;
            Ok(DispatchEvent::ChannelPinsUpdate(event))
        }

        // Guild
        "GUILD_CREATE" => {
            let event = serde_json::from_value(d)?;
            Ok(DispatchEvent::GuildCreate(event))
        }
        "GUILD_UPDATE" => {
            let event = serde_json::from_value(d)?;
            Ok(DispatchEvent::GuildUpdate(event))
        }
        "GUILD_DELETE" => {
            let event = serde_json::from_value(d)?;
            Ok(DispatchEvent::GuildDelete(event))
        }
        "GUILD_BAN_ADD" => {
            let event = serde_json::from_value(d)?;
            Ok(DispatchEvent::GuildBanAdd(event))
        }
        "GUILD_BAN_REMOVE" => {
            let event = serde_json::from_value(d)?;
            Ok(DispatchEvent::GuildBanRemove(event))
        }
        "GUILD_EMOJIS_UPDATE" => {
            let event = serde_json::from_value(d)?;
            Ok(DispatchEvent::GuildEmojisUpdate(event))
        }
        "GUILD_INTEGRATIONS_UPDATE" => {
            let event = serde_json::from_value(d)?;
            Ok(DispatchEvent::GuildIntegrationsUpdate(event))
        }
        "GUILD_MEMBER_ADD" => {
            let event = serde_json::from_value(d)?;
            Ok(DispatchEvent::GuildMemberAdd(event))
        }
        "GUILD_MEMBER_UPDATE" => {
            let event = serde_json::from_value(d)?;
            Ok(DispatchEvent::GuildMemberUpdate(event))
        }
        "GUILD_MEMBER_REMOVE" => {
            let event = serde_json::from_value(d)?;
            Ok(DispatchEvent::GuildMemberRemove(event))
        }
        "GUILD_MEMBER_CHUNK" => {
            let event = serde_json::from_value(d)?;
            Ok(DispatchEvent::GuildMembersChunk(event))
        }
        "GUILD_ROLE_CREATE" => {
            let event = serde_json::from_value(d)?;
            Ok(DispatchEvent::GuildRoleCreate(event))
        }
        "GUILD_ROLE_UPDATE" => {
            let event = serde_json::from_value(d)?;
            Ok(DispatchEvent::GuildRoleUpdate(event))
        }
        "GUILD_ROLE_DELETE" => {
            let event = serde_json::from_value(d)?;
            Ok(DispatchEvent::GuildRoleDelete(event))
        }

        // Message
        "MESSAGE_CREATE" => {
            let event = serde_json::from_value(d)?;
            Ok(DispatchEvent::MessageCreate(event))
        }
        "MESSAGE_UPDATE" => {
            let event = serde_json::from_value(d)?;
            Ok(DispatchEvent::MessageUpdate(event))
        }
        "MESSAGE_DELETE" => {
            let event = serde_json::from_value(d)?;
            Ok(DispatchEvent::MessageDelete(event))
        }
        "MESSAGE_DELETE_BULK" => {
            let event = serde_json::from_value(d)?;
            Ok(DispatchEvent::MessageDeleteBulk(event))
        }
        "MESSAGE_REACTION_ADD" => {
            let event = serde_json::from_value(d)?;
            Ok(DispatchEvent::MessageReactionAdd(event))
        }
        "MESSAGE_REACTION_REMOVE" => {
            let event = serde_json::from_value(d)?;
            Ok(DispatchEvent::MessageReactionRemove(event))
        }
        "MESSAGE_REACTION_REMOVE_ALL" => {
            let event = serde_json::from_value(d)?;
            Ok(DispatchEvent::MessageReactionRemoveAll(event))
        }
        "MESSAGE_REACTION_REMOVE_EMOJI" => {
            let event = serde_json::from_value(d)?;
            Ok(DispatchEvent::MessageReactionRemoveEmoji(event))
        }

        // Presence
        "PRESENCE_UPDATE" => {
            let event = serde_json::from_value(d)?;
            Ok(DispatchEvent::PresenceUpdate(event))
        }
        "TYPING_START" => {
            let event = serde_json::from_value(d)?;
            Ok(DispatchEvent::TypingStart(event))
        }
        "USER_UPDATE" => {
            let event = serde_json::from_value(d)?;
            Ok(DispatchEvent::UserUpdate(event))
        }

        // Voice
        "VOICE_STATE_UPDATE" => {
            let event = serde_json::from_value(d)?;
            Ok(DispatchEvent::VoiceStateUpdate(event))
        }
        "VOICE_SERVER_UPDATE" => {
            let event = serde_json::from_value(d)?;
            Ok(DispatchEvent::VoiceServerUpdate(event))
        }
        _ => Err(PandaError::InvalidPayloadFormat),
    }
}
