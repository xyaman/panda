use crate::models::gateway::payload::Opcode;
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq)]
pub(crate) struct Resume {
    pub(crate) op: Opcode,
    pub(crate) d: ResumeContent,
}

#[derive(Debug, Serialize, PartialEq)]
pub(crate) struct ResumeContent {
    pub(crate) token: String,
    pub(crate) session_id: String,
    pub(crate) seq: Option<u64>,
}
