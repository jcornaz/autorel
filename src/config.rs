use std::fmt::{self, Debug, Display, Formatter};
use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use crate::bump::PreReleaseLabel;
use serde_derive::Deserialize;

pub fn read(path: &Path) -> Result<Config, Error> {
    do_read(&path).map_err(|cause| Error {
        path: path.into(),
        cause,
    })
}

fn do_read(path: &Path) -> Result<Config, Cause> {
    parse(File::open(path)?)
}

fn parse(data: impl Read) -> Result<Config, Cause> {
    let mut result: Config =
        serde_yaml::from_reader(data).map_err(|err| Cause::InvalidConfig(Box::new(err)))?;

    if result.changelog {
        result.commit.files.push(PathBuf::from("CHANGELOG.md"));
    }

    Ok(result)
}

#[derive(Debug, Eq, PartialEq, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub hooks: Hooks,

    #[serde(default = "Config::default_changelog")]
    pub changelog: bool,

    #[serde(default = "Config::default_tag_prefix")]
    pub tag_prefix: String,

    #[serde(default)]
    pub github: Option<GithubConfig>,

    #[serde(default)]
    pub commit: CommitConfig,

    #[serde(default)]
    pub pre_release: Option<PreReleaseLabel>,
}

impl Config {
    #[inline]
    fn default_changelog() -> bool {
        true
    }

    #[inline]
    fn default_tag_prefix() -> String {
        String::from("v")
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Default)]
pub struct GithubConfig {
    pub repo: String,

    #[serde(default)]
    pub files: Vec<PathBuf>,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
pub struct CommitConfig {
    #[serde(default)]
    pub files: Vec<PathBuf>,

    #[serde(default = "CommitConfig::default_message")]
    pub message: String,
}

impl CommitConfig {
    #[inline]
    fn default_message() -> String {
        String::from("chore: release {{version}}")
    }
}

impl Default for CommitConfig {
    fn default() -> Self {
        Self {
            files: Vec::default(),
            message: Self::default_message(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Deserialize, Default)]
pub struct Hooks {
    #[serde(default)]
    pub verify: Vec<String>,
    #[serde(default)]
    pub prepare: Vec<String>,
    #[serde(default)]
    pub publish: Vec<String>,
}

#[derive(Debug)]
pub struct Error {
    path: PathBuf,
    cause: Cause,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.cause {
            Cause::CannotReadFile(err) => {
                write!(f, "Cannot read '{}': {}", self.path.display(), err)
            }
            Cause::InvalidConfig(err) => write!(
                f,
                "Invalid configuration ({}): {}",
                self.path.display(),
                err
            ),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug)]
enum Cause {
    CannotReadFile(io::Error),
    InvalidConfig(Box<dyn std::error::Error>),
}

impl From<io::Error> for Cause {
    fn from(err: io::Error) -> Self {
        Cause::CannotReadFile(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_verify_hook() {
        let config: Config = parse(
            r#"
            hooks:
              verify:
                - script1.sh
                - script2.sh
        "#
            .as_bytes(),
        )
        .expect("Failed to parse config");

        assert_eq!(
            config.hooks.verify,
            vec![String::from("script1.sh"), String::from("script2.sh")]
        )
    }

    #[test]
    fn parse_prepare_hook() {
        let config: Config = parse(
            r#"
            hooks:
              prepare:
                - script3.sh
                - script4.sh
        "#
            .as_bytes(),
        )
        .expect("Failed to parse config");

        assert_eq!(
            config.hooks.prepare,
            vec![String::from("script3.sh"), String::from("script4.sh")]
        )
    }

    #[test]
    fn parse_publish_hook() {
        let config: Config = parse(
            r#"
            hooks:
              publish:
                - script5.sh
                - script6.sh
        "#
            .as_bytes(),
        )
        .expect("Failed to parse config");

        assert_eq!(
            config.hooks.publish,
            vec![String::from("script5.sh"), String::from("script6.sh")]
        )
    }

    #[test]
    fn changelog_generation_is_on_by_default() {
        let config: Config = parse("a: b".as_bytes()).expect("Failed to parse config");

        assert!(config.changelog)
    }

    #[test]
    fn changelog_generation_can_be_turned_off() {
        let config: Config = parse(
            r#"
            changelog: false
            "#
            .as_bytes(),
        )
        .expect("Failed to parse config");

        assert!(!config.changelog)
    }

    #[test]
    fn no_github_repo_by_default() {
        let config: Config = parse(r"a: b".as_bytes()).expect("Failed to parse config");

        assert!(config.github.is_none())
    }

    #[test]
    fn github_repo_can_be_defined() {
        let config: Config = parse(
            r"
            github:
                repo: jcornaz/autorel
            "
            .as_bytes(),
        )
        .expect("Failed to parse config");

        assert_eq!(
            config.github.map(|it| it.repo),
            Some(String::from("jcornaz/autorel"))
        )
    }

    #[test]
    fn can_define_files_to_upload_to_github_release() {
        let config: Config = parse(
            r"
            github:
                repo: jcornaz/autorel
                files:
                    - file.txt
            "
            .as_bytes(),
        )
        .expect("Failed to parse config");

        assert_eq!(
            config.github.map(|it| it.files),
            Some(vec![PathBuf::from("file.txt")])
        )
    }

    #[test]
    fn default_commit_message() {
        let config: Config = parse(r"a: b".as_bytes()).expect("Failed to parse config");

        assert_eq!(
            config.commit.message,
            String::from("chore: release {{version}}")
        )
    }

    #[test]
    fn commit_changelog_by_default() {
        let config: Config = parse(r"a: b".as_bytes()).expect("Failed to parse config");

        assert_eq!(config.commit.files, vec![PathBuf::from("CHANGELOG.md")])
    }

    #[test]
    fn commits_nothing_by_default_if_changelog_is_disabled() {
        let config: Config = parse(r"changelog: false".as_bytes()).expect("Failed to parse config");

        assert!(config.commit.files.is_empty())
    }

    #[test]
    fn can_define_files_to_commit() {
        let config: Config = parse(
            r"
        changelog: false
        commit:
            files:
                - CHANGELOG.md
                - README.md
        "
            .as_bytes(),
        )
        .expect("Failed to parse config");

        assert_eq!(
            config.commit.files,
            vec![PathBuf::from("CHANGELOG.md"), PathBuf::from("README.md")]
        )
    }

    #[test]
    fn can_configure_pre_release() {
        let config: Config = parse("pre_release: beta".as_bytes()).expect("Failed to parse config");

        assert_eq!(config.pre_release, Some("beta".parse().unwrap()))
    }
}
