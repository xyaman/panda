use crate::models::emoji::Emoji;
use serde::{Deserialize, Serialize};
use serde_repr::*;

use std::default::Default;

#[derive(Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Activity {
    pub name: String,
    #[serde(rename = "type")]
    pub kind: ActivityKind,
    pub url: Option<String>,
    // timestamps: Timestamp,
    pub application_id: Option<String>,
    pub details: Option<String>,
    pub state: Option<String>,
    pub emoji: Option<Emoji>,
    pub party: Option<String>,
    // assets: Assets,
    // secrets: Secrets,
    pub instance: Option<bool>,
    pub flags: Option<u64>,
}

#[derive(Debug, Deserialize_repr, Serialize_repr, PartialEq)]
#[repr(u8)]
pub enum ActivityKind {
    Game = 0,
    Streaming = 1,
    Listening = 2,
    Custom = 4,
}

impl Default for ActivityKind {
    fn default() -> Self {
        Self::Game
    }
}

impl Activity {
    pub fn new(kind: ActivityKind, name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            kind,
            ..Default::default()
        }
    }
}
