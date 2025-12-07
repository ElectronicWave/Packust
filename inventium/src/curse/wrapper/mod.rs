/// API Calls code copy from [Furse](https://github.com/gorilla-devs/furse/tree/master/src/api_calls)
/// Under [Furse MIT License](https://github.com/gorilla-devs/furse/blob/master/LICENSE.txt)

pub mod mods;
pub mod search;
pub mod files;
pub mod fingerprint;

use crate::curse::structs::common_structs;

/// API responses are returned in this structure, with the actual results in `data` and optional `pagination`
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Response<T> {
    data: T,
    pagination: Option<common_structs::Pagination>,
}