use crate::{Platform, Request};
use anyhow::Result;

pub mod responses;

pub struct Curse {
    base_url: String,
    request: Request
}

impl Curse {
    pub fn new(url: &str, agent: &str, key: String) -> Result<Self> {
        Ok(Self {
            base_url: url.to_string(),
            request: Request::new(agent, Platform::CurseForge, key)?
        })
    }

    pub fn default(agent: &str, key: String) -> Result<Self> {
        Self::new("https://api.curseforge.com", agent, key)
    }
}