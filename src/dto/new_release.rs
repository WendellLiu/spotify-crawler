use chrono::{DateTime, Utc};
use elasticsearch::IndexParts;
use serde::{Deserialize, Serialize};

use crate::storage::es::EsClient;

static NEW_RELEASES_INDEX: &'static str = "new_releases";

#[derive(Debug, Serialize, Deserialize)]
pub struct NewRelease {
    #[serde(rename = "@timestamp")]
    pub timestamp: DateTime<Utc>,
    pub name: String,
}

impl NewRelease {
    pub async fn create_doc(&self) -> Result<(), Box<dyn std::error::Error>> {
        let es_client = EsClient::new().await?;
        es_client
            .client
            .index(IndexParts::Index(NEW_RELEASES_INDEX))
            .body(self)
            .send()
            .await?;

        Ok(())
    }
}
