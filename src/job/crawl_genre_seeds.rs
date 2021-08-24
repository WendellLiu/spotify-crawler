use crate::client::spotify::SpotifyClient;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let spotify_client = SpotifyClient::new().await?;

    let resp = spotify_client.get_genre_seeds().await?;

    for genre in resp.genres {
        println!("genre: {}", genre);
    }

    Ok(())
}
