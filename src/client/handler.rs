// models
use super::session::SessionData;
use crate::models::gateway::events::*;

use std::{error::Error, sync::Arc};

// async
// Futures
use futures::future::BoxFuture;

pub(crate) type EventResult = Result<(), Box<dyn Error>>;

/// Helper macro to create futures function trait
macro_rules! event_trait {
    ($event: tt) => {
        dyn Fn(Arc<SessionData<S>>, $event) -> BoxFuture<'static, EventResult> + Send + Sync
    };
}

// READY function trait
type ReadyFn<S> = event_trait!(Ready);

// CHANNEL functions trait
type ChannelCreateFn<S> = event_trait!(ChannelCreate);
type ChannelUpdateFn<S> = event_trait!(ChannelUpdate);
type ChannelDeleteFn<S> = event_trait!(ChannelDelete);
type ChannelPinsUpdateFn<S> = event_trait!(ChannelPinsUpdate);

// GUILD functions trait
type GuildCreateFn<S> = event_trait!(GuildCreate);
type GuildUpdateFn<S> = event_trait!(GuildUpdate);
type GuildDeleteFn<S> = event_trait!(GuildDelete);
type GuildBanAddFn<S> = event_trait!(GuildBanAdd);
type GuildBanRemoveFn<S> = event_trait!(GuildBanRemove);
type GuildEmojisUpdateFn<S> = event_trait!(GuildEmojisUpdate);
type GuildIntegrationsUpdateFn<S> = event_trait!(GuildIntegrationsUpdate);
type GuildMemberAddFn<S> = event_trait!(GuildMemberAdd);
type GuildMemberUpdateFn<S> = event_trait!(GuildMemberUpdate);
type GuildMemberRemoveFn<S> = event_trait!(GuildMemberRemove);
type GuildMembersChunkFn<S> = event_trait!(GuildMembersChunk);
type GuildRoleCreateFn<S> = event_trait!(GuildRoleCreate);
type GuildRoleUpdateFn<S> = event_trait!(GuildRoleUpdate);
type GuildRoleDeleteFn<S> = event_trait!(GuildRoleDelete);

// MESSAGE functions trait
type MessageCreateFn<S> = event_trait!(MessageCreate);
type MessageUpdateFn<S> = event_trait!(MessageUpdate);
type MessageDeleteFn<S> = event_trait!(MessageDelete);
type MessageDeleteBulkFn<S> = event_trait!(MessageDeleteBulk);
type MessageReactionAddFn<S> = event_trait!(MessageReactionAdd);
type MessageReactionRemoveFn<S> = event_trait!(MessageReactionRemove);
type MessageReactionRemoveAllFn<S> = event_trait!(MessageReactionRemoveAll);
type MessageReactionRemoveEmojiFn<S> = event_trait!(MessageReactionRemoveEmoji);

// Presence functions trait
type PresenceUpdateFn<S> = event_trait!(PresenceUpdate);
type TypingStartFn<S> = event_trait!(TypingStart);
type UserUpdateFn<S> = event_trait!(UserUpdate);

type OptionBox<T> = Option<Box<T>>;

/// This struct it's where all functions created by the user will be saved
pub(crate) struct EventHandler<S> {
    pub(crate) ready: OptionBox<ReadyFn<S>>,

    // Channel
    pub(crate) channel_create: OptionBox<ChannelCreateFn<S>>,
    pub(crate) channel_update: OptionBox<ChannelUpdateFn<S>>,
    pub(crate) channel_delete: OptionBox<ChannelDeleteFn<S>>,
    pub(crate) channel_pins_update: OptionBox<ChannelPinsUpdateFn<S>>,

    // Guild
    pub(crate) guild_create: OptionBox<GuildCreateFn<S>>,
    pub(crate) guild_update: OptionBox<GuildUpdateFn<S>>,
    pub(crate) guild_delete: OptionBox<GuildDeleteFn<S>>,
    pub(crate) guild_ban_add: OptionBox<GuildBanAddFn<S>>,
    pub(crate) guild_ban_remove: OptionBox<GuildBanRemoveFn<S>>,
    pub(crate) guild_emojis_update: OptionBox<GuildEmojisUpdateFn<S>>,
    pub(crate) guild_integrations_update: OptionBox<GuildIntegrationsUpdateFn<S>>,
    pub(crate) guild_member_add: OptionBox<GuildMemberAddFn<S>>,
    pub(crate) guild_member_remove: OptionBox<GuildMemberRemoveFn<S>>,
    pub(crate) guild_member_update: OptionBox<GuildMemberUpdateFn<S>>,
    pub(crate) guild_members_chunk: OptionBox<GuildMembersChunkFn<S>>,
    pub(crate) guild_role_create: OptionBox<GuildRoleCreateFn<S>>,
    pub(crate) guild_role_update: OptionBox<GuildRoleUpdateFn<S>>,
    pub(crate) guild_role_delete: OptionBox<GuildRoleDeleteFn<S>>,

    // Message
    pub(crate) message_create: OptionBox<MessageCreateFn<S>>,
    pub(crate) message_update: OptionBox<MessageUpdateFn<S>>,
    pub(crate) message_delete: OptionBox<MessageDeleteFn<S>>,
    pub(crate) message_delete_bulk: OptionBox<MessageDeleteBulkFn<S>>,
    pub(crate) message_reaction_add: OptionBox<MessageReactionAddFn<S>>,
    pub(crate) message_reaction_remove: OptionBox<MessageReactionRemoveFn<S>>,
    pub(crate) message_reaction_remove_all: OptionBox<MessageReactionRemoveAllFn<S>>,
    pub(crate) message_reaction_remove_emoji: OptionBox<MessageReactionRemoveEmojiFn<S>>,

    // Presence
    pub(crate) presence_update: OptionBox<PresenceUpdateFn<S>>,
    pub(crate) typing_start: OptionBox<TypingStartFn<S>>,
    pub(crate) user_update: OptionBox<UserUpdateFn<S>>,
}

impl<S> EventHandler<S> {
    pub(crate) fn new() -> Self {
        Self {
            ready: None,

            // Channel
            channel_create: None,
            channel_update: None,
            channel_delete: None,
            channel_pins_update: None,

            // Guild
            guild_create: None,
            guild_update: None,
            guild_delete: None,
            guild_ban_add: None,
            guild_ban_remove: None,
            guild_emojis_update: None,
            guild_integrations_update: None,
            guild_member_add: None,
            guild_member_remove: None,
            guild_member_update: None,
            guild_members_chunk: None,
            guild_role_create: None,
            guild_role_update: None,
            guild_role_delete: None,

            // Message
            message_create: None,
            message_update: None,
            message_delete: None,
            message_delete_bulk: None,
            message_reaction_add: None,
            message_reaction_remove: None,
            message_reaction_remove_all: None,
            message_reaction_remove_emoji: None,
            // Presence
            presence_update: None,
            typing_start: None,
            user_update: None,
        }
    }
}
