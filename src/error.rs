use crate::cvs;
use std::fmt::{Display, Formatter};
use std::{fmt, io};

#[derive(Debug)]
enum Error {
    InvalidConfigFile(toml::de::Error),
    CvsError(cvs::Error),
    CannotRunCmd(String, io::Error),
    CmdFailure(String),
}

impl From<cvs::Error> for Error {
    fn from(err: cvs::Error) -> Self {
        Self::CvsError(err)
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Self::InvalidConfigFile(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidConfigFile(err) => write!(f, "Invalid config file: {}", err),
            Error::CvsError(err) => write!(f, "Cannot use version control: {}", err),
            Error::CannotRunCmd(cmd, err) => write!(f, "Cannot run command '{}': {}", cmd, err),
            Error::CmdFailure(cmd) => write!(f, "The command '{}' failed", cmd),
        }
    }
}

impl std::error::Error for Error {}
