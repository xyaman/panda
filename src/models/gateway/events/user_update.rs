use crate::models::user::User;
use serde::{Deserialize, Serialize};

use std::ops::Deref;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserUpdate(pub User);

impl Deref for UserUpdate {
    type Target = User;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
