use crate::models::user::User;
use crate::models::channel::Embed;
use serde::{Deserialize, Serialize};
use serde_repr::*;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Webhook {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: WebhookKind,
    pub guild_id: Option<String>,
    pub channel_id: String,
    /// The user this webhook was created by. `None` when getting a webhook by its token.
    pub user: Option<User>,
    pub name: Option<String>,
    pub avatar: Option<String>,
    /// The secure token of the webhook. Only `Some` for incoming webhooks ([`WebhookKind::Incoming`])
    ///
    /// [`WebhookKind::Incoming`]: enum.WebhookKind.html#variant.Incoming
    pub token: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ExecuteWebhook {
    #[serde(flatten)]
    pub payload: WebhookPayload,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<url::Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts: Option<bool>,
}

#[derive(Debug, Serialize)]
pub enum WebhookPayload {
    #[serde(rename = "content")]
    MessageContent(String),
    #[serde(rename = "embeds")]
    Embeds(Vec<Embed>),
}

#[derive(Clone, Debug, Deserialize_repr, Serialize_repr, PartialEq)]
#[repr(u8)]
pub enum WebhookKind {
    Incoming = 1,
    ChannelFollower = 2,
}
