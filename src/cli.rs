use std::path::PathBuf;

use clap::{crate_authors, crate_version, AppSettings, Clap};

/// Software release automation.
#[derive(Clap)]
#[clap(version = crate_version!(), author = crate_authors!())]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
    /// Only print in the terminal what would happen, without actually doing anything.
    #[clap(long)]
    pub dry_run: bool,

    /// Path of the configuration file
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
    fn not_dry_run_by_default() {
        let opts = Opts::try_parse_from(vec!["autorel"]).expect("Failed to parse command line");
        assert!(!opts.dry_run)
    }

    #[test]
    fn dry_run() {
        let opts = Opts::try_parse_from(vec!["autorel", "--dry-run"])
            .expect("Failed to parse command line");
        assert!(opts.dry_run)
    }

    #[test]
    fn with_config() {
        let opts = Opts::try_parse_from(vec!["autorel", "--config", "MyConfigFile.toml"])
            .expect("Failed to parse command line");

        assert_eq!(opts.config, PathBuf::from("MyConfigFile.toml"));
    }
}
