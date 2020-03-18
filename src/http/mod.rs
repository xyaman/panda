mod rate_limit;
use rate_limit::RateLimit;

use crate::{
    error::{PandaError, Result},
    models::{
        channel::{Channel, Embed, Message},
        user::User,
    },
};

use isahc::{http::StatusCode, prelude::*, HttpClient as IsachClient};
use serde::Serialize;

pub(crate) const DISCORD_URL: &'static str = "https://discordapp.com/api";

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

    /// This function makes a GET request, and returns the response.
    /// uri: URL where the client will make a GET request.
    /// rt_key: RateLimit key
    async fn _get(&self, uri: String, rt_key: String) -> Result<Response<Body>> {
        // Create request
        let req = Request::builder()
            .method("GET")
            .uri(&uri)
            .header("Authorization", &self.token)
            .body(())
            .unwrap();

        // Check and wait if we reach the limit
        self.rate_limit.check_and_sleep(&rt_key).await;

        // Get response
        let res = self
            .client
            .send_async(req)
            .await
            .map_err(|_| PandaError::HttpNoResponse)?;

        // Catch http errors and return if there is one
        self._catch_http_errors(&res)?;

        // Update the limit with the response headers
        self.rate_limit.update(rt_key, &res).await;

        // if let Err(e) = self.catch_http_error() {
        //      parse error JSON struct
        // }

        Ok(res)
    }

    /// This function makes a POST request, and returns the response.
    async fn _post(&self, uri: String, rt_key: String, body: impl Into<Body>) -> Result<Response<Body>> {
        let req = Request::builder()
            .method("POST")
            .uri(uri)
            .header("Authorization", &self.token)
            .header("Content-Type", "application/json")
            .body(body)
            .unwrap();

        // Check and wait if we reach the limit
        self.rate_limit.check_and_sleep(&rt_key).await;

        let res = self
            .client
            .send_async(req)
            .await
            .map_err(|_| PandaError::HttpNoResponse)?;

        self._catch_http_errors(&res)?;

        // Update the limit with the response headers
        self.rate_limit.update(rt_key, &res).await;
        // if let Err(e) = self.catch_http_error() {
        //      parse error JSON struct
        // }

        Ok(res)
    }

    /// This function makes a PATCH request, and returns the response.
    async fn _patch(&self, uri: String, rt_key: String, body: impl Into<Body>) -> Result<Response<Body>> {
        let req = Request::builder()
            .method("PATCH")
            .uri(uri)
            .header("Authorization", &self.token)
            .header("Content-Type", "application/json")
            .body(body)
            .unwrap();

        // Check and wait if we reach the limit
        self.rate_limit.check_and_sleep(&rt_key).await;

        let res = self
            .client
            .send_async(req)
            .await
            .map_err(|_| PandaError::HttpNoResponse)?;

        self._catch_http_errors(&res)?;

        // Update the limit with the response headers
        self.rate_limit.update(rt_key, &res).await;
        // if let Err(e) = self.catch_http_error() {
        //      parse error JSON struct
        // }

        Ok(res)
    }

    /// This function makes a PATCH request, and returns the response.
    async fn _put(&self, uri: String, rt_key: String) -> Result<Response<Body>> {
        let req = Request::builder()
            .method("PUT")
            .uri(uri)
            .header("Authorization", &self.token)
            .body(())
            .unwrap();

        // Check and wait if we reach the limit
        self.rate_limit.check_and_sleep(&rt_key).await;

        let res = self
            .client
            .send_async(req)
            .await
            .map_err(|_| PandaError::HttpNoResponse)?;

        self._catch_http_errors(&res)?;

        // Update the limit with the response headers
        self.rate_limit.update(rt_key, &res).await;
        // if let Err(e) = self.catch_http_error() {
        //      parse error JSON struct
        // }

        Ok(res)
    }

    /// This function makes a GET request, and returns the response.
    async fn _delete(&self, uri: String, rt_key: String) -> Result<Response<Body>> {
        // Create request
        let req = Request::builder()
            .method("DELETE")
            .uri(uri)
            .header("Authorization", &self.token)
            .body(())
            .unwrap();

        // Check and wait if we reach the limit
        self.rate_limit.check_and_sleep(&rt_key).await;

        // Get response
        let res = self
            .client
            .send_async(req)
            .await
            .map_err(|_| PandaError::HttpNoResponse)?;

        // Catch http errors and return if there is one
        self._catch_http_errors(&res)?;

        // Update the limit with the response headers
        self.rate_limit.update(rt_key, &res).await;

        // if let Err(e) = self.catch_http_error() {
        //      parse error JSON struct
        // }

        Ok(res)
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
            // HANDLED BY RATELIMIT StatusCode::TOO_MANY_REQUESTS => PandaError::HttpNoResponse, // too many requests
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
        // Parse URL
        let uri = format!("{}/channels/{}", DISCORD_URL, channel_id.as_ref());

        // Create RateLimit Key
        let rt_key = format!("channels:{}", channel_id.as_ref());

        let mut res = self._get(uri, rt_key).await?;

        // If an error wasn't returned, it's safe to unwrap
        Ok(res.json().unwrap())
    }

    /// Update a channel's settings. Requires the **MANAGE_CHANNELS** permission for the guild.
    /// Returns a [`Channel`] on success. It's recommended to use [`MessageEdit`] builder.
    /// Fires a [`ChannelUpdate`] event.
    ///
    /// [`Channel`]: ../../panda/models/channel/struct.Channel.html
    /// [`MessageEdit`]: ../../panda/utils/builder/struct.MessageEdit.html
    /// [`ChannelUpdate`]: ../../panda/models/gateway/events/struct.ChannelUpdate.html
    pub async fn edit_channel(&self, channel_id: impl AsRef<str>, body: impl Serialize) -> Result<Channel> {
        // Parse URL
        let uri = format!("{}/channels/{}", DISCORD_URL, channel_id.as_ref());

        // Create RateLimit Key
        let rt_key = format!("channels:{}", channel_id.as_ref());

        let body = serde_json::to_string(&body).unwrap();
        let mut res = self._patch(uri, rt_key, body).await?;

        // If an error wasn't returned, it's safe to unwrap
        Ok(res.json().unwrap())
    }

    /// Delete a channel, or close a private message. Requires the **MANAGE_CHANNELS** permission
    /// for the guild. Returns a [`Channel`] on success.
    /// Fires a [`ChannelDelete`] event.
    ///
    /// [`Channel`]: ../../panda/models/channel/struct.Channel.html
    /// [`ChannelDelete`]: ../../panda/models/gateway/events/struct.ChannelDelete.html
    pub async fn delete_channel(&self, channel_id: impl AsRef<str>) -> Result<Channel> {
        // Parse URL
        let uri = format!("{}/channels/{}", DISCORD_URL, channel_id.as_ref());

        // Create RateLimit Key
        let rt_key = format!("channels:{}", channel_id.as_ref());

        let mut res = self._delete(uri, rt_key).await?;

        // If an error wasn't returned, it's safe to unwrap
        Ok(res.json().unwrap())
    }

    /// Returns a Vec<[`Message`]> of a channel. If operating on a guild channel, this endpoint
    /// requires the **VIEW_CHANNEL** permission to be present on the current user.
    ///
    /// [`Channel`]: ../../panda/models/channel/struct.Channel.html
    pub async fn get_messages_around(
        &self,
        channel_id: impl AsRef<str>,
        msg_id: impl AsRef<str>,
        limit: u8,
    ) -> Result<Vec<Message>> {
        // Parse URL
        let uri = format!(
            "{}/channels/{}/messages?around={}&limit={}",
            DISCORD_URL,
            channel_id.as_ref(),
            msg_id.as_ref(),
            limit
        );

        // Create RateLimit Key
        let rt_key = format!("channels:{}", channel_id.as_ref());

        let mut res = self._get(uri, rt_key).await?;

        // If an error wasn't returned, it's safe to unwrap
        Ok(res.json().unwrap())
    }

    /// Returns a Vec<[`Message`]> of a channel. If operating on a guild channel, this endpoint
    /// requires the **VIEW_CHANNEL** permission to be present on the current user.
    ///
    /// [`Channel`]: ../../panda/models/channel/struct.Channel.html
    pub async fn get_messages_before(
        &self,
        channel_id: impl AsRef<str>,
        msg_id: impl AsRef<str>,
        limit: u8,
    ) -> Result<Vec<Message>> {
        // Parse URL
        let uri = format!(
            "{}/channels/{}/messages?before={}&limit={}",
            DISCORD_URL,
            channel_id.as_ref(),
            msg_id.as_ref(),
            limit
        );

        // Create RateLimit Key
        let rt_key = format!("channels:{}", channel_id.as_ref());

        let mut res = self._get(uri, rt_key).await?;

        Ok(res.json().unwrap())
    }

    /// Returns a Vec<[`Message`]> of a channel. If operating on a guild channel, this endpoint
    /// requires the **VIEW_CHANNEL** permission to be present on the current user.
    ///
    /// [`Channel`]: ../../panda/models/channel/struct.Channel.html
    pub async fn get_messages_after(
        &self,
        channel_id: impl AsRef<str>,
        msg_id: impl AsRef<str>,
        limit: u8,
    ) -> Result<Vec<Message>> {
        // Format uri
        let uri = format!(
            "{}/channels/{}/messages?after={}&limit={}",
            DISCORD_URL,
            channel_id.as_ref(),
            msg_id.as_ref(),
            limit
        );

        // Create RateLimit Key
        let rt_key = format!("channels:{}", channel_id.as_ref());

        let mut res = self._get(uri, rt_key).await?;

        Ok(res.json().unwrap())
    }

    /// Returns a specific [`Message`] in the channel. If operating on a guild channel, this endpoint
    /// requires the **READ_MESSAGE_HISTORY** permission to be present on the current user.
    ///
    /// [`Message`]: ../../panda/models/channel/struct.Message.html
    pub async fn get_message(&self, channel_id: impl AsRef<str>, msg_id: impl AsRef<str>) -> Result<Message> {
        let uri = format!(
            "{}/channel/{}/messages/{}",
            DISCORD_URL,
            channel_id.as_ref(),
            msg_id.as_ref()
        );

        // Create RateLimit Key
        let rt_key = format!("channels:{}", channel_id.as_ref());

        let mut res = self._get(uri, rt_key).await?;

        Ok(res.json().unwrap())
    }

    /// Creates a new message, and returns the [`Message`]. This will also trigger
    /// [`MessageCreate`] event
    ///
    /// [`Message`]: ../../panda/models/channel/struct.Message.html
    /// [`MessageCreate`]: ../../panda/models/gateway/events/struct.MessageCreate.html
    pub async fn send_message(&self, channel_id: impl AsRef<str>, content: impl AsRef<str>) -> Result<Message> {
        let uri = format!("{}/channels/{}/messages", DISCORD_URL, channel_id.as_ref());

        let msg = serde_json::json!({
            "content": content.as_ref(),
            "tts": "false"
        });

        // Create RateLimit Key
        let rt_key = format!("channels:{}", channel_id.as_ref());

        let msg = serde_json::to_string(&msg).unwrap();

        let mut res = self._post(uri, rt_key, msg).await?;

        // If an error wasn't returned, it's safe to unwrap
        Ok(res.json().unwrap())
    }

    /// Creates a new message, and returns the [`Message`]. This will also trigger
    /// [`MessageCreate`] event
    ///
    /// [`Message`]: ../../panda/models/channel/struct.Message.html
    /// [`MessageCreate`]: ../../panda/models/gateway/events/struct.MessageCreate.html
    pub async fn send_embed(&self, channel_id: impl AsRef<str>, embed: Embed) -> Result<Message> {
        let uri = format!("{}/channels/{}/messages", DISCORD_URL, channel_id.as_ref());

        let msg = serde_json::json!({
            "embed": embed,
            "tts": "false"
        });

        // Create RateLimit Key
        let rt_key = format!("channels:{}", channel_id.as_ref());

        let msg = serde_json::to_string(&msg).unwrap();

        let mut res = self._post(uri, rt_key, msg).await?;

        // If an error wasn't returned, it's safe to unwrap
        Ok(res.json().unwrap())
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
        // Encode emoji
        let emoji = encode(emoji.as_ref());

        // Parse URL
        let uri = format!(
            "{}/channels/{}/messages/{}/reactions/{}/@me",
            DISCORD_URL,
            channel_id.as_ref(),
            message_id.as_ref(),
            emoji
        );

        // Create RateLimit Key
        let rt_key = format!("channel:{}:emoji", channel_id.as_ref());

        let _res = self._put(uri, rt_key).await?;

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
        // Encode emoji
        let emoji = encode(emoji.as_ref());

        // Parse URL
        let uri = format!(
            "{}/channels/{}/messages/{}/reactions/{}/@me",
            DISCORD_URL,
            channel_id.as_ref(),
            message_id.as_ref(),
            emoji
        );

        // Create RateLimit Key
        let rt_key = format!("channel:{}:emoji", channel_id.as_ref());

        let _res = self._delete(uri, rt_key).await?;

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
        user: impl AsRef<str>,
        emoji: impl AsRef<str>,
    ) -> Result<()> {
        // Encode emoji
        let emoji = encode(emoji.as_ref());

        // Parse URL
        let uri = format!(
            "{}/channels/{}/messages/{}/reactions/{}/{}",
            DISCORD_URL,
            channel_id.as_ref(),
            message_id.as_ref(),
            emoji,
            user.as_ref()
        );

        // Create RateLimit Key
        let rt_key = format!("channel:{}:emoji", channel_id.as_ref());

        let _res = self._delete(uri, rt_key).await?;

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
        // Encode emoji
        let emoji = encode(emoji.as_ref());

        // Parse URL
        let uri = format!(
            "{}/channels/{}/messages/{}/reactions/{}",
            DISCORD_URL,
            channel_id.as_ref(),
            message_id.as_ref(),
            emoji,
        );

        // Create RateLimit Key
        let rt_key = format!("channel:{}:emoji", channel_id.as_ref());

        let mut res = self._get(uri, rt_key).await?;

        Ok(res.json().unwrap())
    }

    /// Deletes all reactions on a [`Message`]. This endpoint requires the **MANAGE_MESSAGES**
    /// permission to be present on the current user. Fires a [`MessageReactionRemoveAll`].
    ///
    /// [`Message`]: ../../panda/models/channel/struct.Message.html
    /// [`MessageReactionRemoveAll`]: ../../panda/models/gateway/events/struct.MessageReactionRemoveAll.html
    pub async fn remove_all_reactions(&self, channel_id: impl AsRef<str>, message_id: impl AsRef<str>) -> Result<()> {
        // Parse URL
        let uri = format!(
            "{}/channels/{}/messages/{}/reactions",
            DISCORD_URL,
            channel_id.as_ref(),
            message_id.as_ref(),
        );

        // Create RateLimit Key
        let rt_key = format!("channel:{}:emoji", channel_id.as_ref());

        let _res = self._delete(uri, rt_key).await?;

        Ok(())
    }

    // TODO: ADD THIS EVENT. (NEW)
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
        let emoji = encode(emoji.as_ref());
        // Parse URL
        let uri = format!(
            "{}/channels/{}/messages/{}/reactions/{}",
            DISCORD_URL,
            channel_id.as_ref(),
            message_id.as_ref(),
            emoji
        );

        // Create RateLimit Key
        let rt_key = format!("channel:{}:emoji", channel_id.as_ref());

        let _res = self._delete(uri, rt_key).await?;

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
        let uri = format!(
            "{}/channels/{}/messages/{}",
            DISCORD_URL,
            channel_id.as_ref(),
            message_id.as_ref()
        );

        let body = serde_json::to_string(&body).unwrap();

        // Create RateLimit Key
        let rt_key = format!("channels:{}", channel_id.as_ref());

        let mut res = self._patch(uri, rt_key, body).await?;

        // If an error wasn't returned, it's safe to unwrap
        Ok(res.json().unwrap())
    }

    /// Delete a [`Message`], This will also trigger [`MessageDelete`] event
    ///
    /// [`Message`]: ../../panda/models/channel/struct.Message.html
    /// [`MessageDelete`]: ../../panda/models/gateway/events/struct.MessageDelete.html
    pub async fn delete_message(&self, channel_id: impl AsRef<str>, message_id: impl AsRef<str>) -> Result<()> {
        // Parse URL
        let uri = format!(
            "{}/channels/{}/messages/{}",
            DISCORD_URL,
            channel_id.as_ref(),
            message_id.as_ref()
        );

        // Create RateLimit Key
        let rt_key = format!("channels:{}", channel_id.as_ref());

        let _res = self._delete(uri, rt_key).await?;

        // If an error wasn't returned, it's safe to unwrap
        Ok(())
    }

    /// Delete a a bulk of [`Message`] (2 - 100), This will also trigger [`MessageDeleteBulk`] event.
    ///
    /// [`Message`]: ../../panda/models/channel/struct.Message.html
    /// [`MessageDelete`]: ../../panda/models/gateway/events/struct.MessageDelete.html
    pub async fn delete_many_messages(&self, channel_id: impl AsRef<str>, messages: &[&str]) -> Result<()> {
        // Parse URL
        let uri = format!("{}/channels/{}/messages/bulk-delete", DISCORD_URL, channel_id.as_ref(),);

        let body = serde_json::json!({ "messages": messages });
        let msg = serde_json::to_string(&body).unwrap();

        // Create RateLimit Key
        let rt_key = format!("channels:{}", channel_id.as_ref());

        let _res = self._post(uri, rt_key, msg).await?;

        Ok(())
    }

    /// Edit the channel permission overwrites for a user or role in a channel. Only usable
    /// for guild channels. Requires the **MANAGE_ROLES** permission.
    ///
    /// [`Message`]: ../../panda/models/channel/struct.Message.html
    /// [`MessageDelete`]: ../../panda/models/gateway/events/struct.MessageDelete.html
    pub async fn edit_channel_permissions(&self, _channel_id: impl AsRef<str>) -> Result<()> {
        unimplemented!();

        // // Parse URL
        // let uri = format!("{}/channels/{}/permissions/{}", DISCORD_URL, channel_id.as_ref(), "");

        // // Create RateLimit Key
        // let rt_key = format!("channels:{}", channel_id.as_ref());

        // let _res = self._get(uri, rt_key).await?;

        // Ok(())
    }

    // pub async fn get_channel_invites() {}
    // pub async fn create_channel_invite() {}

    // pub async fn delete_channel_permissions() {}

    /// Post a typing indicator for the specified channel.
    /// Fires a [`TypingStart`] Gateway event
    ///
    /// [`TypingStart`]: ../../panda/models/gateway/events/struct.TypingStart.html
    pub async fn trigger_typing(&self, channel_id: impl AsRef<str>) -> Result<()> {
        // Parse URL
        let uri = format!("{}/channels/{}/typing", DISCORD_URL, channel_id.as_ref());

        // Create RateLimit Key
        let rt_key = format!("channels:{}", channel_id.as_ref());

        let _res = self._post(uri, rt_key, ()).await?;

        // If an error wasn't returned, it's safe to unwrap
        Ok(())
    }

    /// Returns all pinned messages in the channel as a Vec of [`Message`] objects.
    ///
    /// [`Message`]: ../../panda/models/channel/struct.Message.html
    pub async fn get_pinned_messages(&self, channel_id: impl AsRef<str>) -> Result<Vec<Message>> {
        // Parse URL
        let uri = format!("{}/channels/{}/pins", DISCORD_URL, channel_id.as_ref());

        // Create RateLimit Key
        let rt_key = format!("channels:{}", channel_id.as_ref());

        let mut res = self._get(uri, rt_key).await?;

        // If an error wasn't returned, it's safe to unwrap
        Ok(res.json().unwrap())
    }

    /// Pin a message in a channel. Requires the **MANAGE_MESSAGES** permission
    pub async fn pin_message(&self, channel_id: impl AsRef<str>, message_id: impl AsRef<str>) -> Result<()> {
        // Parse URL
        let uri = format!(
            "{}/channels/{}/pins/{}",
            DISCORD_URL,
            channel_id.as_ref(),
            message_id.as_ref()
        );

        // Create RateLimit Key
        let rt_key = format!("channels:{}", channel_id.as_ref());

        let _res = self._put(uri, rt_key).await?;

        // If an error wasn't returned, it's safe to unwrap
        Ok(())
    }

    /// Pin a message in a channel. Requires the **MANAGE_MESSAGES** permission.
    pub async fn unpin_message(&self, channel_id: impl AsRef<str>, message_id: impl AsRef<str>) -> Result<()> {
        // Parse URL
        let uri = format!(
            "{}/channels/{}/pins/{}",
            DISCORD_URL,
            channel_id.as_ref(),
            message_id.as_ref()
        );

        // Create RateLimit Key
        let rt_key = format!("channels:{}", channel_id.as_ref());

        let _res = self._delete(uri, rt_key).await?;

        // If an error wasn't returned, it's safe to unwrap
        Ok(())
    }

    // PUT/channels/{channel.id}/recipients/{user.id}

    // DELETE/channels/{channel.id}/recipients/{user.id}
}

/// Used to encode emoji as a valid char in URL
fn encode(data: &str) -> String {
    let mut escaped = String::new();
    for b in data.as_bytes().iter() {
        match *b as char {
            // Accepted characters
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => escaped.push(*b as char),

            // Everything else is percent-encoded
            b => escaped.push_str(format!("%{:02X}", b as u32).as_str()),
        };
    }
    return escaped;
}
