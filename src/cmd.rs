use clap::{AppSettings, Clap};

#[derive(Clap, Debug)]
#[clap(version = "1.0", author = "Wendell Liu <cuk.bas@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
    /// file path of config
    #[clap(short, long, default_value = "config.yml")]
    pub config: String,
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Clap, Debug)]
pub enum SubCommand {
    #[clap(version = "1.0", author = "Wendell Liu <cuk.bas@gmail.com>")]
    NewRelease,
    #[clap(version = "1.0", author = "Wendell Liu <cuk.bas@gmail.com>")]
    ListGenreSeeds,
}
