mod rate_limit;
mod routing;

use rate_limit::RateLimit;
use routing::Route;

use crate::{
    error::{PandaError, Result},
    models::{
        channel::{Channel, Embed, Message},
        user::User,
    },
};

use isahc::{
    http::{Method, StatusCode},
    prelude::*,
    HttpClient as IsachClient,
};
use serde::Serialize;

/// It's the http client of panda, it have methods to make requests to all routes
pub struct HttpClient {
    token: String,
    client: IsachClient,
    rate_limit: RateLimit,
}

impl HttpClient {
    /// Creates a new http client
    pub fn new(token: impl Into<String>) -> HttpClient {
        let client = IsachClient::new().expect("Can't create Http Client");
        HttpClient {
            token: token.into(),
            client,
            rate_limit: RateLimit::default(),
        }
    }

    async fn _make_request<B: Into<Body>>(&self, route: Route<B>) -> Result<Response<Body>> {
        // Check and wait if we reach the limit
        self.rate_limit.check_and_sleep(&route.bucket_key).await;

        // TODO: Improve this
        let response = match route.method {
            Method::GET | Method::PUT | Method::DELETE => {
                let request = Request::builder()
                    .method(route.method)
                    .uri(&route.uri)
                    .header("Authorization", &self.token)
                    .body(())
                    .unwrap();

                // Get response
                self.client.send_async(request).await?
            }
            Method::POST | Method::PATCH => {
                let request = Request::builder()
                    .method(route.method)
                    .uri(&route.uri)
                    .header("Authorization", &self.token)
                    .header("Content-Type", "application/json")
                    .body(route.body)
                    .unwrap();

                // Get response
                self.client.send_async(request).await?
            }
            _ => unimplemented!(),
        };

        // Update the limit with the response headers
        self.rate_limit.update(route.bucket_key, &response).await;

        Ok(response)
    }

    // TODO: Rename this and improve
    fn _catch_http_errors(&self, res: &Response<Body>) -> Result<()> {
        let err = match res.status() {
            StatusCode::OK
            | StatusCode::CREATED
            | StatusCode::NO_CONTENT
            | StatusCode::NOT_MODIFIED
            | StatusCode::TOO_MANY_REQUESTS => return Ok(()),

            StatusCode::BAD_REQUEST => PandaError::HttpImproperlyFormatted,
            StatusCode::FORBIDDEN => PandaError::HttpForbidden, // no autorizado
            StatusCode::NOT_FOUND => PandaError::HttpInvalidParameters, // not found or bad format
            StatusCode::METHOD_NOT_ALLOWED => PandaError::HttpNoResponse, // method not allowed
            // HANDLED BY RATELIMIT StatusCode::TOO_MANY_REQUESTS => PandaError::HttpNoResponse,
            StatusCode::BAD_GATEWAY => PandaError::HttpNoResponse, // gateway unavailable
            _ => PandaError::HttpNoResponse,
        };

        Err(err)

        //TODO: Remove HttpNoResponse here
    }

    // *******************************************************************************
    // * HTTP METHODS
    // *******************************************************************************

    /// Get a channel by ID. Returns a [`Channel`] object, it will fail if the ID it's invalid
    ///
    /// [`Channel`]: ../../panda/models/channel/struct.Channel.html
    pub async fn get_channel(&self, channel_id: impl AsRef<str>) -> Result<Channel> {
        // Create Route
        let route = Route::get_channel(channel_id);
        let mut res = self._make_request(route).await?;

        Ok(res.json()?)
    }

    /// Update a channel's settings. Requires the **MANAGE_CHANNELS** permission for the guild.
    /// Returns a [`Channel`] on success. It's recommended to use [`MessageEdit`] builder.
    /// Fires a [`ChannelUpdate`] event.
    ///
    /// [`Channel`]: ../../panda/models/channel/struct.Channel.html
    /// [`MessageEdit`]: ../../panda/utils/builder/struct.MessageEdit.html
    /// [`ChannelUpdate`]: ../../panda/models/gateway/events/struct.ChannelUpdate.html
    pub async fn edit_channel(&self, channel_id: impl AsRef<str>, body: impl Serialize) -> Result<Channel> {
        // Create route
        let body = serde_json::to_string(&body)?;
        let route = Route::edit_channel(channel_id, body);

        let mut res = self._make_request(route).await?;

        // If an error wasn't returned, it's safe to unwrap
        Ok(res.json()?)
    }

