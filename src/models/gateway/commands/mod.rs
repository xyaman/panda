// commands
mod identify;
use identify::{Identify, IdentifyContent, IdentifyProperties};

mod heartbeat;
use heartbeat::Heartbeat;

mod resume;
use resume::{Resume, ResumeContent};

use super::payload::{Opcode, Payload};

use async_tungstenite::tungstenite::Message as TungsteniteMessage;
use std::env::consts::OS;

#[allow(dead_code)] // TODO: Add support for Voice
#[derive(Debug, PartialEq)]
pub(crate) enum Command {
    Identify(Identify),
    Resume(Resume),
    Heartbeat(Heartbeat),
    RequestGuildMembers(Payload),
    UpdateVoiceState(Payload),
    UpdateStatus(Payload),
    Close,
}

impl Command {
    /// This function transform a command into a TungsteniteMessage and needs the last
    /// gateway sequence in order to send it correctly
    pub(crate) fn to_tungstenite_message(self, sequence: Option<u64>) -> TungsteniteMessage {
        match self {
            Self::Identify(mut i) => {
                i.s = sequence;
                let cmd_str = serde_json::to_string(&i).unwrap();
                TungsteniteMessage::Text(cmd_str)
            }
            Self::Heartbeat(mut h) => {
                h.d = sequence;
                let cmd_str = serde_json::to_string(&h).unwrap();
                TungsteniteMessage::Text(cmd_str)
            }
            Self::Resume(r) => {
                let cmd_str = serde_json::to_string(&r).unwrap();
                TungsteniteMessage::Text(cmd_str)
            }
            _ => todo!(),
        }
    }

    /// Returns a Identify Command
    pub(crate) fn new_identify(
        token: impl Into<String>,
        large_threshold: u8,
        guild_subscriptions: bool,
        shard: [u64; 2],
    ) -> Command {
        let identify_properties = IdentifyProperties {
            os: OS,
            browser: "discord".into(),
            device: "discord".into(),
        };

        let identify = Identify {
            op: Opcode::Identify,
            d: IdentifyContent {
                token: token.into(),
                properties: identify_properties,
                compress: true,
                large_threshold: Some(large_threshold),
                shard: Some(shard),
                presence: None,
                guild_subscriptions: Some(guild_subscriptions),
            },
            s: None,
        };

        Command::Identify(identify)
    }

    /// Returns a Heartbeat(without sequence) command
    pub(crate) fn new_heartbeat() -> Command {
        Command::Heartbeat(Heartbeat {
            op: Opcode::Heartbeat,
            d: None,
        })
    }

    /// Returns a Resume command
    pub(crate) fn new_resume(token: String, session_id: String, seq: Option<u64>) -> Command {
        let resume_content = ResumeContent { token, session_id, seq };

        let resume = Resume {
            op: Opcode::Resume,
            d: resume_content,
        };

        Command::Resume(resume)
    }
}
