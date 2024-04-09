use lemmy_api_common::sensitive::Sensitive;
use serde_derive::{Deserialize, Serialize};

use crate::settings::JWT;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: Sensitive<String>,
    pub is_logged: bool,
}

impl User {
    pub fn new(username: Sensitive<String>, is_logged: bool) -> Self {
        let is_logged = if let Ok(jwt) = confy::load::<JWT>("lemnux", "user") {
            jwt.token.is_some()
        } else {
            is_logged
        };

        Self {
            username,
            is_logged,
        }
    }
}
