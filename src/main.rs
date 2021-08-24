mod client;
mod config;
mod dto;
mod job;
mod storage;

use clap::{AppSettings, Clap};
use config::{SystemConfig, SYSTEM_CONFIG};
use job::crawl_new_release;

#[derive(Clap, Debug)]
#[clap(version = "1.0", author = "Wendell Liu <cuk.bas@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// file path of config
    #[clap(short, long, default_value = "config.yml")]
    config: String,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap, Debug)]
enum SubCommand {
    #[clap(version = "1.0", author = "Wendell Liu <cuk.bas@gmail.com>")]
    NewRelease,
    #[clap(version = "1.0", author = "Wendell Liu <cuk.bas@gmail.com>")]
    Recommendation,
}

#[derive(Clap)]
struct NewRelease {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts: Opts = Opts::parse();

    let system_config_instance = SystemConfig::new(opts.config);
    SYSTEM_CONFIG.set(system_config_instance).unwrap();

    match opts.subcmd {
        SubCommand::NewRelease => {
            crawl_new_release::run().await?;
        }
        _ => {}
    }

    Ok(())
}
