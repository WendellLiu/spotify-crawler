mod client;
mod config;

use client::spotify::SpotifyClient;
use config::{SystemConfig, SYSTEM_CONFIG};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let system_config_instance = SystemConfig::new();
    SYSTEM_CONFIG.set(system_config_instance).unwrap();

    let spotify_client = SpotifyClient::new().await?;

    println!("{:?}", spotify_client);

    //let resp = spotify_client.get_new_releases().await?;
    //println!("{:#?}", resp);

    Ok(())
}
