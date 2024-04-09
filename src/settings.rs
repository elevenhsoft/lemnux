use crate::{api::Instance, app::user::User};
use lemmy_api_common::sensitive::Sensitive;
use serde_derive::{Deserialize, Serialize};

pub const LEMNUX_UA: &str = "Lemnux v0.1.0";

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct JWT {
    pub token: Option<Sensitive<String>>,
    pub registration_created: bool,
    pub verify_email_sent: bool,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Settings {
    pub user: Option<User>,
    pub jwt: Option<JWT>,
    pub instance: Option<Instance>,
}

impl Settings {}
