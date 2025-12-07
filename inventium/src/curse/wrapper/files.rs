use crate::curse::Curse;
use crate::curse::structs::file_structs::File;
use crate::curse::structs::ID;
use anyhow::Result;

impl Curse {
    pub async fn get_mod_files(&self, mod_id: ID) -> Result<Vec<File>> {
        let mut url = self.base_url
            .join("mods/")?
            .join(&(mod_id.to_string() + "/"))?
            .join("files")?;
        url.set_query(Some("pageSize=10000"));
        Ok(self.get(url).await?.data)
    }

    pub async fn get_mod_file(&self, mod_id: ID, file_id: ID) -> Result<File> {
        Ok(self
            .get(
                self.base_url
                    .join("mods/")?
                    .join(&(mod_id.to_string() + "/"))?
                    .join("files/")?
                    .join(&file_id.to_string())?,
            )
            .await?
            .data)
    }

    pub async fn get_mod_file_changelog(&self, mod_id: ID, file_id: ID) -> Result<String> {
        Ok(self
            .get(
                self.base_url
                    .join("mods/")?
                    .join(&(mod_id.to_string() + "/"))?
                    .join("files/")?
                    .join(&(file_id.to_string() + "/"))?
                    .join("changelog")?,
            )
            .await?
            .data)
    }

    pub async fn file_download_url(&self, mod_id: ID, file_id: ID) -> Result<url::Url> {
        Ok(self
            .get(
                self.base_url
                    .join("mods/")?
                    .join(&(mod_id.to_string() + "/"))?
                    .join("files/")?
                    .join(&(file_id.to_string() + "/"))?
                    .join("download-url")?,
            )
            .await?
            .data)
    }

    pub async fn get_files(&self, file_ids: Vec<ID>) -> Result<Vec<Option<File>>> {
        #[derive(serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct GetFilesBodyRequestBody {
            file_ids: Vec<ID>,
        }

        let file_ids = GetFilesBodyRequestBody { file_ids };
        let mut files: Vec<File> = self
            .post(self.base_url.join("mods/")?.join("files")?, &file_ids)
            .await?
            .data;
        let mut ordered_files = Vec::new();
        for file_id in file_ids.file_ids {
            if let Some(index) = files.iter().position(|file| file.id == file_id) {
                ordered_files.push(Some(files.swap_remove(index)));
            } else {
                ordered_files.push(None);
            }
        }
        Ok(ordered_files)
    }
}