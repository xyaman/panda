//! # Panda error types
//! TODO: ADD DISCORD LINKS

use async_tungstenite::tungstenite::Error as TungsteniteError;
use std::{error::Error, fmt, result::Result as StdResult};

/// This library use a shared result type, because all functions returns the same error type
pub type Result<T> = StdResult<T, PandaError>;

/// The error enum for this "Discord" library
#[derive(Debug)]
pub enum PandaError {
    /// Returned when there was an authentication error and the gateway is closed
    AuthenticationFailed,

    /// Returned when "discord" fails to connect to the gateway, it can only be returned at
    /// the first connection, all reconnections are handled by "discord"
    CantConnectToGateway,

    /// Returned when the gateway connection is unexpected closed
    ConnectionClosed,

    /// Returned when "discord" receives a unknown message format
    UnknownPayloadReceived,

    /// Returned when panda sent an invalid Opcode
    UnknownOpcodeSent,

    /// Returned when panda sent an invalid payload
    InvalidDecodeSent,

    /// Returned when "discord" recevies a invalid message format
    InvalidPayloadFormat,

    /// Returned when "discord" receives a unexpected message format like IDENTIFY
    UnexpectedPayloadReceived,

    /// Returned when "discord" receives a not zlib compressed payload
    WrongCompression,

    // Http Errors
    /// Returned when "discord" http client didn't receive a response of
    /// Discord API
    HttpNoResponse,

    /// Returned when http request format was invalid
    HttpImproperlyFormatted,

    /// Returned when http request has an invalid token
    HttpUnauthorized,

    /// Returned when the client doesn't have enough permissions
    HttpForbidden,

    /// Returned when the http request URL had invalid parameters,
    /// such as wrong {channel_id}
    HttpInvalidParameters,

    /// Returned when the gateway couldn't close the connection succesfully
    UnsuccessfulConnectionClose,

    /// Returned when you send an invalid shard
    InvalidShard,

    /// Returned when you handled too many guilds, and shard is necessary
    ShardingRequired,

    // Invalid API version (gateway)
    InvalidApiGatewayVersion,

    /// serde_json
    SerdeError(serde_json::Error),

    /// tungstenite
    TungsteniteError(TungsteniteError),
}

impl fmt::Display for PandaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AuthenticationFailed => write!(f, "Authentication failed"),
            Self::CantConnectToGateway => write!(f, "'Discord' couldn't connect to gateway"),
            Self::ConnectionClosed => write!(f, "Connection closed unexpectedly"),
            Self::UnknownPayloadReceived => write!(f, "Unknown payload format received"),
            Self::InvalidPayloadFormat => write!(f, "Invalid payload format received",),
            Self::UnexpectedPayloadReceived => write!(f, "Unexpected payload received"),
            Self::WrongCompression => write!(f, "Wrong zlib compression"),
            Self::HttpNoResponse => write!(f, "Discord HTTP API didn't response"),
            Self::HttpImproperlyFormatted => write!(f, "Invalid format of request body"),
            Self::HttpUnauthorized => write!(f, "The client token is invalid"),
            Self::HttpForbidden => write!(f, "The client did not have permission to the resource"),
            Self::HttpInvalidParameters => write!(f, "The request had invalid parameters"),
            Self::UnsuccessfulConnectionClose => write!(f, "The gateway couldn't close succesfully the connection"),
            Self::InvalidShard => write!(f, "You sent an invalid shard"),
            Self::ShardingRequired => write!(f, "The session would have handled too many guilds - you are required to shard your connection in order to connect."),
            Self::InvalidApiGatewayVersion => write!(f, "panda needs to update the gateway version"),
            Self::SerdeError(e) => write!(f, "Serde Error: {}", e),
            Self::TungsteniteError(e) => write!(f, "Tungstenite Error: {}", e),
            Self::UnknownOpcodeSent => write!(f, "panda sent an invalid Opcode, please report the bug"),
            Self::InvalidDecodeSent => write!(f, "panda sent an invalid payload, please report the bug"),
        }
    }
}

impl Error for PandaError {}

// Error parsing
impl From<serde_json::Error> for PandaError {
    fn from(error: serde_json::Error) -> Self {
        // InvalidPayloadFormat

        if error.is_data() {
            return PandaError::InvalidPayloadFormat;
        }

        PandaError::SerdeError(error)
    }
}

impl From<TungsteniteError> for PandaError {
    fn from(error: TungsteniteError) -> Self {
        // TODO: Improve this (IO) errors

        match error {
            TungsteniteError::ConnectionClosed => PandaError::ConnectionClosed,
            _ => PandaError::TungsteniteError(error),
        }
    }
}
