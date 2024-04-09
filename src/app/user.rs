use lemmy_api_common::sensitive::Sensitive;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: Sensitive<String>,
    pub is_logged: bool,
}

impl User {
    pub fn new(username: Sensitive<String>, is_logged: bool) -> Self {
        Self {
            username,
            is_logged,
        }
    }
}
