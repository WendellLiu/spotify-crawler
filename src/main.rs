mod client;
mod config;

use client::spotify::SpotifyClient;
use config::{SystemConfig, SYSTEM_CONFIG};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let system_config_instance = SystemConfig::new();
    SYSTEM_CONFIG.set(system_config_instance).unwrap();

    let system_config = SystemConfig::global();

    let spotify_client = SpotifyClient::new(&system_config.spotify.token);

    let resp = spotify_client.get_new_releases().await?;
    println!("{:#?}", resp);

    Ok(())
}
