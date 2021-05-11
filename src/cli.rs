use std::path::PathBuf;

use clap::{crate_authors, crate_version, AppSettings, Clap};

/// `autorel` parses tag and commit messages of the commits since the last release to decide if there is something to
/// release.
///
/// If there is indeed something to release, it infers the next version number
/// (according to the semantic versioning rules)
/// and invoke the hooks defined in the configuration file (`release.yml` by default)
///
/// For the reference of the configuration file see:
/// https://github.com/jcornaz/autorel#Configuration
///
/// By default it'll also generate a changelog using `clog`.
/// To customize the changelog generation see: https://github.com/clog-tool/clog-lib/tree/0.9.0#default-options
///
/// This utility must run from the root of a git repository that follows the conventional-commits convention.
/// See: https://www.conventionalcommits.org
#[derive(Clap)]
#[clap(version = crate_version!(), author = crate_authors!())]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
    /// Only prints what would be done if the this flag wasn't specified.
    /// Without actually doing anything.
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
