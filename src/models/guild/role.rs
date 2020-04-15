//! Guild related models

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Role {
    id: String,
    name: String,
    color: u64, // maybe create type
    hoist: bool,
    position: u64,
    permissions: u64,
    managed: bool,
    mentionable: bool,
}

impl Role {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn color(&self) -> u64 {
        self.color
    }

    pub fn hoist(&self) -> bool {
        self.hoist
    }

    pub fn position(&self) -> u64 {
        self.position
    }

    pub fn permissions(&self) -> u64 {
        self.permissions
    }

    pub fn managed(&self) -> bool {
        self.managed
    }

    pub fn mentionable(&self) -> bool {
        self.mentionable
    }
}
