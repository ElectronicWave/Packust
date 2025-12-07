//! Miscellaneous API calls
//!
//! [documentation](https://docs.modrinth.com/api-spec/#tag/misc)

use super::*;
use crate::rinth::request::RequestBuilderCustomSend;
use crate::rinth::structs::misc::Statistics;
use crate::rinth::url::UrlJoinAll;
use crate::rinth::Rinth;

impl Rinth {
    pub async fn instance_statistics(&self) -> Result<Statistics> {
        self.request
            .get(self.base_url.join_all(vec!["statistics"]))
            .await.custom_send_json()
            .await
    }
}
