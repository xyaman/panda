// modules
pub(crate) mod heartbeat;
mod process;
use process::gateway_process;

// crate imports
use crate::{
    error::{DiscordError, Result},
    models::gateway::{commands::Command, events::Event},
};

// std
use std::{
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};

// async
use async_std::{sync::Arc, task};
use async_tungstenite::async_std::connect_async;
use futures::{
    channel::mpsc::{self, UnboundedReceiver, UnboundedSender},
    stream::StreamExt,
};

pub(crate) struct GatewayConnection {
    last_sequence: Arc<AtomicU64>,
    pub(crate) heartbeat_interval: u64,
    pub(crate) from_gateway: UnboundedReceiver<Event>,
    pub(crate) to_gateway: UnboundedSender<Command>,
}

impl GatewayConnection {
    pub(crate) async fn new() -> Result<GatewayConnection> {
        // Parse discord url
        let url = url::Url::parse("wss://gateway.discord.gg/?v=6&encoding=json").unwrap();

        // Connect to the discord gateway through a websocket
        let (ws, _) = connect_async(url)
            .await
            .map_err(|_| DiscordError::CantConnectToGateway)?;

        // Spawn gateway process manager
        let (to_client, mut from_gateway) = mpsc::unbounded();
        let (to_gateway, from_client) = mpsc::unbounded();

        let last_sequence = Arc::new(AtomicU64::default());
        let last_sequence_clone = Arc::clone(&last_sequence);

        task::spawn(async move {
            gateway_process(ws, to_client, from_client, last_sequence_clone).await;
        });

        // Receive Hello event from the gatewat
        let event = from_gateway
            .next()
            .await
            .ok_or_else(|| DiscordError::ConnectionClosed)?;

        let heartbeat_interval = match event {
            Event::Hello(v) => v,
            _ => return Err(DiscordError::UnknownPayloadReceived.into()),
        };

        Ok(GatewayConnection {
            last_sequence,
            heartbeat_interval,
            from_gateway,
            to_gateway,
        })
    }

    pub(crate) fn close_channels(&mut self) -> Result<()> {
        self.from_gateway.close();
        self.to_gateway.close_channel();
        Ok(())
    }

    pub(crate) async fn reconnect(&mut self) -> Option<u64> {
        // Transform last sequence to option
        let last_sequence = match self.last_sequence.load(Ordering::Relaxed) {
            0 => None,
            seq => Some(seq),
        };

        loop {
            log::error!("Disconnected from the gateway, starting reconnect...");
            match GatewayConnection::new().await {
                Ok(g) => {
                    std::mem::replace(self, g);
                    log::info!("Connected succesfully");
                    break;
                }
                Err(_e) => {
                    log::error!("Couldn't reconnect, trying in 3 seconds...");
                    task::sleep(Duration::from_secs(3)).await;
                }
            }
        }

        last_sequence
    }
}
