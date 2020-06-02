use super::Activity;
use serde::{Deserialize, Serialize};

use std::fmt::{self, Display, Formatter};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct StatusUpdate {
    since: Option<u64>,
    game: Option<Activity>,
    status: String,
    afk: bool,
}

/// The user new status
pub enum Status {
    /// Online
    Online,

    /// Do not disturb
    Dnd,

    /// AFK
    AFK,

    /// Invisible and shown as offline
    Invisible,

    /// Offline
    Offline,
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Online => write!(f, "online"),
            Self::Dnd => write!(f, "dnd"),
            Self::AFK => write!(f, "idle"),
            Self::Invisible => write!(f, "invisible"),
            Self::Offline => write!(f, "offline"),
        }
    }
}

impl StatusUpdate {
    pub fn new() -> Self {
        StatusUpdate {
            since: None,
            game: None,
            status: String::new(),
            afk: false,
        }
    }

    pub fn since(&mut self, since: u64) -> &mut Self {
        self.since = Some(since);

        self
    }

    pub fn set_activity(&mut self, game: Activity) -> &mut Self {
        self.game = Some(game);

        self
    }

    pub fn set_status(&mut self, status: Status) -> &mut Self {
        self.status = format!("{}", status);

        if self.status == "idle" {
            self.afk = true;
        }

        self
    }
}
