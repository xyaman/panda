use crate::models::guild::Role;
use crate::models::user::User;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Emoji {
    pub id: Option<String>,
    pub name: Option<String>,
    #[serde(default)]
    pub roles: Vec<Role>,
    pub user: Option<User>,
    pub require_colons: Option<bool>,
    pub managed: Option<bool>,
    pub animated: Option<bool>,
}
