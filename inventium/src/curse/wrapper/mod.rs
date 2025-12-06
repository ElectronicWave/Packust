mod mods;
mod search;

use crate::curse::structs::common_structs;

/// API responses are returned in this structure, with the actual results in `data` and optional `pagination`
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Response<T> {
    data: T,
    pagination: Option<common_structs::Pagination>,
}