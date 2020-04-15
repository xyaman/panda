//! SessionData

use crate::HttpClient;

use std::sync::atomic::{AtomicBool, Ordering};

use futures::lock::Mutex;

/// The struct of the current session of the bot.
pub struct SessionData<S> {
    id: Mutex<String>,
    pub http: HttpClient,
    pub state: S,
    is_resumable: AtomicBool,
}

impl<S> SessionData<S> {
    pub(crate) fn new(token: String, state: S) -> Self {
        SessionData {
            id: Mutex::new("".into()),
            http: HttpClient::new(token),
            state,
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
