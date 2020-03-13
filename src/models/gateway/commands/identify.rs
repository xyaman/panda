use crate::models::gateway::payload::Opcode;
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq)]
pub(crate) struct Identify {
    pub(crate) op: Opcode,
    pub(crate) d: IdentifyContent,
    pub(crate) s: Option<u64>,
}

#[derive(Debug, Serialize, PartialEq)]
pub(crate) struct IdentifyContent {
    pub(crate) token: String,
    pub(crate) properties: IdentifyProperties,
    pub(crate) compress: bool,
    pub(crate) large_threshold: Option<u8>,
    pub(crate) shard: Option<[u64; 2]>,
    pub(crate) presence: Option<()>,
    pub(crate) guild_subscriptions: Option<bool>,
}

#[derive(Debug, Serialize, PartialEq)]
pub(crate) struct IdentifyProperties {
    #[serde(rename = "$os")]
    pub(crate) os: &'static str,

    #[serde(rename = "$browser")]
    pub(crate) browser: String,

    #[serde(rename = "$device")]
    pub(crate) device: String,
}
