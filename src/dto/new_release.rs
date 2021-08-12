use chrono::{DateTime, Utc};

use elasticsearch::{BulkOperation, BulkParts, IndexParts};
use serde::{Deserialize, Serialize};

use crate::client::spotify::{AlbumItem, Albums, ExternalUrlsObject, ImageObject};
use crate::storage::es::EsClient;

static NEW_RELEASES_INDEX: &'static str = "new_releases";

#[derive(Debug, Serialize, Deserialize)]
pub struct NewReleaseItem {
    #[serde(rename = "@timestamp")]
    pub timestamp: DateTime<Utc>,
    pub id: String,
    pub name: String,
    pub external_urls: ExternalUrlsObject,
    #[serde(default)]
    pub genres: Vec<String>,
    #[serde(default)]
    pub images: Vec<ImageObject>,
    #[serde(default)]
    pub label: String,
    pub release_date: String,
    pub release_date_precision: String,
    pub total_tracks: u32,
}

impl From<AlbumItem> for NewReleaseItem {
    fn from(item: AlbumItem) -> Self {
        NewReleaseItem {
            timestamp: Utc::now(),
            id: item.id,
            name: item.name,
            external_urls: item.external_urls,
            genres: item.genres,
            images: item.images,
            label: item.label,
            release_date: item.release_date,
            release_date_precision: item.release_date_precision,
            total_tracks: item.total_tracks,
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
