use lemmy_api_common::sensitive::Sensitive;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub username: Sensitive<String>,
    pub password: Sensitive<String>,
}
