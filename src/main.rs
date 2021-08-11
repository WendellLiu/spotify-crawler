mod client;
mod config;
mod dto;
mod storage;

use chrono::prelude::*;

use client::spotify::{Country, SpotifyClient};
use config::{SystemConfig, SYSTEM_CONFIG};
use dto::new_release::NewRelease;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let system_config_instance = SystemConfig::new();
    SYSTEM_CONFIG.set(system_config_instance).unwrap();

    let spotify_client = SpotifyClient::new().await?;

    let resp = spotify_client.get_new_releases(Some(Country::TW)).await?;
    println!("{:?}", resp);

    let resp = spotify_client.get_new_releases(None).await?;
    println!("{:?}", resp);

    let new_release = NewRelease {
        timestamp: Utc::now(),
        name: resp.albums.items[0].name.clone(),
    };

    new_release.create_doc().await?;

    Ok(())
}
