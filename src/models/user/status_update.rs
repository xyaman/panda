use super::Activity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct StatusUpdate {
    since: Option<u64>,
    game: Option<Activity>,
    status: String,
    afk: bool,
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

    pub fn game(&mut self, game: Activity) -> &mut Self {
        self.game = Some(game);

        self
    }

    pub fn status(&mut self, status: impl Into<String>) -> &mut Self {
        self.status = status.into();

        self
    }

    pub fn afk(&mut self, afk: bool) -> &mut Self {
        self.afk = afk;

        self
    }
}
