use crate::models::voice::*;
use serde::{ Deserialize, Serialize };

#[derive(Debug, Deserialize, Serialize)]
pub struct VoiceStateUpdate(pub VoiceState);