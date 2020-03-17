use serde::{Deserialize, Serialize};
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct MessageEdit {
    content: Option<String>,
    embed: Option<()>,
    flags: u64, // TODO: SEE FLAGS
}

impl MessageEdit {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn content(mut self, s: impl Into<String>) -> Self {
        self.content = Some(s.into());

        self
    }

    pub fn embed(self) {
        unimplemented!()
    }

    pub fn flags(self) {
        unimplemented!()
    }
}
