use clap::{crate_authors, crate_version, AppSettings, Clap};
use std::path::PathBuf;

#[derive(Clap)]
#[clap(version = crate_version!(), author = crate_authors!(), about = "\
Software release automation.

Runs the scripts `.release/verify.sh`, `.release/prepare.sh` and `.release/publish.sh` if they exist.
")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
    #[clap(long, default_value = "release.yml")]
    pub config: PathBuf,
}

pub fn parse() -> Opts {
    Clap::parse()
}

#[cfg(test)]
mod tests {
    use super::Clap;
    use super::*;

    #[test]
    fn can_run_without_arguments() {
        assert!(Opts::try_parse_from(vec!["autorel"]).is_ok());
    }

    #[test]
    fn default_config() {
        let opts = Opts::try_parse_from(vec!["autorel"]).expect("Failed to parse command line");

        assert_eq!(opts.config, PathBuf::from("release.yml"));
    }

    #[test]
    fn with_config() {
        let opts = Opts::try_parse_from(vec!["autorel", "--config", "MyConfigFile.toml"])
            .expect("Failed to parse command line");

        assert_eq!(opts.config, PathBuf::from("MyConfigFile.toml"));
    }
}
