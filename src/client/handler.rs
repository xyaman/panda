// models
use super::session::Session;
use crate::models::gateway::events::*;

use std::{default::Default, error::Error};

// async
use async_std::sync::Arc;

// Futures
use futures::future::BoxFuture;

pub(crate) type EventResult = Result<(), Box<dyn Error>>;

/// Helper macro to create futures function trait
macro_rules! event_trait {
    ($event: tt) => {
        dyn Fn(Arc<Session>, $event) -> BoxFuture<'static, EventResult> + Send + Sync
    };
}

// READY function trait
type ReadyFn = event_trait!(Ready);

// CHANNEL functions trait
type ChannelCreateFn = event_trait!(ChannelCreate);
type ChannelUpdateFn = event_trait!(ChannelUpdate);
type ChannelDeleteFn = event_trait!(ChannelDelete);
type ChannelPinsUpdateFn = event_trait!(ChannelPinsUpdate);

// GUILD functions trait
type GuildCreateFn = event_trait!(GuildCreate);
type GuildUpdateFn = event_trait!(GuildUpdate);
type GuildDeleteFn = event_trait!(GuildDelete);
type GuildBanAddFn = event_trait!(GuildBanAdd);
type GuildBanRemoveFn = event_trait!(GuildBanRemove);
type GuildEmojisUpdateFn = event_trait!(GuildEmojisUpdate);
type GuildIntegrationsUpdateFn = event_trait!(GuildIntegrationsUpdate);
type GuildMemberAddFn = event_trait!(GuildMemberAdd);
type GuildMemberUpdateFn = event_trait!(GuildMemberUpdate);
type GuildMemberRemoveFn = event_trait!(GuildMemberRemove);
type GuildMembersChunkFn = event_trait!(GuildMembersChunk);
type GuildRoleCreateFn = event_trait!(GuildRoleCreate);
type GuildRoleUpdateFn = event_trait!(GuildRoleUpdate);
type GuildRoleDeleteFn = event_trait!(GuildRoleDelete);

// MESSAGE functions trait
type MessageCreateFn = event_trait!(MessageCreate);
type MessageUpdateFn = event_trait!(MessageUpdate);
type MessageDeleteFn = event_trait!(MessageDelete);
type MessageDeleteBulkFn = event_trait!(MessageDeleteBulk);
type MessageReactionAddFn = event_trait!(MessageReactionAdd);
type MessageReactionRemoveFn = event_trait!(MessageReactionRemove);
type MessageReactionRemoveAllFn = event_trait!(MessageReactionRemoveAll);

// Presence functions trait
type PresenceUpdateFn = event_trait!(PresenceUpdate);
type TypingStartFn = event_trait!(TypingStart);
type UserUpdateFn = event_trait!(UserUpdate);

type OptionBox<T> = Option<Box<T>>;

/// This struct it's where all functions created by the user will be saved
#[derive(Default)]
pub(crate) struct EventHandler {
    pub(crate) ready: OptionBox<ReadyFn>,

    // Channel
    pub(crate) channel_create: OptionBox<ChannelCreateFn>,
    pub(crate) channel_update: OptionBox<ChannelUpdateFn>,
    pub(crate) channel_delete: OptionBox<ChannelDeleteFn>,
    pub(crate) channel_pins_update: OptionBox<ChannelPinsUpdateFn>,

    // Guild
    pub(crate) guild_create: OptionBox<GuildCreateFn>,
    pub(crate) guild_update: OptionBox<GuildUpdateFn>,
    pub(crate) guild_delete: OptionBox<GuildDeleteFn>,
    pub(crate) guild_ban_add: OptionBox<GuildBanAddFn>,
    pub(crate) guild_ban_remove: OptionBox<GuildBanRemoveFn>,
    pub(crate) guild_emojis_update: OptionBox<GuildEmojisUpdateFn>,
    pub(crate) guild_integrations_update: OptionBox<GuildIntegrationsUpdateFn>,
    pub(crate) guild_member_add: OptionBox<GuildMemberAddFn>,
    pub(crate) guild_member_remove: OptionBox<GuildMemberRemoveFn>,
    pub(crate) guild_member_update: OptionBox<GuildMemberUpdateFn>,
    pub(crate) guild_members_chunk: OptionBox<GuildMembersChunkFn>,
    pub(crate) guild_role_create: OptionBox<GuildRoleCreateFn>,
    pub(crate) guild_role_update: OptionBox<GuildRoleUpdateFn>,
    pub(crate) guild_role_delete: OptionBox<GuildRoleDeleteFn>,

    // Message
    pub(crate) message_create: OptionBox<MessageCreateFn>,
    pub(crate) message_update: OptionBox<MessageUpdateFn>,
    pub(crate) message_delete: OptionBox<MessageDeleteFn>,
    pub(crate) message_delete_bulk: OptionBox<MessageDeleteBulkFn>,
    pub(crate) message_reaction_add: OptionBox<MessageReactionAddFn>,
    pub(crate) message_reaction_remove: OptionBox<MessageReactionRemoveFn>,
    pub(crate) message_reaction_remove_all: OptionBox<MessageReactionRemoveAllFn>,

    // Presence
    pub(crate) presence_update: OptionBox<PresenceUpdateFn>,
    pub(crate) typing_start: OptionBox<TypingStartFn>,
    pub(crate) user_update: OptionBox<UserUpdateFn>,
}

impl EventHandler {
    pub(crate) fn new() -> Self {
        Self::default()
    }
}
