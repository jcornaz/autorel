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
/// 5. Update git repository (commit user-defined files, tag and push)
///
/// 6. Run user-defined publication commands (see configuration file)
///
/// 7. Create a github release (only if configured)
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

    /// Force to proceed with the release, even if no previous version was found in the tags
    #[clap(long)]
    pub force: bool,

    /// Ensure to release a stable version number (>= 1.0.0)
    #[clap(long)]
    pub stable: bool,
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

    #[test]
    fn force_is_false_by_default() {
        let opts = Opts::try_parse_from(vec!["autorel"]).expect("Failed to parse command line");

        assert!(!opts.force);
    }

    #[test]
    fn force_flag_can_be_used() {
        let opts =
            Opts::try_parse_from(vec!["autorel", "--force"]).expect("Failed to parse command line");

        assert!(opts.force);
    }

    #[test]
    fn stable_is_false_by_default() {
        let opts = Opts::try_parse_from(vec!["autorel"]).expect("Failed to parse command line");

        assert!(!opts.stable);
    }

    #[test]
    fn stable_flag_can_be_used() {
        let opts = Opts::try_parse_from(vec!["autorel", "--stable"])
            .expect("Failed to parse command line");

        assert!(opts.stable);
    }
}
