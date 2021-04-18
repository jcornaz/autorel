use clap::{crate_authors, crate_version, AppSettings, Clap};

#[derive(Clap)]
#[clap(version = crate_version!(), author = crate_authors!(), about = "\
Software release automation.

Runs the scripts `.release/verify.sh`, `.release/prepare.sh` and `.release/publish.sh` if they exist.
")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts;

pub fn parse() -> Opts {
    Clap::parse()
}
