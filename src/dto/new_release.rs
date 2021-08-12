use chrono::{DateTime, Utc};

use elasticsearch::{BulkOperation, BulkParts, IndexParts};
use serde::{Deserialize, Serialize};

use crate::client::spotify::{AlbumItem, Albums};
use crate::storage::es::EsClient;

static NEW_RELEASES_INDEX: &'static str = "new_releases";

#[derive(Debug, Serialize, Deserialize)]
pub struct NewReleaseItem {
    #[serde(rename = "@timestamp")]
    pub timestamp: DateTime<Utc>,
    pub name: String,
}

impl From<AlbumItem> for NewReleaseItem {
    fn from(item: AlbumItem) -> Self {
        NewReleaseItem {
            timestamp: Utc::now(),
            name: item.name.clone(),
        }
    }
}

impl From<Albums> for Vec<NewReleaseItem> {
    fn from(albums: Albums) -> Self {
        albums.items.into_iter().map(|item| item.into()).collect()
    }
}

pub struct NewRelease {
    pub payload: NewReleaseItem,
}

pub struct NewReleases {
    pub payload: Vec<NewReleaseItem>,
}

impl NewRelease {
    pub async fn create_doc(&self) -> Result<(), Box<dyn std::error::Error>> {
        let es_client = EsClient::new().await?;
        es_client
            .client
            .index(IndexParts::Index(NEW_RELEASES_INDEX))
            .body(&self.payload)
            .send()
            .await?;

        Ok(())
    }
}

impl NewReleases {
    pub async fn create_doc(&self) -> Result<(), Box<dyn std::error::Error>> {
        let es_client = EsClient::new().await?;
        let body: Vec<BulkOperation<_>> = self
            .payload
            .iter()
            .map(|new_release| {
                //let id = new_release.id().to_string();
                BulkOperation::index(new_release).into()
            })
            .collect();

        es_client
            .client
            .bulk(BulkParts::Index(NEW_RELEASES_INDEX))
            .body(body)
            .send()
            .await?;

        Ok(())
    }
}
