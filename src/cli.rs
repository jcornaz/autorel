use std::path::PathBuf;

use clap::{crate_authors, crate_version, AppSettings, Clap};

/// `autorel` parses the commit messages since the last version tag to decide if there is something to release.
///
/// It requires running from a git repository that follows the conventional-commits convention.
/// See: https://www.conventionalcommits.org
///
/// This tools also expects to find a non-empty configuration file ('release.yml' by default) that defines
/// command-lines that should run as part of the release process.
/// See: https://github.com/jcornaz/autorel#Configuration
///
/// If there is something to release (according to the commits found since last release), it performs the following steps:
///
/// 1. Compute next version number (according to the semantic versioning rules)
///
/// 2. Runs user-defined verification command
///
/// 3. Generate a changelog (can be disabled)
///
/// 4. Run user-defined preparation command
///
/// 5. Commit changes made during the preparation (and the changelog if generated)
///
/// 6. Run user-defined publication command
///
/// 7. Push git commits
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
