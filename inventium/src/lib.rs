use reqwest::{Client, IntoUrl, RequestBuilder, Response};
use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, ACCEPT, CONTENT_TYPE, USER_AGENT};
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::key::ApiKey;

pub mod curse;
pub mod key;

pub enum Platform {
    CurseForge,
    Modrinth
}

pub struct Request {
    pub client: Client,
    pub key: ApiKey
}

impl Request {
    pub fn new(agent: &str, platform: Platform, key: String) -> Result<Self> {
        let mut builder = reqwest::ClientBuilder::new()
            .user_agent(agent);

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

        builder = builder.default_headers(headers);

        Ok(Self {
            client: builder.build()?,
            key: ApiKey::new(key, platform)?
        })
    }

    pub async fn get(&self, url: impl IntoUrl) -> RequestBuilder {
        self.key.insert(self.client.get(url))
    }

    pub fn post<B: Serialize>(&self, url: impl IntoUrl, body: &B) -> RequestBuilder {
        self.key.insert(self.client.post(url).json(body))
    }
}