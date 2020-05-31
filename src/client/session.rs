//! SessionData

use crate::{
    error::{Result, PandaError},
    models::{ gateway::commands::Command, user::StatusUpdate },
    HttpClient};

use std::sync::atomic::{AtomicBool, Ordering};

use futures::{ channel::mpsc::UnboundedSender, lock::Mutex, sink::SinkExt };

/// The struct of the current session of the bot.
pub struct SessionData<S> {
    id: Mutex<String>,
    pub http: HttpClient,
    pub state: S,
    is_resumable: AtomicBool,
    to_gateway_ch: Mutex<UnboundedSender<Command>>
}

impl<S> SessionData<S> {
    pub(crate) fn new(token: String, state: S, to_gateway_ch: UnboundedSender<Command>) -> Self {
        SessionData {
            id: Mutex::new("".into()),
            http: HttpClient::new(token),
            state,
            is_resumable: AtomicBool::new(true),
            to_gateway_ch: Mutex::new(to_gateway_ch)
        }
    }

    /// Set the value to resumable field
    pub(crate) fn set_resumable(&self, b: bool) {
        self.is_resumable.store(b, Ordering::Relaxed);
    }

    /// Get the value of resumable field
    pub(crate) fn is_resumable(&self) -> bool {
        self.is_resumable.load(Ordering::Relaxed)
    }

    /// Set the value to id field
    pub(crate) async fn set_id(&self, id: String) {
        let mut session_id = self.id.lock().await;
        *session_id = id;
    }

    /// Get the value to id field
    pub(crate) async fn id(&self) -> String {
        let session_id = self.id.lock().await;
        session_id.clone()
    }

    //Send
    pub async fn update_status(&self, status_update: StatusUpdate) -> Result<()> {

        let cmd = Command::new_status_update(status_update);

        // TODO: Into<Error>
        self.to_gateway_ch.lock().await.send(cmd).await.map_err(|_| PandaError::ConnectionClosed)?;

        Ok(())
    }
}
