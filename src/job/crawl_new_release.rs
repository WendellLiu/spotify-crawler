use crate::client::spotify::{Country, SpotifyClient};
use crate::dto::new_release::NewRelease;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let spotify_client = SpotifyClient::new().await?;

    let resp = spotify_client.get_new_releases(Some(Country::TW)).await?;
    println!("{:?}", resp);

    let resp = spotify_client.get_new_releases(None).await?;
    println!("{:?}", resp);

    let new_release = NewRelease {
        payload: (&resp.albums.items[0]).into(),
    };

    new_release.create_doc().await?;

    Ok(())
}
