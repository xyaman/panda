use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Overwrite {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub allow: u64,
    pub deny: u64,
}