    /// Delete a channel, or close a private message. Requires the **MANAGE_CHANNELS** permission
    /// for the guild. Returns a [`Channel`] on success.
    /// Fires a [`ChannelDelete`] event.
    ///
    /// [`Channel`]: ../../panda/models/channel/struct.Channel.html
    /// [`ChannelDelete`]: ../../panda/models/gateway/events/struct.ChannelDelete.html
    pub async fn delete_channel(&self, channel_id: impl AsRef<str>) -> Result<Channel> {
        // Parse URL
        let route = Route::delete_channel(channel_id);

        let mut res = self._make_request(route).await?;

        Ok(res.json()?)
    }

    /// Returns a Vec<[`Message`]> of a channel. If operating on a guild channel, this endpoint
    /// requires the **VIEW_CHANNEL** permission to be present on the current user.
    ///
    /// [`Channel`]: ../../panda/models/channel/struct.Channel.html
    pub async fn get_messages_around(
        &self,
        channel_id: impl AsRef<str>,
        message_id: impl AsRef<str>,
        limit: u8,
    ) -> Result<Vec<Message>> {
        // Create route
        let route = Route::get_channel_messages("around", channel_id, message_id, limit);

        let mut res = self._make_request(route).await?;

        Ok(res.json()?)
    }

    /// Returns a Vec<[`Message`]> of a channel. If operating on a guild channel, this endpoint
    /// requires the **VIEW_CHANNEL** permission to be present on the current user.
    ///
    /// [`Channel`]: ../../panda/models/channel/struct.Channel.html
    pub async fn get_messages_before(
        &self,
        channel_id: impl AsRef<str>,
        message_id: impl AsRef<str>,
        limit: u8,
    ) -> Result<Vec<Message>> {
        // Create route
        let route = Route::get_channel_messages("before", channel_id, message_id, limit);

        let mut res = self._make_request(route).await?;

        Ok(res.json()?)
    }

    /// Returns a Vec<[`Message`]> of a channel. If operating on a guild channel, this endpoint
    /// requires the **VIEW_CHANNEL** permission to be present on the current user.
    ///
    /// [`Channel`]: ../../panda/models/channel/struct.Channel.html
    pub async fn get_messages_after(
        &self,
        channel_id: impl AsRef<str>,
        message_id: impl AsRef<str>,
        limit: u8,
    ) -> Result<Vec<Message>> {
        // Create route
        let route = Route::get_channel_messages("after", channel_id, message_id, limit);

        let mut res = self._make_request(route).await?;

        Ok(res.json()?)
    }

    /// Returns a specific [`Message`] in the channel. If operating on a guild channel, this endpoint
    /// requires the **READ_MESSAGE_HISTORY** permission to be present on the current user.
    ///
    /// [`Message`]: ../../panda/models/channel/struct.Message.html
    pub async fn get_message(&self, channel_id: impl AsRef<str>, msg_id: impl AsRef<str>) -> Result<Message> {
        // Create route
        let route = Route::get_channel_message(channel_id, msg_id);

        let mut res = self._make_request(route).await?;

        Ok(res.json()?)
    }

    /// Creates a new message, and returns the [`Message`]. This will also trigger
    /// [`MessageCreate`] event
    ///
    /// [`Message`]: ../../panda/models/channel/struct.Message.html
    /// [`MessageCreate`]: ../../panda/models/gateway/events/struct.MessageCreate.html
    pub async fn send_message(&self, channel_id: impl AsRef<str>, content: impl AsRef<str>) -> Result<Message> {
        // Create message body
        let body = serde_json::json!({
            "content": content.as_ref(),
            "tts": false
        });
        // Parse to a valid Body, isahc::Body
        let body = serde_json::to_string(&body)?;

        // Create route
        let route = Route::create_message(channel_id, body);
        let mut res = self._make_request(route).await?;

        Ok(res.json()?)
    }

    /// Creates a new message, and returns the [`Message`]. This will also trigger
    /// [`MessageCreate`] event
    ///
    /// [`Message`]: ../../panda/models/channel/struct.Message.html
    /// [`MessageCreate`]: ../../panda/models/gateway/events/struct.MessageCreate.html
    pub async fn send_embed(&self, channel_id: impl AsRef<str>, embed: Embed) -> Result<Message> {
        let body = serde_json::json!({
            "embed": embed,
            "tts": false
        });

        let body = serde_json::to_string(&body)?;

        // Create route
        let route = Route::create_message(channel_id, body);

        let mut res = self._make_request(route).await?;

        Ok(res.json()?)
    }

