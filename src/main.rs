mod client;
mod cmd;
mod config;
mod dto;
mod job;
mod storage;

use clap::Clap;
use cmd::{Opts, SubCommand};
use config::{SystemConfig, SYSTEM_CONFIG};
use job::{crawl_genre_seeds, crawl_new_release};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts: Opts = Opts::parse();

    let system_config_instance = SystemConfig::new(opts.config);
    SYSTEM_CONFIG.set(system_config_instance).unwrap();

    match opts.subcmd {
        SubCommand::NewRelease => {
            crawl_new_release::run().await?;
        }
        SubCommand::ListGenreSeeds => {
            crawl_genre_seeds::run().await?;
        }
        _ => {}
    }

    Ok(())
}
