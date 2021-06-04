use clap::{AppSettings, Clap};

#[derive(Clap)]
#[clap(version = "1.0", author = "BlankZhu")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
    #[clap(short, long, default_value = "config.yaml")]
    pub config: String,
}
