use std::path::PathBuf;

use clap::{crate_authors, crate_version, AppSettings, Clap};

/// Given a git repository that follows conventional-commits convention,
/// `autorel` parses the commit messages since the last version tag to decide if there is something to release.
///
/// If there is indeed something to release, it performs the following steps:
///
/// 1. Compute next version number (according to the semantic versioning rules)
///
/// 2. Run user-defined verification commands (see configuration file)
///
/// 3. Update changelog file (can be disabled)
///
/// 4. Run user-defined preparation commands (see configuration file)
///
/// 5. Commit user-defined files (and the changelog if generated)
///
/// 6. Create new git tag
///
/// 7. Run user-defined publication commands (see configuration file)
///
/// 8. Push git commits (if any), and the new tag
///
///
/// Any failure in one of these steps will abort the release process.
///
/// This tools also expects to find a non-empty configuration file ('release.yml' by default) that defines
/// command-lines that should run as part of the release process.
/// See: https://github.com/jcornaz/autorel#Configuration
///
/// Conventional commit convention: https://www.conventionalcommits.org
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
