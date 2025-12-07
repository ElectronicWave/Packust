//! API calls related to versions
//!
//! [documentation](https://docs.modrinth.com/api-spec/#tag/versions)

use super::*;
use crate::rinth::structs::version::Version;
use crate::rinth::url::{UrlJoinAll, UrlWithQuery};
use crate::rinth::Rinth;
use anyhow::Result;

impl Rinth {
    pub async fn version_list(&self, project_id: &str) -> Result<Vec<Version>> {
        check_id_slug(&[project_id])?;
        self.request
            .get(self.base_url.join_all(vec!["project", project_id, "version"]))
            .custom_send_json()
            .await
    }
    
    pub async fn version_list_filtered(
        &self,
        project_id: &str,
        loaders: Option<&[&str]>,
        game_versions: Option<&[&str]>,
        featured: Option<bool>,
    ) -> Result<Vec<Version>> {
        check_id_slug(&[project_id])?;
        let mut url = self.base_url.join_all(vec!["project", project_id, "version"]);
        if let Some(loaders) = loaders {
            url = url.with_query_json("loaders", loaders)?;
        }
        if let Some(game_versions) = game_versions {
            url = url.with_query_json("game_versions", game_versions)?;
        }
        if let Some(featured) = featured {
            url = url.with_query_json("featured", featured)?;
        }
        self.request.get(url).custom_send_json().await
    }
    
    pub async fn version_get(&self, version_id: &str) -> Result<Version> {
        check_id_slug(&[version_id])?;
        self.request
            .get(self.base_url.join_all(vec!["version", version_id]))
            .custom_send_json()
            .await
    }
    
    pub async fn version_get_from_number(&self, project_id: &str, number: &str) -> Result<Version> {
        check_id_slug(&[project_id])?;
        self.request
            .get(self.base_url.join_all(vec!["project", project_id, "version", number]))
            .custom_send_json()
            .await
    }
    
    pub async fn version_get_multiple(&self, version_ids: &[&str]) -> Result<Vec<Version>> {
        check_id_slug(version_ids)?;
        self.request
            .get(
                self.base_url
                    .join_all(vec!["versions"])
                    .with_query_json("ids", version_ids)?,
            )
            .custom_send_json()
            .await
    }
}
