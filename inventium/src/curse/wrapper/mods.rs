use crate::curse::Curse;
use crate::curse::structs::ID;
use anyhow::Result;
use crate::curse::structs::common_structs::{Category, ModLoaderType};
use crate::curse::structs::mod_structs::Mod;

impl Curse {
    pub async fn get_mod(&self, mod_id: ID) -> Result<Mod> {
        Ok(self
            .get(self.base_url.join("mods/")?.join(&mod_id.to_string())?)
            .await?
            .data)
    }

    pub async fn get_mods(&self, mod_ids: Vec<ID>) -> Result<Vec<Mod>> {
        #[derive(serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct GetModsByIdsListRequestBody {
            mod_ids: Vec<ID>,
        }

        Ok(self
            .post(
                self.base_url.join("mods")?,
                &GetModsByIdsListRequestBody { mod_ids },
            )
            .await?
            .data)
    }

    pub async fn get_mod_description(&self, mod_id: ID) -> Result<String> {
        Ok(self
            .get(
                self.base_url
                    .join("mods/")?
                    .join(&(mod_id.to_string() + "/"))?
                    .join("description")?,
            )
            .await?
            .data)
    }

    pub async fn search_mods(&self, query: &str) -> Result<Vec<Mod>> {

        Ok(self
            .get(
                self.base_url.join("mods/search?")?,
            )
            .await?
            .data)
    }
}