use isahc::{http::Method, Body};

// const DISCORD_URL: &'static str = "https://discord.com/api/v6";

macro_rules! bucket_key {
    (channel: $id: expr) => {
        format!("channels:{}", $id.as_ref());
    };
    (guild: $id: expr) => {
        format!("guild:{}", $id.as_ref());
    };
    (emoji: $id: expr) => {
        format!("emoji:{}", $id.as_ref());
    };
}

macro_rules! api_request {
    ($url: expr, $($rest: expr),*) => {
        format!(concat!("https://discord.com/api/v6", $url), $($rest),*)
    };
}
pub(crate) struct Route<B> {
    pub(crate) method: Method,
    pub(crate) uri: String,
    pub(crate) bucket_key: String,
    pub(crate) body: B,
}

// Routes without body
impl Route<()> {
    // GET/channels/{channel.id}
    pub(crate) fn get_channel(channel_id: impl AsRef<str>) -> Self {
        let method = Method::GET;
        let uri = api_request!("/channels/{}", channel_id.as_ref());
        let bucket_key = bucket_key!(channel: channel_id);

        Route {
            method,
            uri,
            bucket_key,
            body: (),
        }
    }

    // DELETE/channels/{channel.id}
    pub(crate) fn delete_channel(channel_id: impl AsRef<str>) -> Self {
        let method = Method::DELETE;
        let uri = api_request!("/channels/{}", channel_id.as_ref());
        let bucket_key = bucket_key!(channel: channel_id);

        Route {
            method,
            uri,
            bucket_key,
            body: (),
        }
    }

    // GET/channels/{channel.id}/messages
    pub(crate) fn get_channel_messages(
        kind: &str,
        channel_id: impl AsRef<str>,
        message_id: impl AsRef<str>,
        limit: u8,
    ) -> Self {
        let method = Method::GET;
        let uri = api_request!(
            "/channels/{}/messages?{}={}&limit={}",
            channel_id.as_ref(),
            kind,
            message_id.as_ref(),
            limit
        );
        let bucket_key = bucket_key!(channel: channel_id);

        Route {
            method,
            uri,
            bucket_key,
            body: (),
        }
    }

    // GET/channels/{channel.id}/messages/{message.id}
    pub(crate) fn get_channel_message(channel_id: impl AsRef<str>, msg_id: impl AsRef<str>) -> Self {
        let method = Method::GET;
        let uri = api_request!("/channels/{}/messages/{}", channel_id.as_ref(), msg_id.as_ref());
        let bucket_key = bucket_key!(channel: channel_id);

        Route {
            method,
            uri,
            bucket_key,
            body: (),
        }
    }

    // PUT/channels/{channel.id}/messages/{message.id}/reactions/{emoji}
    pub(crate) fn create_reaction(
        channel_id: impl AsRef<str>,
        msg_id: impl AsRef<str>,
        emoji: impl AsRef<str>,
    ) -> Self {
        let method = Method::PUT;
        let emoji = encode(emoji);
        let uri = format!(
            "/channels/{}/messages/{}/reactions/{}/@me",
            channel_id.as_ref(),
            msg_id.as_ref(),
            emoji
        );
        let bucket_key = bucket_key!(emoji: channel_id);

        Route {
            method,
            uri,
            bucket_key,
            body: (),
        }
    }

    // DELETE/channels/{channel.id}/messages/{message.id}/reactions/{emoji}/@me
    pub(crate) fn delete_own_reaction(
        channel_id: impl AsRef<str>,
        msg_id: impl AsRef<str>,
        emoji: impl AsRef<str>,
    ) -> Self {
        let method = Method::DELETE;
        let emoji = encode(emoji);
        let uri = api_request!(
            "/channels/{}/messages/{}/reactions/{}/@me",
            channel_id.as_ref(),
            msg_id.as_ref(),
            emoji
        );
        let bucket_key = bucket_key!(emoji: channel_id);

        Route {
            method,
            uri,
            bucket_key,
            body: (),
        }
    }

    // DELETE/channels/{channel.id}/messages/{message.id}/reactions/{emoji}/{user.id}
    pub(crate) fn delete_user_reaction(
        channel_id: impl AsRef<str>,
        msg_id: impl AsRef<str>,
        emoji: impl AsRef<str>,
        user_id: impl AsRef<str>,
    ) -> Self {
        let method = Method::DELETE;
        let emoji = encode(emoji);
        let uri = api_request!(
            "/channels/{}/messages/{}/reactions/{}/{}",
            channel_id.as_ref(),
            msg_id.as_ref(),
            emoji,
            user_id.as_ref()
        );

        let bucket_key = bucket_key!(emoji: channel_id);

        Route {
            method,
            uri,
            bucket_key,
            body: (),
        }
    }

    // GET/channels/{channel.id}/messages/{message.id}/reactions/{emoji}
    pub(crate) fn get_reactions(channel_id: impl AsRef<str>, msg_id: impl AsRef<str>, emoji: impl AsRef<str>) -> Self {
        let method = Method::GET;
        let emoji = encode(emoji);
        let uri = api_request!(
            "/channels/{}/messages/{}/reactions/{}",
            channel_id.as_ref(),
            msg_id.as_ref(),
            emoji
        );
        let bucket_key = bucket_key!(emoji: channel_id);

        Route {
            method,
            uri,
            bucket_key,
            body: (),
        }
    }

