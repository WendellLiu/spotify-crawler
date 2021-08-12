use crate::client::spotify::{Country, SpotifyClient};
use crate::dto::new_release::NewReleases;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let spotify_client = SpotifyClient::new().await?;

    //let resp = spotify_client.get_new_releases(Some(Country::TW)).await?;

    let resp = spotify_client.get_new_releases(None).await?;

    let new_releases = NewReleases {
        payload: resp.albums.into(),
    };

    new_releases.create_doc().await?;

    Ok(())
}
