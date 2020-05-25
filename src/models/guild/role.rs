//! Guild related models

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub color: u64, // maybe create type
    pub hoist: bool,
    pub position: u64,
    pub permissions: u64,
    pub managed: bool,
    pub mentionable: bool,
}
