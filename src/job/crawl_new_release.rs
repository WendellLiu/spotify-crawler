use std::str::FromStr;

use crate::client::spotify::{Country, SpotifyClient};
use crate::config::SystemConfig;
use crate::dto::new_release::NewReleases;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let system_config = SystemConfig::global();
    let countries = system_config.event_payload.new_releases.countries.clone();

    let spotify_client = SpotifyClient::new().await?;

    for country in countries {
        println!("start to fetch: country: {}", country);
        let resp = spotify_client
            .get_new_releases(Some(Country::from_str(&country).unwrap()))
            .await?;
        let new_releases = NewReleases {
            payload: resp.albums.items,
            country: Country::from_str(&country).unwrap(),
        };

        println!("finish to fetch: country: {}", country);

        new_releases.create_doc().await?;
    }

    Ok(())
}
