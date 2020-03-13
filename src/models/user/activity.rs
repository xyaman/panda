use crate::models::emoji::Emoji;
use serde::{Deserialize, Serialize};
use serde_repr::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Activity {
    name: String,
    #[serde(rename = "type")]
    kind: ActivityKind,
    url: Option<String>,
    // timestamps: Timestamp,
    application_id: Option<String>,
    details: Option<String>,
    state: Option<String>,
    emoji: Option<Emoji>,
    party: Option<String>,
    // assets: Assets,
    // secrets: Secrets,
    instance: Option<bool>,
    flags: Option<u64>,
}

#[derive(Debug, Deserialize_repr, Serialize_repr, PartialEq)]
#[repr(u8)]
pub enum ActivityKind {
    Game = 0,
    Streaming = 1,
    Listening = 2,
    Custom = 4,
}

impl Activity {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn kind(&self) -> &ActivityKind {
        &self.kind
    }

    pub fn url(&self) -> Option<&str> {
        self.url.as_ref().map(|s| s.as_str())
    }

    // pub fn timestamps Timestamp,

    pub fn application_id(&self) -> Option<&str> {
        self.application_id.as_ref().map(|s| s.as_str())
    }

    pub fn details(&self) -> Option<&str> {
        self.details.as_ref().map(|s| s.as_str())
    }

    pub fn state(&self) -> Option<&str> {
        self.state.as_ref().map(|s| s.as_str())
    }

    pub fn emoji(&self) -> Option<&Emoji> {
        self.emoji.as_ref()
    }

    pub fn party(&self) -> Option<&str> {
        self.party.as_ref().map(|s| s.as_str())
    }

    // assets: Assets,
    // secrets: Secrets,
    //
    pub fn instance(&self) -> Option<bool> {
        self.instance
    }

    pub fn flags(&self) -> Option<u64> {
        self.flags
    }
}
