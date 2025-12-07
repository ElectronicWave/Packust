use super::*;
use crate::rinth::request::RequestBuilderCustomSend;
use crate::rinth::structs::version::{HashAlgorithm, LatestVersionBody, LatestVersionsBody, Version};
use crate::rinth::url::{UrlJoinAll, UrlWithQuery};
use crate::rinth::Rinth;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Rinth {
    pub async fn version_get_from_hash(&self, hash: &str) -> Result<Version> {
        check_sha1_hash(&[hash])?;
        self.request
            .get(self.base_url.join_all(vec!["version_file", hash]))
            .await.custom_send_json()
            .await
    }

    pub async fn version_get_from_multiple_hashes(
        &self,
        hashes: Vec<String>,
    ) -> Result<HashMap<String, Version>> {
        #[derive(Deserialize, Serialize, Debug, Clone)]
        pub struct HashesBody {
            pub hashes: Vec<String>,
            pub algorithm: HashAlgorithm,
        }

        check_sha1_hash(&hashes)?;
        self.request
            .post(self.base_url.join_all(vec!["version_files"]), &HashesBody {
                hashes,
                algorithm: HashAlgorithm::SHA1,
            })
            .await.custom_send_json()
            .await
    }

    pub async fn version_get_latest_from_hash(
        &self,
        hash: &str,
        filters: &LatestVersionBody,
    ) -> Result<Version> {
        check_sha1_hash(&[hash])?;
        self.request
            .post(
                self.base_url
                    .join_all(vec!["version_file", hash, "update"])
                    .with_query_json("algorithm", HashAlgorithm::SHA1)?,
            filters)
            .await.custom_send_json()
            .await
    }

    pub async fn version_get_latest_from_multiple_hashes(
        &self,
        hashes: Vec<String>,
        filters: LatestVersionBody,
    ) -> Result<HashMap<String, Version>> {
        check_sha1_hash(&hashes)?;
        self.request
            .post(self.base_url.join_all(vec!["version_files", "update"]), &LatestVersionsBody {
                hashes,
                algorithm: HashAlgorithm::SHA1,
                loaders: filters.loaders,
                game_versions: filters.game_versions,
            })
            .await.custom_send_json()
            .await
    }
}
