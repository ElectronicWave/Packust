use super::*;
use crate::rinth::structs::search::{Facet, Response, Sort};
use crate::rinth::structs::Int;
use crate::rinth::url::{UrlJoinAll, UrlWithQuery};
use crate::rinth::Rinth;
use anyhow::Result;

impl Rinth {
    pub async fn search_paged(
        &self,
        query: impl ToString,
        sort: Sort,
        limit: Int,
        offset: Int,
        mut facets: Vec<Vec<Facet>>,
    ) -> Result<Response> {
        let mut url = self.base_url
            .join_all(vec!["search"])
            .with_query("query", query)
            .with_query("index", sort)
            .with_query("limit", limit)
            .with_query("offset", offset);

        facets.retain(|e| !e.is_empty());
        if !facets.is_empty() {
            url = url.with_query_json("facets", facets)?
        }

        self.request.get(url).custom_send_json().await
    }

    pub async fn search(
        &self,
        query: &str,
        sort: &Sort,
        mut facets: Vec<Vec<Facet>>,
    ) -> Result<Response> {
        let mut url = self.base_url
            .join_all(vec!["search"])
            .with_query("query", query)
            .with_query("index", sort);

        facets.retain(|e| !e.is_empty());
        if !facets.is_empty() {
            url = url.with_query_json("facets", facets)?
        }

        self.request.get(url).custom_send_json().await
    }
}
