// crate imports
use crate::error::DiscordError;

use std::{convert::TryFrom, io::Read};

use async_tungstenite::tungstenite::Message as TungsteniteMessage;
use flate2::read::ZlibDecoder;

// Serde
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::*;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub(crate) struct Payload {
    // Opcode
    pub op: Opcode,

    // Event Data
    pub d: Option<Value>,

    // Sequence number
    pub s: Option<u64>,

    // Event name, only for opcode 0
    pub t: Option<String>,
}

#[derive(Debug, Deserialize_repr, Serialize_repr, PartialEq)]
#[repr(u8)]
pub(crate) enum Opcode {
    Dispatch = 0,
    Heartbeat = 1,
    Identify = 2,
    Resume = 6,
    Reconnect = 7,
    RequestGuildMember = 8,
    InvalidSession = 9,
    Hello = 10,
    HeartbeatACK = 11,
}

impl TryFrom<TungsteniteMessage> for Payload {
    type Error = DiscordError;

    fn try_from(value: TungsteniteMessage) -> Result<Payload, Self::Error> {
        let payload = match value {
            TungsteniteMessage::Text(v) => {
                serde_json::from_str(&v).map_err(|_| DiscordError::UnknownPayloadReceived)?
            }
            TungsteniteMessage::Binary(v) => {
                let mut decoder = ZlibDecoder::new(v.as_slice());
                let mut value = String::new();

                decoder
                    .read_to_string(&mut value)
                    .map_err(|_| DiscordError::WrongCompression)?;

                serde_json::from_str(&value).map_err(|_| DiscordError::UnknownPayloadReceived)?
            }
            TungsteniteMessage::Close(v) => {
                println!("Close Frame {:?}", v);
                return Err(DiscordError::AuthenticationFailed);
            }
            _ => todo!(),
        };

        Ok(payload)
    }
}
