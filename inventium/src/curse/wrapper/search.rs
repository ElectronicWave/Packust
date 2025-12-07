use crate::curse::Curse;
use crate::curse::structs::common_structs::{Category, ModLoaderType};
use crate::curse::structs::mod_structs::Mod;

impl Curse {
    pub async fn search_mods(&self, param: SearchModsParam) -> anyhow::Result<Vec<Mod>> {
        Ok(self
            .get(
                self.base_url.join(&*param.build(UrlBuilder::new("mods/search")).url)?,
            )
            .await?
            .data)
    }
}

pub struct UrlBuilder {
    pub url: String,
}

impl UrlBuilder {
    pub fn new(base_url: &str) -> Self {
        Self {
            url: base_url.to_string() + "?",
        }
    }

    pub fn add_param(mut self, key: &str, value: &str) -> Self {
        self.url.push_str(&format!("{}={}&", key, value));
        self
    }

    pub fn as_str(&self) -> &str {
        &self.url
    }
}

#[derive(serde::Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SearchModsParam {
    pub class_id: Option<i32>,
    pub category_id: Option<Category>,
    pub category_ids: Option<Vec<Category>>,
    pub game_version: Option<String>,
    pub game_versions: Option<String>,
    pub search_filter: Option<String>,
    pub sort_field: Option<i32>,
    pub sort_order: Option<String>,
    pub mod_loader_type: Option<ModLoaderType>,
    pub mod_loader_types: Option<Vec<ModLoaderType>>,
    pub game_version_type_id: Option<i32>,
    pub author_id: Option<i32>,
    pub primary_author_id: Option<i32>,
    pub slug: Option<String>,
    pub index: Option<i32>,
    pub page_size: Option<i32>,
}

impl SearchModsParam {
    pub fn build(&self, builder: UrlBuilder) -> UrlBuilder {
        let mut b = builder;
        b = b.add_param("gameId", "432");
        if let Some(class_id) = self.class_id {
            b = b.add_param("classId", &class_id.to_string());
        }
        if let Some(category_id) = &self.category_id {
            b = b.add_param("categoryId", &category_id.id.to_string());
        }
        if let Some(category_ids) = &self.category_ids {
            let ids: Vec<String> = category_ids.iter().map(|c| c.id.to_string()).collect();
            b = b.add_param("categoryIds", &ids.join(","));
        }
        if let Some(game_version) = &self.game_version {
            b = b.add_param("gameVersion", game_version);
        }
        if let Some(game_versions) = &self.game_versions {
            b = b.add_param("gameVersions", game_versions);
        }
        if let Some(search_filter) = &self.search_filter {
            b = b.add_param("searchFilter", search_filter);
        }
        if let Some(sort_field) = self.sort_field {
            b = b.add_param("sortField", &sort_field.to_string());
        }
        if let Some(sort_order) = &self.sort_order {
            b = b.add_param("sortOrder", sort_order);
        }
        if let Some(mod_loader_type) = self.mod_loader_type {
            b = b.add_param("modLoaderType", &*mod_loader_type.id().to_string());
        }
        if let Some(mod_loader_types) = &self.mod_loader_types {
            let ids: Vec<String> = mod_loader_types
                .iter()
                .map(|m| m.id().to_string())
                .collect();
            b = b.add_param("modLoaderTypes", &ids.join(","));
        }
        if let Some(game_version_type_id) = self.game_version_type_id {
            b = b.add_param("gameVersionTypeId", &game_version_type_id.to_string());
        }
        if let Some(author_id) = self.author_id {
            b = b.add_param("authorId", &author_id.to_string());
        }
        if let Some(primary_author_id) = self.primary_author_id {
            b = b.add_param("primaryAuthorId", &primary_author_id.to_string());
        }
        if let Some(slug) = &self.slug {
            b = b.add_param("slug", slug);
        }
        if let Some(index) = self.index {
            b = b.add_param("index", &index.to_string());
        }
        if let Some(page_size) = self.page_size {
            b = b.add_param("pageSize", &page_size.to_string());
        }
        b
    }
}