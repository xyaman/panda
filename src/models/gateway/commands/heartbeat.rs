use crate::models::gateway::payload::Opcode;
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq)]
pub(crate) struct Heartbeat {
    pub(crate) op: Opcode,
    pub(crate) d: Option<u64>,
}
