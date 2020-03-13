//! # "Discord" error types
//! TODO: ADD DISCORD LINKS

use std::{error::Error, fmt, result::Result as StdResult};

/// This library use a shared result type, because all functions returns the same error type
pub type Result<T> = StdResult<T, DiscordError>;

// UNRECOVERABLE ERRORS

/// The error enum for this "Discord" library
#[derive(Debug)]
pub enum DiscordError {
    /// Returned when there was an authentication error and the gateway is closed
    AuthenticationFailed,

    /// Returned when "discord" fails to connect to the gateway, it can only be returned at
    /// the first connection, all reconnections are handled by "discord"
    CantConnectToGateway,

    /// Returned when the gateway connection is unexpected closed
    ConnectionError,

    /// Returned when "discord" receives a unknown message format
    UnknownPayloadReceived,

    /// Returned when "discord" recevies a invalid message format
    InvalidPayloadFormat(&'static str),

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
}

impl fmt::Display for DiscordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::AuthenticationFailed => write!(f, "Authentication failed"),
            Self::CantConnectToGateway => write!(f, "'Discord' couldn't connect to gateway"),
            Self::ConnectionError => write!(f, "Connection closed unexpectedly"),
            Self::UnknownPayloadReceived => write!(f, "Unknown payload format received"),
            Self::InvalidPayloadFormat(p) => write!(f, "Invalid payload format received({})", p),
            Self::UnexpectedPayloadReceived => write!(f, "Unexpected payload received"),
            Self::WrongCompression => write!(f, "Wrong zlib compression"),
            Self::HttpNoResponse => write!(f, "Discord HTTP API didn't response"),
            Self::HttpImproperlyFormatted => write!(f, "Invalid format of request body"),
            Self::HttpUnauthorized => write!(f, "The client token is invalid"),
            Self::HttpForbidden => write!(f, "The client did not have permission to the resource"),
            Self::HttpInvalidParameters => write!(f, "The request had invalid parameters"),
            Self::UnsuccessfulConnectionClose => {
                write!(f, "The gateway couldn't close succesfully the connection")
            }
        }
    }
}

impl Error for DiscordError {}
