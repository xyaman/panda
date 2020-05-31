use crate::models::user::StatusUpdate;
use serde::{Deserialize, Serialize};

use super::Opcode;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub(crate) struct StatusUpdatePayload {
    pub(crate) op: Opcode,
    pub(crate) d: StatusUpdate,
    // pub(crate) s: Option<u64>,
}
