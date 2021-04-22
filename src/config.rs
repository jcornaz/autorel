use std::fmt::{Debug, Display, Formatter};
use std::path::{Path, PathBuf};
use std::{fmt, fs, io};

use serde_derive::Deserialize;

pub fn read(path: PathBuf) -> Result<Config, Error> {
    do_read(&path).map_err(|cause| Error { path, cause })
}

fn do_read(path: impl AsRef<Path>) -> Result<Config, Cause> {
    Ok(toml::from_slice(&fs::read(&path)?)?)
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
    InvalidConfig(toml::de::Error),
}

impl From<io::Error> for Cause {
    fn from(err: io::Error) -> Self {
        Cause::CannotReadFile(err)
    }
}

impl From<toml::de::Error> for Cause {
    fn from(err: toml::de::Error) -> Self {
        Cause::InvalidConfig(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_can_be_empty() {
        assert_eq!(toml::from_str::<Config>(""), Ok(Config::default()));
    }

    #[test]
    fn parse_verify_hook() {
        let config: Config = toml::from_str(
            r#"
            [hooks]
            verify = "myscript.sh"
        "#,
        )
        .expect("Failed to parse config");

        assert_eq!(config.hooks.verify, Some(String::from("myscript.sh")))
    }

    #[test]
    fn parse_prepare_hook() {
        let config: Config = toml::from_str(
            r#"
            [hooks]
            prepare = "myscript.sh"
        "#,
        )
        .expect("Failed to parse config");

        assert_eq!(config.hooks.prepare, Some(String::from("myscript.sh")))
    }

    #[test]
    fn parse_publish_hook() {
        let config: Config = toml::from_str(
            r#"
            [hooks]
            publish = "myscript.sh"
        "#,
        )
        .expect("Failed to parse config");

        assert_eq!(config.hooks.publish, Some(String::from("myscript.sh")))
    }
}
