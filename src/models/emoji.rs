use crate::models::guild::Role;
use crate::models::user::User;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Emoji {
    id: Option<String>,
    name: Option<String>,
    roles: Option<Vec<Role>>,
    user: Option<User>,
    require_colons: Option<bool>,
    managed: Option<bool>,
    animated: Option<bool>,
}

impl Emoji {
    pub fn id(&self) -> Option<&str> {
        self.id.as_ref().map(|s| s.as_str())
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|s| s.as_str())
    }

    pub fn roles(&self) -> Option<&[Role]> {
        self.roles.as_ref().map(|r| r.as_slice())
    }

    pub fn user(&self) -> Option<&User> {
        self.user.as_ref()
    }

    pub fn require_colons(&self) -> Option<bool> {
        self.require_colons
    }

    pub fn managed(&self) -> Option<bool> {
        self.managed
    }

    pub fn animated(&self) -> Option<bool> {
        self.animated
    }
}
