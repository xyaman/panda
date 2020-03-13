use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Overwrite {
    id: String,
    #[serde(rename = "type")]
    kind: String,
    allow: u64,
    deny: u64,
}