    /// Add a reaction to a [`Message`], it needs the [`Channel`] ID, and [`Message`] ID
    ///
    /// [`Channel`]: ../../panda/models/channel/struct.channel.html
    /// [`Message`]: ../../panda/models/channel/struct.Message.html
    pub async fn add_reaction(
        &self,
        channel_id: impl AsRef<str>,
        message_id: impl AsRef<str>,
        emoji: impl AsRef<str>,
    ) -> Result<()> {
        // Create route
        let route = Route::create_reaction(channel_id, message_id, emoji);

        let _res = self._make_request(route).await?;

        Ok(())
    }

    /// Remove a own reaction to a [`Message`], it needs the [`Channel`] ID, and [`Message`] ID
    ///
    /// [`Channel`]: ../../panda/models/channel/struct.channel.html
    /// [`Message`]: ../../panda/models/channel/struct.Message.html
    pub async fn remove_own_reaction(
        &self,
        channel_id: impl AsRef<str>,
        message_id: impl AsRef<str>,
        emoji: impl AsRef<str>,
    ) -> Result<()> {
        // Create route
        let route = Route::delete_own_reaction(channel_id, message_id, emoji);

        let _res = self._make_request(route).await?;

        Ok(())
    }

    /// Remove an [`User`] reaction to a [`Message`], it needs the [`Channel`] ID, [`Message`] ID
    /// and [`User`] ID.
    ///
    /// [`Channel`]: ../../panda/models/channel/struct.channel.html
    /// [`Message`]: ../../panda/models/channel/struct.Message.html
    /// [`User`]: ../../panda/models/user/struct.User.html
    pub async fn remove_user_reaction(
        &self,
        channel_id: impl AsRef<str>,
        message_id: impl AsRef<str>,
        user_id: impl AsRef<str>,
        emoji: impl AsRef<str>,
    ) -> Result<()> {
        // Create route
        let route = Route::delete_user_reaction(channel_id, message_id, emoji, user_id);

        let _res = self._make_request(route).await?;

        Ok(())
    }

    /// Get all [`User`]s that reacted with given emoji to a [`Message`],
    /// it needs the [`Channel`] ID, [`Message`] ID
    ///
    /// [`Channel`]: ../../panda/models/channel/struct.channel.html
    /// [`Message`]: ../../panda/models/channel/struct.Message.html
    /// [`User`]: ../../panda/models/user/struct.User.html
    pub async fn get_reactions(
        &self,
        channel_id: impl AsRef<str>,
        message_id: impl AsRef<str>,
        emoji: impl AsRef<str>,
    ) -> Result<Vec<User>> {
        let route = Route::get_reactions(channel_id, message_id, emoji);

        let mut res = self._make_request(route).await?;

        Ok(res.json()?)
    }

    /// Deletes all reactions on a [`Message`]. This endpoint requires the **MANAGE_MESSAGES**
    /// permission to be present on the current user. Fires a [`MessageReactionRemoveAll`].
    ///
    /// [`Message`]: ../../panda/models/channel/struct.Message.html
    /// [`MessageReactionRemoveAll`]: ../../panda/models/gateway/events/struct.MessageReactionRemoveAll.html
    pub async fn remove_all_reactions(&self, channel_id: impl AsRef<str>, message_id: impl AsRef<str>) -> Result<()> {
        let route = Route::delete_all_reactions(channel_id, message_id);

        let _res = self._make_request(route).await?;

        Ok(())
    }

    /// Deletes all reactions on a [`Message`]. This endpoint requires the **MANAGE_MESSAGES**
    /// permission to be present on the current user. Fires a [`MessageReactionRemoveEmoji`].
    ///
    /// [`Message`]: ../../panda/models/channel/struct.Message.html
    /// [`MessageReactionRemoveEmoji`]: ../../panda/models/gateway/events/struct.MessageReactionRemoveEmoji.html
    pub async fn remove_all_emoji_reactions(
        &self,
        channel_id: impl AsRef<str>,
        message_id: impl AsRef<str>,
        emoji: impl AsRef<str>,
    ) -> Result<()> {
        let route = Route::delete_all_reactions_for_emoji(channel_id, message_id, emoji);

        let _res = self._make_request(route).await?;

        Ok(())
    }