    // DELETE/channels/{channel.id}/messages/{message.id}/reactions
    pub(crate) fn delete_all_reactions(channel_id: impl AsRef<str>, msg_id: impl AsRef<str>) -> Self {
        let method = Method::DELETE;
        let uri = api_request!(
            "/channels/{}/messages/{}/reactions",
            channel_id.as_ref(),
            msg_id.as_ref()
        );

        let bucket_key = bucket_key!(emoji: channel_id);

        Route {
            method,
            uri,
            bucket_key,
            body: (),
        }
    }

    // DELETE/channels/{channel.id}/messages/{message.id}/reactions/{emoji}
    pub(crate) fn delete_all_reactions_for_emoji(
        channel_id: impl AsRef<str>,
        msg_id: impl AsRef<str>,
        emoji: impl AsRef<str>,
    ) -> Self {
        let method = Method::DELETE;
        let emoji = encode(emoji);
        let uri = api_request!(
            "/channels/{}/messages/{}/reactions/{}",
            channel_id.as_ref(),
            msg_id.as_ref(),
            emoji
        );

        let bucket_key = bucket_key!(emoji: channel_id);

        Route {
            method,
            uri,
            bucket_key,
            body: (),
        }
    }

    // DELETE/channels/{channel.id}/messages/{message.id}
    pub(crate) fn delete_message(channel_id: impl AsRef<str>, msg_id: impl AsRef<str>) -> Self {
        let method = Method::DELETE;
        let uri = format!("/channels/{}/messages/{}", channel_id.as_ref(), msg_id.as_ref());

        let bucket_key = bucket_key!(channel: channel_id);

        Route {
            method,
            uri,
            bucket_key,
            body: (),
        }
    }
}

// Routes with body
impl<B: Into<Body>> Route<B> {
    // pub(crate) fn as_request(self, token: &str) -> (String, Request<Body>) {
    //     // let request = match self.method {
    //     //     Method::GET | Method::PUT | Method::DELETE => Request::builder()
    //     //         .method(&self.method)
    //     //         .uri(&self.uri)
    //     //         .header("Authorization", token)
    //     //         .body(())
    //     //         .unwrap(),
    //     //     Method::POST | Method::PATCH => Request::builder()
    //     //         .method(&self.method)
    //     //         .uri(&self.uri)
    //     //         .header("Authorization", token)
    //     //         .header("Content-Type", "application/json")
    //     //         .body(self.body.into())
    //     //         .unwrap(),
    //     //     _ => unimplemented!(),
    //     // };

    //     let request = match self.body {
    //         () => Request::builder()
    //             .method(&self.method)
    //             .uri(&self.uri)
    //             .header("Authorization", token)
    //             .body(())
    //             .unwrap(),
    //         _ => Request::builder()
    //             .method(&self.method)
    //             .uri(&self.uri)
    //             .header("Authorization", token)
    //             .header("Content-Type", "application/json")
    //             .body(self.body)
    //             .unwrap(),
    //     };

    //     println!("{:?}", request);

    //     (self.bucket_key, request)
    // }

    // PATCH/channels/{channel.id}
    pub(crate) fn edit_channel(channel_id: impl AsRef<str>, body: B) -> Self {
        let method = Method::PATCH;
        let uri = api_request!("/channels/{}", channel_id.as_ref());
        let bucket_key = bucket_key!(channel: channel_id);

        Self {
            method,
            uri,
            bucket_key,
            body,
        }
    }

    // POST/channels/{channel.id}/messages
    pub(crate) fn create_message(channel_id: impl AsRef<str>, body: B) -> Self {
        let method = Method::POST;
        let uri = api_request!("/channels/{}/messages", channel_id.as_ref());
        let bucket_key = bucket_key!(channel: channel_id);

        Self {
            method,
            uri,
            bucket_key,
            body,
        }
    }

    // PATCH/channels/{channel.id}/messages/{message.id}
    pub(crate) fn edit_message(channel_id: impl AsRef<str>, msg_id: impl AsRef<str>, body: B) -> Self {
        let method = Method::PATCH;
        let uri = format!("/channels/{}/messages/{}", channel_id.as_ref(), msg_id.as_ref());

        let bucket_key = bucket_key!(channel: channel_id);

        Self {
            method,
            uri,
            bucket_key,
            body,
        }
    }

    // POST/channels/{channel.id}/messages/bulk-delete
    pub(crate) fn bulk_delete_messages(channel_id: impl AsRef<str>, body: B) -> Self {
        let method = Method::POST;
        let uri = api_request!("/channels/{}/messages/bulk-delete", channel_id.as_ref());

        let bucket_key = bucket_key!(channel: channel_id);

        Route {
            method,
            uri,
            bucket_key,
            body,
        }
    }

    // PUT/channels/{channel.id}/permissions/{overwrite.id}
    // TODO: Check {overwrite.id}
    // pub(crate) fn edit_channel_permissions(
    //     channel_id: impl AsRef<str>,
    //     overwrite_id: impl AsRef<str>,
    //     body: B,
    // ) -> Self {
    //     let method = Method::PUT;
    //     let uri = api_request!(
    //         "/channels/{}/permissions/{}",
    //         channel_id.as_ref(),
    //         overwrite_id.as_ref()
    //     );

    //     let bucket_key = bucket_key!(channel: channel_id);

    //     Route {
    //         method,
    //         uri,
    //         bucket_key,
    //         body,
    //     }
    // }
}

/// Used to encode emoji as a valid char in URL
fn encode(data: impl AsRef<str>) -> String {
    let mut escaped = String::new();
    for b in data.as_ref().as_bytes().iter() {
        match *b as char {
            // Accepted characters
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => escaped.push(*b as char),

            // Everything else is percent-encoded
            b => escaped.push_str(format!("%{:02X}", b as u32).as_str()),
        };
    }
    return escaped;
}
