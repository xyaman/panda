// crate imports
use crate::error::PandaError;

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
    StatusUpdate = 3,
    Resume = 6,
    Reconnect = 7,
    RequestGuildMember = 8,
    InvalidSessionData = 9,
    Hello = 10,
    HeartbeatACK = 11,
}

impl TryFrom<TungsteniteMessage> for Payload {
    type Error = PandaError;

    fn try_from(value: TungsteniteMessage) -> Result<Payload, Self::Error> {
        let payload = match value {
            // Normal text
            TungsteniteMessage::Text(v) => serde_json::from_str(&v).map_err(|_| PandaError::UnknownPayloadReceived)?,

            // Compressed Text
            TungsteniteMessage::Binary(v) => {
                let mut decoder = ZlibDecoder::new(v.as_slice());
                let mut value = String::new();

                decoder
                    .read_to_string(&mut value)
                    .map_err(|_| PandaError::WrongCompression)?;

                serde_json::from_str(&value).map_err(|_| PandaError::UnknownPayloadReceived)?
            }

            // Close frame, returned when Discord gateway close/refuse the connection
            TungsteniteMessage::Close(reason) => {
                let reason = reason.ok_or_else(|| PandaError::ConnectionClosed)?;

                // Get the code as a u16
                // https://discordapp.com/developers/docs/topics/opcodes-and-status-codes#gateway-gateway-close-event-codes
                let code: u16 = reason.code.into();

                match code {
                    4000 => return Err(PandaError::ConnectionClosed),
                    4001 => return Err(PandaError::UnknownOpcodeSent),
                    4002 => return Err(PandaError::InvalidDecodeSent),
                    // 4003 => this shouldn't happen
                    4004 => return Err(PandaError::AuthenticationFailed),
                    // 4005 => this shouldn't happen
                    4007 => {
                        log::error!("Panda error: Invalid seq sended");
                        return Err(PandaError::ConnectionClosed);
                    }
                    4008 => return Err(PandaError::ConnectionClosed), // TODO: Improve this
                    4009 => return Err(PandaError::ConnectionClosed), // TODO: Improve this
                    4010 => return Err(PandaError::InvalidShard),
                    4011 => return Err(PandaError::ShardingRequired),
                    4012 => return Err(PandaError::InvalidApiGatewayVersion),
                    _ => panic!("UNDEFINED Close error received"),
                }
            }
            _ => todo!(),
        };

        Ok(payload)
    }
}
