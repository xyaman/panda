// models
use super::session::Session;
use crate::models::gateway::events::*;

use std::default::Default;

// async
use async_std::sync::Arc;

// Futures
use futures::future::BoxFuture;

// READY function trait
type ReadyFn = dyn Fn(Arc<Session>, Ready) -> BoxFuture<'static, ()> + Send + Sync;

// CHANNEL functions trait
type ChannelCreateFn = dyn Fn(Arc<Session>, ChannelCreate) -> BoxFuture<'static, ()> + Send + Sync;
type ChannelUpdateFn = dyn Fn(Arc<Session>, ChannelUpdate) -> BoxFuture<'static, ()> + Send + Sync;
type ChannelDeleteFn = dyn Fn(Arc<Session>, ChannelDelete) -> BoxFuture<'static, ()> + Send + Sync;
type ChannelPinsUpdateFn =
    dyn Fn(Arc<Session>, ChannelPinsUpdate) -> BoxFuture<'static, ()> + Send + Sync;

// GUILD functions trait
type GuildCreateFn = dyn Fn(Arc<Session>, GuildCreate) -> BoxFuture<'static, ()> + Send + Sync;
type GuildUpdateFn = dyn Fn(Arc<Session>, GuildUpdate) -> BoxFuture<'static, ()> + Send + Sync;
type GuildDeleteFn = dyn Fn(Arc<Session>, GuildDelete) -> BoxFuture<'static, ()> + Send + Sync;
type GuildBanAddFn = dyn Fn(Arc<Session>, GuildBanAdd) -> BoxFuture<'static, ()> + Send + Sync;
type GuildBanRemoveFn =
    dyn Fn(Arc<Session>, GuildBanRemove) -> BoxFuture<'static, ()> + Send + Sync;
type GuildEmojisUpdateFn =
    dyn Fn(Arc<Session>, GuildEmojisUpdate) -> BoxFuture<'static, ()> + Send + Sync;
type GuildIntegrationsUpdateFn =
    dyn Fn(Arc<Session>, GuildIntegrationsUpdate) -> BoxFuture<'static, ()> + Send + Sync;
type GuildMemberAddFn =
    dyn Fn(Arc<Session>, GuildMemberAdd) -> BoxFuture<'static, ()> + Send + Sync;
type GuildMemberUpdateFn =
    dyn Fn(Arc<Session>, GuildMemberUpdate) -> BoxFuture<'static, ()> + Send + Sync;
type GuildMemberRemoveFn =
    dyn Fn(Arc<Session>, GuildMemberRemove) -> BoxFuture<'static, ()> + Send + Sync;
type GuildMembersChunkFn =
    dyn Fn(Arc<Session>, GuildMembersChunk) -> BoxFuture<'static, ()> + Send + Sync;
type GuildRoleCreateFn =
    dyn Fn(Arc<Session>, GuildRoleCreate) -> BoxFuture<'static, ()> + Send + Sync;
type GuildRoleUpdateFn =
    dyn Fn(Arc<Session>, GuildRoleUpdate) -> BoxFuture<'static, ()> + Send + Sync;
type GuildRoleDeleteFn =
    dyn Fn(Arc<Session>, GuildRoleDelete) -> BoxFuture<'static, ()> + Send + Sync;

// MESSAGE functions trait
type MessageCreateFn = dyn Fn(Arc<Session>, MessageCreate) -> BoxFuture<'static, ()> + Send + Sync;
type MessageUpdateFn = dyn Fn(Arc<Session>, MessageUpdate) -> BoxFuture<'static, ()> + Send + Sync;
type MessageDeleteFn = dyn Fn(Arc<Session>, MessageDelete) -> BoxFuture<'static, ()> + Send + Sync;
type MessageDeleteBulkFn =
    dyn Fn(Arc<Session>, MessageDeleteBulk) -> BoxFuture<'static, ()> + Send + Sync;
type MessageReactionAddFn =
    dyn Fn(Arc<Session>, MessageReactionAdd) -> BoxFuture<'static, ()> + Send + Sync;
type MessageReactionRemoveFn =
    dyn Fn(Arc<Session>, MessageReactionRemove) -> BoxFuture<'static, ()> + Send + Sync;
type MessageReactionRemoveAllFn =
    dyn Fn(Arc<Session>, MessageReactionRemoveAll) -> BoxFuture<'static, ()> + Send + Sync;

// Presence functions trait
type PresenceUpdateFn =
    dyn Fn(Arc<Session>, PresenceUpdate) -> BoxFuture<'static, ()> + Send + Sync;
type TypingStartFn = dyn Fn(Arc<Session>, TypingStart) -> BoxFuture<'static, ()> + Send + Sync;
type UserUpdateFn = dyn Fn(Arc<Session>, UserUpdate) -> BoxFuture<'static, ()> + Send + Sync;

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
