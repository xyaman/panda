use crate::{
    error::{DiscordError, Result},
    models::gateway::{commands::Command, events::Event, payload::Payload},
};

// std
use std::{
    convert::TryFrom,
    result::Result as StdResult,
    sync::atomic::{AtomicU64, Ordering},
};

// async std
use async_std::{net::TcpStream, sync::Arc};

// futures
use futures::{
    channel::mpsc::{UnboundedReceiver, UnboundedSender},
    select,
    sink::SinkExt,
    stream::StreamExt,
};

// tungstenite
use async_tls::client::TlsStream;
use async_tungstenite::{
    stream::Stream,
    tungstenite::{error::Error as TungsteniteError, Message as TungsteniteMessage},
    WebSocketStream,
};

use log::error;

// Websocket types
type WebSocketSender = futures::stream::SplitSink<
    WebSocketStream<Stream<TcpStream, TlsStream<TcpStream>>>,
    TungsteniteMessage,
>;
type WebSocket = WebSocketStream<Stream<TcpStream, TlsStream<TcpStream>>>;
type TungsteniteOptionResult = Option<StdResult<TungsteniteMessage, TungsteniteError>>;

/// This function manages all library/gateway commands and events
#[allow(unused_must_use)]
pub(crate) async fn gateway_process(
    ws: WebSocket,
    mut to_client: UnboundedSender<Event>,
    mut from_client: UnboundedReceiver<Command>,
    last_sequence: Arc<AtomicU64>,
) {
    // Split the websocket
    let (mut ws_sender, ws_receiver) = ws.split();
    let mut from_gateway = ws_receiver.fuse();
    loop {
        select! {
            // gateway -> client
            tm = from_gateway.next()  => {
                let last_sequence = Arc::clone(&last_sequence);

                if let Err(e) = from_gateway_process(tm, &mut to_client, last_sequence).await {
                    error!("Error when receiving an event: {}", e);
                    // Check if there are unrecoverable errors
                    match e {
                        DiscordError::AuthenticationFailed | DiscordError::ConnectionError => {
                            to_client.send(Event::Close(e)).await.expect("EVENT CLOSE");
                            break;
                        },
                        _ => {},
                    };
                }
            },
            // client -> gateway
            cmd = from_client.next() => {
                let last_sequence = Arc::clone(&last_sequence);
                // An error means that the connection was closed
                if let Err(e) = to_gateway_process(cmd, &mut ws_sender, last_sequence).await {
                    error!("Error when sending command to gateway: {}", e);
                    // Unhandled result, TODO: Handle result
                    to_client.send(Event::Close(DiscordError::ConnectionError)).await;
                    break;
                }
            }

            // receive actions from client and send to gatewat
        }
    }
}

/// This function manages all events received
async fn from_gateway_process(
    tm: TungsteniteOptionResult,
    to_client: &mut UnboundedSender<Event>,
    last_sequence: Arc<AtomicU64>,
) -> Result<()> {
    // This error means connection error
    let tm = tm.ok_or_else(|| DiscordError::ConnectionError)?;
    let msg = tm.map_err(|_| DiscordError::ConnectionError)?;

    // Get Payload from TungsteniteMessage
    let p = Payload::try_from(msg)?;

    // Get Payload sequence
    if let Some(seq) = p.s {
        last_sequence.store(seq, Ordering::Relaxed);
    }

    // Transform Event from Payload
    let event = Event::try_from(p)?;

    // Send Event to client
    to_client
        .send(event)
        .await
        .map_err(|_| DiscordError::ConnectionError)?;

    Ok(())
}

/// This function manages all commands sended
async fn to_gateway_process(
    command: Option<Command>,
    to_gateway: &mut WebSocketSender,
    last_sequence: Arc<AtomicU64>,
) -> Result<()> {
    // Get the command
    let command = command.ok_or_else(|| DiscordError::ConnectionError)?;

    // Check if it's a Close command
    if command == Command::Close {
        return to_gateway
            .send(TungsteniteMessage::Close(None))
            .await
            .map_err(|_| DiscordError::ConnectionError);
    }

    // Get the last sequence
    let seq = match last_sequence.load(Ordering::Relaxed) {
        0 => None,
        v => Some(v),
    };

    // Transform command to TungsteniteMessage
    let tm = command.to_tungstenite_message(seq);

    // Send command to gateway
    to_gateway
        .send(tm)
        .await
        .map_err(|_| DiscordError::ConnectionError)?;

    Ok(())
}
