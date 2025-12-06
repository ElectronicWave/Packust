use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    pub index: i64,
    pub page_size: i64,
    pub result_count: i64,
    pub total_count: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct Response<T> {
    data: T,
    pagination: Option<Pagination>,
}