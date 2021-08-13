use chrono::{DateTime, Utc};

use elasticsearch::{BulkOperation, BulkParts, IndexParts};
use serde::{Deserialize, Serialize};

use crate::client::spotify::{AlbumItem, Country, ExternalUrlsObject, ImageObject};
use crate::storage::es::EsClient;

static NEW_RELEASES_INDEX: &'static str = "new_releases";

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub country: Country,
}

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
    pub metadata: Metadata,
}

impl From<&NewRelease> for NewReleaseItem {
    fn from(new_release: &NewRelease) -> Self {
        let item = &new_release.payload;
        NewReleaseItem {
            timestamp: Utc::now(),
            id: item.id.clone(),
            name: item.name.clone(),
            external_urls: item.external_urls.clone(),
            genres: item.genres.clone(),
            images: item.images.clone(),
            label: item.label.clone(),
            release_date: item.release_date.clone(),
            release_date_precision: item.release_date_precision.clone(),
            total_tracks: item.total_tracks,
            metadata: Metadata {
                country: new_release.country.clone(),
            },
        }
    }
}

impl From<&NewReleases> for Vec<NewReleaseItem> {
    fn from(new_releases: &NewReleases) -> Self {
        let items = &new_releases.payload;
        let country = &new_releases.country;
        items
            .iter()
            .map(|item| NewReleaseItem {
                timestamp: Utc::now(),
                id: item.id.clone(),
                name: item.name.clone(),
                external_urls: item.external_urls.clone(),
                genres: item.genres.clone(),
                images: item.images.clone(),
                label: item.label.clone(),
                release_date: item.release_date.clone(),
                release_date_precision: item.release_date_precision.clone(),
                total_tracks: item.total_tracks,
                metadata: Metadata {
                    country: country.clone(),
                },
            })
            .collect()
    }
}

pub struct NewRelease {
    pub payload: AlbumItem,
    pub country: Country,
}

pub struct NewReleases {
    pub payload: Vec<AlbumItem>,
    pub country: Country,
}

impl NewRelease {
    pub async fn create_doc(&self) -> Result<(), Box<dyn std::error::Error>> {
        let es_client = EsClient::new().await?;
        let body = NewReleaseItem::from(self);
        es_client
            .client
            .index(IndexParts::Index(NEW_RELEASES_INDEX))
            .body(body)
            .send()
            .await?;

        Ok(())
    }
}

impl NewReleases {
    pub async fn create_doc(&self) -> Result<(), Box<dyn std::error::Error>> {
        let es_client = EsClient::new().await?;
        let new_releases = Vec::<NewReleaseItem>::from(self);
        let body: Vec<BulkOperation<_>> = new_releases
            .iter()
            .map(|new_release| BulkOperation::index(new_release).into())
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
