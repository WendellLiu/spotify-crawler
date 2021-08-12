mod client;
mod config;
mod dto;
mod job;
mod storage;

use config::{SystemConfig, SYSTEM_CONFIG};
use job::crawl_new_release;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let system_config_instance = SystemConfig::new();
    SYSTEM_CONFIG.set(system_config_instance).unwrap();

    crawl_new_release::run().await?;
    Ok(())
}