    /// Edits message, and returns the [`Message`]. This will also trigger [`MessageUpdate`] event
    ///
    /// [`Message`]: ../../panda/models/channel/struct.Message.html
    /// [`MessageUpdate`]: ../../panda/models/gateway/events/struct.MessageUpdate.html
    pub async fn edit_message(
        &self,
        channel_id: impl AsRef<str>,
        message_id: impl AsRef<str>,
        body: impl Serialize,
    ) -> Result<Message> {
        let body = serde_json::to_string(&body)?;

        let route = Route::edit_message(channel_id, message_id, body);

        let mut res = self._make_request(route).await?;

        Ok(res.json()?)
    }

    /// Delete a [`Message`], This will also trigger [`MessageDelete`] event
    ///
    /// [`Message`]: ../../panda/models/channel/struct.Message.html
    /// [`MessageDelete`]: ../../panda/models/gateway/events/struct.MessageDelete.html
    pub async fn delete_message(&self, channel_id: impl AsRef<str>, message_id: impl AsRef<str>) -> Result<()> {
        let route = Route::delete_message(channel_id, message_id);

        let _res = self._make_request(route).await?;

        Ok(())
    }

    /// Delete a a bulk of [`Message`] (2 - 100), This will also trigger [`MessageDeleteBulk`] event.
    ///
    /// [`Message`]: ../../panda/models/channel/struct.Message.html
    /// [`MessageDelete`]: ../../panda/models/gateway/events/struct.MessageDelete.html
    pub async fn delete_many_messages(&self, channel_id: impl AsRef<str>, messages: &[&str]) -> Result<()> {
        let body = serde_json::json!({ "messages": messages });
        let body = serde_json::to_string(&body).unwrap();

        let route = Route::bulk_delete_messages(channel_id, body);

        let _res = self._make_request(route).await?;

        Ok(())
    }

    /// Edit the channel permission overwrites for a user or role in a channel. Only usable
    /// for guild channels. Requires the **MANAGE_ROLES** permission.
    ///
    /// [`Message`]: ../../panda/models/channel/struct.Message.html
    /// [`MessageDelete`]: ../../panda/models/gateway/events/struct.MessageDelete.html
    pub async fn edit_channel_permissions(&self, _channel_id: impl AsRef<str>) -> Result<()> {
        unimplemented!();
        // // Create RateLimit Key
        // let route = Route::edit_channel_permissions(channel_id, overwrite_id, body);

        // let _res = self._make_request(route).await?;

        // Ok(())
    }

    // // pub async fn get_channel_invites() {}
    // // pub async fn create_channel_invite() {}

    // // pub async fn delete_channel_permissions() {}

    /// Post a typing indicator for the specified channel.
    /// Fires a [`TypingStart`] Gateway event
    ///
    /// [`TypingStart`]: ../../panda/models/gateway/events/struct.TypingStart.html
    pub async fn trigger_typing(&self, channel_id: impl AsRef<str>) -> Result<()> {
        let route = Route::trigger_typing_indicator(channel_id);
        let _res = self._make_request(route).await?;

        Ok(())
    }

    /// Returns all pinned messages in the channel as a Vec of [`Message`] objects.
    ///
    /// [`Message`]: ../../panda/models/channel/struct.Message.html
    pub async fn get_pinned_messages(&self, channel_id: impl AsRef<str>) -> Result<Vec<Message>> {

        // TODO: Returns a 204 empty response on success.

        let route = Route::get_pinned_messages(channel_id);

        let mut res = self._make_request(route).await?;

        Ok(res.json()?)
    }

    /// Pin a message in a channel. Requires the **MANAGE_MESSAGES** permission
    /// **The max pinned messages is 50.**
    pub async fn pin_message(&self, channel_id: impl AsRef<str>, message_id: impl AsRef<str>) -> Result<()> {

        // TODO: Returns a 204 empty response on success.

        let route = Route::add_pinned_channel_message(channel_id, message_id);

        let _ = self._make_request(route).await?;

        Ok(())
    }

    /// Pin a message in a channel. Requires the **MANAGE_MESSAGES** permission.
    pub async fn unpin_message(&self, channel_id: impl AsRef<str>, message_id: impl AsRef<str>) -> Result<()> {

        // TODO: Returns a 204 empty response on success.

        let route = Route::delete_pinned_channel_message(channel_id, message_id);

        let _ = self._make_request(route).await?;

        Ok(())
    }

    // PUT/channels/{channel.id}/recipients/{user.id}

    // DELETE/channels/{channel.id}/recipients/{user.id}
}
