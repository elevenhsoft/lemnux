use anyhow::Result;
use lemmy_api_common::{
    person::{Login, LoginResponse},
    sensitive::Sensitive,
};
use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT, USER_AGENT},
    Client, ClientBuilder,
};
use serde::{Deserialize, Serialize};

use crate::settings::{Settings, JWT, LEMNUX_UA};

const API_URL: &str = "/api";
const API_VER: &str = "/v3";

#[derive(Debug)]
pub struct API {
    pub domain: String,
    pub url: String,
    client: Client,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instances {
    pub federated_instances: FederatedInstance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederatedInstance {
    pub linked: Vec<Instance>,
    pub allowed: Vec<Instance>,
    pub blocked: Vec<Instance>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instance {
    pub id: u64,
    pub domain: String,
    pub published: String,
    pub updated: Option<String>,
    pub software: Option<String>,
    pub version: Option<String>,
    pub federation_state: Option<FederationState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederationState {
    pub instance_id: u64,
    pub last_successful_id: Option<u64>,
    pub last_successful_published_time: Option<String>,
    pub fail_count: u64,
    pub last_retry: Option<String>,
    pub next_retry: Option<String>,
}

impl Instances {
    pub async fn new() -> Result<Instances> {
        let domain = "lemmy.ml";
        let url = format!(
            "https://{}{}{}/federated_instances",
            domain, API_URL, API_VER
        );
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(USER_AGENT, HeaderValue::from_static(LEMNUX_UA));
        let client = ClientBuilder::new().default_headers(headers).build()?;

        Ok(client.get(url).send().await?.json().await?)
    }

    pub async fn mine(id: u64) -> Option<Instance> {
        let data = Self::new().await;

        if let Ok(data) = data {
            return data
                .federated_instances
                .linked
                .into_iter()
                .find(|item| item.id == id);
        }

        None
    }
}

lazy_static! {
    static ref INS_SETTINGS: Settings = confy::load("lemnux", "instance").unwrap();
    static ref REQUEST: API = API::new(true, INS_SETTINGS.instance.clone().unwrap().domain);
}

impl API {
    pub fn new(secure: bool, domain: String) -> Self {
        let client = Client::new();
        let url = format!(
            "http{}://{}{}{}",
            if secure { "s" } else { "" },
            domain,
            API_URL,
            API_VER
        );

        Self {
            domain,
            url,
            client,
        }
    }
}

pub async fn login(
    username_or_email: Sensitive<String>,
    password: Sensitive<String>,
    totp_2fa_token: Option<String>,
) -> Option<JWT> {
    let params = Login {
        username_or_email,
        password,
        totp_2fa_token,
    };

    let url = format!("{}/user/login", REQUEST.url.clone());

    let response = REQUEST
        .client
        .post(url)
        .json(&params)
        .send()
        .await
        .unwrap()
        .json::<LoginResponse>()
        .await
        .unwrap();

    let jwt = JWT {
        token: response.jwt,
        registration_created: response.registration_created,
        verify_email_sent: response.verify_email_sent,
    };

    Some(jwt)
}
