use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageApplication {
    pub id: String,
    pub cover_image: Option<String>,
    pub description: String,
    pub icon: Option<String>,
    pub name: String,
}
