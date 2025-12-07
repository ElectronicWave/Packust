//! API calls related to tags
//!
//! [documentation](https://docs.modrinth.com/api-spec/#tag/tags)

use super::*;
use crate::rinth::structs::tag::{Category, DonationPlatform, GameVersion, License, Loader};
use crate::rinth::url::UrlJoinAll;
use crate::rinth::Rinth;
use anyhow::Result;
use crate::rinth::request::RequestBuilderCustomSend;

impl Rinth {
    pub async fn tag_list_categories(&self) -> Result<Vec<Category>> {
        self.request
            .get(self.base_url.join_all(vec!["tag", "category"]))
            .await.custom_send_json()
            .await
    }

    pub async fn tag_list_loaders(&self) -> Result<Vec<Loader>> {
        self.request
            .get(self.base_url.join_all(vec!["tag", "loader"]))
            .await.custom_send_json()
            .await
    }

    pub async fn tag_list_game_versions(&self) -> Result<Vec<GameVersion>> {
        self.request
            .get(self.base_url.join_all(vec!["tag", "game_version"]))
            .await.custom_send_json()
            .await
    }

    pub async fn tag_license_text_and_title(&self, id: &str) -> Result<License> {
        self.request
            .get(self.base_url.join_all(vec!["tag", "license", id]))
            .await.custom_send_json()
            .await
    }

    pub async fn tag_list_donation_platforms(&self) -> Result<Vec<DonationPlatform>> {
        self.request
            .get(self.base_url.join_all(vec!["tag", "donation_platform"]))
            .await.custom_send_json()
            .await
    }

    pub async fn tag_list_report_types(&self) -> Result<Vec<String>> {
        self.request
            .get(self.base_url.join_all(vec!["tag", "report_type"]))
            .await.custom_send_json()
            .await
    }
}
