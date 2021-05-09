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

#[derive(Debug, Eq, PartialEq, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub hooks: Hooks,
}

#[derive(Debug, Eq, PartialEq, Deserialize, Default)]
pub struct Hooks {
    verify: Option<String>,
    prepare: Option<String>,
    publish: Option<String>,
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
                verify: myscript.sh
        "#
            .as_bytes(),
        )
        .expect("Failed to parse config");

        assert_eq!(config.hooks.verify, Some(String::from("myscript.sh")))
    }

    #[test]
    fn parse_prepare_hook() {
        let config: Config = parse(
            r#"
            hooks:
                prepare: myscript.sh
        "#
            .as_bytes(),
        )
        .expect("Failed to parse config");

        assert_eq!(config.hooks.prepare, Some(String::from("myscript.sh")))
    }

    #[test]
    fn parse_publish_hook() {
        let config: Config = parse(
            r#"
            hooks:
                publish: myscript.sh
        "#
            .as_bytes(),
        )
        .expect("Failed to parse config");

        assert_eq!(config.hooks.publish, Some(String::from("myscript.sh")))
    }
}
