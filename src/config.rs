use std::fmt::{self, Debug, Display, Formatter};
use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use serde_derive::Deserialize;

pub fn read(path: PathBuf) -> Result<Config, Error> {
    do_read(&path).map_err(|cause| Error { path, cause })
}

fn do_read(path: &Path) -> Result<Config, Cause> {
    parse(File::open(path)?)
}

fn parse(data: impl Read) -> Result<Config, Cause> {
    serde_yaml::from_reader(data).map_err(|err| Cause::InvalidConfig(Box::new(err)))
}

#[derive(Debug, Eq, PartialEq, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub hooks: Hooks,
}

#[derive(Debug, Eq, PartialEq, Deserialize, Default)]
pub struct Hooks {
    #[serde(default)]
    verify: Vec<String>,
    #[serde(default)]
    prepare: Vec<String>,
    #[serde(default)]
    publish: Vec<String>,
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
}
