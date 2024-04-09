#![allow(clippy::unnecessary_to_owned, clippy::to_string_in_format_args)]

use anyhow::Result;
use lemmy_api_common::{
    lemmy_db_schema::{newtypes::CommunityId, ListingType, SortType},
    lemmy_db_views::structs::PaginationCursor,
    person::{Login, LoginResponse},
    post::GetPostsResponse,
    sensitive::Sensitive,
};
use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, USER_AGENT},
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
    static ref INST_SETTINGS: Settings = confy::load("lemnux", "instance").unwrap();
    static ref USER_SETTINGS: Settings = confy::load("lemnux", "user").unwrap();
    static ref REQUEST: API = API::new(
        true,
        INST_SETTINGS.instance.clone().unwrap().domain,
        USER_SETTINGS.jwt.clone().unwrap().token
    );
}

impl API {
    pub fn new(secure: bool, domain: String, jwt: Option<Sensitive<String>>) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(USER_AGENT, HeaderValue::from_static(LEMNUX_UA));

        let client = if jwt.is_some() {
            let bearer_token = format!("Bearer {}", jwt.unwrap().to_string());
            headers.insert(AUTHORIZATION, HeaderValue::from_str(&bearer_token).unwrap());
            ClientBuilder::new()
                .default_headers(headers)
                .build()
                .unwrap()
        } else {
            ClientBuilder::new()
                .default_headers(headers)
                .build()
                .unwrap()
        };

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

#[derive(Debug, Serialize, Deserialize)]
pub struct PostsList {
    pub type_: Option<ListingType>,
    pub sort: Option<SortType>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub community_id: Option<CommunityId>,
    pub community_name: Option<String>,
    pub saved_only: Option<bool>,
    pub liked_only: Option<bool>,
    pub disliked_only: Option<bool>,
    pub page_cursor: Option<PaginationCursor>,
}

impl PostsList {
    pub fn new(page_cursor: Option<PaginationCursor>) -> Self {
        Self {
            type_: Some(ListingType::Local),
            sort: Some(SortType::Hot),
            page: None,
            limit: Some(20),
            community_id: None,
            community_name: None,
            saved_only: Some(false),
            liked_only: Some(false),
            disliked_only: Some(false),
            page_cursor,
        }
    }
}

impl Default for PostsList {
    fn default() -> Self {
        Self::new(None)
    }
}

pub async fn get_posts(page_cursor: Option<PaginationCursor>) -> Option<GetPostsResponse> {
    let post_config = PostsList::new(page_cursor);

    let url = format!("{}/post/list", REQUEST.url.clone());

    let response = REQUEST
        .client
        .get(url)
        .query(&post_config)
        .send()
        .await
        .unwrap()
        .json::<GetPostsResponse>()
        .await
        .unwrap();

    Some(response)
}
