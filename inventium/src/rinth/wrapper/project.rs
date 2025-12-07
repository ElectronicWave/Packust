//! API calls related to projects
//!
//! [documentation](https://docs.modrinth.com/api-spec/#tag/projects)

use crate::rinth::structs::project::{Project, ProjectDependencies};
use crate::rinth::structs::Int;
use crate::rinth::url::{UrlJoinAll, UrlWithQuery};
use crate::rinth::wrapper::check_id_slug;
use crate::rinth::Rinth;
use anyhow::Result;
use crate::rinth::request::RequestBuilderCustomSend;

impl Rinth {
    pub async fn project_get(&self, project_id: &str) -> Result<Project> {
        check_id_slug(&[project_id])?;
        self.request
            .get(self.base_url.join_all(vec!["project", project_id]))
            .await.custom_send_json()
            .await
    }

    pub async fn project_get_multiple(&self, project_ids: &[&str]) -> Result<Vec<Project>> {
        check_id_slug(project_ids)?;
        self.request
            .get(
                self.base_url
                    .join_all(vec!["projects"])
                    .with_query_json("ids", project_ids)?,
            )
            .await.custom_send_json()
            .await
    }

    pub async fn project_get_random(&self, count: Int) -> Result<Vec<Project>> {
        self.request
            .get(
                self.base_url
                    .join_all(vec!["projects_random"])
                    .with_query("count", count),
            )
            .await.custom_send_json()
            .await
    }

    pub async fn project_check_validity(&self, project_id: &str) -> Result<String> {
        #[derive(serde::Deserialize)]
        struct Response {
            id: String,
        }
        check_id_slug(&[project_id])?;
        let res: Response = self
            .request
            .get(self.base_url.join_all(vec!["project", project_id, "check"]))
            .await.custom_send_json()
            .await?;
        Ok(res.id)
    }

    pub async fn project_get_dependencies(&self, project_id: &str) -> Result<ProjectDependencies> {
        check_id_slug(&[project_id])?;
        self.request
            .get(self.base_url.join_all(vec!["project", project_id, "dependencies"]))
            .await.custom_send_json()
            .await
    }
}
