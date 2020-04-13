use crate::models::gateway::commands::Command;

use futures::{channel::mpsc::UnboundedSender, sink::SinkExt};
use std::time::Duration;

/// This function needs to be spawned to work in the background,
/// it will send a heartbeat COMMAND to gateway every heartbeat_interval.
/// When the channel is closed, it will be terminated
pub(crate) async fn heartbeater(heartbeat_interval: u64, mut to_gateway: UnboundedSender<Command>) {
    loop {
        tokio::time::delay_for(Duration::from_millis(heartbeat_interval)).await;

        // Always check first if the channel it's open
        if to_gateway.is_closed() {
            log::info!("Old heartbeater exited");
            break;
        }
        let heartbeat = Command::new_heartbeat();
        if let Err(e) = to_gateway.send(heartbeat).await {
            log::error!("Error when sending Heartbeat: {}", e);
        };
    }
}
