//! SessionData

use crate::HttpClient;

use std::sync::atomic::{AtomicBool, Ordering};

use futures::lock::Mutex;

/// The struct of the current SessionData of the bot.
pub struct SessionData {
    id: Mutex<String>,
    pub http: HttpClient,

    #[allow(dead_code)]
    pub(crate) state: (), // Maybe add a "global" state in the future

    is_resumable: AtomicBool,
}

impl SessionData {
    pub(crate) fn new(token: String) -> Self {
        SessionData {
            id: Mutex::new("".into()),
            http: HttpClient::new(token),
            state: (),
            is_resumable: AtomicBool::new(true),
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
}
