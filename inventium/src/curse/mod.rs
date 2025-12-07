/// Portions of this file are from [Furse](https://github.com/gorilla-devs/furse) by [GorillaDevs Inc](https://github.com/gorilla-devs).
/// Licensed under the MIT License.
/// Copyright (c) 2021 theRookieCoder.

use crate::curse::wrapper::Response;
use crate::{Platform, Request};
use anyhow::Result;
use serde::de::DeserializeOwned;
use serde::Serialize;
use url::Url;

pub mod wrapper;
pub mod structs;

pub struct Curse {
    base_url: Url,
    request: Request
}

impl Curse {
    pub fn new(url: Url, agent: &str, key: String) -> Result<Self> {
        Ok(Self {
            base_url: url,
            request: Request::new(agent, Platform::CurseForge, key)?
        })
    }

    pub fn default(agent: &str, key: String) -> Result<Self> {
        Self::new(Url::parse("https://api.curseforge.com/v1/")?, agent, key)
    }

    async fn get<T: DeserializeOwned>(&self, url: Url) -> Result<Response<T>> {
        let byte = self.request.get(url).await.send().await?.error_for_status()?.bytes().await?;
        Ok(serde_json::from_slice(&byte)?)
    }

    async fn post<T: DeserializeOwned, B: Serialize>(&self, url: Url, body: B) -> Result<Response<T>> {
        let byte = self.request.post(url, &body).await.send().await?.error_for_status()?.bytes().await?;
        Ok(serde_json::from_slice(&byte)?)
    }
}