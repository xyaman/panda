use crate::models::channel::Overwrite;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ChannelEdit {
    name: Option<String>,
    position: Option<u64>,
    topic: Option<String>,
    nsfw: Option<bool>,
    rate_limit_per_user: Option<u64>,
    bitrate: Option<u64>,
    user_limit: Option<u64>,
    permission_overwrites: Option<Vec<Overwrite>>,
    parent_id: Option<u64>,
}

impl ChannelEdit {
    pub fn new() -> Self {
        ChannelEdit::default()
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());

        self
    }

    pub fn position(mut self, position: u64) -> Self {
        self.position = Some(position);

        self
    }

    pub fn topic(mut self, topic: impl Into<String>) -> Self {
        self.topic = Some(topic.into());

        self
    }

    pub fn is_nsfw(mut self, nsfw: bool) -> Self {
        self.nsfw = Some(nsfw);

        self
    }

    pub fn rate_limit_per_user(mut self, rate_limit_per_user: u64) -> Self {
        self.rate_limit_per_user = Some(rate_limit_per_user);

        self
    }

    pub fn bitrate(mut self, bitrate: u64) -> Self {
        self.bitrate = Some(bitrate);

        self
    }

    pub fn user_limit(mut self, user_limit: u64) -> Self {
        self.user_limit = Some(user_limit);

        self
    }

    pub fn permission_overwrite(mut self, permission_overwrite: Overwrite) -> Self {
        if let Some(vec) = &mut self.permission_overwrites {
            vec.push(permission_overwrite);
        } else {
            self.permission_overwrites = Some(vec![permission_overwrite]);
        }

        self
    }

    pub fn parent_id(mut self, parent_id: u64) -> Self {
        self.parent_id = Some(parent_id);

        self
    }
}
