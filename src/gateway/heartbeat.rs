use crate::models::gateway::commands::Command;

use async_std::task;
use futures::{channel::mpsc::UnboundedSender, sink::SinkExt};
use log::error;
use std::time::Duration;

/// This function needs to be spawned to work in the background,
/// it will send a heartbeat COMMAND to gateway every heartbeat_interval.
/// When the channel is closed, it will be terminated
pub(crate) async fn heartbeater(heartbeat_interval: u64, mut to_gateway: UnboundedSender<Command>) {
    loop {
        task::sleep(Duration::from_millis(heartbeat_interval)).await;

        // Always check first if the channel it's open
        if to_gateway.is_closed() {
            break;
        }
        let heartbeat = Command::new_heartbeat();
        if let Err(e) = to_gateway.send(heartbeat).await {
            error!("Error when sending Heartbeat: {}", e);
        };
    }
}
