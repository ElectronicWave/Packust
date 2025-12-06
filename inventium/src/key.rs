use crate::Platform;
use anyhow::Result;
use reqwest::RequestBuilder;

pub struct ApiKey {
    key: String,
    platform: Platform
}

impl ApiKey {
    pub fn new(key: String, platform: Platform) -> Result<Self> {
        Ok(Self { key, platform })
    }

    pub fn insert(&self, req: RequestBuilder) -> RequestBuilder {
        match self.platform {
            Platform::CurseForge => {
                req.header("x-api-key", &self.key)
            }
            Platform::Modrinth => {
                req.header("Authorization", &self.key)
            }
        }
    }
}